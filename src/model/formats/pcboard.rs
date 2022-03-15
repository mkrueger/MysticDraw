use std::io;

use crate::model::{Buffer, Position, TextAttribute};

use super::{ParseStates, SaveOptions};

fn conv_ch(ch: u8) -> u8 {
    if (b'0'..=b'9').contains(&ch) {
        return ch - b'0';
    }
    if (b'a'..=b'f').contains(&ch) {
        return 10 + ch - b'a';
    }
    if (b'A'..=b'F').contains(&ch) {
        return 10 + ch - b'A';
    }
    0
}

const HEX_TABLE: &[u8;16] = b"0123456789ABCDEF";

pub fn convert_to_pcb(buf: &Buffer, options: &SaveOptions) -> io::Result<Vec<u8>>
{
    let mut result = Vec::new();
    let mut last_attr = TextAttribute::DEFAULT;
    let mut pos = Position::new();
    let height = buf.height as i32;
    let mut first_char = true;

    match options.screen_preparation {
        super::ScreenPreperation::None | super::ScreenPreperation::Home => {}, // home not supported
        super::ScreenPreperation::ClearScreen => { result.extend(b"@CLS@"); },
    }

    while pos.y < height {
        let line_length = buf.get_line_length(pos.y);
        
        while pos.x < line_length {
            let ch = buf.get_char(pos).unwrap_or_default();

            if first_char || ch.attribute != last_attr {
                result.extend_from_slice(b"@X");
                result.push(HEX_TABLE[ch.attribute.get_background() as usize]);
                result.push(HEX_TABLE[ch.attribute.get_foreground() as usize]);
                last_attr = ch.attribute;
            }

            result.push(if ch.char_code == 0 { b' ' } else { ch.char_code });
            first_char = false;
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
        buf.write_sauce_info(&crate::model::SauceFileType::PCBoard, &mut result)?;
    }
    Ok(result)
}

pub fn get_save_sauce_default_pcb(buf: &Buffer) -> (bool, String)
{
    if buf.width != 80 {
        return (true, "width != 80".to_string() );
    }

    if buf.has_sauce_relevant_data() { return (true, String::new()); }

    ( false, String::new() )
}


#[allow(non_snake_case)]
pub fn display_PCBoard(data: &mut ParseStates, ch: u8) -> Option<u8> {
    if data.pcb_color {
        data.pcb_pos += 1;
        if data.pcb_pos < 3 {
            match data.pcb_pos {
                1 => {
                    data.pcb_value = conv_ch(ch);
                    return None;
                }
                2 => {
                    data.pcb_value = (data.pcb_value << 4) + conv_ch(ch);
                    data.text_attr = TextAttribute::from_u8(data.pcb_value);
                }
                _ => {}
            }
        }
        data.pcb_color = false;
        data.pcb_code = false;
        return None;
    }

    if data.pcb_code {
        match ch {
            b'@' => {
                data.pcb_code = false;
            }
            b'X' => {
                data.pcb_color = true;
                data.pcb_pos = 0;
            }
            _ => {}
        }
        return None;
    }
    match ch {
        b'@' => {
            data.pcb_code = true;

            None
        }
        _ => Some(ch),
    }
}
