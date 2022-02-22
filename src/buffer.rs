use std::{
    cmp::max,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use crate::sauce::Sauce;

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

#[derive(Clone, Copy, Debug, Default)]
pub struct DosChar {
    pub char_code: u8,
    pub attribute: u8,
}

impl DosChar {
    pub fn new() -> Self {
        DosChar {
            char_code: 0,
            attribute: DEFAULT_ATTRIBUTE,
        }
    }

    pub fn get_background_srgb(&self) -> (f32, f32, f32, f32) {
        let o = ((self.attribute & 0b0111_0000) >> 4) as usize;

        let c = DOS_DEFAULT_PALETTE[o];

        (
            c.0 as f32 / 255_f32,
            c.1 as f32 / 255_f32,
            c.2 as f32 / 255_f32,
            1_f32,
        )
    }
}

pub static mut ALL_BUFFERS: Vec<Buffer> = Vec::new();

#[derive(Clone, Debug, Default)]
pub struct Line {
    chars: Vec<DosChar>,
}

impl Line {
    pub fn new() -> Self {
        Line { chars: Vec::new() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Layer {
    pub width: usize,
    pub height: usize,
    lines: Vec<Line>,
}

impl Layer {
    pub fn new() -> Self {
        Layer {
            width: 0,
            height: 0,
            lines: Vec::new(),
        }
    }
}
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct BitFont {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u16>,
}

#[derive(Debug)]
pub struct Buffer {
    pub file_name: Box<PathBuf>,
    pub base_layer: Layer,
    pub font: Option<BitFont>,
    pub layers: Vec<Layer>,
    pub sauce: Option<Sauce>,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            file_name: Box::new(PathBuf::new()),
            base_layer: Layer::new(),
            font: None,
            layers: Vec::new(),
            sauce: None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new() -> Self {
        Position { x: 0, y: 0 }
    }
    pub fn from(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

struct LoadData {
    // ANSI
    ans_esc: bool,
    ans_code: bool,
    cur_pos: Position,
    saved_pos: Position,
    text_attr: u8,

    ans_numbers: Vec<i32>,

    // PCB
    pcb_code: bool,
    pcb_color: bool,
    pcb_value: u8,
    pcb_pos: i32,

    // Avatar
    avatar_state: i32,
    avt_color: bool,
    avt_rep: bool,
    avt_command : bool,
    avt_repeat_char: u8,
    avt_repeat_count: i32
}

const DEFAULT_ATTRIBUTE: u8 = 7;
const ANSI_CSI: u8 = b'[';
const ANSI_ESC: u8 = 27;
/// Starts most of the useful sequences, terminated by a byte in the range 0x40 through 0x7E
const EOF: u8 = 26;

pub fn get_color(color: u8) -> &'static str
{
    match color {
        0 => "Black",
        1 => "Blue",
        2 => "Green",
        3 => "Aqua",
        4 => "Red",
        5 => "Purple",
        6 => "Brown",
        7 => "Light Gray",
        8 => "Gray",
        9 => "Light Blue",
        10 => "Light Green",
        11 => "Light Aqua",
        12 => "Light Red",
        13 => "Light Purple",
        14 => "Light Yelllow",
        15 => "White",
        _ => "Unknown"
    }
}

impl Buffer {
    fn conv_ch(ch: u8) -> u8 {
        if (b'0'..=b'9').contains(&ch) {
            return ch - b'0';
        }
        if (b'a'..=b'f').contains(&ch) {
            return ch - b'a';
        }
        if (b'A'..=b'F').contains(&ch) {
            return ch - b'A';
        }
        0
    }
    
    #[allow(non_snake_case)]
    fn display_PCBoard(data: &mut LoadData, ch: u8) -> u8 {
        if data.pcb_color {
            data.pcb_pos += 1;
            if data.pcb_pos < 3 {
                match data.pcb_pos {
                    1 => {
                        data.pcb_value = Buffer::conv_ch(ch);
                        return 0;
                    }
                    2 => {
                        data.pcb_value = (data.pcb_value << 4) + Buffer::conv_ch(ch);
                        data.text_attr = data.pcb_value;
                    }
                    _ => {}
                }
            }
            data.pcb_color = false;
            data.pcb_code = false;
            return 0;
        }

        if data.pcb_code {
            match ch {
                b'@' => {
                    data.pcb_code = false;
                }
                b'X' => {
                    data.pcb_color = true;
                    data.pcb_pos = 0;
                }
                _ => {}
            }
            return 0;
        }
        match ch {
            b'@' => {
                data.pcb_code = true;
                0
            }
            _ => ch,
        }
    }
    const COLOR_OFFSETS : [u8; 8] = [ 0, 4, 2, 6, 1, 5, 3, 7 ];

    fn display_ansi(data: &mut LoadData, ch: u8) -> u8 {
        if data.ans_esc {
            if ch == ANSI_CSI {
                data.ans_esc = false;
                data.ans_code = true;
                data.ans_numbers.clear();
                return 0;
            } else {
                // ignore all other ANSI escape codes
                data.ans_esc = false;
                return 0;
            }
        }

        if data.ans_code {
            match ch {
                b'm' => { // Select Graphic Rendition 
                    let mut fgFlag = 0b1111_1000;
                    for n in &data.ans_numbers {
                        match n {
                            0 => { data.text_attr = DEFAULT_ATTRIBUTE; /*  fgFlag = 0b1111_0000;*/ }, // Reset or normal 
                            1 => { data.text_attr |= 0b0000_1000;/*  fgFlag = 0b1111_1000;*/ },      // Bold or increased intensity 
                            5 => data.text_attr |= 0b1000_1000,                                 // Slow blink 
                            // set foreaground color
                            30..=37 => data.text_attr = (data.text_attr & fgFlag) | Buffer::COLOR_OFFSETS[*n as usize - 30],
                            // set background color
                            40..=47 => data.text_attr = (data.text_attr & 0b1000_1111) | (Buffer::COLOR_OFFSETS[*n as usize - 40] << 4),
                            _ => { eprintln!("Unsupported ANSI graphic code {}", n); }
                        }
                    }
                    data.ans_code = false;
                    return 0;
                }
                b'H' | b'f' => { // Cursor Position + Horizontal Vertical Position ('f')
                    if !data.ans_numbers.is_empty() {
                        data.cur_pos.y = data.ans_numbers[0] as usize;
                        if data.ans_numbers.len() > 1 {
                            data.cur_pos.x = data.ans_numbers[1] as usize;
                        } else {
                            data.cur_pos.x = 0;
                        }
                    }
                    data.ans_code = false;
                    return 0;
                }
                b'C' => { // Cursor Forward 
                    if data.ans_numbers.is_empty() {
                        data.cur_pos.x += 1;
                    } else {
                        data.cur_pos.x += data.ans_numbers[0] as usize;
                    }
                    data.ans_code = false;
                    return 0;
                }
                b'D' => { // Cursor Back 
                    if data.ans_numbers.is_empty() {
                        data.cur_pos.x = data.cur_pos.x.saturating_sub(1);
                    } else {
                        data.cur_pos.x = data.cur_pos.x.saturating_sub(data.ans_numbers[0] as usize);
                    }
                    data.ans_code = false;
                    return 0;
                }
                b'A' => { // Cursor Up 
                    if data.ans_numbers.is_empty() {
                        data.cur_pos.y = data.cur_pos.y.saturating_sub(1);
                    } else {
                        data.cur_pos.y =
                            data.cur_pos.y.saturating_sub(data.ans_numbers[0] as usize);
                    }
                    data.ans_code = false;
                    return 0;
                }
                b'B' => { // Cursor Down 
                    if data.ans_numbers.is_empty() {
                        data.cur_pos.y += 1;
                    } else {
                        data.cur_pos.y += data.ans_numbers[0] as usize;
                    }
                    data.ans_code = false;
                    return 0;
                }
                b's' => { // Save Current Cursor Position
                    data.saved_pos = data.cur_pos;
                    data.ans_code = false;
                    return 0;
                }
                b'u' => { // Restore Saved Cursor Position 
                    data.cur_pos = data.saved_pos;
                    data.ans_code = false;
                    return 0;
                }
                b'J' => { // Erase in Display 
                    data.ans_code = false;
                    if data.ans_numbers.is_empty() {
                        data.cur_pos = Position::new();
                    } else {
                        match data.ans_numbers[0] {
                            0 => {} // TODO: clear from cursor to the end of the screen 
                            2 => {  // clear entire screen
                                data.cur_pos = Position::new();
                                // TODO: Clear
                            } 
                            3 => { // TODO: clear entire screen and delete all lines saved in the scrollback buffer
                                data.cur_pos = Position::new();
                            }
                            _ => {eprintln!("unknown ANSI J sequence {}", data.ans_numbers[0])}
                        }
                    }
                    
                    return 0;
                }
                _ => {
                    if (0x40..=0x7E).contains(&ch) {
                        // unknown control sequence, terminate reading
                        data.ans_code = false;
                        data.ans_esc = false;
                        eprintln!("unknown control sequence, terminating.");
                        return 0;
                    }

                    if (b'0'..=b'9').contains(&ch) {
                        if data.ans_numbers.is_empty() {
                            data.ans_numbers.push(0);
                        }
                        let d = data.ans_numbers.pop().unwrap();
                        data.ans_numbers.push(d * 10 + (ch - b'0') as i32);
                    } else if ch == b';' {
                        data.ans_numbers.push(0);
                        return 0;
                    } else {
                        // error in control sequence, terminate reading
                        eprintln!(
                            "error in ANSI control sequence: {:?}!",
                            char::from_u32(ch as u32)
                        );
                        data.ans_code = false;
                        data.ans_esc = false;
                    }
                    return 0;
                }
            }
        }

        if ch == ANSI_ESC {
            data.ans_code = false;
            data.ans_esc = true;
            0
        } else {
            ch
        }
    }
    

    /// Starts Avatar command
    const AVT_CMD: u8 = 22;

    /// clear the current window and set current attribute to default.
    const AVT_CLR: u8 = 12;

    ///  Read two bytes from the modem. Send the first one to the screen as many times as the binary value
    ///  of the second one. This is the exception where the two bytes may have their high bit set. Do not reset it here!
    const AVT_REP: u8 = 25;

    // Advanced Video Attribute Terminal Assembler and Recreator
    fn display_avatar(data: &mut LoadData, ch: u8) -> (u8, bool) {
        if data.avt_rep {
            match data.avatar_state {
                1=> {
                    data.avt_repeat_char = ch;
                    data.avatar_state = 2;
                    return (0, false);
                }
                2 => {
                    data.avt_repeat_count = ch as i32;
                    data.avatar_state = 3;
                    return (0, false);
                }
                3 => {
                    if data.avt_repeat_count > 0 {
                        data.avt_repeat_count -= 1;
                        return (data.avt_repeat_char, data.avt_repeat_count >= 0);
                    }
                }
                _ => {}
            }
            data.avt_rep = false;
        }
        data.avatar_state = 0;

        if data.avt_color {
            data.text_attr     = ch;
            data.avt_command = false;
            data.avt_color   = false;
            return (0, false);
        }

        if data.avt_command {
            println!("Command {}", ch);
            match ch {
                1 => {
                    data.avt_color = true;
                }
                2 => {
                    data.avt_command = false;
                }
                // TODO implement commands from FSC0025.txt
                _ => { eprintln!("unsupported avatar command {}", ch); }
            }
            return (0, false);
        }
            
        match ch {
            Buffer::AVT_CLR => {
                data.cur_pos = Position::new();
                data.text_attr = DEFAULT_ATTRIBUTE;
            }
            Buffer::AVT_REP => {
                data.avt_rep     = true;
                data.avatar_state = 1;
            }
            Buffer::AVT_CMD => {
                data.avt_command = true;
            }
            _ => {
                return (ch, false);
            }
        }
        (0, false)
    }

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
        let mut data = LoadData {
            ans_code: false,
            ans_esc: false,
            cur_pos: Position::new(),
            saved_pos: Position::new(),
            text_attr: DEFAULT_ATTRIBUTE,
            ans_numbers: Vec::new(),
            pcb_code: false,
            pcb_color: false,
            pcb_value: 0,
            pcb_pos: 0,

            avatar_state: 0,
            avt_color: false,
            avt_rep: false,
            avt_command : false,
            avt_repeat_char: 0,
            avt_repeat_count: 0
        };

        for b in bytes {
            if *b == EOF {
                break;
            }
            let mut ch = Buffer::display_ansi(&mut data, *b);
            ch = Buffer::display_PCBoard(&mut data, ch);

            let mut avt_result = Buffer::display_avatar(&mut data, ch);
            let ch = avt_result.0;
            Buffer::output_char(&mut result, &mut data, ch);
            while avt_result.1 {
                avt_result = Buffer::display_avatar(&mut data, 0);
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


