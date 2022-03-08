use crate::model::{Buffer, DosChar, BitFont, Size, Rectangle, buffer};
use super::{ Position, TextAttribute};

// http://fileformats.archiveteam.org/wiki/ICEDraw


const HEADER_SIZE: usize = 4 + 4 * 2;

const IDF_V1_3_HEADER: &[u8] = b"\x041.3";
const IDF_V1_4_HEADER: &[u8] = b"\x041.4";

const FONT_SIZE: usize = 4096;
const PALETTE_SIZE: usize = 3 * 16;

pub fn read_idf(result: &mut Buffer, bytes: &[u8], file_size: usize)
{
    assert!(file_size >= HEADER_SIZE + FONT_SIZE + PALETTE_SIZE, "too small for IDF.");
    let version = &bytes[0..4];
    assert!(version == IDF_V1_3_HEADER || version == IDF_V1_4_HEADER, "no supported idf version.");
    let mut o = 4;
    let x1 = (bytes[o] as u16 + ((bytes[o + 1] as u16) << 8)) as i32;
    o += 2;
    let y1 = (bytes[o] as u16 + ((bytes[o + 1] as u16) << 8)) as i32;
    o += 2;
    let x2 = (bytes[o] as u16 + ((bytes[o + 1] as u16) << 8)) as i32;
    o += 2;
    // skip y2
    o += 2;
    assert!(x1 <= x2, "invalid idf bounds.");
    result.width  = (x2 + 1) as usize;
    let data_size = file_size - FONT_SIZE - PALETTE_SIZE;
    let mut pos = Position::from(x1, y1);

    while o + 1 < data_size {
        let mut rle_count = 1;
        let mut char_code = bytes[o];
        o += 1;
        let mut attr =  bytes[o];
        o += 1;

        if char_code == 1 && attr == 0 {
            rle_count = bytes[o] as i32 + ((bytes[o + 1] as i32) << 8);

            if o + 3 >= data_size { break; }
            o += 2;
            char_code = bytes[o];
            o += 1;
             attr =  bytes[o];
            o += 1;
        }
        while rle_count > 0 {
            result.set_char(0, pos, Some(DosChar {
                char_code,
                attribute: TextAttribute::from_u8(attr)
            }));
            advance_pos(x1, x2, &mut pos);
            rle_count -= 1;
        }
    }
    result.font = Some(BitFont {
        size: Size::from(8, 16),
        data: bytes[o..(o + FONT_SIZE)].iter().map(|x| *x as u32).collect()
    });
    o += FONT_SIZE;

    result.custom_palette = Some((&bytes[o..(o + PALETTE_SIZE)]).iter().map(|x| x << 2 | x >> 4).collect());

    result.height = pos.y as usize;
}

pub fn convert_to_idf(buffer: &Buffer) -> Vec<u8>
{
    let mut result = IDF_V1_4_HEADER.to_vec();
    
    // x1
    result.push(0);
    result.push(0);

    // y1
    result.push(0);
    result.push(0);
    
    let w = buffer.width - 1;
    result.push(w as u8);
    result.push((w >> 8) as u8);

    let h = buffer.height - 1;
    result.push(h as u8);
    result.push((h >> 8) as u8);

    let len = (buffer.height * buffer.width) as i32;
    let mut x = 0;
    while x < len {
        let ch = buffer.get_char(Position::from_index(buffer, x)).unwrap_or_default();
        let mut rle_count = 1;
        while x + rle_count < len && rle_count < (u16::MAX) as i32 {
            if ch != buffer.get_char(Position::from_index(buffer, x + rle_count)).unwrap_or_default() {
                break;
            }
            rle_count += 1;
        }
        if rle_count > 3 || ch.char_code == 1 {
            result.push(1);
            result.push(0);

            result.push(rle_count as u8);
            result.push((rle_count >> 8) as u8);
        } else {
            rle_count = 1;
        }
        result.push(ch.char_code);
        result.push(ch.attribute.as_u8());

        x += rle_count;
    }

    // font
    if let Some(font) = &buffer.font {
        if font.data.len() == 4096 {
            let vec: Vec<u8> = font.data.iter().map(|x| *x as u8).collect();
            result.extend(vec);
        } else {
            result.extend(crate::DEFAULT_FONT);
        }
    } else {
        result.extend(crate::DEFAULT_FONT);
    }

    // palette
    for i in 0..16 {
        let col = buffer.get_rgb(i as u8);
        result.push(col.0 >> 2 | col.0 << 4);
        result.push(col.1 >> 2 | col.1 << 4);
        result.push(col.2 >> 2 | col.2 << 4);
    }

    result
}

fn advance_pos(x1: i32, x2: i32, pos: &mut Position) -> bool
{
    pos.x += 1;
    if pos.x > x2  {
        pos.x = x1;
        pos.y += 1;
    }
    true
}