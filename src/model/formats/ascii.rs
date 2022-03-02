use crate::model::{Buffer, Position};

pub fn convert_to_asc(buf: &Buffer) -> Vec<u8>
{
    let mut result = Vec::new();
    let mut pos = Position::new();
    let height = buf.height as i32;
    let mut last_line_skipped = false;

    while pos.y < height {
        let line_length = buf.get_line_length(pos.y);
        if line_length == 0 && last_line_skipped {
            result.push(13);
            result.push(10);
        }

        while pos.x < line_length {
            let ch = buf.get_char(pos);
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::model::{Buffer};

    fn test_ascii(data: &[u8])
    {
        let buf = Buffer::from_bytes(&PathBuf::from("test.ans"), &None, data);
        let converted = super::convert_to_asc(&buf);

        // more gentle output.
        let b : Vec<u8> = converted.iter().map(|&x| if x == 27 { b'x' } else { x }).collect();
        let converted  = String::from_utf8_lossy(b.as_slice());

        let b : Vec<u8> = data.iter().map(|&x| if x == 27 { b'x' } else { x }).collect();
        let expected  = String::from_utf8_lossy(b.as_slice());

        assert_eq!(expected, converted);
    }

    #[test]
    fn test_ws_skip() {
        let data = b"123456789012345678901234567890123456789012345678901234567890123456789012345678902ndline";
        test_ascii(data);
    }

    #[test]
    fn test_ws_skip_empty_line() {
        let data = b"12345678901234567890123456789012345678901234567890123456789012345678901234567890\r\n\r\n2ndline";
        test_ascii(data);
    }
    
    #[test]
    fn test_eol_start() {
        let data = b"\r\n2ndline";
        test_ascii(data);
    }
}