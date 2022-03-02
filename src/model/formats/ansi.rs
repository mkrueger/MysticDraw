use std::cmp::{max, min};

use crate::model::{Buffer, Position};
use crate::model::TextAttribute;

use super::ParseStates;

const ANSI_CSI: u8 = b'[';
const ANSI_ESC: u8 = 27;

const COLOR_OFFSETS : [u8; 8] = [ 0, 4, 2, 6, 1, 5, 3, 7 ];
const FG_TABLE: [&[u8;2];8] = [ b"30", b"34", b"32", b"36", b"31", b"35", b"33", b"37" ];
const BG_TABLE: [&[u8;2];8] = [ b"40", b"44", b"42", b"46", b"41", b"45", b"43", b"47" ];

pub fn display_ans(data: &mut ParseStates, ch: u8) -> Option<u8> {
    if data.ans_esc {
        if ch == ANSI_CSI {
            data.ans_esc = false;
            data.ans_code = true;
            data.ans_numbers.clear();
            return None;
        }
        // ignore all other ANSI escape codes
        data.ans_esc = false;
        return None;
    }

    if data.ans_code {
        match ch {
            b'm' => { // Select Graphic Rendition 
                for n in &data.ans_numbers {
                    match n {
                        0 => data.text_attr = TextAttribute::DEFAULT, // Reset or normal 
                        1 => data.text_attr.set_bold(true),      // Bold or increased intensity 
                        5 => data.text_attr.set_blink(true),                                 // Slow blink 
                        // set foreaground color
                        30..=37 => data.text_attr.set_foreground_without_bold(COLOR_OFFSETS[*n as usize - 30]),
                        // set background color
                        40..=47 => data.text_attr.set_background(COLOR_OFFSETS[*n as usize - 40]),
                        _ => { eprintln!("Unsupported ANSI graphic code {}", n); }
                    }
                }
                data.ans_code = false;
                return None;
            }
            b'H' | b'f' => { // Cursor Position + Horizontal Vertical Position ('f')
                if !data.ans_numbers.is_empty() {
                    if data.ans_numbers[0] > 0 { 
                        data.cur_pos.y =  max(0, data.ans_numbers[0] - 1);
                    }
                    if data.ans_numbers.len() > 1 {
                        if data.ans_numbers[1] > 0 {
                            data.cur_pos.x =  max(0, data.ans_numbers[1] - 1);
                        }
                    } else {
                        data.cur_pos.x = 0;
                    }
                }
                data.ans_code = false;
                return None;
            }
            b'C' => { // Cursor Forward 
                if data.ans_numbers.is_empty() {
                    data.cur_pos.x += 1;
                } else {
                    data.cur_pos.x += data.ans_numbers[0];
                }
                data.cur_pos.x = min(data.screen_width - 1, data.cur_pos.x);
                data.ans_code = false;
                return None;
            }
            b'D' => { // Cursor Back 
                if data.ans_numbers.is_empty() {
                    data.cur_pos.x = max(0, data.cur_pos.x - 1);
                } else {
                    data.cur_pos.x =  max(0, data.cur_pos.x.saturating_sub(data.ans_numbers[0]));
                }
                data.cur_pos.x = max(0, data.cur_pos.x);
                data.ans_code = false;
                return None;
            }
            b'A' => { // Cursor Up 
                if data.ans_numbers.is_empty() {
                    data.cur_pos.y =  max(0, data.cur_pos.y - 1);
                } else {
                    data.cur_pos.y = max(0, data.cur_pos.y.saturating_sub(data.ans_numbers[0]));
                }
                data.cur_pos.y = max(0, data.cur_pos.y);
                data.ans_code = false;
                return None;
            }
            b'B' => { // Cursor Down 
                if data.ans_numbers.is_empty() {
                    data.cur_pos.y += 1;
                } else {
                    data.cur_pos.y += data.ans_numbers[0];
                }
                data.ans_code = false;
                return None;
            }
            b's' => { // Save Current Cursor Position
                data.saved_pos = data.cur_pos;
                data.ans_code = false;
                return None;
            }
            b'u' => { // Restore Saved Cursor Position 
                data.cur_pos = data.saved_pos;
                data.ans_code = false;
                return None;
            }
            b'J' => { // Erase in Display 
                data.ans_code = false;
                if data.ans_numbers.is_empty() {
                    data.cur_pos = Position::new();
                } else {
                    match data.ans_numbers.get(0).unwrap() {
                        0 => {} // TODO: clear from cursor to the end of the screen 
                        2 |  // clear entire screen
                        3 // TODO: clear entire screen and delete all lines saved in the scrollback buffer
                        => {
                            data.cur_pos = Position::new();
                            // TODO: Clear
                        } 
                        _ => {eprintln!("unknown ANSI J sequence {}", data.ans_numbers[0])}
                    }
                }
                return None;
            }
            _ => {
                if (0x40..=0x7E).contains(&ch) {
                    // unknown control sequence, terminate reading
                    data.ans_code = false;
                    data.ans_esc = false;
                    eprintln!("unknown control sequence, terminating.");
                    return None;
                }

                if (b'0'..=b'9').contains(&ch) {
                    if data.ans_numbers.is_empty() {
                        data.ans_numbers.push(0);
                    }
                    let d = data.ans_numbers.pop().unwrap();
                    data.ans_numbers.push(d * 10 + (ch - b'0') as i32);
                } else if ch == b';' {
                    data.ans_numbers.push(0);
                    return None;
                } else {
                    // error in control sequence, terminate reading
                    eprintln!(
                        "error in ANSI control sequence: {:?}!",
                        char::from_u32(ch as u32)
                    );
                    data.ans_code = false;
                    data.ans_esc = false;
                }
                return None;
            }
        }
    }

    if ch == ANSI_ESC {
        data.ans_code = false;
        data.ans_esc = true;
        None
    } else {
        Some(ch)
    }
}

pub fn convert_to_ans(buf: &Buffer) -> Vec<u8>
{
    let mut result = Vec::new();
    let mut last_attr = TextAttribute::DEFAULT;
    let mut pos = Position::new();
    let height = buf.height as i32;
    let mut first_char = true;
    let mut last_line_skipped = false;

    while pos.y < height {
        let line_length = buf.get_line_length(pos.y);
        if line_length == 0 && last_line_skipped {
            result.push(13);
            result.push(10);
        }
        while pos.x < line_length {
            let mut space_count = 0;
            let mut ch = buf.get_char(pos);
            let mut cur_attr = ch.attribute;

            while (ch.char_code == b' ' || ch.char_code == 0) && last_attr.get_background() == cur_attr.get_background() && pos.x < line_length {
                space_count += 1;
                pos.x += 1;                     
                ch = buf.get_char(pos);
            }

            // optimize color output for empty space lines.
            if space_count > 0 && cur_attr.get_background() == ch.attribute.get_background() {
                cur_attr = ch.attribute;
            }

            if last_attr != cur_attr || first_char {
                result.extend_from_slice(b"\x1b[");
                let mut wrote_part = false;

                // handle bold change
                if (!last_attr.is_bold() || first_char) && cur_attr.is_bold() {
                    // if blinking is turned off "0;" will be written which would reset the bold state here
                    // bold state is set again after blink reset.
                    if (!last_attr.is_blink() && !first_char) || cur_attr.is_blink() {
                        result.push(b'1');
                        wrote_part = true;
                    }
                } else if (last_attr.is_bold() || first_char) && !cur_attr.is_bold()  {
                    result.push(b'0');
                    last_attr = TextAttribute::DEFAULT;
                    first_char = false; // attribute set.
                    wrote_part = true;
                }

                // handle blink change
                if (!last_attr.is_blink() || first_char) && cur_attr.is_blink()  {
                    if wrote_part {
                        result.push(b';');
                    }
                    result.push(b'5');
                    wrote_part = true;
                } else if (last_attr.is_blink() || first_char) && !cur_attr.is_blink()  {
                    if wrote_part {
                        result.push(b';');
                    }
                    result.push(b'0');
                    if cur_attr.is_bold() || first_char {
                        result.extend_from_slice(b";1");
                    }
                    last_attr = TextAttribute::DEFAULT;
                    wrote_part = true;
                }

                // color changes
                if last_attr.get_foreground_without_bold() != cur_attr.get_foreground_without_bold() {
                    if wrote_part {
                        result.push(b';');
                    }
                    result.extend_from_slice(FG_TABLE[cur_attr.get_foreground_without_bold() as usize]);
                    wrote_part = true;
                }
                if last_attr.get_background() != cur_attr.get_background() {
                    if wrote_part {
                        result.push(b';');
                        print!(";");
                    }
                    result.extend_from_slice(BG_TABLE[cur_attr.get_background() as usize]);
                }
                result.push(b'm');
                last_attr = cur_attr;
            }

            first_char = false;
            
            if space_count > 0 {
                if space_count < 5 {
                    result.resize(result.len() + space_count, b' ');
                } else {
                    result.extend_from_slice(b"\x1b[");
                    push_int(&mut result, space_count);
                    result.push(b'C');
                }
                continue;
            }
            
            result.push(if ch.char_code == 0 { b' ' } else { ch.char_code });
            pos.x += 1;
        }
        pos.y += 1;

        // do not end with eol
        last_line_skipped = pos.y >= height || pos.x >= buf.width as i32;
        if !last_line_skipped {
            result.push(13);
            result.push(10);
        }
        pos.x = 0;
    }
    result
}

fn push_int(result: &mut Vec<u8>, number: usize) 
{
    result.extend_from_slice(number.to_string().as_bytes());
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::model::{Buffer, Position};

    #[test]
    fn test_ansi_sequence() {
      let buf = Buffer::from_bytes(&PathBuf::from("test.ans"), &None, b"\x1B[0;40;37mFoo-\x1B[1mB\x1B[0ma\x1B[35mr");
       
       let line = &buf.layers[0].lines[0];
       assert_eq!(b'F', line.chars[0].char_code);
       assert_eq!(7, line.chars[0].attribute.as_u8());
       assert_eq!(b'o', line.chars[1].char_code);
       assert_eq!(7, line.chars[1].attribute.as_u8());
       assert_eq!(b'o', line.chars[2].char_code);
       assert_eq!(7, line.chars[2].attribute.as_u8());
       assert_eq!(b'-', line.chars[3].char_code);
       assert_eq!(7, line.chars[3].attribute.as_u8());
       assert_eq!(b'B', line.chars[4].char_code);
       assert_eq!(15, line.chars[4].attribute.as_u8());
       assert_eq!(b'a', line.chars[5].char_code);
       assert_eq!(7, line.chars[5].attribute.as_u8());
       assert_eq!(b'r', line.chars[6].char_code);
       assert_eq!(5, line.chars[6].attribute.as_u8());
    }

    #[test]
    fn test_ansi_30() {
    let buf = Buffer::from_bytes(&PathBuf::from("test.ans"), &None, b"\x1B[1;35mA\x1B[30mB\x1B[0mC");
       let line = &buf.layers[0].lines[0];
       assert_eq!(b'A', line.chars[0].char_code);
       assert_eq!(13, line.chars[0].attribute.as_u8());
       assert_eq!(b'B', line.chars[1].char_code);
       assert_eq!(8, line.chars[1].attribute.as_u8());
       assert_eq!(b'C', line.chars[2].char_code);
       assert_eq!(7, line.chars[2].attribute.as_u8());
    }

    #[test]
    fn test_bg_colorrsequence() {
        let buf = Buffer::from_bytes(&std::path::PathBuf::from("test.ans"), &None, b"\x1B[1;30m1\x1B[0;34m2\x1B[33m3\x1B[1;41m4\x1B[40m5\x1B[43m6\x1B[40m7");
       
       let line = &buf.layers[0].lines[0];
       assert_eq!(b'1', line.chars[0].char_code);
       assert_eq!(8, line.chars[0].attribute.as_u8());
       assert_eq!(b'2', line.chars[1].char_code);
       assert_eq!(1, line.chars[1].attribute.as_u8());
       assert_eq!(b'3', line.chars[2].char_code);
       assert_eq!(6, line.chars[2].attribute.as_u8());
       assert_eq!(b'4', line.chars[3].char_code);
       assert_eq!(14 + (4 << 4), line.chars[3].attribute.as_u8());
       assert_eq!(b'5', line.chars[4].char_code);
       assert_eq!(14, line.chars[4].attribute.as_u8());
       assert_eq!(b'6', line.chars[5].char_code);
       assert_eq!(14 + (6 << 4), line.chars[5].attribute.as_u8());
       assert_eq!(b'7', line.chars[6].char_code);
       assert_eq!(14, line.chars[6].attribute.as_u8());
    }

    #[test]
    fn test_linebreak_bug() {
        let buf = Buffer::from_bytes(&std::path::PathBuf::from("test.ans"), &None, b"XX");
       
        assert_eq!(0x16, buf.get_char(Position {x: 1, y: 0}).char_code);
    }

    #[test]
    fn test_char_missing_bug() {
        let buf = Buffer::from_bytes(&PathBuf::from("test.ans"), &None, b"\x1B[1;35mA\x1B[30mB\x1B[0mC");
       
       let line = &buf.layers[0].lines[0];
       assert_eq!(b'A', line.chars[0].char_code);
       assert_eq!(13, line.chars[0].attribute.as_u8());
       assert_eq!(b'B', line.chars[1].char_code);
       assert_eq!(8, line.chars[1].attribute.as_u8());
       assert_eq!(b'C', line.chars[2].char_code);
       assert_eq!(7, line.chars[2].attribute.as_u8());
    }

    #[test]
    fn test_cursor_forward() {
        let buf = Buffer::from_bytes(&PathBuf::from("test.ans"), &None, b"\x1B[70Ctest_me\x1B[20CF");
        let line = &buf.layers[0].lines[0];
        assert_eq!(b'F', line.chars[79].char_code);
 
    }
    
    #[test]
    fn test_cursor_forward_at_eol() {
        let buf = Buffer::from_bytes(&PathBuf::from("test.ans"), &None, b"\x1B[75CTEST_\x1B[2CF");
        let line = &buf.layers[0].lines[1];
        assert_eq!(b'F', line.chars[2].char_code);
    }

    #[test]
    fn test_char0_bug() {
        let buf = Buffer::from_bytes(&PathBuf::from("test.ans"), &None, b"\x00A");
        let line = &buf.layers[0].lines[0];
        assert_eq!(b'A', line.chars[1].char_code);
    }

    fn test_ansi(data: &[u8])
    {
        let buf = Buffer::from_bytes(&PathBuf::from("test.ans"), &None, data);
        let converted = super::convert_to_ans(&buf);

        // more gentle output.
        let b : Vec<u8> = converted.iter().map(|&x| if x == 27 { b'x' } else { x }).collect();
        let converted  = String::from_utf8_lossy(b.as_slice());

        let b : Vec<u8> = data.iter().map(|&x| if x == 27 { b'x' } else { x }).collect();
        let expected  = String::from_utf8_lossy(b.as_slice());

        assert_eq!(expected, converted);
    }

    #[test]
    fn test_space_compression() {
        let data = b"\x1B[0mA A  A   A    A\x1B[5CA\x1B[6CA\x1B[8CA";
        test_ansi(data);
    }

    #[test]
    fn test_fg_color_change() {
        let data = b"\x1B[0ma\x1B[32ma\x1B[33ma\x1B[1ma\x1B[35ma\x1B[0;35ma\x1B[1;32ma\x1B[0;36ma";
        test_ansi(data);
    }

    #[test]
    fn test_bg_color_change() {
        let data = b"\x1B[0mA\x1B[44mA\x1B[45mA\x1B[31;40mA\x1B[42mA\x1B[40mA\x1B[1;46mA\x1B[0mA\x1B[1;47mA\x1B[0;47mA";
        test_ansi(data);
    }

    #[test]
    fn test_blink_change() {
        let data = b"\x1B[0mA\x1B[5mA\x1B[0mA\x1B[1;5;42mA\x1B[0;1;42mA\x1B[0;5mA\x1B[0;36mA\x1B[5;33mA\x1B[0;1mA";
        test_ansi(data);
    }

    #[test]
    fn test_eol_skip() {
        let data = b"\x1B[0;1m\x1B[79Cdd";
        test_ansi(data);
    }

    #[test]
    fn test_first_char_color() {
        let data = b"\x1B[0;1;36mA";
        test_ansi(data);
        let data = b"\x1B[0;31mA";
        test_ansi(data);
        let data = b"\x1B[0;33;45mA";
        test_ansi(data);
        let data = b"\x1B[0;1;33;45mA";
        test_ansi(data);
    }

}