use std::io;

use crate::model::{Buffer, DosChar};
use super::{ Position, TextAttribute};

// http://fileformats.archiveteam.org/wiki/TUNDRA

const TUNDRA_VER: u8 = 24;
const TUNDRA_HEADER: &[u8] = b"TUNDRA24";

const TUNDRA_POSITION:u8 = 1;
const TUNDRA_COLOR_FOREGROUND:u8 = 2;
const TUNDRA_COLOR_BACKGROUND:u8 = 4;

pub fn read_tnd(result: &mut Buffer, bytes: &[u8], file_size: usize, screen_width: i32) -> io::Result<bool>
{
    if file_size <  1 + TUNDRA_HEADER.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid TND - file too short"));
    }
    let mut o = 1;

    let header = &bytes[1..=TUNDRA_HEADER.len()];

    if header != TUNDRA_HEADER  {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid TND wrong ID"));
    }
    o += TUNDRA_HEADER.len();

    result.width = screen_width as usize;
    result.palette.clear();
    result.palette.get_color(0, 0, 0);

    let mut pos = Position::new();
    let mut attr = TextAttribute::from_u8(0);

    while o < file_size {
        println!("{}", pos);
        let mut cmd = bytes[o];
        o += 1;
        if cmd == TUNDRA_POSITION {
            pos.y = to_u32(&bytes[o..]);
            if pos.y >= (u16::MAX) as i32 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid TND - jump position out of bounds ({} height is {} maximum is 65k LOC)", pos.x, result.height)));
            }
            o += 4;
            pos.x = to_u32(&bytes[o..]);
            if pos.x >= result.width as i32 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid TND - jump position out of bounds ({} width is {})", pos.x, result.width)));
            }

            o += 4;
            continue;
        } 
        
        if cmd > 1 && cmd <= 6 {
            let ch = bytes[o];
            o += 1;
        
            if cmd & TUNDRA_COLOR_FOREGROUND  != 0 {
                o += 1;
                let r = bytes[o];
                o += 1;
                let g = bytes[o];
                o += 1;
                let b = bytes[o];
                o += 1;
                attr.set_foreground(result.palette.get_color(r, g, b));
            }
            if cmd & TUNDRA_COLOR_BACKGROUND  != 0 {
                o += 1;
                let r = bytes[o];
                o += 1;
                let g = bytes[o];
                o += 1;
                let b = bytes[o];
                o += 1;
                attr.set_background(result.palette.get_color(r, g, b));
            }
            cmd = ch;
        }
        result.set_char(0, pos, Some(DosChar { 
            char_code: cmd, 
            attribute: attr
        }));
        advance_pos(result, &mut pos);
    }
    result.set_height_for_pos(pos);
    result.palette.fill_to_16();

    Ok(true)    
}

fn advance_pos(result: &Buffer, pos: &mut Position) -> bool
{
    pos.x += 1;
    if pos.x >= result.width as i32 {
        pos.x = 0;
        pos.y += 1;
    }
    true
}

fn to_u32(bytes: &[u8]) -> i32 {
    bytes[3] as i32 |
    (bytes[2] as i32) << 8 |
    (bytes[1] as i32) << 16 |
    (bytes[0] as i32) << 24
}

const TND_GOTO_BLOCK_LEN: i32 = 1 + 2 * 4;

pub fn convert_to_tnd(buf: &Buffer) -> io::Result<Vec<u8>>
{
    let mut result = vec![TUNDRA_VER]; // version
    result.extend(TUNDRA_HEADER);
    let mut attr = TextAttribute::from_u8(0);
    let mut skip_pos = None;
    for y in 0..buf.height {
        for x in 0..buf.width {
            let pos = Position::from(x as i32, y as i32);
            let ch = buf.get_char(pos);
            if ch.is_none() {
                if skip_pos.is_none() { skip_pos = Some(pos) }
                continue;
            }
            let ch = ch.unwrap();
            if ch.is_transparent() && attr.get_background() == 0 {
                if skip_pos.is_none() { skip_pos = Some(pos) }
                continue;
            }

            if let Some(pos2) = skip_pos {
                let skip_len = (pos.x + pos.y * buf.width as i32) - (pos2.x + pos2.y * buf.width as i32);
                if skip_len <= TND_GOTO_BLOCK_LEN  {
                    result.resize(result.len() + skip_len as usize, 0);
                } else {
                    result.push(1);
                    result.extend(i32::to_be_bytes(pos.y));
                    result.extend(i32::to_be_bytes(pos.x));
                }
                skip_pos = None;
            }
            if attr != ch.attribute {
                let mut cmd = 0; 
                if attr.get_foreground() != ch.attribute.get_foreground() { cmd |= 2 }
                if attr.get_background() != ch.attribute.get_background() { cmd |= 4 }

                result.push(cmd);
                result.push(ch.char_code);
                if attr.get_foreground() != ch.attribute.get_foreground() { 
                    let rgb = buf.palette.colors[ch.attribute.get_foreground() as usize].get_rgb();
                    result.push(0); 
                    result.push(rgb.0); 
                    result.push(rgb.1); 
                    result.push(rgb.2); 
                }
                if attr.get_background() != ch.attribute.get_background() { 
                    let rgb = buf.palette.colors[ch.attribute.get_background() as usize].get_rgb();
                    result.push(0); 
                    result.push(rgb.0); 
                    result.push(rgb.1); 
                    result.push(rgb.2); 
                }
                attr = ch.attribute;
                continue;
            }
            if ch.char_code >= 1 && ch.char_code <= 6 {
                // fake color change
                result.push(2);
                result.push(ch.char_code);

                let rgb = buf.palette.colors[attr.get_foreground() as usize].get_rgb();
                result.push(0); 
                result.push(rgb.0); 
                result.push(rgb.1); 
                result.push(rgb.2); 
                continue;
            }
            result.push(ch.char_code);
        }
    }
    if let Some(pos2) = skip_pos {
        let pos = Position::from((buf.width - 1) as i32, (buf.height - 1) as i32);

        let skip_len = (pos.x + pos.y * buf.width as i32) - (pos2.x + pos2.y * buf.width as i32) + 1;
        result.resize(result.len() + skip_len as usize, 0);
    }
    if buf.sauce.is_some() || buf.width != 80 {
        crate::model::Sauce::generate(buf, &crate::model::SauceFileType::TundraDraw)?;
    }
    Ok(result)
}