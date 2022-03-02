use crate::model::{Buffer, DosChar, BitFont, Size};

use super::{ Position, TextAttribute };


const FLAG_PALETTE:u8      = 0b_0000_0001;
const FLAG_FONT:u8         = 0b_0000_0010;
const FLAG_COMPRESS:u8     = 0b_0000_0100;

#[allow(dead_code)]
const FLAG_512CHAR_MODE:u8 = 0b_0000_1000;
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum Compression {
    Off  = 0b0000_0000,
    Char = 0b0100_0000,
    Attr = 0b1000_0000,
    Full = 0b1100_0000,
}


pub fn read_xb(result: &mut Buffer, bytes: &[u8], _file_size: usize, _screen_width: i32)
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
        result.font = Some(BitFont {
            size: Size::from(8, font_size as usize),
            data: bytes[o..(o+font_length)].iter().map(|x| *x as u32).collect()
        });
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

fn read_data_compressed(result: &mut Buffer, bytes: &[u8])
{
    let mut pos = Position::new();
    let mut o = 0;
    while o < bytes.len() {
        let xbin_compression = bytes[o];
        o += 1;
        let compression = unsafe { std::mem::transmute(xbin_compression & 0b_1100_0000) };
        let repeat_counter = (xbin_compression & 0b_0011_1111) + 1;

        match compression {
            Compression::Off => {
                for _ in 0..repeat_counter {
                    if o + 2 > bytes.len() { return; }
                    result.set_char(0, pos, DosChar { 
                        char_code: bytes[o], 
                        attribute: TextAttribute::from_u8(bytes[o + 1])
                    });
                    o += 2;
                    if !advance_pos(result, &mut pos) {
                        return;
                    }
                }
            }
            Compression::Char => {
                let ch = bytes[o];
                o += 1;
                for _ in 0..repeat_counter {
                    if o + 1 > bytes.len() { return; }
                    result.set_char(0, pos, DosChar { 
                        char_code: ch, 
                        attribute: TextAttribute::from_u8(bytes[o])
                    });
                    o += 1;
                    if !advance_pos(result, &mut pos) {
                        return;
                    }
                }
            }
            Compression::Attr => {
                let attr = TextAttribute::from_u8(bytes[o]);
                o += 1;
                for _ in 0..repeat_counter {
                    if o + 1 > bytes.len() { return; }
                    result.set_char(0, pos, DosChar { 
                        char_code: bytes[o], 
                        attribute: attr
                    });
                    o += 1;
                    if !advance_pos(result, &mut pos) {
                        return;
                    }
                }
            }
            Compression::Full => {
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
                    result.set_char(0, pos, rep_ch);
                    if !advance_pos(result, &mut pos) {
                        return;
                    }
                }
            }
        }
    }
}

fn read_data_uncompressed(result: &mut Buffer, bytes: &[u8])
{
    let mut pos = Position::new();
    let mut o = 0;
    while o + 2 <= bytes.len() {
        result.set_char(0, pos, DosChar { 
            char_code: bytes[o], 
            attribute: TextAttribute::from_u8(bytes[o + 1])
        });
        o += 2;
        if !advance_pos(result, &mut pos) {
            break;
        }
    }
}

pub fn convert_to_xb(buf: &Buffer) -> Vec<u8>
{
    let mut result = Vec::new();

    result.extend_from_slice(b"XBIN");
    result.push(0x1A); // CP/M EOF char (^Z) - used by DOS as well
    assert!(u16::try_from(buf.height).is_ok() && u16::try_from(buf.width).is_ok(), "buffer dimensions too large to save as xbin.");
    result.push(buf.width as u8);
    result.push((buf.width >> 8) as u8);
    result.push(buf.height as u8);
    result.push((buf.height >> 8) as u8);

    let mut flags = 0;
    if let Some(font) = &buf.font {
        assert!(!(font.size.width != 8 || font.size.height < 1 || font.size.height > 32), "font not supported by the .xb format only fonts with 8px width and a height from 1 to 32 are supported.");
        result.push(font.size.height as u8);
        flags |= FLAG_FONT;
    } else {
        // default font
        result.push(16);
    }
    if buf.custom_palette.is_some() {
        flags |= FLAG_PALETTE;
    }
    flags |= FLAG_COMPRESS;
    result.push(flags);

    if let Some(palette) = &buf.custom_palette {
        result.extend(palette);
    }

    if let Some(font) = &buf.font {
        let vec: Vec<u8> = font.data.iter().map(|x| *x as u8).collect();
        result.extend(&vec);
    }
    if (flags & FLAG_COMPRESS) == FLAG_COMPRESS  {
        compress_greedy(&mut result, buf);
    } else {
        // store uncompressed
        for y in 0..buf.height {
            for x in 0..buf.width {
                let ch = buf.get_char(Position::from(x as i32, y as i32));
                result.push(ch.char_code);
                result.push(ch.attribute.as_u8());
            }
        }
    }
    
    result
}

fn compress_greedy(outputdata: &mut Vec<u8>, buffer: &Buffer)
{
    let mut run_mode = Compression::Off;
    let mut run_count = 0;
    let mut run_buf = Vec::new();
    let mut run_ch = DosChar::default();
    let len = (buffer.height * buffer.width) as i32;
    for x in 0..len {
        let cur = buffer.get_char(Position::from_index(buffer, x));

        let next = if x < len - 1 {
            buffer.get_char(Position::from_index(buffer, x + 1))
        } else {
            DosChar::default()
        };
        
        if run_count > 0 {
            let mut end_run = false;
            if run_count >= 64 {
                end_run = true;
            } else if run_count > 0 {
                match run_mode {
                    Compression::Off => {
                        if x < len - 2 && cur == next {
                            end_run = true;
                        }
                        else if x < len - 2 {
                            let next2 = buffer.get_char(Position::from_index(buffer, x + 2));
                            end_run = cur.char_code == next.char_code && cur.char_code == next2.char_code ||
                                      cur.attribute == next.attribute && cur.attribute == next2.attribute;
                        }
                    }
                    Compression::Char => {
                        if cur.char_code != run_ch.char_code {
                            end_run = true;
                        } else if x < len - 3 {
                            let next2 = buffer.get_char(Position::from_index(buffer, x + 2));
                            let next3 = buffer.get_char(Position::from_index(buffer, x + 3));
                            end_run = cur == next && cur == next2 && cur == next3;
                        }
                    }
                    Compression::Attr => {
                        if cur.attribute != run_ch.attribute {
                            end_run = true;
                        } else if x < len - 3 {
                            let next2 = buffer.get_char(Position::from_index(buffer, x + 2));
                            let next3 = buffer.get_char(Position::from_index(buffer, x + 3));
                            end_run = cur == next && cur == next2 && cur == next3;
                        }
                    }
                    Compression::Full => {
                        end_run = cur != run_ch;
                    }
                }
            }

            if end_run {
                outputdata.push((run_mode as u8) | (run_count - 1));
                outputdata.extend(&run_buf);
                run_count = 0;
            }
        }

        if run_count > 0 {
            match run_mode {
                Compression::Off => {
                    run_buf.push(cur.char_code);
                    run_buf.push(cur.attribute.as_u8());
                }
                Compression::Char => {
                    run_buf.push(cur.attribute.as_u8());
                }
                Compression::Attr => {
                    run_buf.push(cur.char_code);
                }
                Compression::Full => {
                    // nothing
                }    
            }
        }
        else
        {
            run_buf.clear();
            if x < len - 1 {
                if cur == next {
                    run_mode = Compression::Full;
                }
                else if cur.char_code == next.char_code {
                    run_mode = Compression::Char;
                }
                else if cur.attribute == next.attribute {
                    run_mode = Compression::Attr;
                }
                else {
                    run_mode = Compression::Off;
                }
            }
            else {
                run_mode = Compression::Off;
            }

            if let Compression::Attr = run_mode { 
                run_buf.push(cur.attribute.as_u8());
                run_buf.push(cur.char_code);
            }
            else
            {
                run_buf.push(cur.char_code);
                run_buf.push(cur.attribute.as_u8());
            }

            run_ch = cur;
        }
        run_count += 1;
    }

    if run_count > 0 {
        outputdata.push((run_mode as u8) | (run_count - 1));
        outputdata.extend(run_buf);
    }
}