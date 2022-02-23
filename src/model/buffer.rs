use crate::model::{DEFAULT_ATTRIBUTE, display_ans, display_avt, display_PCBoard, DosChar, Line, ParseStates, Position, read_binary};
use std::{
    cmp::{max, min},
    fs::File,
    io::{self, Read},
    path::{PathBuf, Path},
};
use std::ffi::OsStr;

use super::{Layer, read_xbin, DOS_DEFAULT_PALETTE};
use crate::sauce::{read_sauce, Sauce, SauceDataType};

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct BitFont {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u16>,
}

#[derive(Debug)]
pub struct Buffer {
    pub file_name: Option<PathBuf>,

    pub width: usize,
    pub height: usize,
    pub custom_palette: Option<Vec<u8>>,
    pub custom_font: Option<Vec<u8>>,
    pub base_layer: Layer,

    pub font_dimensions: Position,
    pub font: Option<BitFont>,
    pub layers: Vec<Layer>,
    pub sauce: Option<Sauce>,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            file_name: None,
            width: 0,
            height: 0,
            custom_palette: None,
            custom_font: None,
            base_layer: Layer::new(),
            font_dimensions: Position::new(),
            font: None,
            layers: Vec::new(),
            sauce: None,
        }
    }

    pub fn get_font_scanline(&self, ch: u8, y: usize) -> u8
    {
        if let Some(font) = &self.custom_font {
            font[ch as usize * self.font_dimensions.y as usize + y]
        } else {
            crate::DEFAULT_FONT[ch as usize * 16 + y]
        }
    }

    pub fn get_font_dimensions(&self) -> Position
    {
        if self.custom_font.is_some() {
            self.font_dimensions
        } else {
            // default font.
            Position::from(8, 16)
        }
    }

    pub fn set_char(&mut self, pos: Position, dos_char: DosChar) {
        if pos.y >= self.base_layer.lines.len() as i32 {
            self.base_layer.lines.resize(pos.y as usize + 1, Line::new());
            self.height = max(self.height, pos.y as usize + 1);
            self.base_layer.height = self.height;
        }
        self.width = max(self.width, pos.x as usize + 1);
        self.base_layer.width = self.width;

        let cur_line = &mut self.base_layer.lines[pos.y as usize];
        cur_line.chars.resize(pos.x as usize + 1, DosChar::new());
        cur_line.chars[pos.x as usize] = dos_char;
    }

    pub fn get_char(&self, pos: Position) -> DosChar {
        if pos.y >= self.base_layer.lines.len() as i32 {
            return DosChar::new();
        }

        let cur_line = &self.base_layer.lines[pos.y as usize];
        if pos.x >= cur_line.chars.len() as i32 {
            DosChar::new()
        } else {
            cur_line.chars[pos.x as usize]
        }
    }

    pub fn load_buffer(file_name: &Path) -> io::Result<Buffer> {

        let sauce_info = read_sauce(file_name)?;
        let mut f = File::open(file_name)?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;

        Ok(Buffer::from_bytes(file_name, &sauce_info, &bytes))
    }

    pub fn from_bytes(file_name: &Path, sauce_info: &Option<Sauce>, bytes: &[u8]) -> Buffer {
        let mut result = Buffer::new();
        result.file_name = Some(file_name.to_path_buf());
        let mut data = ParseStates::new();

        let mut screen_width = 0;
        let mut file_size = bytes.len();
        if let Some(sauce) = &sauce_info {
            file_size = min(file_size, sauce.file_size as usize);
            match sauce.data_type {
                SauceDataType::Character => {
                    if sauce.t_info1 > 0 {
                        screen_width = sauce.t_info1 as i32;
                    }
                }
                SauceDataType::BinaryText => {
                    if sauce.file_type > 0 {
                        screen_width = (sauce.file_type as i32) * 2;
                    }
                }
                _ => {}
            }
        }
        let ext = file_name.extension();
        let mut parse_avt  = false;
        let mut parse_pcb  = false;
        let mut parse_ansi = false;
        if let Some(ext) = ext {
            let ext = OsStr::to_str(ext).unwrap().to_lowercase();
            match ext.as_str() {
                "bin" => {
                    if screen_width == 0 { screen_width = 160; }
                    read_binary(&mut result, bytes, file_size, screen_width);
                    return result;
                }
                "xb" => {
                    if screen_width == 0 { screen_width = 160; }
                    read_xbin(&mut result, bytes, file_size, screen_width);
                    return result;
                }
                "ans" => { parse_ansi = true; }
                "avt" => { parse_avt = true;  }
                "pcb" => { parse_pcb = true; parse_ansi = true; }
                _ => {}
            }
        }
        if screen_width == 0 { screen_width = 80; }
        for b in bytes.iter().take(file_size) {
            let mut ch = *b;
            if parse_ansi {
                ch = display_ans(&mut data, ch);
            }
            if parse_pcb {
                ch = display_PCBoard(&mut data, ch);
            }

            if parse_avt {
                let mut avt_result = display_avt(&mut data, ch);
                let ch = avt_result.0;
                Buffer::output_char(&mut result, screen_width, &mut data, ch);
                while avt_result.1 {
                    avt_result = display_avt(&mut data, 0);
                    Buffer::output_char(&mut result, screen_width, &mut data, avt_result.0);
                }
            } else {
                Buffer::output_char(&mut result, screen_width, &mut data, ch);
            }
        }
        result
    }

    fn output_char(result: &mut Buffer, screen_width : i32, data: &mut ParseStates, ch: u8) {
        if ch != 0 {
            match ch {
                10 => {
                    data.cur_pos.x = 0;
                    data.cur_pos.y += 1;
                }
                12 => {
                    data.cur_pos.x = 0;
                    data.cur_pos.y = 1;
                    data.text_attr = DEFAULT_ATTRIBUTE;
                }
                13 => {
                    data.cur_pos.x = 0;
                }
                _ => {
                    if data.cur_pos.x >= screen_width {
                        data.cur_pos.x = 0;
                        data.cur_pos.y += 1;
                    }
                    result.set_char(
                        data.cur_pos,
                        DosChar {
                            char_code: ch,
                            attribute: data.text_attr,
                        },
                    );
                    data.cur_pos.x += 1;
                }
            }
        }
    }

    pub fn get_rgb_f64(&self, color: u8) -> (f64, f64, f64) {
        let rgb = self.get_rgb(color);
        (
            rgb.0 as f64 / 255_f64,
            rgb.1 as f64 / 255_f64,
            rgb.2 as f64 / 255_f64
        )
    }

    pub fn get_rgb(&self, color: u8) -> (u8, u8, u8) {
        debug_assert!(color <= 15);

        if let Some(pal) = &self.custom_palette  {
            let o = (color * 3) as usize;
            if o + 2 >= pal.len() {
                eprintln!("illegal palette color {}, palette is {} colors long.", color, pal.len() / 3);
                return (255, 0, 0);
            }

            return (
                pal[o] << 2,
                pal[o + 1] << 2,
                pal[o + 2] << 2
            );
        }
        
        let c = DOS_DEFAULT_PALETTE[color as usize];
        (
            c.0,
            c.1,
            c.2
        )
    }

}

impl Default for Buffer {
    fn default() -> Self {
        Buffer::new()
    }
}
