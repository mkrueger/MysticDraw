use crate::model::{Position, DEFAULT_ATTRIBUTE};

use super::LoadData;

/// Starts Avatar command
const AVT_CMD: u8 = 22;

/// clear the current window and set current attribute to default.
const AVT_CLR: u8 = 12;

///  Read two bytes from the modem. Send the first one to the screen as many times as the binary value
///  of the second one. This is the exception where the two bytes may have their high bit set. Do not reset it here!
const AVT_REP: u8 = 25;

// Advanced Video Attribute Terminal Assembler and Recreator
pub fn display_avatar(data: &mut LoadData, ch: u8) -> (u8, bool) {
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
        AVT_CLR => {
            data.cur_pos = Position::new();
            data.text_attr = DEFAULT_ATTRIBUTE;
        }
        AVT_REP => {
            data.avt_rep     = true;
            data.avatar_state = 1;
        }
        AVT_CMD => {
            data.avt_command = true;
        }
        _ => {
            return (ch, false);
        }
    }
    (0, false)
}
