use std::{path::Path, fs::File, io::{Read}};

use super::{Buffer, Position, TextAttribute, DosChar};

#[derive(Copy, Clone, Debug)]
enum TheDrawFontType {
    Outline,
    Block,
    Color
}

#[allow(dead_code)]
pub struct TheDrawFont
{
    name: String,
    font_type: TheDrawFontType,
    pub spaces: i32,
    char_table: Vec<u16>,
    font_data: Vec<u8>
}

static THE_DRAW_FONT_ID : &[u8;18] = b"TheDraw FONTS file";
const THE_DRAW_FONT_HEADER_SIZE: usize = 233;

impl TheDrawFont
{
    pub fn load(file_name: &Path) -> Option<TheDrawFont>
    {
        let mut f = File::open(file_name).expect("error while opening file");
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes).expect("error while reading file");

        if bytes.len() < THE_DRAW_FONT_HEADER_SIZE  {
            eprintln!("no ttf file - file too short");
            return None;
        }
        
        if bytes[0] != 19 || THE_DRAW_FONT_ID != &bytes[1..19] {
            eprintln!("no ttf file - wrong id");
            return None;
        }
        // skip data
        
        let mut o = 24;
        let font_name_len = bytes[o] as usize;
        o += 1;
        if font_name_len > 16 {
            eprintln!("invalid ttf font - name length was: {}", font_name_len);
            return None;
        }
        let name = String::from_utf8_lossy(&bytes[o..(o + font_name_len)]).to_string();
        o = 41;
        #[allow(clippy::match_on_vec_items)]
        let font_type = match bytes[o] {
            0 => TheDrawFontType::Outline,
            1 => TheDrawFontType::Block,
            2 => TheDrawFontType::Color,
            _ => { 
                eprintln!("unsupported ttf font type {}", bytes[o]);
                return None;
            }
        };
        o += 1;
        let spaces = bytes[o] as i32;
        o += 1;
        let font_data_size = bytes[o] as u16 | ((bytes[o + 1] as u16) << 8);
        o += 2;

        let mut char_table= Vec::new();
    	for _ in 0..94 {
            let cur_char = bytes[o] as u16 | ((bytes[o + 1] as u16) << 8);
            // println!("{}:{:>04X}", char::from_u32((i as u32 ) + (b' ' as u32) + 1).unwrap(),  cur_char);
            char_table.push(cur_char);
            o += 2;
        }
        debug_assert!(o == THE_DRAW_FONT_HEADER_SIZE);
        let font_data= bytes[o..(o + font_data_size as usize)].to_vec();

        Some(TheDrawFont {
            name,
            font_type,
            spaces,
            char_table,
            font_data
        })
    }
    
    fn transform_outline(ch: u8) -> u8
    {
//        let currentOutline = 0;
        if ch - 64 > 0 && ch - 64 <= 17 {
           TheDrawFont::OUTLINE_CHAR_SET[0][(ch - 65) as usize ]
        } else {
            b' '
        }
    }

    pub fn get_font_height(&self) -> u8
    {
        self.font_data[1]
    }
    
    pub fn render(&self, buf: &mut Buffer, pos: Position, color: TextAttribute, char_code: u8) -> i32
    {
        let char_offset = (char_code as i32) - b' '  as i32 - 1;
        if char_offset < 0 || char_offset > self.char_table.len() as i32 {
            return -1;
        }
        let mut char_offset = self.char_table[char_offset as usize] as usize;
        if char_offset == 0xFFFF {
            return -1;
        }
        let max_x = self.font_data[char_offset];
        char_offset += 1;
        // let max_y = self.font_data[char_offset];
        char_offset += 1;
        let mut x = pos.x;
        let mut y = pos.y;
        loop {
            let ch = self.font_data[char_offset];
            char_offset += 1;
            if ch == 0 { break; }
            if ch == 13 {
                x = pos.x;
                y += 1;
            } else {
                let dest_pos = Position::from(x, y);
                let dos_char = match self.font_type {
                    TheDrawFontType::Outline => {
                        DosChar { char_code: TheDrawFont::transform_outline(ch), attribute: color }
                    }
                    TheDrawFontType::Block => {
                        DosChar { char_code: ch, attribute: color }
                    }
                    TheDrawFontType::Color => {
                        let ch_attr = TextAttribute::from_u8(self.font_data[char_offset]);
                        char_offset += 1;
                        DosChar { char_code: ch, attribute: ch_attr }
                    }
                };
                if dest_pos.x >= 0 && dest_pos.y >= 0 && dest_pos.x < buf.width as i32 && dest_pos.y < buf.height as i32 {
                    buf.set_char(dest_pos, dos_char);
                }
                x += 1;
            }
        }

        max_x as i32
    }

    const OUTLINE_CHAR_SET : [[u8; 17]; 19] = [
        [0xC4, 0xC4, 0xB3, 0xB3, 0xDA, 0xBF, 0xDA, 0xBF, 0xC0, 0xD9, 0xC0, 0xD9, 0xB4, 0xC3, 0x20, 0x20, 0x20],
        [0xCD, 0xC4, 0xB3, 0xB3, 0xD5, 0xB8, 0xDA, 0xBF, 0xD4, 0xBE, 0xC0, 0xD9, 0xB5, 0xC3, 0x20, 0x20, 0x20],
        [0xC4, 0xCD, 0xB3, 0xB3, 0xDA, 0xBF, 0xD5, 0xB8, 0xC0, 0xD9, 0xD4, 0xBE, 0xB4, 0xC6, 0x20, 0x20, 0x20],
        [0xCD, 0xCD, 0xB3, 0xB3, 0xD5, 0xB8, 0xD5, 0xB8, 0xD4, 0xBE, 0xD4, 0xBE, 0xB5, 0xC6, 0x20, 0x20, 0x20],
        [0xC4, 0xC4, 0xBA, 0xB3, 0xD6, 0xBF, 0xDA, 0xB7, 0xC0, 0xBD, 0xD3, 0xD9, 0xB6, 0xC3, 0x20, 0x20, 0x20],
        [0xCD, 0xC4, 0xBA, 0xB3, 0xC9, 0xB8, 0xDA, 0xB7, 0xD4, 0xBC, 0xD3, 0xD9, 0xB9, 0xC3, 0x20, 0x20, 0x20],
        [0xC4, 0xCD, 0xBA, 0xB3, 0xD6, 0xBF, 0xD5, 0xBB, 0xC0, 0xBD, 0xC8, 0xBE, 0xB6, 0xC6, 0x20, 0x20, 0x20],
        [0xCD, 0xCD, 0xBA, 0xB3, 0xC9, 0xB8, 0xD5, 0xBB, 0xD4, 0xBC, 0xC8, 0xBE, 0xB9, 0xC6, 0x20, 0x20, 0x20],
        [0xC4, 0xC4, 0xB3, 0xBA, 0xDA, 0xB7, 0xD6, 0xBF, 0xD3, 0xD9, 0xC0, 0xBD, 0xB4, 0xC7, 0x20, 0x20, 0x20],
        [0xCD, 0xC4, 0xB3, 0xBA, 0xD5, 0xBB, 0xD6, 0xBF, 0xC8, 0xBE, 0xC0, 0xBD, 0xB5, 0xC7, 0x20, 0x20, 0x20],
        [0xC4, 0xCD, 0xB3, 0xBA, 0xDA, 0xB7, 0xC9, 0xB8, 0xD3, 0xD9, 0xD4, 0xBC, 0xB4, 0xCC, 0x20, 0x20, 0x20],
        [0xCD, 0xCD, 0xB3, 0xBA, 0xD5, 0xBB, 0xC9, 0xB8, 0xC8, 0xBE, 0xD4, 0xBC, 0xB5, 0xCC, 0x20, 0x20, 0x20],
        [0xC4, 0xC4, 0xBA, 0xBA, 0xD6, 0xB7, 0xD6, 0xB7, 0xD3, 0xBD, 0xD3, 0xBD, 0xB6, 0xC7, 0x20, 0x20, 0x20],
        [0xCD, 0xC4, 0xBA, 0xBA, 0xC9, 0xBB, 0xD6, 0xB7, 0xC8, 0xBC, 0xD3, 0xBD, 0xB9, 0xC7, 0x20, 0x20, 0x20],
        [0xC4, 0xCD, 0xBA, 0xBA, 0xD6, 0xB7, 0xC9, 0xBB, 0xD3, 0xBD, 0xC8, 0xBC, 0xB6, 0xCC, 0x20, 0x20, 0x20],
        [0xCD, 0xCD, 0xBA, 0xBA, 0xC9, 0xBB, 0xC9, 0xBB, 0xC8, 0xBC, 0xC8, 0xBC, 0xB9, 0xCC, 0x20, 0x20, 0x20],
        [0xDC, 0xDC, 0xDB, 0xDB, 0xDC, 0xDC, 0xDC, 0xDC, 0xDB, 0xDB, 0xDB, 0xDB, 0xDB, 0xDB, 0x20, 0x20, 0x20],
        [0xDF, 0xDF, 0xDB, 0xDB, 0xDB, 0xDB, 0xDB, 0xDB, 0xDF, 0xDF, 0xDF, 0xDF, 0xDB, 0xDB, 0x20, 0x20, 0x20],
        [0xDF, 0xDC, 0xDE, 0xDD, 0xDE, 0xDD, 0xDC, 0xDC, 0xDF, 0xDF, 0xDE, 0xDD, 0xDB, 0xDB, 0x20, 0x20, 0x20],
    ];
}