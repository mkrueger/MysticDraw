use std::io;

use crate::model::{Buffer, DosChar, BitFont, Size, Palette, SauceString};
use super::{ Position, TextAttribute};

// http://fileformats.archiveteam.org/wiki/ArtWorx_Data_Format

// u8                   Version
// 3 * 64 = 192 u8      Palette
// 256 * 16 = 4096 u8   Font Data (only 8x16 supported)
// [ch u8, attr u8]*    Screen data
//
// A very simple format with a weird palette storage. Only 16 colors got used but a full 64 color palette is stored.
// Maybe useful for DOS demos running in text mode.

pub fn read_adf(result: &mut Buffer, bytes: &[u8], file_size: usize) -> io::Result<bool>
{
    result.width = 80;
    let mut o = 0;
    let mut pos = Position::new();
    if file_size <  1 + 3 * 64 + 4096 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid ADF - file too short"));
    }

    let version = bytes[o];
    if version != 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unsupported ADF version {}", version)));
    }
    o += 1;

    // convert EGA -> VGA colors.
    let palette_size = 3 * 64;
    result.palette = Palette::from(&bytes[o..(o + palette_size)]).cycle_ega_colors();
    o += palette_size;

    let font_size = 4096;
    result.font = Some(BitFont {
        name: SauceString::new(),
        extended_font: false,
        size: Size::from(8, 16),
        data: bytes[o..(o + font_size)].iter().map(|x| *x as u32).collect()
    });
    o += font_size;

    loop {
        for _ in 0..result.width {
            if o + 2 > file_size {
                result.set_height_for_pos(pos);
                return Ok(true);
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

pub fn convert_to_adf(buf: &Buffer) -> io::Result<Vec<u8>>
{
    let mut result = vec![1]; // version

    result.extend(buf.palette.to_ega_palette());
    if let Some(font) = &buf.font {
        if font.size.width != 8 || font.size.height != 16 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Only 8x16 fonts are supported by adf."));
        }

        if font.data.len() == 4096 {
            let vec: Vec<u8> = font.data.iter().map(|x| *x as u8).collect();
            result.extend(vec);
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Unexpected - invalid font data."));
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
    if buf.write_sauce || buf.width != 80 {
        buf.write_sauce_info(&crate::model::SauceFileType::Ansi, &mut result)?;
    }
    Ok(result)
}