use std::cmp::{max, min};

use crate::model::{Position, DEFAULT_ATTRIBUTE};

use super::LoadData;

/// Starts Avatar command
const AVT_CMD: u8 = 22;

/// clear the current window and set current attribute to default.
const AVT_CLR: u8 = 12;

///  Read two bytes from the modem. Send the first one to the screen as many times as the binary value
///  of the second one. This is the exception where the two bytes may have their high bit set. Do not reset it here!
const AVT_REP: u8 = 25;

pub enum AvatarReadState {
    Chars,
    RepeatChars,
    ReadCommand,
    MoveCursor,
    ReadColor,
}

// Advanced Video Attribute Terminal Assembler and Recreator
pub fn display_avatar(data: &mut LoadData, ch: u8) -> (u8, bool) {

    match data.avt_state {
        AvatarReadState::Chars => {
            match ch {
                AVT_CLR => {
                    data.cur_pos = Position::new();
                    data.text_attr = DEFAULT_ATTRIBUTE;
                }
                AVT_REP => {
                    data.avt_state = AvatarReadState::RepeatChars;
                    data.avatar_state = 1;
                }
                AVT_CMD => {
                    data.avt_state = AvatarReadState::ReadCommand;
                }
                _ => {
                    return (ch, false);
                }
            }
            (0, false)
        }
        AvatarReadState::ReadCommand => {
            match ch {
                1 => {
                    data.avt_state = AvatarReadState::ReadColor;
                    return (0, false);
                }
                2 => {
                    data.text_attr.set_blink(true);
                }
                3 => {
                    data.cur_pos.y = max(0, data.cur_pos.y - 1);
                }
                4 => {
                    data.cur_pos.y += 1;
                }  
                
                5 => {
                    data.cur_pos.x = max(0, data.cur_pos.x - 1);
                }
                6 => {
                    data.cur_pos.x = min(79, data.cur_pos.x + 1);
                }           
                7 => {
                    // TODO: clreol
                    eprintln!("todo: avt cleareol.")
                } 
                8 =>  {
                    data.avt_state = AvatarReadState::MoveCursor;
                    data.avatar_state = 1;
                    return (0, false);
                }
                // TODO implement commands from FSC0025.txt
                _ => { eprintln!("unsupported avatar command {}", ch); }
            }
            data.avt_state = AvatarReadState::Chars;
            return (0, false);
        }
        AvatarReadState::RepeatChars => {
            match data.avatar_state {
                1=> {
                    data.avt_repeat_char = ch;
                    data.avatar_state = 2;
                    return (0, false);
                }
                2 => {
                    data.avt_repeat_count = ch as i32;
                    data.avatar_state = 3;
                    return (0, true);
                }
                3 => {
                    if data.avt_repeat_count > 0 {
                        data.avt_repeat_count -= 1;
                        if data.avt_repeat_count == 0 {
                            data.avt_state = AvatarReadState::Chars;
                        }
                        return (data.avt_repeat_char, data.avt_repeat_count > 0);
                    }
                    return (0, false);
                }
                _ => { 
                    eprintln!("error in reading avt state"); 
                    data.avt_state = AvatarReadState::Chars;
                    return (0, false);
                }
            }
        }
        AvatarReadState::ReadColor => {
            data.text_attr   = crate::model::TextAttribute::from_u8(ch);
            data.avt_state = AvatarReadState::Chars;
            return (0, false);
        }
        AvatarReadState::MoveCursor => {
            match data.avatar_state {
                1=> {
                    data.avt_repeat_char = ch;
                    data.avatar_state = 2;
                    return (0, false);
                }
                2 => {
                    data.cur_pos.x = data.avt_repeat_char as i32;
                    data.cur_pos.y = ch as i32;
                    
                    data.avt_state = AvatarReadState::Chars;
                    return (0, false);
                }
                _ => { eprintln!("error in reading avt avt_gotoxy"); }
            }
            data.avt_state = AvatarReadState::Chars;
            return (0, false);
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::model::Buffer;

    #[test]
    fn test_linebreak_bug() {
        let buf = Buffer::from_bytes(std::path::PathBuf::from("test"), &[12,22,1,8,32,88,22,1,15,88,25,32,4,88,22,1,8,88,32,32,32,22,1,3,88,88,22,1,57,88,88,88,25,88,7,22,1,9,25,88,4,22,1,25,88,88,88,88,88,88,22,1,1,25,88,13]);
        assert_eq!(1, buf.height);
        assert_eq!(47, buf.width);
    }
}

