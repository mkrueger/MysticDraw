use crate::model::{Buffer, TextAttribute, Position};

const FG_TABLE: [&[u8;2];8] = [ b"30", b"34", b"32", b"36", b"31", b"35", b"33", b"37" ];
const BG_TABLE: [&[u8;2];8] = [ b"40", b"44", b"42", b"46", b"41", b"45", b"43", b"47" ];

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
    use crate::model::{Buffer};

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