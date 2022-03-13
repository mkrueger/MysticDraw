use std::{
    fs::File,
    io::{self, Read},
    path::{PathBuf, Path},
};
use std::ffi::OsStr;

use super::{Layer, read_xb, Position, DosChar,  ParseStates, read_binary, display_ans, display_PCBoard,  display_avt, TextAttribute, Size, UndoOperation, Palette, SauceString, Line, BitFont };

pub struct Buffer {
    pub file_name: Option<PathBuf>,
    pub file_name_changed: Box<dyn Fn ()>,

    pub title: SauceString<35, b' '>,
    pub author: SauceString<20, b' '>,
    pub group: SauceString<20, b' '>,
    pub comments: Vec<SauceString<64, 0>>,

    pub width: u16,
    pub height: u16,

    pub use_ice: bool,
    pub use_512_chars: bool,
    pub write_sauce: bool,

    pub palette: Palette,
    pub overlay_layer: Option<Layer>,

    pub font: BitFont,
    pub layers: Vec<Layer>,

    pub undo_stack: Vec<Box<dyn UndoOperation>>,
    pub redo_stack: Vec<Box<dyn UndoOperation>>,
}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer").field("file_name", &self.file_name).field("width", &self.width).field("height", &self.height).field("custom_palette", &self.palette).field("font", &self.font).field("layers", &self.layers).finish()
    }
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            file_name: None,
            width: 80,
            height: 25,

            title: SauceString::new(),
            author: SauceString::new(),
            group: SauceString::new(),
            comments: Vec::new(),

            use_ice: true,
            use_512_chars: false,
            write_sauce: false,

            palette: Palette::new(),

            font: BitFont::default(),
            overlay_layer: None,
            layers: vec!(Layer::new()),
            file_name_changed: Box::new(|| {}),
            undo_stack: Vec::new(),
            redo_stack: Vec::new()
        }
    }

    pub fn create(width: u16, height: u16) -> Self {
        let mut res = Buffer::new();
        res.width = width;
        res.height = height;
        res.layers[0].lines.resize(height as usize, Line::create(width));
        res.layers[0].is_locked = true;

        let mut editing_layer =Layer::new();
        editing_layer.title = "Editing".to_string();
        res.layers.insert(0, editing_layer);

        res
    }
    
    pub fn get_overlay_layer(&mut self) -> &mut Option<Layer>
    {
        if self.overlay_layer.is_none() {
            self.overlay_layer = Some(Layer::new());
        }

        &mut self.overlay_layer
    }

    pub fn remove_overlay(&mut self) -> Option<Layer>
    {
        std::mem::replace( &mut self.overlay_layer, None)
    }

    pub fn get_font_scanline(&self, ch: u16, y: usize) -> u32
    {
        self.font.get_scanline(ch, y)
    }

    pub fn get_font_dimensions(&self) -> Size<u8>
    {
        self.font.size
    }

    pub fn set_char(&mut self, layer: usize, pos: Position, dos_char: Option<DosChar>) {
        if layer >= self.layers.len() { return; }

        let cur_layer  = &mut self.layers[layer];
        cur_layer.set_char(pos, dos_char);
    }

    pub fn get_char_from_layer(&mut self, layer: usize, pos: Position) -> Option<DosChar> {
        if let Some(layer) = self.layers.get(layer) {
            layer.get_char(pos)
        } else {
            None
        }
    }

    pub fn get_char(&self, pos: Position) ->  Option<DosChar> {
        if let Some(overlay) = &self.overlay_layer  {
            let ch = overlay.get_char(pos);
            if ch.is_some() {
                return ch;
            }
        }

        for i in 0..self.layers.len() {
            let cur_layer = &self.layers[i];
            if !cur_layer.is_visible { continue; }
            let ch = cur_layer.get_char(pos);
            if ch.is_some() { return ch; }
        }

        None
    }

    pub fn load_buffer(file_name: &Path) -> io::Result<Buffer> {
        let mut f = File::open(file_name)?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;

        Buffer::from_bytes(file_name, &bytes)
    }

    pub fn clear_layer(&mut self, layer_num: i32) -> super::ClearLayerOperation {
        
        let layers = std::mem::take(&mut self.layers[layer_num as usize].lines);
        super::ClearLayerOperation {
            layer_num,
            lines: layers,
        }
    }

    pub fn to_bytes(&self, extension: &str) -> io::Result<Vec<u8>>
    {
        match extension {
            "mdf" => super::convert_to_mdf(self),
            "bin" => super::convert_to_binary(self),
            "xb" => super::convert_to_xb(self),
            "ice" |
            "ans" => super::convert_to_ans(self),
            "avt" => super::convert_to_avt(self),
            "pcb" => super::convert_to_pcb(self),
            "adf" => super::convert_to_adf(self),
            "idf" => super::convert_to_idf(self),
            "tnd" => super::convert_to_tnd(self),
            _ => super::convert_to_asc(self)
        }
    }

    pub fn from_bytes(file_name: &Path, bytes: &[u8]) -> io::Result<Buffer> {
        let mut result = Buffer::new();
        result.file_name = Some(file_name.to_path_buf());
        let ext = file_name.extension();

        if let Some(ext) = ext {
            // mdf doesn't need sauce info.
            let ext = OsStr::to_str(ext).unwrap().to_lowercase();
            if ext.as_str() ==  "mdf" {
                super::read_mdf(&mut result, bytes)?;
                return Ok(result);
            }
        }

        let (sauce_type, file_size) = result.read_sauce_info(bytes)?;
        let mut parse_avt  = false;
        let mut parse_pcb  = false;
        let mut parse_ansi = false;
        let mut check_extension = false;

        match sauce_type {
            super::SauceFileType::Ascii => { parse_ansi = true; /* There are files that are marked as Ascii but contain ansi codes. */ },
            super::SauceFileType::Ansi => { parse_ansi = true; check_extension = true; },
            super::SauceFileType::ANSiMation => { parse_ansi = true; eprintln!("WARNING: ANSiMations are not fully supported."); },
            super::SauceFileType::PCBoard => { parse_pcb = true; parse_ansi = true; },
            super::SauceFileType::Avatar => { parse_avt = true; },
            super::SauceFileType::TundraDraw => {
                if result.width == 0 { result.width = 80; }
                super::read_tnd(&mut result, bytes, file_size)?;
                return Ok(result);
            },
            super::SauceFileType::Bin => {
                if result.width == 0 { result.width = 160; }
                read_binary(&mut result, bytes, file_size)?;
                return Ok(result);
            },
            super::SauceFileType::XBin => {
                read_xb(&mut result, bytes, file_size)?;
                return Ok(result);
            },
            super::SauceFileType::Undefined => { check_extension = true; },
        }
        
        if check_extension {
            if let Some(ext) = ext {
                let ext = OsStr::to_str(ext).unwrap().to_lowercase();
                match ext.as_str() {
                    "bin" => {
                        if result.width == 0 { result.width = 160; }
                        read_binary(&mut result, bytes, file_size)?;
                        return Ok(result);
                    }
                    "xb" => {
                        read_xb(&mut result, bytes, file_size)?;
                        return Ok(result);
                    }
                    "adf" => {
                        if result.width == 0 { result.width = 80; }
                        super::read_adf(&mut result, bytes, file_size)?;
                        return Ok(result);
                    }
                    "idf" => {
                        super::read_idf(&mut result, bytes, file_size)?;
                        return Ok(result);
                    }
                    "tnd" => {
                        if result.width == 0 { result.width = 80; }
                        super::read_tnd(&mut result, bytes, file_size)?;
                        return Ok(result);
                    }
                    "ans" => { parse_ansi = true; }
                    "ice" => { parse_ansi = true; result.use_ice = true; }
                    "avt" => { parse_avt = true;  }
                    "pcb" => { parse_pcb = true; parse_ansi = true; }
                    _ => {}
                }
            }
        }

        let mut data = ParseStates::new();
        if result.width == 0 { result.width = 80; }
        result.height = 1;
        data.screen_width = result.width;

        for b in bytes.iter().take(file_size) {
            let mut ch = Some(*b);
            data.cur_input_pos.x += 1;

            if parse_ansi {
                if let Some(c) = ch {
                    ch = display_ans(&mut result, &mut data, c)?;
                }
            }
            if parse_pcb {
                if let Some(c) = ch {
                    ch = display_PCBoard(&mut data, c);
                }
            }

            if parse_avt {
                if let Some(c) = ch { 
                    let mut avt_result = display_avt(&mut data, c)?;
                    let ch = avt_result.0;
                    if let Some(26) = ch { break; }
                    Buffer::output_char(&mut result, &mut data, ch);
                    while avt_result.1 {
                        avt_result = display_avt(&mut data, 0)?;
                        Buffer::output_char(&mut result, &mut data, avt_result.0);
                    }
                }
            } else {
                if let Some(26) = ch { break; }
                Buffer::output_char(&mut result,  &mut data, ch);
            }
        }
        Ok(result)
    }

    fn output_char(result: &mut Buffer, data: &mut ParseStates, ch: Option<u8>) {
        if let Some(ch) = ch {
            match ch {
                10 => {
                    for x in data.cur_pos.x..(result.width as i32) {
                        let p =Position::from(x, data.cur_pos.y);
                        if result.get_char(p).is_none() {
                            result.set_char(0, p, Some(DosChar::new()));
                        }
                    }
    
                    data.cur_pos.x = 0;
                    data.cur_pos.y += 1;
                    data.cur_input_pos.y += 1;
                    result.height = data.cur_pos.y as u16 + 1;
                    data.cur_input_pos.x = 1;
                }
                12 => {
                    data.cur_pos.x = 0;
                    data.cur_pos.y = 1;
                    data.text_attr = TextAttribute::DEFAULT;
                }
                13 => {
                    data.cur_pos.x = 0;
                }
                _ => {
                    result.height = data.cur_pos.y as u16 + 1;
                    result.set_char(
                        0,
                        data.cur_pos,
                        Some(DosChar {
                            char_code: ch,
                            attribute: data.text_attr,
                        }),
                    );
                    data.cur_pos.x += 1;
                    if data.cur_pos.x >= result.width as i32 {
                        data.cur_pos.x = 0;
                        data.cur_pos.y += 1;
                    }
                }
            }
        }
    }


    pub fn to_screenx(&self, x: i32) -> f64
    {
        let font_dimensions = self.get_font_dimensions();
        x as f64 * font_dimensions.width as f64
    }

    pub fn to_screeny(&self, y: i32) -> f64
    {
        let font_dimensions = self.get_font_dimensions();
        y as f64 * font_dimensions.height as f64 
    }

    pub fn get_line_length(&self, line: i32) -> i32
    {
        let mut length = 0;
        let mut pos = Position::from(0, line);
        for x in 0..(self.width as i32) {
            pos.x = x;
            if let Some(ch) = self.get_char(pos) {
                if x > 0 && ch.is_transparent() {
                    if let Some(prev) = self.get_char(pos  + Position::from(-1, 0)) {
                        if prev.attribute.get_background() > 0 {
                            length = x + 1;
                        }

                    }
                } else if !ch.is_transparent() {
                    length = x + 1;
                }
            }
        }
        length
    }

    pub fn set_height_for_pos(&mut self, pos: Position)
    {
        if pos.x == 0 {
            self.height = pos.y as u16; 
        } else {
            self.height = pos.y as u16 + 1;
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer::new()
    }
}
