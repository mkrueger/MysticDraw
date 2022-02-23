use crate::Buffer;
use crate::model::{DosChar, Position, TextAttribute};

pub fn read_xbin(result: &mut Buffer, bytes: &[u8], _file_size: usize, _screen_width: i32)
{
    if b"XBIN" != &bytes[0..4] {
        eprintln!("no valid xbin.");
        return;
    }
    let mut o = 4;

    // let eof_char = bytes[o];
    o += 1;
    result.width = (bytes[o] as u16 + ((bytes[o + 1] as u16) << 8)) as usize;
    o += 2;
    result.height = (bytes[o] as u16 + ((bytes[o + 1] as u16) << 8)) as usize;
    o += 2;

    let font_size = bytes[o];
    o += 1;
    let flags = bytes[o];
    o += 1;

    let has_custom_palette    = (flags &  1) == 1;
    let has_custom_font       = (flags &  2) == 2;
    let is_compressed         = (flags &  4) == 4;
    let is_blink_mode        = (flags &  8) != 8;
    let is_extended_char_mode = (flags & 16) == 16;

    println!("xbin {}x{} font_size: {} ", result.width, result.height, font_size);
    println!("custom palette {}, custom font {} , compressed {}, blink {}, extended char {}", has_custom_palette, has_custom_font, is_compressed, is_blink_mode, is_extended_char_mode);

    if has_custom_palette {
        result.custom_palette = Some((&bytes[o..(o+48)]).to_vec());
        o += 48;
    }
    if has_custom_font {
        let font_length = font_size as usize * if is_extended_char_mode { 512 } else { 256 };
        result.custom_font = Some((&bytes[o..(o+font_length)]).to_vec());
        result.font_dimensions = Position::from(8, font_size as i32);
        o += font_length;
    }

    if is_compressed {
        read_data_compressed(result, &bytes[o..]);
    } else {
        read_data_uncompressed(result, &bytes[o..]);
    }
}

fn advance_pos(result: &Buffer, pos: &mut Position) -> bool
{
    pos.x += 1;
    if pos.x >= result.width as i32 {
        pos.x = 0;
        pos.y += 1;
        if pos.y >= result.height as i32 {
            return false;
        }
    }
    true
}

const NO_COMPRESSION: u8 = 0;
const CHAR_COMPRESSION: u8 = 1;
const ATTR_COMPRESSION: u8 = 2;
const FULL_COMPRESSION: u8 = 3;

fn read_data_compressed(result: &mut Buffer, bytes: &[u8])
{
    let mut pos = Position::new();
    let mut o = 0;
    while o < bytes.len() {
        let xbin_compression = bytes[o];
        o += 1;
        let compression_type = (xbin_compression & 0b_1100_0000) >> 6;
        let repeat_counter   = (xbin_compression & 0b_0011_1111) + 1;

        match compression_type {
            NO_COMPRESSION => {
                for _ in 0..repeat_counter {
                    if o + 2 > bytes.len() { return; }
                    result.set_char(pos, DosChar { 
                        char_code: bytes[o], 
                        attribute: TextAttribute::from_u8(bytes[o + 1])
                    });
                    o += 2;
                    if !advance_pos(result, &mut pos) {
                        return;
                    }
                }
            }
            CHAR_COMPRESSION => {
                let ch = bytes[o];
                o += 1;
                for _ in 0..repeat_counter {
                    if o + 1 > bytes.len() { return; }
                    result.set_char(pos, DosChar { 
                        char_code: ch, 
                        attribute: TextAttribute::from_u8(bytes[o])
                    });
                    o += 1;
                    if !advance_pos(result, &mut pos) {
                        return;
                    }
                }
            }
            ATTR_COMPRESSION=> {
                let attr = TextAttribute::from_u8(bytes[o]);
                o += 1;
                for _ in 0..repeat_counter {
                    if o + 1 > bytes.len() { return; }
                    result.set_char(pos, DosChar { 
                        char_code: bytes[o], 
                        attribute: attr
                    });
                    o += 1;
                    if !advance_pos(result, &mut pos) {
                        return;
                    }
                }
            }
            FULL_COMPRESSION=> {
                let ch = bytes[o];
                o += 1;
                if o + 1 > bytes.len() { return; }
                let attr = TextAttribute::from_u8(bytes[o]);
                o += 1;
                let rep_ch = DosChar { 
                    char_code: ch, 
                    attribute: attr
                };

                for _ in 0..repeat_counter {
                    result.set_char(pos, rep_ch);
                    if !advance_pos(result, &mut pos) {
                        return;
                    }
                }
            }
            _ => {}
        }
    }
}

fn read_data_uncompressed(result: &mut Buffer, bytes: &[u8])
{
    let mut pos = Position::new();
    let mut o = 0;
    while  o + 2 < bytes.len() {
        result.set_char(pos, DosChar { 
            char_code: bytes[o], 
            attribute: TextAttribute::from_u8(bytes[o + 1])
        });
        o += 2;
        if !advance_pos(result, &mut pos) {
            break;
        }
    }
}