use crate::model::{Buffer, Position};

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