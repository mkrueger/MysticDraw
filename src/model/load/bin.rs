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