use crate::model::{Buffer, Position};

const FLAG_PALETTE:u8      = 0b_0000_0001;
const FLAG_FONT:u8         = 0b_0000_0010;

#[allow(dead_code)]
const FLAG_COMPRESS:u8     = 0b_0000_0100;

#[allow(dead_code)]
const FLAG_512CHAR_MODE:u8 = 0b_0000_1000;

pub fn convert_to_xb(buf: &Buffer) -> Vec<u8>
{
    let mut result = Vec::new();

    result.extend_from_slice(b"XBIN");
    result.push(26); // EOF char
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
    result.push(flags);

    if let Some(palette) = &buf.custom_palette {
        result.extend(palette);
    }

    if let Some(font) = &buf.font {
        let vec: Vec<u8> = font.data.iter().map(|x| *x as u8).collect();
        result.extend(&vec);
    }

    // store uncompressed
    for y in 0..buf.height {
        for x in 0..buf.width {
            let ch = buf.get_char(Position::from(x as i32, y as i32));
            result.push(ch.char_code);
            result.push(ch.attribute.as_u8());
        }
    }
    
    result
}