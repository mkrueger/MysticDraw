mod ansi;
pub use ansi::*;

mod pcboard;
pub use pcboard::*;

mod avatar;
pub use avatar::*;

mod bin;
pub use bin::*;

mod xbinary;
pub use xbinary::*;

use super::{Position, TextAttribute};

#[allow(clippy::struct_excessive_bools)]
pub struct ParseStates {
    // ANSI
    pub ans_esc: bool,
    pub ans_code: bool,
    pub cur_pos: Position,
    pub saved_pos: Position,
    pub text_attr: TextAttribute,

    pub ans_numbers: Vec<i32>,

    // PCB
    pub pcb_code: bool,
    pub pcb_color: bool,
    pub pcb_value: u8,
    pub pcb_pos: i32,

    // Avatar
    pub avt_state: AvtReadState,
    pub avatar_state: i32,
    pub avt_repeat_char: u8,
    pub avt_repeat_count: i32
}

impl ParseStates {
    pub fn new() -> Self {
        ParseStates {
            ans_code: false,
            ans_esc: false,
            cur_pos: Position::new(),
            saved_pos: Position::new(),
            text_attr: super::TextAttribute::DEFAULT,
            ans_numbers: Vec::new(),
            pcb_code: false,
            pcb_color: false,
            pcb_value: 0,
            pcb_pos: 0,

            avatar_state: 0,
            avt_state: AvtReadState::Chars,
            avt_repeat_char: 0,
            avt_repeat_count: 0
        }
    }
}