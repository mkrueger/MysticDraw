use crate::model::{Buffer, DosChar};
use super::{ Position, TextAttribute};

pub fn read_binary(result: &mut Buffer, bytes: &[u8], file_size: usize, screen_width: i32)
{
    let mut o = 0;
    let mut pos = Position::new();
    loop {
        for _ in 0..screen_width {
            if o + 2 > file_size {
                return;
            }
            result.set_char(pos, DosChar {
                char_code: bytes[o],
                attribute: TextAttribute::from_u8(bytes[o + 1])
            });
            pos.x += 1;
            o += 2;
        }
        pos.x = 0;
        pos.y += 1;
    }
}

pub fn convert_to_binary(buf: &Buffer) -> Vec<u8>
{
    let mut result = Vec::new();

    for y in 0..buf.height {
        for x in 0..buf.width {
            let ch = buf.get_char(Position::from(x as i32, y as i32));
            result.push(ch.char_code);
            result.push(ch.attribute.as_u8());
        }
    }
    
    result
}