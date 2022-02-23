use crate::model::{DEFAULT_ATTRIBUTE, Position};

use super::LoadData;

const ANSI_CSI: u8 = b'[';
const ANSI_ESC: u8 = 27;

const COLOR_OFFSETS : [u8; 8] = [ 0, 4, 2, 6, 1, 5, 3, 7 ];

pub fn display_ansi(data: &mut LoadData, ch: u8) -> u8 {
    if data.ans_esc {
        if ch == ANSI_CSI {
            data.ans_esc = false;
            data.ans_code = true;
            data.ans_numbers.clear();
            return 0;
        } else {
            // ignore all other ANSI escape codes
            data.ans_esc = false;
            return 0;
        }
    }

    if data.ans_code {
        match ch {
            b'm' => { // Select Graphic Rendition 
                for n in &data.ans_numbers {
                    match n {
                        0 => data.text_attr = DEFAULT_ATTRIBUTE, // Reset or normal 
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
                return 0;
            }
            b'H' | b'f' => { // Cursor Position + Horizontal Vertical Position ('f')
                if !data.ans_numbers.is_empty() {
                    if data.ans_numbers[0] > 0 { 
                        data.cur_pos.y = data.ans_numbers[0] - 1;
                    }
                    if data.ans_numbers.len() > 1 {
                        if data.ans_numbers[1] > 0 {
                            data.cur_pos.x = data.ans_numbers[1] - 1;
                        }
                    } else {
                        data.cur_pos.x = 0;
                    }
                }
                data.ans_code = false;
                return 0;
            }
            b'C' => { // Cursor Forward 
                if data.ans_numbers.is_empty() {
                    data.cur_pos.x += 1;
                } else {
                    data.cur_pos.x += data.ans_numbers[0];
                }
                data.ans_code = false;
                return 0;
            }
            b'D' => { // Cursor Back 
                if data.ans_numbers.is_empty() {
                    data.cur_pos.x = data.cur_pos.x.saturating_sub(1);
                } else {
                    data.cur_pos.x = data.cur_pos.x.saturating_sub(data.ans_numbers[0]);
                }
                data.ans_code = false;
                return 0;
            }
            b'A' => { // Cursor Up 
                if data.ans_numbers.is_empty() {
                    data.cur_pos.y = data.cur_pos.y.saturating_sub(1);
                } else {
                    data.cur_pos.y =
                        data.cur_pos.y.saturating_sub(data.ans_numbers[0]);
                }
                data.ans_code = false;
                return 0;
            }
            b'B' => { // Cursor Down 
                if data.ans_numbers.is_empty() {
                    data.cur_pos.y += 1;
                } else {
                    data.cur_pos.y += data.ans_numbers[0];
                }
                data.ans_code = false;
                return 0;
            }
            b's' => { // Save Current Cursor Position
                data.saved_pos = data.cur_pos;
                data.ans_code = false;
                return 0;
            }
            b'u' => { // Restore Saved Cursor Position 
                data.cur_pos = data.saved_pos;
                data.ans_code = false;
                return 0;
            }
            b'J' => { // Erase in Display 
                data.ans_code = false;
                if data.ans_numbers.is_empty() {
                    data.cur_pos = Position::new();
                } else {
                    match data.ans_numbers[0] {
                        0 => {} // TODO: clear from cursor to the end of the screen 
                        2 => {  // clear entire screen
                            data.cur_pos = Position::new();
                            // TODO: Clear
                        } 
                        3 => { // TODO: clear entire screen and delete all lines saved in the scrollback buffer
                            data.cur_pos = Position::new();
                        }
                        _ => {eprintln!("unknown ANSI J sequence {}", data.ans_numbers[0])}
                    }
                }
                return 0;
            }
            _ => {
                if (0x40..=0x7E).contains(&ch) {
                    // unknown control sequence, terminate reading
                    data.ans_code = false;
                    data.ans_esc = false;
                    eprintln!("unknown control sequence, terminating.");
                    return 0;
                }

                if (b'0'..=b'9').contains(&ch) {
                    if data.ans_numbers.is_empty() {
                        data.ans_numbers.push(0);
                    }
                    let d = data.ans_numbers.pop().unwrap();
                    data.ans_numbers.push(d * 10 + (ch - b'0') as i32);
                } else if ch == b';' {
                    data.ans_numbers.push(0);
                    return 0;
                } else {
                    // error in control sequence, terminate reading
                    eprintln!(
                        "error in ANSI control sequence: {:?}!",
                        char::from_u32(ch as u32)
                    );
                    data.ans_code = false;
                    data.ans_esc = false;
                }
                return 0;
            }
        }
    }

    if ch == ANSI_ESC {
        data.ans_code = false;
        data.ans_esc = true;
        0
    } else {
        ch
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::model::Buffer;

    #[test]
    fn test_ansi_sequence() {
        let buf = Buffer::from_bytes(PathBuf::from("test"), b"[0;40;37mFoo-[1mB[0ma[35mr");
       
       assert_eq!(1, buf.height);
       assert_eq!(7, buf.width); // 'Foo-Bar'
        
       let line = &buf.base_layer.lines[0];
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
        let buf = Buffer::from_bytes(PathBuf::from("test"), b"[1;35mA[30mB[0mC");
       
       let line = &buf.base_layer.lines[0];
       assert_eq!(b'A', line.chars[0].char_code);
       assert_eq!(13, line.chars[0].attribute.as_u8());
       assert_eq!(b'B', line.chars[1].char_code);
       assert_eq!(8, line.chars[1].attribute.as_u8());
       assert_eq!(b'C', line.chars[2].char_code);
       assert_eq!(7, line.chars[2].attribute.as_u8());
    }

    #[test]
    fn test_bg_colorrsequence() {
        let buf = Buffer::from_bytes(std::path::PathBuf::from("test"), b"[1;30m1[0;34m2[33m3[1;41m4[40m5[43m6[40m7");
       
       let line = &buf.base_layer.lines[0];
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
        let buf = Buffer::from_bytes(std::path::PathBuf::from("test"), b"[40m[2J[2C[0;34mX   [1;37mX   [0;34mX[31mX[33mXXX[s
[u[31mXXXXXXXXXXX[33mX[31mX[33mX[31mX[33mXXX[31mX[s
[uXXXXX[33mX[31mX[33mXXX[31mXXXXXXXXXXXXXXX[33mX[s
[u[31mXXXX[33mX[31mX[33mXX[31mXXX[33mX[31mX[34mX[s
[u   X[2;1H  XXXX[1;37mX[0;34mXX[31mX[33mX[34mX[s");
       
       assert_eq!(2, buf.height);
    }
}

