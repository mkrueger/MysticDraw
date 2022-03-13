use std::{io, cmp::min};

use crate::model::{Buffer, DosChar, BitFont, Palette, SauceString};

use super::{ Position, TextAttribute };

const XBIN_HEADER_SIZE:usize = 11;

const FLAG_PALETTE:u8        = 0b_0000_0001;
const FLAG_FONT:u8           = 0b_0000_0010;
const FLAG_COMPRESS:u8       = 0b_0000_0100;
const FLAG_NON_BLINK_MODE:u8 = 0b_0000_1000;
const FLAG_512CHAR_MODE:u8   = 0b_0001_0000;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum Compression {
    Off  = 0b0000_0000,
    Char = 0b0100_0000,
    Attr = 0b1000_0000,
    Full = 0b1100_0000,
}

pub fn read_xb(result: &mut Buffer, bytes: &[u8], file_size: usize) -> io::Result<bool>
{
    if file_size < XBIN_HEADER_SIZE {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid XBin.\nFile too short."));
    }
    if b"XBIN" != &bytes[0..4] {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid XBin.\nID doesn't match."));
    }

    let mut o = 4;

    // let eof_char = bytes[o];
    o += 1;
    result.width = bytes[o] as u16 + ((bytes[o + 1] as u16) << 8);
    o += 2;
    result.height = bytes[o] as u16 + ((bytes[o + 1] as u16) << 8);
    o += 2;

    let font_size = bytes[o];
    o += 1;
    let flags = bytes[o];
    o += 1;

    let has_custom_palette    = (flags & FLAG_PALETTE) == FLAG_PALETTE;
    let has_custom_font       = (flags & FLAG_FONT) == FLAG_FONT;
    let is_compressed         = (flags & FLAG_COMPRESS) == FLAG_COMPRESS;
    result.use_ice                 = (flags & FLAG_NON_BLINK_MODE) == FLAG_NON_BLINK_MODE;
    result.use_512_chars           = (flags & FLAG_512CHAR_MODE) == FLAG_512CHAR_MODE;
    
    if has_custom_palette {
        result.palette = Palette::from(&bytes[o..(o + 48)]);
        o += 48;
    }
    if has_custom_font {
        let font_length = font_size as usize * if result.use_512_chars { 512 } else { 256 };
        result.font = BitFont::create_8(SauceString::new(), result.use_512_chars, 8, font_size, &bytes[o..(o+font_length)]);
        o += font_length;
    }

    if is_compressed {
        read_data_compressed(result, &bytes[o..], file_size - o)
    } else {
        read_data_uncompressed(result, &bytes[o..], file_size - o)
    }
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

fn read_data_compressed(result: &mut Buffer, bytes: &[u8], file_size: usize) -> io::Result<bool>
{
    let mut pos = Position::new();
    let mut o = 0;
    while o < file_size {
        let xbin_compression = bytes[o];
        if o > file_size {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Invalid XBin.\nRead block start at EOF."));
        }

        o += 1;
        let compression = unsafe { std::mem::transmute(xbin_compression & 0b_1100_0000) };
        let repeat_counter = (xbin_compression & 0b_0011_1111) + 1;

        match compression {
            Compression::Off => {
                for _ in 0..repeat_counter {
                    if o + 2 > bytes.len() { 
                        eprintln!("Invalid XBin. Read char block beyond EOF.");
                        break;
                    }
                    result.set_char(0, pos, Some(DosChar { 
                        char_code: bytes[o], 
                        attribute: TextAttribute::from_u8(bytes[o + 1])
                    }));
                    o += 2;
                    if !advance_pos(result, &mut pos) {
                        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "data out of bounds"));
                    }
                }
            }
            Compression::Char => {
                let char_code = bytes[o];
                o += 1;
                for _ in 0..repeat_counter {
                    if o + 1 > bytes.len() {
                        eprintln!("Invalid XBin. Read char compression block beyond EOF.");
                        break;
                    }
                    result.set_char(0, pos, Some(DosChar { 
                        char_code, 
                        attribute: TextAttribute::from_u8(bytes[o])
                    }));
                    o += 1;
                    if !advance_pos(result, &mut pos) {
                        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "data out of bounds"));
                    }
                }
            }
            Compression::Attr => {
                let attribute = TextAttribute::from_u8(bytes[o]);
                o += 1;
                for _ in 0..repeat_counter {
                    if o + 1 > bytes.len() {
                        eprintln!("Invalid XBin. Read attribute compression block beyond EOF.");
                        break;
                    }
                    result.set_char(0, pos, Some(DosChar { 
                        char_code: bytes[o], 
                        attribute
                    }));
                    o += 1;
                    if !advance_pos(result, &mut pos) {
                        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "data out of bounds"));
                    }
                }
            }
            Compression::Full => {
                let char_code = bytes[o];
                o += 1;
                if o + 1 > bytes.len() { 
                    eprintln!("Invalid XBin. nRead compression block beyond EOF.");
                    break;
                }
                let attr = TextAttribute::from_u8(bytes[o]);
                o += 1;
                let rep_ch = Some(DosChar { 
                    char_code, 
                    attribute: attr
                });

                for _ in 0..repeat_counter {
                    result.set_char(0, pos, rep_ch);
                    if !advance_pos(result, &mut pos) {
                        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "data out of bounds"));
                    }
                }
            }
        }
    }

    Ok(true)
}

fn read_data_uncompressed(result: &mut Buffer, bytes: &[u8], file_size: usize) -> io::Result<bool>
{
    let mut pos = Position::new();
    let mut o = 0;
    while o < file_size {
        if o + 1 > file_size {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Invalid XBin.\n Uncompressed data length needs to be % 2 == 0"));
        }
        result.set_char(0, pos, Some(DosChar { 
            char_code: bytes[o], 
            attribute: TextAttribute::from_u8(bytes[o + 1])
        }));
        o += 2;
        if !advance_pos(result, &mut pos) {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "data out of bounds"));
        }
    }

    Ok(true)
}

pub fn convert_to_xb(buf: &Buffer) -> io::Result<Vec<u8>>
{
    let mut result = Vec::new();

    result.extend_from_slice(b"XBIN");
    result.push(0x1A); // CP/M EOF char (^Z) - used by DOS as well

    result.push(buf.width as u8);
    result.push((buf.width >> 8) as u8);
    result.push(buf.height as u8);
    result.push((buf.height >> 8) as u8);

    let mut flags = 0;
    if buf.font.size.width != 8 || buf.font.size.height < 1 || buf.font.size.height > 32 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "font not supported by the .xb format only fonts with 8px width and a height from 1 to 32 are supported."));
    }

    result.push(buf.font.size.height as u8);
    if !buf.font.is_default() {
        flags |= FLAG_FONT;
    }

    if !buf.palette.is_default() {
        flags |= FLAG_PALETTE;
    }
    
    flags |= FLAG_COMPRESS;

    if buf.use_ice {
        flags |= FLAG_NON_BLINK_MODE;
    }

    if buf.use_512_chars {
        flags |= FLAG_512CHAR_MODE;
    }
    result.push(flags);
    
    if (flags & FLAG_PALETTE) == FLAG_PALETTE {
        result.extend(buf.palette.to_16color_vec());
    }

    if flags & FLAG_FONT == FLAG_FONT {
        buf.font.push_u8_data(&mut result);
    }

    if (flags & FLAG_COMPRESS) == FLAG_COMPRESS  {
        compress_backtrack(&mut result, buf);
    } else {
        // store uncompressed
        for y in 0..buf.height {
            for x in 0..buf.width {
                let ch = buf.get_char(Position::from(x as i32, y as i32)).unwrap_or_default();
                result.push(ch.char_code);
                result.push(ch.attribute.as_u8());
            }
        }
    }
    if buf.write_sauce  {
        buf.write_sauce_info(&crate::model::SauceFileType::XBin, &mut result)?;
    }

    Ok(result)
}

/* 
fn compress_greedy(outputdata: &mut Vec<u8>, buffer: &Buffer)
{
    let mut run_mode = Compression::Off;
    let mut run_count = 0;
    let mut run_buf = Vec::new();
    let mut run_ch = DosChar::default();
    let len = (buffer.height * buffer.width) as i32;
    for x in 0..len {
        let cur = buffer.get_char(Position::from_index(buffer, x)).unwrap_or_default();

        let next = if x < len - 1 {
            buffer.get_char(Position::from_index(buffer, x + 1)).unwrap_or_default()
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
                            let next2 = buffer.get_char(Position::from_index(buffer, x + 2)).unwrap_or_default();
                            end_run = cur.char_code == next.char_code && cur.char_code == next2.char_code ||
                                      cur.attribute == next.attribute && cur.attribute == next2.attribute;
                        }
                    }
                    Compression::Char => {
                        if cur.char_code != run_ch.char_code {
                            end_run = true;
                        } else if x < len - 3 {
                            let next2 = buffer.get_char(Position::from_index(buffer, x + 2)).unwrap_or_default();
                            let next3 = buffer.get_char(Position::from_index(buffer, x + 3)).unwrap_or_default();
                            end_run = cur == next && cur == next2 && cur == next3;
                        }
                    }
                    Compression::Attr => {
                        if cur.attribute != run_ch.attribute {
                            end_run = true;
                        } else if x < len - 3 {
                            let next2 = buffer.get_char(Position::from_index(buffer, x + 2)).unwrap_or_default();
                            let next3 = buffer.get_char(Position::from_index(buffer, x + 3)).unwrap_or_default();
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
}*/

fn count_length(mut run_mode: Compression, mut run_ch: DosChar, mut end_run: Option<bool>, mut run_count: u8, buffer: &Buffer, mut x: i32) -> i32 {
    let len = min(x + 256, (buffer.height * buffer.width) as i32 - 1);
    let mut count = 0;
    while x < len {
        let cur = buffer.get_char(Position::from_index(buffer, x)).unwrap_or_default();
        let next = buffer.get_char(Position::from_index(buffer, x + 1)).unwrap_or_default();
        
        if run_count > 0 {
            if end_run.is_none() {
                if run_count >= 64 {
                    end_run = Some(true);
                } else if run_count > 0 {
                    match run_mode {
                        Compression::Off => {
                            if x < len - 2 && cur == next {
                                end_run = Some(true);
                            }
                            else if x < len - 2 {
                                let next2 = buffer.get_char(Position::from_index(buffer, x + 2)).unwrap_or_default();
                                end_run = Some(cur.char_code == next.char_code && cur.char_code == next2.char_code ||
                                          cur.attribute == next.attribute && cur.attribute == next2.attribute);
                            }
                        }
                        Compression::Char => {
                            if cur.char_code != run_ch.char_code {
                                end_run = Some(true);
                            } else if x < len - 3 {
                                let next2 = buffer.get_char(Position::from_index(buffer, x + 2)).unwrap_or_default();
                                let next3 = buffer.get_char(Position::from_index(buffer, x + 3)).unwrap_or_default();
                                end_run = Some(cur == next && cur == next2 && cur == next3);
                            }
                        }
                        Compression::Attr => {
                            if cur.attribute != run_ch.attribute {
                                end_run = Some(true);
                            } else if x < len - 3 {
                                let next2 = buffer.get_char(Position::from_index(buffer, x + 2)).unwrap_or_default();
                                let next3 = buffer.get_char(Position::from_index(buffer, x + 3)).unwrap_or_default();
                                end_run = Some(cur == next && cur == next2 && cur == next3);
                            }
                        }
                        Compression::Full => {
                            end_run = Some(cur != run_ch);
                        }
                    }
                }
            }

            if let Some(true) = end_run {
                count += 1;
                run_count = 0;
            }
        }
        end_run = None;

        if run_count > 0 {
            match run_mode {
                Compression::Off => {
                    count += 2;
                }
                Compression::Char | Compression::Attr => {
                    count += 1;
                }
                Compression::Full => {
                    // nothing
                }    
            }
        }
        else
        {
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
            count += 2;
            run_ch = cur;
            end_run = None;
        }
        run_count += 1;
        x += 1;
    }
    count
}

fn compress_backtrack(outputdata: &mut Vec<u8>, buffer: &Buffer)
{
    let mut run_mode = Compression::Off;
    let mut run_count = 0;
    let mut run_buf = Vec::new();
    let mut run_ch = DosChar::default();
    let len = (buffer.height * buffer.width) as i32;
    for x in 0..len {
        let cur = buffer.get_char(Position::from_index(buffer, x)).unwrap_or_default();

        let next = if x < len - 1 {
            buffer.get_char(Position::from_index(buffer, x + 1)).unwrap_or_default()
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
                        if x < len - 2 && (cur.char_code == next.char_code || cur.attribute == next.attribute) {
                            let l1 = count_length(run_mode, run_ch, Some(true), run_count, buffer, x);
                            let l2 = count_length(run_mode, run_ch, Some(false), run_count, buffer, x);
                            end_run = l1 < l2;
                        }
                    }
                    Compression::Char => {
                        if cur.char_code != run_ch.char_code {
                            end_run = true;
                        } else if x < len - 4 {
                            let next2 = buffer.get_char(Position::from_index(buffer, x + 2)).unwrap_or_default();
                            if cur.attribute == next.attribute && cur.attribute == next2.attribute {
                                let l1 = count_length(run_mode, run_ch, Some(true), run_count, buffer, x);
                                let l2 = count_length(run_mode, run_ch, Some(false), run_count, buffer, x);
                                end_run = l1 < l2;
                            }
                        }
                    }
                    Compression::Attr => {
                        if cur.attribute != run_ch.attribute {
                            end_run = true;
                        } else if x < len - 3 {
                            let next2 = buffer.get_char(Position::from_index(buffer, x + 2)).unwrap_or_default();
                            if cur.char_code == next.char_code && cur.char_code == next2.char_code  {
                                let l1 = count_length(run_mode, run_ch, Some(true), run_count, buffer, x);
                                let l2 = count_length(run_mode, run_ch, Some(false), run_count, buffer, x);
                                end_run = l1 < l2;
                            }
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
