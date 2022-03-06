use std::{
    cmp::{ min},
    fs::File,
    io::{self, Read},
    path::{PathBuf, Path},
};
use std::ffi::OsStr;

use super::{Layer, read_xb, Sauce, read_sauce, SauceDataType, Position, DosChar,  ParseStates, read_binary, display_ans, display_PCBoard,  display_avt, TextAttribute, Size, OverlayLayer};

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct BitFont {
    pub size: Size,
    pub data: Vec<u32>,
}

pub struct Buffer {
    pub file_name: Option<PathBuf>,
    pub file_name_changed: Box<dyn Fn ()>,

    pub width: usize,
    pub height: usize,
    pub custom_palette: Option<Vec<u8>>,
    overlay_layer: Option<OverlayLayer>,

    pub font: Option<BitFont>,
    pub layers: Vec<Layer>,
    pub sauce: Option<Sauce>,
}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer").field("file_name", &self.file_name).field("width", &self.width).field("height", &self.height).field("custom_palette", &self.custom_palette).field("font", &self.font).field("layers", &self.layers).field("sauce", &self.sauce).finish()
    }
}

const DEFAULT_FONT: &[u8] = include_bytes!("../../data/font.fnt");

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            file_name: None,
            width: 80,
            height: 25,
            custom_palette: None,
            font: None,
            overlay_layer: None,
            layers: vec!(Layer::new()),
            sauce: None,
            file_name_changed: Box::new(|| {})
        }
    }

    pub fn get_overlay_layer(&mut self) -> &mut Option<OverlayLayer>
    {
        if self.overlay_layer.is_none() {
            self.overlay_layer = Some(OverlayLayer::new());
        }

        &mut self.overlay_layer
    }
    
    pub fn remove_overlay(&mut self)
    {
        self.overlay_layer = None;
    }

    pub fn join_overlay(&mut self, i: i32)
    {
        if let Some(layer) = &self.overlay_layer {
            if i < self.layers.len() as i32 {
                self.layers[i as usize].join_overlay(layer);
            }
            self.remove_overlay();
        }
    }

    pub fn get_font_scanline(&self, ch: u8, y: usize) -> u32
    {
        if let Some(font) = &self.font {
            font.data[ch as usize * font.size.height as usize + y]
        } else {
            DEFAULT_FONT[ch as usize * 16 + y] as u32
        }
    }

    pub fn get_font_dimensions(&self) -> Size
    {
        if let Some(font) = &self.font {
            font.size
        } else {
            // default font.
            Size::from(8, 16)
        }
    }

    pub fn set_char(&mut self, layer: usize, pos: Position, dos_char: DosChar) {
        if layer >= self.layers.len() { return; }

        let cur_layer  = &mut self.layers[layer];
        cur_layer.set_char(pos, dos_char);
    }

    pub fn get_char(&self, pos: Position) -> DosChar {
        if let Some(overlay) = &self.overlay_layer  {
            if let Some(ch) = overlay.get_char(pos) {
                return ch;
            }
        }

        for cur_layer in &self.layers {
            if !cur_layer.is_visible { continue; }
            let ch = cur_layer.get_char(pos);
            if !ch.is_transparent() {
                return ch;
            }
        }

        DosChar::new()
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
                    read_xb(&mut result, bytes, file_size, screen_width);
                    return result;
                }
                "ans" => { parse_ansi = true; }
                "avt" => { parse_avt = true;  }
                "pcb" => { parse_pcb = true; parse_ansi = true; }
                _ => {}
            }
        }
        if screen_width == 0 { screen_width = 80; }

        result.width = screen_width as usize;
        data.screen_width = screen_width;

        for b in bytes.iter().take(file_size) {
            let mut ch = Some(*b);
            if parse_ansi {
                if let Some(c) = ch {
                    ch = display_ans(&mut data, c);
                }
            }
            if parse_pcb {
                if let Some(c) = ch {
                    ch = display_PCBoard(&mut data, c);
                }
            }

            if parse_avt {
                if let Some(c) = ch { 
                    let mut avt_result = display_avt(&mut data, c);
                    let ch = avt_result.0;
                    if let Some(26) = ch { break; }
                    Buffer::output_char(&mut result, screen_width, &mut data, ch);
                    while avt_result.1 {
                        avt_result = display_avt(&mut data, 0);
                        Buffer::output_char(&mut result, screen_width, &mut data, avt_result.0);
                    }
                }
            } else {
                if let Some(26) = ch { break; }
                Buffer::output_char(&mut result, screen_width, &mut data, ch);
            }
        }

        result
    }

    fn output_char(result: &mut Buffer, screen_width : i32, data: &mut ParseStates, ch: Option<u8>) {
        if let Some(ch) = ch {
            match ch {
                10 => {
                    data.cur_pos.x = 0;
                    data.cur_pos.y += 1;
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
                    result.set_char(
                        0,
                        data.cur_pos,
                        DosChar {
                            char_code: ch,
                            attribute: data.text_attr,
                        },
                    );
                    data.cur_pos.x += 1;
                    if data.cur_pos.x >= screen_width {
                        data.cur_pos.x = 0;
                        data.cur_pos.y += 1;
                    }
                }
            }
            if data.cur_pos.y >= result.height as i32 {
                result.height = data.cur_pos.y as usize + 1;
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

    pub const DOS_DEFAULT_PALETTE: [(u8, u8, u8); 16] = [
        (0x00, 0x00, 0x00), // black
        (0x00, 0x00, 0xAA), // blue
        (0x00, 0xAA, 0x00), // green
        (0x00, 0xAA, 0xAA), // cyan
        (0xAA, 0x00, 0x00), // red
        (0xAA, 0x00, 0xAA), // magenta
        (0xAA, 0x55, 0x00), // brown
        (0xAA, 0xAA, 0xAA), // lightgray
        (0x55, 0x55, 0x55), // darkgray
        (0x55, 0x55, 0xFF), // lightblue
        (0x55, 0xFF, 0x55), // lightgreen
        (0x55, 0xFF, 0xFF), // lightcyan
        (0xFF, 0x55, 0x55), // lightred
        (0xFF, 0x55, 0xFF), // lightmagenta
        (0xFF, 0xFF, 0x55), // yellow
        (0xFF, 0xFF, 0xFF), // white
    ];
    
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
        
        let c = Buffer::DOS_DEFAULT_PALETTE[color as usize];
        (
            c.0,
            c.1,
            c.2
        )
    }

    pub fn get_rgba_u32(&self, color: u8) -> u32 {
        debug_assert!(color <= 15);

        if let Some(pal) = &self.custom_palette  {
            let o = (color * 3) as usize;
            if o + 2 >= pal.len() {
                eprintln!("illegal palette color {}, palette is {} colors long.", color, pal.len() / 3);
                return 0;
            }
            // need to << 2 all palette data - custom palette is 0..63 and not 0..255
            return (pal[o] as u32) << 26 |
            (pal[o + 1] as u32) << 18 |
            (pal[o + 2] as u32) << 10 |
            0xFF;
        }
        
        let c = Buffer::DOS_DEFAULT_PALETTE[color as usize];
        (c.0 as u32) << 24 | (c.1 as u32) << 16 | (c.2 as u32) << 8 | 0xFF
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
            let ch = self.get_char(pos);
            if !ch.is_transparent() {
                length = x + 1;
            }
        }
        length
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer::new()
    }
}
