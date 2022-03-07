use std::cmp::min;

use crate::model::{Buffer, DosChar, BitFont, Size};
use super::{ Position, TextAttribute};

pub fn read_adf(result: &mut Buffer, bytes: &[u8], file_size: usize, screen_width: i32)
{
    result.width = 80;
    let mut o = 0;
    let mut pos = Position::new();
    if bytes.len() < 1 + 3 * 64 + 4096 {
        panic!("no valid adf file, too small");
    }
    // let version = bytes[o];
    o += 1;

    // convert EGA -> VGA colors.
    let palette_size = 3 * 64;
    result.custom_palette = Some((&bytes[o..(o + palette_size)]).to_vec());
    o += palette_size;

    let font_size = 4096;
    result.font = Some(BitFont {
        size: Size::from(8, 16),
        data: bytes[o..(o + font_size)].iter().map(|x| *x as u32).collect()
    });
    o += font_size;
    loop {
        for _ in 0..screen_width {
            if o + 2 > file_size {
                return;
            }
            result.set_char(0, pos, Some(DosChar {
                char_code: bytes[o],
                attribute: TextAttribute::from_u8(bytes[o + 1])
            }));
            pos.x += 1;
            o += 2;
        }
        pos.x = 0;
        pos.y += 1;
    }
}

pub fn convert_to_adf(buf: &Buffer) -> Vec<u8>
{
    let mut result = Vec::new();
    result.push(1); // version

    if let Some(pal) = &buf.custom_palette {
        let upper = min(64, pal.len());
        for b in 0..upper {
            result.push(pal[b]);
        }
    } else {
        for c in Buffer::DOS_DEFAULT_PALETTE {
            result.push(c.0);
            result.push(c.1);
            result.push(c.2);
        }
    }
    result.resize(1 + 3 * 64,0);

    if let Some(font) = &buf.font {
        if font.data.len() == 4096 {
            let vec: Vec<u8> = font.data.iter().map(|x| *x as u8).collect();
            result.extend(vec);
        } else {
            result.extend(crate::DEFAULT_FONT);
        }
    } else {
        result.extend(crate::DEFAULT_FONT);
    }

    for y in 0..buf.height {
        for x in 0..buf.width {
            let ch = buf.get_char(Position::from(x as i32, y as i32)).unwrap_or_default();
            result.push(ch.char_code);
            result.push(ch.attribute.as_u8());
        }
    }
    
    result
}