use crate::model::{Buffer, Position, TextAttribute};

const HEX_TABLE: &[u8;16] = b"0123456789ABCDEF";

pub fn convert_to_pcb(buf: &Buffer) -> Vec<u8>
{
    let mut result = Vec::new();
    let mut last_attr = TextAttribute::DEFAULT;
    let mut pos = Position::new();
    let height = buf.height as i32;
    let mut last_line_skipped = false;
    let mut first_char = true;
    // @CLS@ or @HOME@

    while pos.y < height {
        let line_length = buf.get_line_length(pos.y);
        if line_length == 0 && last_line_skipped {
            result.push(13);
            result.push(10);
        }

        while pos.x < line_length {
            let ch = buf.get_char(pos);

            if first_char || ch.attribute != last_attr {
                result.extend_from_slice(b"@X");
                result.push(HEX_TABLE[ch.attribute.get_background_ice() as usize]);
                result.push(HEX_TABLE[ch.attribute.get_foreground() as usize]);
                last_attr = ch.attribute;
            }

            result.push(if ch.char_code == 0 { b' ' } else { ch.char_code });
            first_char = false;
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