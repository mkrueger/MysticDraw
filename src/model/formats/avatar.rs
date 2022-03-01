use std::cmp::{max, min};

use crate::model::{Buffer, Position, TextAttribute};

use super::ParseStates;

/// Starts Avatar command
const AVT_CMD: u8 = 22;

/// clear the current window and set current attribute to default.
const AVT_CLR: u8 = 12;

///  Read two bytes from the modem. Send the first one to the screen as many times as the binary value
///  of the second one. This is the exception where the two bytes may have their high bit set. Do not reset it here!
const AVT_REP: u8 = 25;

pub enum AvtReadState {
    Chars,
    RepeatChars,
    ReadCommand,
    MoveCursor,
    ReadColor,
}

// Advanced Video Attribute Terminal Assembler and Recreator
pub fn display_avt(data: &mut ParseStates, ch: u8) -> (Option<u8>, bool) {

    match data.avt_state {
        AvtReadState::Chars => {
            match ch {
                AVT_CLR => {
                    data.cur_pos = Position::new();
                    data.text_attr = TextAttribute::DEFAULT;
                }
                AVT_REP => {
                    data.avt_state = AvtReadState::RepeatChars;
                    data.avatar_state = 1;
                }
                AVT_CMD => {
                    data.avt_state = AvtReadState::ReadCommand;
                }
                _ => {
                    return (Some(ch), false);
                }
            }
            (None, false)
        }
        AvtReadState::ReadCommand => {
            match ch {
                1 => {
                    data.avt_state = AvtReadState::ReadColor;
                    return (None, false);
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
                    eprintln!("todo: avt cleareol.");
                } 
                8 =>  {
                    data.avt_state = AvtReadState::MoveCursor;
                    data.avatar_state = 1;
                    return (None, false);
                }
                // TODO implement commands from FSC0025.txt & FSC0037.txt
                _ => { eprintln!("unsupported avatar command {}", ch); }
            }
            data.avt_state = AvtReadState::Chars;
            (None, false)
        }
        AvtReadState::RepeatChars => {
            match data.avatar_state {
                1=> {
                    data.avt_repeat_char = ch;
                    data.avatar_state = 2;
                    (None, false)
                }
                2 => {
                    data.avt_repeat_count = ch as i32;
                    data.avatar_state = 3;
                    (None, true)
                }
                3 => {
                    if data.avt_repeat_count > 0 {
                        data.avt_repeat_count -= 1;
                        if data.avt_repeat_count == 0 {
                            data.avt_state = AvtReadState::Chars;
                        }
                        return (Some(data.avt_repeat_char), data.avt_repeat_count > 0);
                    }
                    (None, false)
                }
                _ => { 
                    eprintln!("error in reading avt state"); 
                    data.avt_state = AvtReadState::Chars;
                    (None, false)
                }
            }
        }
        AvtReadState::ReadColor => {
            data.text_attr = TextAttribute::from_u8(ch);
            data.avt_state = AvtReadState::Chars;
            (None, false)
        }
        AvtReadState::MoveCursor => {
            match data.avatar_state {
                1=> {
                    data.avt_repeat_char = ch;
                    data.avatar_state = 2;
                    return (None, false);
                }
                2 => {
                    data.cur_pos.x = data.avt_repeat_char as i32;
                    data.cur_pos.y = ch as i32;
                    
                    data.avt_state = AvtReadState::Chars;
                    return (None, false);
                }
                _ => { eprintln!("error in reading avt avt_gotoxy"); }
            }
            data.avt_state = AvtReadState::Chars;
            (None, false)
        }
    }
}


pub fn convert_to_avt(buf: &Buffer) -> Vec<u8>
{
    let mut result = Vec::new();
    let mut last_attr = TextAttribute::DEFAULT;
    let mut pos = Position::new();
    let height = buf.height as i32;
    let mut last_line_skipped = false;
    let mut first_char = true;

    // TODO: implement repeat pattern compression (however even TheDraw never bothered to implement this cool RLE from fsc0037)
    while pos.y < height {
        let line_length = buf.get_line_length(pos.y);
        if line_length == 0 && last_line_skipped {
            result.push(13);
            result.push(10);
        }

        while pos.x < line_length {
            let mut repeat_count = 1;
            let mut ch = buf.get_char(pos);

            while pos.x < buf.width as i32 - 3 && ch == buf.get_char(pos + Position::from(1, 0)) {
                repeat_count += 1;
                pos.x += 1;                     
                ch = buf.get_char(pos);
            }

            if first_char || ch.attribute != last_attr {
                result.push(22);
                result.push(1);
                result.push(ch.attribute.as_u8());
                last_attr = ch.attribute;
            }
            first_char = false;

            if repeat_count > 1 {
                if repeat_count < 4 && (ch.char_code != 22 && ch.char_code != 12 && ch.char_code != 25) {
                    result.resize(result.len() + repeat_count, ch.char_code);
                } else {
                    result.push(25);
                    result.push(ch.char_code);
                    result.push(repeat_count as u8);
                }
                pos.x += 1;

                continue;
            }

            // avt control codes need to be represented as repeat once.
            if ch.char_code == 22 || ch.char_code == 12 || ch.char_code == 25 {
                result.push(25);
                result.push(ch.char_code);
                result.push(1);
            } else {
                result.push(if ch.char_code == 0 { b' ' } else { ch.char_code });
            }
            pos.x += 1;
        }
        pos.y += 1;

        // do not end with eol
        last_line_skipped = pos.y >= height || pos.x >= buf.width as i32;
        if !last_line_skipped {
            result.push(13);
            result.push(10);
        }
        pos.x = 0;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::model::{Buffer, Position};

    #[test]
    fn test_clear() {
        let buf = Buffer::from_bytes(&std::path::PathBuf::from("test.avt"), &None, 
        &[b'X', 12, b'X']);
        assert_eq!(1, buf.height);
        assert_eq!(1, buf.width);
    }


    #[test]
    fn test_repeat() {
        let buf = Buffer::from_bytes(&std::path::PathBuf::from("test.avt"), &None, 
        &[b'X', 25, b'b', 3, b'X']);
        assert_eq!(1, buf.height);
        assert_eq!(5, buf.width);
        assert_eq!(b'X', buf.get_char(Position::from(0, 0)).char_code);
        assert_eq!(b'b', buf.get_char(Position::from(1, 0)).char_code);
        assert_eq!(b'b', buf.get_char(Position::from(2, 0)).char_code);
        assert_eq!(b'b', buf.get_char(Position::from(3, 0)).char_code);
        assert_eq!(b'X', buf.get_char(Position::from(4, 0)).char_code);
    }

    #[test]
    fn test_zero_repeat() {
        let buf = Buffer::from_bytes(&std::path::PathBuf::from("test.avt"), &None, 
        &[25, b'b', 0]);
        assert_eq!(0, buf.height);
        assert_eq!(0, buf.width);
    }

    #[test]
    fn test_linebreak_bug() {
        let buf = Buffer::from_bytes(&std::path::PathBuf::from("test.avt"), &None, &[12,22,1,8,32,88,22,1,15,88,25,32,4,88,22,1,8,88,32,32,32,22,1,3,88,88,22,1,57,88,88,88,25,88,7,22,1,9,25,88,4,22,1,25,88,88,88,88,88,88,22,1,1,25,88,13]);
        assert_eq!(1, buf.height);
        assert_eq!(47, buf.width);
    }


    fn output_avt(data: &[u8]) -> Vec<u8>
    {
        let mut result = Vec::new();
        let mut prev = 0;

        for d in data {
            match d {
                12 => result.extend_from_slice(b"^L"),
                25 => result.extend_from_slice(b"^Y"),
                22 => result.extend_from_slice(b"^V"),
                _ => {
                    if prev == 22 {
                        match d {
                            1 => result.extend_from_slice(b"<SET_COLOR>"),
                            2 => result.extend_from_slice(b"<BLINK_ON>"),
                            3 => result.extend_from_slice(b"<MOVE_UP>"),
                            4 => result.extend_from_slice(b"<MOVE_DOWN>"),
                            5 => result.extend_from_slice(b"<MOVE_RIGHT"),
                            6 => result.extend_from_slice(b"<MOVE_LEFT>"),
                            7 => result.extend_from_slice(b"<CLR_EOL>"),
                            8 => result.extend_from_slice(b"<GOTO_XY>"),
                            _ => result.extend_from_slice(b"<UNKNOWN_CMD>"),
                        }
                        prev = *d;
                        continue;
                    }

                    result.push(*d);
                }
            }
            prev = *d;
        }
        result
    }

    fn test_avt(data: &[u8])
    {
        let buf = Buffer::from_bytes(&std::path::PathBuf::from("test.avt"), &None, data);
        let converted = super::convert_to_avt(&buf);

        // more gentle output.
        let b : Vec<u8> = output_avt(&converted);
        let converted  = String::from_utf8_lossy(b.as_slice());

        let b : Vec<u8> = output_avt(data);
        let expected  = String::from_utf8_lossy(b.as_slice());

        assert_eq!(expected, converted);
    }

    #[test]
    fn test_char_compression() {
        let data = b"\x16\x01\x07A-A--A---A\x19-\x04A\x19-\x05A\x19-\x06A\x19-\x07A";
        test_avt(data);
    }
}