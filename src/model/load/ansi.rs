use crate::model::{DEFAULT_ATTRIBUTE, Position};

use super::LoadData;

const ANSI_CSI: u8 = b'[';
const ANSI_ESC: u8 = 27;

const COLOR_OFFSETS : [u8; 8] = [ 0, 4, 2, 6, 1, 5, 3, 7 ];

pub fn display_ansi(data: &mut LoadData, ch: u8) -> u8 {
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
                let fg_flag = 0b1111_1000;
                for n in &data.ans_numbers {
                    match n {
                        0 => { data.text_attr = DEFAULT_ATTRIBUTE; /*  fgFlag = 0b1111_0000;*/ }, // Reset or normal 
                        1 => { data.text_attr |= 0b0000_1000;/*  fgFlag = 0b1111_1000;*/ },      // Bold or increased intensity 
                        5 => data.text_attr |= 0b1000_1000,                                 // Slow blink 
                        // set foreaground color
                        30..=37 => data.text_attr = (data.text_attr & fg_flag) | COLOR_OFFSETS[*n as usize - 30],
                        // set background color
                        40..=47 => data.text_attr = (data.text_attr & 0b1000_1111) | (COLOR_OFFSETS[*n as usize - 40] << 4),
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

