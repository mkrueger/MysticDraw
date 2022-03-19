use std::io;

use crate::model::{Buffer, DosChar};
use super::{ Position, TextAttribute, SaveOptions};

pub fn read_binary(result: &mut Buffer, bytes: &[u8], file_size: usize) -> io::Result<bool>
{
    let mut o = 0;
    let mut pos = Position::new();
    loop {
        for _ in 0..result.width {
            if o >= file_size {
                result.set_height_for_pos(pos);
                return Ok(true);
            }

            if o + 1 > file_size {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "invalid file - needs to be % 2 == 0"));
            }

            result.set_char(0, pos, Some(DosChar {
                char_code: bytes[o] as u16,
                attribute: TextAttribute::from_u8(bytes[o + 1], result.buffer_type)
            }));
            pos.x += 1;
            o += 2;
        }
        pos.x = 0;
        pos.y += 1;
    }
}

pub fn convert_to_binary(buf: &Buffer, options: &SaveOptions) -> io::Result<Vec<u8>>
{
    let mut result = Vec::new();

    for y in 0..buf.height {
        for x in 0..buf.width {
            let ch = buf.get_char(Position::from(x as i32, y as i32)).unwrap_or_default();
            result.push(ch.char_code as u8);
            result.push(ch.attribute.as_u8());
        }
    }
    if options.save_sauce {
        buf.write_sauce_info(&crate::model::SauceFileType::Bin, &mut result)?;
    }
    Ok(result)
}

pub fn get_save_sauce_default_binary(buf: &Buffer) -> (bool, String)
{
    if buf.width != 160 {
        return (true, "width != 160".to_string() );
    }

    if buf.has_sauce_relevant_data() { return (true, String::new()); }

    ( false, String::new() )
}