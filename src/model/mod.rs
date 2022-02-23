use std::{
    cmp::max,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

mod text_attribute;
pub use text_attribute::*;

mod dos_char;
pub use dos_char::*;

mod layer;
pub use layer::*;

mod position;
pub use position::*;

mod buffer;
pub use  buffer::*;

mod load;
pub use load::*;

/// Starts most of the useful sequences, terminated by a byte in the range 0x40 through 0x7E
const EOF: u8 = 26;


impl Buffer {
    pub fn set_char(&mut self, pos: &Position, dos_char: DosChar) {
        if pos.y as usize >= self.base_layer.lines.len() {
            self.base_layer.lines.resize(pos.y as usize + 1, Line::new());
            self.height = pos.y as usize + 1;
            self.base_layer.height = self.height;
        }
        self.width = max(self.width, pos.x as usize + 1);
        self.base_layer.width = self.width;

        let cur_line = &mut self.base_layer.lines[pos.y as usize];
        cur_line.chars.resize(pos.x as usize + 1, DosChar::new());
        cur_line.chars[pos.x as usize] = dos_char;
    }

    pub fn get_char(&self, pos: &Position) -> DosChar {
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

    pub fn load_buffer(file_name: PathBuf) -> io::Result<Buffer> {
        let mut f = File::open(file_name.clone())?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;

        Ok(Buffer::from_bytes(file_name, &bytes))
    }

    pub fn from_bytes(file_name: PathBuf, bytes: &[u8]) -> Buffer {
        let mut result = Buffer::new();
        result.file_name = Some(file_name);
        let mut data = LoadData::new();

        for b in bytes {
            let mut ch = display_ansi(&mut data, *b);
            ch = display_PCBoard(&mut data, ch);
            let mut avt_result = display_avatar(&mut data, ch);
            let ch = avt_result.0;
            if ch == EOF {
                break;
            }
            Buffer::output_char(&mut result, &mut data, ch);
            while avt_result.1 {
                avt_result = display_avatar(&mut data, 0);
                Buffer::output_char(&mut result, &mut data, avt_result.0);
            }
        }
        result
    }

    fn output_char(result: &mut Buffer, data: &mut LoadData, ch: u8) {
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
                    if data.cur_pos.x > 79 {
                        data.cur_pos.x = 0;
                        data.cur_pos.y += 1;
                    }
                    result.set_char(
                        &data.cur_pos,
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

