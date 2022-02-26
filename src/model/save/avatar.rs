use crate::model::{Buffer, Position, TextAttribute};

pub fn convert_to_avt(buf: &Buffer) -> Vec<u8>
{
    let mut result = Vec::new();
    let mut last_attr = TextAttribute::DEFAULT;
    let mut pos = Position::new();
    let height = buf.height as i32;
    let mut last_line_skipped = false;
    let mut first_char = true;

    // TODO: implement repeat pattern compression (however even TheDraw never bothered to implement this cool RLE from fsc0037)
    while pos.y < height {
        let line_length = buf.get_line_length(pos.y);
        if line_length == 0 && last_line_skipped {
            result.push(13);
            result.push(10);
        }

        while pos.x < line_length {
            let mut repeat_count = 1;
            let mut ch = buf.get_char(pos);

            while pos.x < buf.width as i32 - 3 && ch == buf.get_char(pos + Position::from(1, 0)) {
                repeat_count += 1;
                pos.x += 1;                     
                ch = buf.get_char(pos);
            }

            if first_char || ch.attribute != last_attr {
                result.push(22);
                result.push(1);
                result.push(ch.attribute.as_u8());
                last_attr = ch.attribute;
            }
            first_char = false;

            if repeat_count > 1 {
                if repeat_count < 4 && (ch.char_code != 22 && ch.char_code != 12 && ch.char_code != 25) {
                    result.resize(result.len() + repeat_count, ch.char_code);
                } else {
                    result.push(25);
                    result.push(ch.char_code);
                    result.push(repeat_count as u8);
                }
                pos.x += 1;

                continue;
            }

            // avt control codes need to be represented as repeat once.
            if ch.char_code == 22 || ch.char_code == 12 || ch.char_code == 25 {
                result.push(25);
                result.push(ch.char_code);
                result.push(1);
            } else {
                result.push(if ch.char_code == 0 { b' ' } else { ch.char_code });
            }
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::model::{Buffer};

    fn output_avt(data: &[u8]) -> Vec<u8>
    {
        let mut result = Vec::new();
        let mut prev = 0;

        for d in data {
            match d {
                12 => result.extend_from_slice(b"^L"),
                25 => result.extend_from_slice(b"^Y"),
                22 => result.extend_from_slice(b"^V"),
                _ => {
                    if prev == 22 {
                        match d {
                            1 => result.extend_from_slice(b"<SET_COLOR>"),
                            2 => result.extend_from_slice(b"<BLINK_ON>"),
                            3 => result.extend_from_slice(b"<MOVE_UP>"),
                            4 => result.extend_from_slice(b"<MOVE_DOWN>"),
                            5 => result.extend_from_slice(b"<MOVE_RIGHT"),
                            6 => result.extend_from_slice(b"<MOVE_LEFT>"),
                            7 => result.extend_from_slice(b"<CLR_EOL>"),
                            8 => result.extend_from_slice(b"<GOTO_XY>"),
                            _ => result.extend_from_slice(b"<UNKNOWN_CMD>"),
                        }
                        prev = *d;
                        continue;
                    }

                    result.push(*d);
                }
            }
            prev = *d;
        }
        result
    }

    fn test_avt(data: &[u8])
    {
        let buf = Buffer::from_bytes(&PathBuf::from("test.avt"), &None, data);
        let converted = super::convert_to_avt(&buf);

        // more gentle output.
        let b : Vec<u8> = output_avt(&converted);
        let converted  = String::from_utf8_lossy(b.as_slice());

        let b : Vec<u8> = output_avt(data);
        let expected  = String::from_utf8_lossy(b.as_slice());

        assert_eq!(expected, converted);
    }

    #[test]
    fn test_char_compression() {
        let data = b"\x16\x01\x07A-A--A---A\x19-\x04A\x19-\x05A\x19-\x06A\x19-\x07A";
        test_avt(data);
    }
}