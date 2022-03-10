use std::io;

use crate::model::{Buffer, Position, SauceString, Color, BitFont, Size, Layer, DosChar, TextAttribute};
const MDF_HEADER: &[u8] = b"MDf";
const MDF_VERSION: u16 = 0;
const ID_SIZE: usize = 4;
const HEADER_SIZE: usize = 83;
const CRC32_SIZE: usize = 4;

pub fn read_mdf(result: &mut Buffer, bytes: &[u8]) -> io::Result<bool>
{
    if bytes.len() < ID_SIZE + CRC32_SIZE + HEADER_SIZE {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid MDF.\nFile too short"));
    }
    if &bytes[0..3] != MDF_HEADER {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid MDF.\nInvalid header"));
    }
    let crc32 = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
    let mut o = ID_SIZE + CRC32_SIZE;
    if crc32 != crc32fast::hash(&bytes[o..]) {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid MDF.\nCRC32 mismatch"));
    }
    result.layers.clear();
    o += 2;// skip version

    o += result.title.read(&bytes[o..]);
    o += result.author.read(&bytes[o..]);
    o += result.group.read(&bytes[o..]);

    result.width = u16::from_be_bytes(bytes[o..(o + 2)].try_into().unwrap());
    o += 2;
    result.height = u16::from_be_bytes(bytes[o..(o + 2)].try_into().unwrap());
    o += 2;

    let flags = u16::from_be_bytes(bytes[o..(o + 2)].try_into().unwrap());
    o += 2;

    result.use_ice = (flags & MDF_FLAG_ICE) == MDF_FLAG_ICE;
    result.write_sauce = (flags & MDF_FLAG_WRITE_SAUCE) == MDF_FLAG_WRITE_SAUCE;
    result.use_512_chars = (flags & MDF_FLAG_512_CHARS) == MDF_FLAG_512_CHARS;
    while o < bytes.len() {
        let block = bytes[o];
        o += 1;
        match block {
            BLK_COMMENT => {
                let mut comments = bytes[o];
                o += 1;
                while comments > 0 {
                    let mut comment: SauceString<64, 0> = SauceString::new();
                    o += comment.read(&bytes[o..]);
                    result.comments.push(comment);
                    comments -= 1;
                }
            }
            BLK_PALETTE => {
                let mut colors = u32::from_be_bytes(bytes[o..(o + 4)].try_into().unwrap());
                result.palette.colors.clear();
                o += 4;
                while colors > 0 {
                    let r = bytes[o];
                    o += 1;
                    let g = bytes[o];
                    o += 1;
                    let b = bytes[o];
                    o += 1;
                                
                    result.palette.colors.push(Color::new(r, g, b));
                    colors -= 1;
                }

            }
            BLK_FONT_NAME => {
                let mut font_name: SauceString<22, 0> = SauceString::new();
                o += font_name.read(&bytes[o..]);
                result.font_name = Some(font_name);
            }
            BLK_FONT => {
                let mut font_name: SauceString<22, 0> = SauceString::new();
                o += font_name.read(&bytes[o..]);
                let width = bytes[o];
                o += 1;
                let height = bytes[o];
                o += 1;
                let flags = bytes[o];
                let extended_font =  (flags & 1) == 1;
                o += 1;
                let mut data = Vec::new();

                let upper = if extended_font { 512 } else { 256 };
                for _ in 0..upper {
                    for _ in 0..height {
                        if width  < 9  {
                            data.push(bytes[o] as u32);
                            o += 1;
                        } else {
                            let d = u16::from_be_bytes(bytes[o..(o + 2)].try_into().unwrap());
                            o += 2;
                            data.push(d as u32);
                        }
                    }
                }
                result.font = Some(BitFont {
                    name: font_name,
                    extended_font,
                    size: Size { width: width as usize, height: height as usize },
                    data,
                });
            } 
            BLK_LAYER => {
                let title_len = u16::from_be_bytes(bytes[o..(o + 2)].try_into().unwrap()) as usize;
                o += 2;
                let title = String::from_utf8_lossy(&bytes[o..(o + title_len)]).to_string();
                o += title_len;
                // skip mode
                o += 1;
                let flags = u16::from_be_bytes(bytes[o..(o + 2)].try_into().unwrap());
                let attr_mode = flags & 0b_0110;
                o += 2;
                let x = i32::from_be_bytes(bytes[o..(o + 4)].try_into().unwrap());
                o += 4;
                let y = i32::from_be_bytes(bytes[o..(o + 4)].try_into().unwrap());
                o += 4;
                let width = u16::from_be_bytes(bytes[o..(o + 2)].try_into().unwrap());
                o += 2;

                let mut layer = Layer {
                    title,
                    is_visible: true,
                    is_locked: false,
                    is_position_locked: false,
                    offset: Position::from(x, y),
                    lines: Vec::new(),
                };
                let mut i = 0;
                if width > 0 {
                    loop {
                        let len = u16::from_be_bytes(bytes[o..(o + 2)].try_into().unwrap());
                        o += 2;
                        if len == 0 { break; }

                        let is_empty = (len & 0b1000_0000_0000_0000) == 0b1000_0000_0000_0000;
                        let mut len = len & !0b1000_0000_0000_0000;
                        
                        if is_empty { 
                            i += len as i32;
                        } else if flags & LAYER_COMPRESSED == LAYER_COMPRESSED {
                            decompress(&mut layer, bytes, &mut o, i, len, width, attr_mode);
                            i += len as i32;
                        } else {
                            while len > 0 {
                                let char_code = bytes[o];
                                o += 1;
                                let attribute = decode_attribute(bytes, &mut o, attr_mode);
                                let pos = Position { x: i % (width as i32) , y: i / (width as i32)};
                                layer.set_char(pos, Some(DosChar {
                                    char_code,
                                    attribute
                                }));
                                len -= 1;
                                i += 1;
                            }
                        }
                    }
                }
                layer.is_visible = (flags & LAYER_IS_VISIBLE) == LAYER_IS_VISIBLE;
                layer.is_locked = (flags & LAYER_EDIT_LOCK) == LAYER_EDIT_LOCK;
                layer.is_position_locked = (flags & LAYER_POS_LOCK) == LAYER_POS_LOCK;

                result.layers.push(layer);
            }
            _ => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid MDF.\nUnsupported block type {}", block)));
            }
        }
    }
    Ok(true)
}

fn decompress(result: &mut Layer, bytes: &[u8], o: &mut usize, mut i: i32, len: u16, width: u16, attr_mode: u16)
{
    let end = i + len as i32;
    while i < end {
        let xbin_compression = bytes[*o];
        *o += 1;

        let compression = unsafe { std::mem::transmute(xbin_compression & 0b_1100_0000) };
        let repeat_counter = (xbin_compression & 0b_0011_1111) + 1;

        match compression {
            Compression::Off => {
                for _ in 0..repeat_counter {
                    let char_code = bytes[*o];
                    *o += 1;
                    let attribute = decode_attribute(bytes, o, attr_mode);
                    let pos = Position { x: i % (width as i32), y: i / (width as i32)};
                    result.set_char(pos, Some(DosChar { char_code, attribute }));
                    i += 1;
                }
            }
            Compression::Char => {
                let char_code = bytes[*o];
                *o += 1;
                for _ in 0..repeat_counter {
                    let attribute = decode_attribute(bytes, o, attr_mode);
                    let pos = Position { x: i % (width as i32), y: i / (width as i32)};
                    result.set_char(pos, Some(DosChar { char_code, attribute }));
                    i += 1;
                }
            }
            Compression::Attr => {
                let attribute = decode_attribute(bytes, o, attr_mode);
                for _ in 0..repeat_counter {
                    let char_code = bytes[*o];
                    *o += 1;
                    let pos = Position { x: i % (width as i32), y: i / (width as i32)};
                    result.set_char(pos, Some(DosChar { char_code, attribute }));
                    i += 1;
                }
            }
            Compression::Full => {
                let char_code = bytes[*o];
                *o += 1;
                let attribute = decode_attribute(bytes, o, attr_mode);
                
                let rep_ch = Some(DosChar { 
                    char_code, 
                    attribute
                });

                for _ in 0..repeat_counter {
                    let pos = Position { x: i % (width as i32) , y: i / (width as i32)};
                    result.set_char(pos, rep_ch);
                    i += 1;
                }
            }
        }
    }
}

fn decode_attribute(bytes: &[u8], o: &mut usize, attr_mode: u16) -> TextAttribute {
    match attr_mode { 
        ATTR_MODE_U8 => { 
            let attr = bytes[*o];
            *o += 1;
            TextAttribute::from_u8(attr)
        }
        ATTR_MODE_255 => {
            let fg = bytes[*o];
            *o += 1;
            let bg = bytes[*o];
            *o += 1;
            TextAttribute::from_color(fg, bg)
        }
        ATTR_MODE_U16 => {
            let fg = u16::from_be_bytes(bytes[*o..(*o + 2)].try_into().unwrap());
            *o += 2;
            let bg = u16::from_be_bytes(bytes[*o..(*o + 2)].try_into().unwrap());
            *o += 2;
            TextAttribute::from_color(fg as u8, bg as u8)
        }
        ATTR_MODE_U32 => {
            let fg = u32::from_be_bytes(bytes[*o..(*o + 4)].try_into().unwrap());
            *o += 4;
            let bg = u32::from_be_bytes(bytes[*o..(*o + 4)].try_into().unwrap());
            *o += 4;
            TextAttribute::from_color(fg as u8, bg as u8)
        }
        _ => { panic!("unsupported attr_mode."); }
    }
}
const MDF_FLAG_ICE: u16         = 0b0001;
const MDF_FLAG_WRITE_SAUCE: u16 = 0b0010;
const MDF_FLAG_512_CHARS: u16   = 0b0100;
const CHECKSUM_OFFSET: usize = 4;

const BLK_COMMENT:u8   = 1;
const BLK_PALETTE:u8   = 2;
const BLK_FONT_NAME:u8 = 3;
const BLK_FONT:u8      = 4;
const BLK_LAYER:u8     = 5;

const ATTR_MODE_U8:u16   = 0b_0000;
const ATTR_MODE_255:u16  = 0b_0010;
const ATTR_MODE_U16:u16  = 0b_0100;
const ATTR_MODE_U32:u16  = 0b_0110;

pub fn convert_to_mdf(buf: &Buffer) -> io::Result<Vec<u8>>
{
    let mut result = MDF_HEADER.to_vec();
    result.push(0x1A); // CP/M EOF char (^Z) - used by DOS as well
    
    result.push(0); // CRC32 will be calculated at the end
    result.push(0);
    result.push(0);
    result.push(0);


    result.push(MDF_VERSION as u8);
    result.push((MDF_VERSION >> 8) as u8);
    buf.title.append_to(&mut result);
    buf.author.append_to(&mut result);
    buf.group.append_to(&mut result);
    result.extend(u16::to_be_bytes(buf.width));
    result.extend(u16::to_be_bytes(buf.height));

    let mut flags = 0;
    if buf.use_ice { flags |= MDF_FLAG_ICE; }
    if buf.write_sauce { flags |= MDF_FLAG_WRITE_SAUCE; }
    if buf.use_512_chars { flags |= MDF_FLAG_512_CHARS; }
    result.extend(u16::to_be_bytes(flags));
    
    if !buf.comments.is_empty() {
        result.push(BLK_COMMENT);
        if buf.comments.len() > 255 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "too many comments. Maximum of 255 are supported"));
        }
        result.push(buf.comments.len() as u8);
        for cmt in &buf.comments {
            cmt.append_to(&mut result);
        }
    }

    if !buf.palette.is_default() {
        result.push(BLK_PALETTE);
        result.extend(u32::to_be_bytes(buf.palette.len()));
        for col in &buf.palette.colors {
            let rgb = col.get_rgb();
            result.push(rgb.0);
            result.push(rgb.1);
            result.push(rgb.2);
        }
    }

    if let Some(font) = &buf.font {
        result.push(BLK_FONT);
        font.name.append_to(&mut result);
        result.push(font.size.width as u8);
        result.push(font.size.height as u8);
        result.push(if font.extended_font { 1 } else { 0 });
        for data in &font.data {
            if font.size.width > 8  { 
                result.push((data >> 8) as u8);
            }
            result.push(*data as u8);
        }
    } else if let Some(name) = &buf.font_name {
        result.push(BLK_FONT_NAME);
        name.append_to(&mut result);
    }

    for layer in &buf.layers {
        result.push(BLK_LAYER);
        let bytes = layer.title.as_bytes();
        if buf.comments.len() > u16::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "layer name length too wide"));
        }
        result.extend(u16::to_be_bytes(bytes.len() as u16));
        result.extend(bytes);
        result.push(0); // mode (unused atm)
        let mut flags = LAYER_COMPRESSED;
        if layer.is_visible { flags |= LAYER_IS_VISIBLE; }
        if layer.is_locked { flags |= LAYER_EDIT_LOCK; }
        if layer.is_position_locked { flags |= LAYER_POS_LOCK; }

        let attr_mode = if buf.palette.colors.len() <= (1 << 4) {
            ATTR_MODE_U8
        } else if buf.palette.colors.len() <= (1 << 7) {
            ATTR_MODE_255
        } else if buf.palette.colors.len() <= (1 << 16) {
            ATTR_MODE_U16
        } else {
            ATTR_MODE_U32
        };
        flags |= attr_mode;

        result.extend(u16::to_be_bytes(flags));

        result.extend(i32::to_be_bytes(layer.get_offset().x));
        result.extend(i32::to_be_bytes(layer.get_offset().y));

        if let Some(width) = layer.lines.iter().map(|l| l.chars.len()).max() {
            result.extend(u16::to_be_bytes(width as u16));

            let len = (width * layer.lines.len()) as i32;
            let mut i = 0;
            while i < len {
                let ch = layer.get_char(Position { x: i % (width as i32) , y: i / (width as i32) });
                let mut rle_count = 1;
                let mut j = i + rle_count;
                while j < len && rle_count < 0b1000_0000_0000_0000 {
                    let n = layer.get_char(Position { x: j % (width as i32) , y: j / (width as i32)});
                    if ch.is_some() != n.is_some() || ch.is_none() != n.is_none() {
                        break;
                    }
                    rle_count += 1;
                    j += 1;
                }
                if ch.is_none() {
                    rle_count |= 0b1000_0000_0000_0000;
                }
                result.extend(u16::to_be_bytes(rle_count as u16));
        
                if ch.is_some() {
                    if flags & LAYER_COMPRESSED == LAYER_COMPRESSED {
                        compress_greedy(&mut result, layer, i, rle_count, width, attr_mode);
                        i += rle_count;
                    } else {
                        while rle_count > 0 {
                            let ch = layer.get_char(Position { x: i % (width as i32) , y: i / (width as i32)}).unwrap();
                            result.push(ch.char_code);
                            encode_attribte(&mut result, ch, attr_mode);
                            i += 1;
                            rle_count -= 1;
                        }
                    }
                } else {
                    i += rle_count & !0b1000_0000_0000_0000;
                }
            }
        }
        // write either width == 0, or end data block.
        result.push(0);
        result.push(0);
    }
    let crc = u32::to_be_bytes(crc32fast::hash(&result[8..]));
    result[CHECKSUM_OFFSET..(CHECKSUM_OFFSET + crc.len())].clone_from_slice(&crc[..]);
    Ok(result)
}

fn encode_attribte(result: &mut Vec<u8>, ch: DosChar, attr_mode: u16) {
    match attr_mode { 
        ATTR_MODE_U8 => { result.push(ch.attribute.as_u8()); }
        ATTR_MODE_255 => {
            result.push(ch.attribute.get_foreground());
            result.push(ch.attribute.get_background());
        }
        ATTR_MODE_U16 => {
            result.extend(u16::to_be_bytes(ch.attribute.get_foreground() as u16));
            result.extend(u16::to_be_bytes(ch.attribute.get_background() as u16));
        }
        ATTR_MODE_U32 => {
            result.extend(u32::to_be_bytes(ch.attribute.get_foreground() as u32));
            result.extend(u32::to_be_bytes(ch.attribute.get_background() as u32));
        }
        _ => { panic!("unsupported attr_mode."); }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum Compression {
    Off  = 0b0000_0000,
    Char = 0b0100_0000,
    Attr = 0b1000_0000,
    Full = 0b1100_0000,
}

fn compress_greedy(result: &mut Vec<u8>, layer: &Layer, i: i32, rle_count: i32, width: usize, attr_mode: u16) {
    let mut run_mode = Compression::Off;
    let mut run_count = 0;
    let mut run_buf = Vec::new();
    let mut run_ch = DosChar::default();
    let len = i + rle_count;
    for x in i..len {
        let cur = layer.get_char(Position { x: x % (width as i32) , y: x / (width as i32)}).unwrap();

        let next = if x < len - 1 {
            layer.get_char(Position { x: (x + 1) % (width as i32) , y: (x + 1) / (width as i32)}).unwrap()
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
                            let next2 = layer.get_char(Position { x: (x + 2) % (width as i32) , y: (x + 2) / (width as i32)}).unwrap();
                            end_run = cur.char_code == next.char_code && cur.char_code == next2.char_code ||
                                      cur.attribute == next.attribute && cur.attribute == next2.attribute;
                        }
                    }
                    Compression::Char => {
                        if cur.char_code != run_ch.char_code {
                            end_run = true;
                        } else if x < len - 3 {
                            let next2 = layer.get_char(Position { x: (x + 2) % (width as i32) , y: (x + 2) / (width as i32)}).unwrap();
                            let next3 = layer.get_char(Position { x: (x + 3) % (width as i32) , y: (x + 3) / (width as i32)}).unwrap();
                            end_run = cur == next && cur == next2 && cur == next3;
                        }
                    }
                    Compression::Attr => {
                        if cur.attribute != run_ch.attribute {
                            end_run = true;
                        } else if x < len - 3 {
                            let next2 = layer.get_char(Position { x: (x + 2) % (width as i32) , y: (x + 2) / (width as i32)}).unwrap();
                            let next3 = layer.get_char(Position { x: (x + 3) % (width as i32) , y: (x + 3) / (width as i32)}).unwrap();
                            end_run = cur == next && cur == next2 && cur == next3;
                        }
                    }
                    Compression::Full => {
                        end_run = cur != run_ch;
                    }
                }
            }

            if end_run {
                result.push((run_mode as u8) | (run_count - 1));
                result.extend(&run_buf);
                run_count = 0;
            }
        }

        if run_count > 0 {
            match run_mode {
                Compression::Off => {
                    run_buf.push(cur.char_code);
                    encode_attribte(&mut run_buf, cur, attr_mode);
                }
                Compression::Char => {
                    encode_attribte(&mut run_buf, cur, attr_mode);
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
                encode_attribte(&mut run_buf, cur, attr_mode);
                run_buf.push(cur.char_code);
            }
            else
            {
                run_buf.push(cur.char_code);
                encode_attribte(&mut run_buf, cur, attr_mode);
            }

            run_ch = cur;
        }
        run_count += 1;
    }

    if run_count > 0 {
        result.push((run_mode as u8) | (run_count - 1));
        result.extend(run_buf);
    }
}

const LAYER_COMPRESSED:u16 = 0b0000_0001;
const LAYER_IS_VISIBLE:u16 = 0b0010_0000;
const LAYER_EDIT_LOCK:u16  = 0b0100_0000;
const LAYER_POS_LOCK:u16   = 0b1000_0000;
