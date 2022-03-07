use crate::model::{Buffer, DosChar, BitFont, Size};
use super::{ Position, TextAttribute};

// http://fileformats.archiveteam.org/wiki/ArtWorx_Data_Format

// u8                   Version
// 3 * 64 = 192 u8      Palette
// 256 * 16 = 4096 u8   Font Data (only 8x16 supported)
// [ch u8, attr u8]*    Screen data
//
// A very simple format with a weird palette storage. Only 16 colors got used but a full 64 color palette is stored.
// Maybe useful for DOS demos running in text mode.

static COLOR_OFFSETS: [usize;16] = [ 0, 1, 2, 3, 4, 5, 20, 7, 56, 57, 58, 59, 60, 61, 62, 63 ];

fn convert_palette(pal: &[u8]) -> Vec<u8>
{   
    let mut res = Vec::new();
    for i in COLOR_OFFSETS {
        let o = i * 3;
        res.push(pal[o] << 2 | pal[o] >> 4);
        res.push(pal[o + 1] << 2 | pal[o + 1] >> 4);
        res.push(pal[o + 2] << 2 | pal[o + 2] >> 4);
    }
    res
}

fn generate_palette(buf: &Buffer) -> Vec<u8>
{
    let mut res = Vec::new();
    res.resize(3 * 64, 0);

    #[allow(clippy::needless_range_loop)]
    for i in 0..16 {
        let col = buf.get_rgb(i as u8);

        let o = COLOR_OFFSETS[i] * 3;
        res[o] = col.0 >> 2 | col.0 << 4;
        res[o + 1] = col.1 >> 2 | col.1 << 4;
        res[o + 2] = col.2 >> 2 | col.2 << 4;
    }

    res
}

pub fn read_adf(result: &mut Buffer, bytes: &[u8], file_size: usize, screen_width: i32)
{
    result.width = 80;
    let mut o = 0;
    let mut pos = Position::new();
    assert!(bytes.len() >= 1 + 3 * 64 + 4096, "no valid adf file, too small");
    // let version = bytes[o];
    o += 1;

    // convert EGA -> VGA colors.
    let palette_size = 3 * 64;
    result.custom_palette = Some(convert_palette(&bytes[o..(o + palette_size)]));


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
                result.height = pos.y as usize;
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
    let mut result = vec![1]; // version

    result.extend(generate_palette(buf));

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