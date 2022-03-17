use std::io;

use crate::model::{Buffer, Position};

use super::SaveOptions;

pub fn convert_to_asc(buf: &Buffer, options: &SaveOptions) -> io::Result<Vec<u8>>
{
    let mut result = Vec::new();
    let mut pos = Position::new();
    let height = buf.height as i32;

    while pos.y < height {
        let line_length = buf.get_line_length(pos.y);
        while pos.x < line_length {
            let ch = buf.get_char(pos).unwrap_or_default();
            result.push(if ch.char_code == 0 { b' ' } else { ch.char_code as u8});
            pos.x += 1;
        }

        // do not end with eol
        if pos.x < buf.width as i32 && pos.y + 1 < height {
            result.push(13);
            result.push(10);
        }

        pos.x = 0;
        pos.y += 1;
    }

    if options.save_sauce {
        buf.write_sauce_info(&crate::model::SauceFileType::Ascii, &mut result)?;
    }
    Ok(result)
}

pub fn get_save_sauce_default_asc(buf: &Buffer) -> (bool, String)
{
    if buf.width != 80 {
        return (true, "width != 80".to_string() );
    }

    if buf.has_sauce_relevant_data() { return (true, String::new()); }


    ( false, String::new() )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::model::{Buffer, SaveOptions};

    fn test_ascii(data: &[u8])
    {
        let buf = Buffer::from_bytes(&PathBuf::from("test.ans"), data).unwrap();
        let converted = super::convert_to_asc(&buf, &SaveOptions::new()).unwrap();

        // more gentle output.
        let b : Vec<u8> = converted.iter().map(|&x| if x == 27 { b'x' } else { x }).collect();
        let converted  = String::from_utf8_lossy(b.as_slice());

        let b : Vec<u8> = data.iter().map(|&x| if x == 27 { b'x' } else { x }).collect();
        let expected  = String::from_utf8_lossy(b.as_slice());

        assert_eq!(expected, converted);
    }

    #[test]
    fn test_full_line_height() {
        let mut vec = Vec::new();
        vec.resize(80, b'-');
        let buf = Buffer::from_bytes(&PathBuf::from("test.asc"), &vec).unwrap();
        assert_eq!(1, buf.height);
        vec.push(b'-');
        let buf = Buffer::from_bytes(&PathBuf::from("test.asc"), &vec).unwrap();
        assert_eq!(2, buf.height);
    }


    #[test]
    fn test_emptylastline_height() {
        let mut vec = Vec::new();
        vec.resize(80, b'-');
        vec.resize(80 * 2, b' ');
        let buf = Buffer::from_bytes(&PathBuf::from("test.asc"), &vec).unwrap();
        assert_eq!(2, buf.height);
    }


    #[test]
    fn test_emptylastline_roundtrip() {
        let mut vec = Vec::new();
        vec.resize(80, b'-');
        vec.resize(80 * 2, b' ');

        let buf = Buffer::from_bytes(&PathBuf::from("test.asc"), &vec).unwrap();
        assert_eq!(2, buf.height);
        let vec2 = buf.to_bytes("asc", &SaveOptions::new()).unwrap();
        let buf2 = Buffer::from_bytes(&PathBuf::from("test.asc"), &vec2).unwrap();
        assert_eq!(2, buf2.height);
    }


    #[test]
    fn test_eol() {
        let data = b"foo\r\n";
        let buf = Buffer::from_bytes(&PathBuf::from("test.asc"), data).unwrap();
        assert_eq!(2, buf.height);
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