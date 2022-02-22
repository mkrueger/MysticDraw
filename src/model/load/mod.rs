mod ansi;
pub use ansi::*;

mod pcboard;
pub use pcboard::*;

mod avatar;
pub use avatar::*;

use super::Position;

pub struct LoadData {
    // ANSI
    pub ans_esc: bool,
    pub ans_code: bool,
    pub cur_pos: Position,
    pub saved_pos: Position,
    pub text_attr: u8,

    pub ans_numbers: Vec<i32>,

    // PCB
    pub pcb_code: bool,
    pub pcb_color: bool,
    pub pcb_value: u8,
    pub pcb_pos: i32,

    // Avatar
    pub avatar_state: i32,
    pub avt_color: bool,
    pub avt_rep: bool,
    pub avt_command : bool,
    pub avt_repeat_char: u8,
    pub avt_repeat_count: i32
}

impl LoadData {
    pub fn new() -> Self {
        LoadData {
            ans_code: false,
            ans_esc: false,
            cur_pos: Position::new(),
            saved_pos: Position::new(),
            text_attr: super::DEFAULT_ATTRIBUTE,
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
        }
    }
}