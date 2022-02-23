use crate::model::{DEFAULT_ATTRIBUTE, display_ans, display_avt, display_PCBoard, DosChar, Line, ParseStates, Position, read_binary};
use std::{
    cmp::{max, min},
    fs::File,
    io::{self, Read},
    path::{PathBuf, Path},
};
use std::ffi::OsStr;

use super::{Layer, read_xbin};
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
            font: None,
            layers: Vec::new(),
            sauce: None,
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
        println!("ans{} avt{} pcb{}", parse_ansi, parse_avt, parse_pcb);
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
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer::new()
    }
}
