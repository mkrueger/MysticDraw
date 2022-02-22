use std::{
    cmp::max,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

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
            self.base_layer.lines.resize(pos.y + 1, Line::new());
            self.base_layer.height = pos.y + 1;
        }
        self.base_layer.width = max(self.base_layer.width, pos.x + 1);

        let cur_line = &mut self.base_layer.lines[pos.y];
        cur_line.chars.resize(pos.x + 1, DosChar::new());
        cur_line.chars[pos.x] = dos_char;
    }

    pub fn get_char(&self, pos: &Position) -> DosChar {
        if pos.y >= self.base_layer.height {
            return DosChar::new();
        }

        let cur_line = &self.base_layer.lines[pos.y];
        if pos.x >= cur_line.chars.len() {
            DosChar::new()
        } else {
            cur_line.chars[pos.x]
        }
    }

    pub fn load_buffer(file_name: PathBuf) -> io::Result<Buffer> {
        let mut f = File::open(file_name.clone())?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;

        Ok(Buffer::from_bytes(file_name, &bytes))
    }

    pub fn from_bytes(file_name: PathBuf, bytes: &[u8]) -> Buffer {
        let mut result = Buffer {
            file_name: Box::new(file_name),
            base_layer: Layer::new(),
            font: None,
            layers: Vec::new(),
            sauce: None,
        };
        let mut data = LoadData::new();

        for b in bytes {
            if *b == EOF {
                break;
            }
            let mut ch = display_ansi(&mut data, *b);
            ch = display_PCBoard(&mut data, ch);

            let mut avt_result = display_avatar(&mut data, ch);
            let ch = avt_result.0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi_sequence() {
        let buf = Buffer::from_bytes(PathBuf::from("test"), b"[0;40;37mFoo-[1mB[0ma[35mr");
       
       assert_eq!(1, buf.base_layer.height);
       assert_eq!(7, buf.base_layer.width); // 'Foo-Bar'
        
       let line = &buf.base_layer.lines[0];
       assert_eq!(b'F', line.chars[0].char_code);
       assert_eq!(7, line.chars[0].attribute);
       assert_eq!(b'o', line.chars[1].char_code);
       assert_eq!(7, line.chars[1].attribute);
       assert_eq!(b'o', line.chars[2].char_code);
       assert_eq!(7, line.chars[2].attribute);
       assert_eq!(b'-', line.chars[3].char_code);
       assert_eq!(7, line.chars[3].attribute);
       assert_eq!(b'B', line.chars[4].char_code);
       assert_eq!(15, line.chars[4].attribute);
       assert_eq!(b'a', line.chars[5].char_code);
       assert_eq!(7, line.chars[5].attribute);
       assert_eq!(b'r', line.chars[6].char_code);
       assert_eq!(5, line.chars[6].attribute);
    }

    #[test]
    fn test_ansi_30() {
        let buf = Buffer::from_bytes(PathBuf::from("test"), b"[1;35mA[30mB[0mC");
       
       let line = &buf.base_layer.lines[0];
       assert_eq!(b'A', line.chars[0].char_code);
       assert_eq!(13, line.chars[0].attribute);
       assert_eq!(b'B', line.chars[1].char_code);
       assert_eq!(8, line.chars[1].attribute);
       assert_eq!(b'C', line.chars[2].char_code);
       assert_eq!(7, line.chars[2].attribute);
    }

    #[test]
    fn test_bg_colorrsequence() {
        let buf = Buffer::from_bytes(PathBuf::from("test"), b"[1;30m1[0;34m2[33m3[1;41m4[40m5[43m6[40m7");
       
       let line = &buf.base_layer.lines[0];
       assert_eq!(b'1', line.chars[0].char_code);
       assert_eq!(8, line.chars[0].attribute);
       assert_eq!(b'2', line.chars[1].char_code);
       assert_eq!(1, line.chars[1].attribute);
       assert_eq!(b'3', line.chars[2].char_code);
       assert_eq!(6, line.chars[2].attribute);
       assert_eq!(b'4', line.chars[3].char_code);
       assert_eq!(14 + (4 << 4), line.chars[3].attribute);
       assert_eq!(b'5', line.chars[4].char_code);
       assert_eq!(14, line.chars[4].attribute);
       assert_eq!(b'6', line.chars[5].char_code);
       assert_eq!(14 + (6 << 4), line.chars[5].attribute);
       assert_eq!(b'7', line.chars[6].char_code);
       assert_eq!(14, line.chars[6].attribute);
    }

}


