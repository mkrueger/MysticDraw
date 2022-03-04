use crate::model::{tool::handle_outline_insertion, Layer, TextAttribute, DosChar};

use super::{Editor, Event, MKey, MKeyCode, MModifiers, Position, Tool};
use std::{cell::RefCell, rc::Rc};

pub struct LineTool {
    pub old_pos: Position
}

const CORNER_UPPER_LEFT:i32 = 0;
const CORNER_UPPER_RIGHT:i32 = 1;
const CORNER_LOWER_LEFT:i32 = 2;
const CORNER_LOWER_RIGHT:i32 = 3;

const HORIZONTAL_CHAR:i32 = 4;
const VERTICAL_CHAR:i32 = 5;

const VERT_RIGHT_CHAR:i32 = 6;
const VERT_LEFT_CHAR:i32 = 7;

const HORIZ_UP_CHAR:i32 = 8;
const HORIZ_DOWN_CHAR:i32 = 9;

impl LineTool {

    pub fn get_new_horiz_char(editor: &std::cell::RefMut<Editor>, new_char: u8, to_left: bool) -> i32
    {
        if new_char == editor.get_outline_char_code(VERTICAL_CHAR).unwrap() {
            if to_left { 
                VERT_RIGHT_CHAR 
            } else { 
                VERT_LEFT_CHAR 
            }
        } else if new_char == editor.get_outline_char_code(CORNER_LOWER_RIGHT).unwrap() || new_char == editor.get_outline_char_code(CORNER_LOWER_LEFT).unwrap() { 
            HORIZ_UP_CHAR
        } else if new_char == editor.get_outline_char_code(CORNER_UPPER_RIGHT).unwrap() || new_char == editor.get_outline_char_code(CORNER_UPPER_LEFT).unwrap() { 
            HORIZ_DOWN_CHAR
        } else {
            HORIZONTAL_CHAR
        }
    }

    pub fn get_old_horiz_char(&self, editor: &std::cell::RefMut<Editor>, old_char: u8, to_left: bool) -> Option<u8>
    {
        let pos = editor.cursor.get_position();
        if old_char == editor.get_outline_char_code(VERTICAL_CHAR).unwrap() {
            if self.old_pos.y > pos.y {
                Some(editor.get_outline_char_code(if to_left {CORNER_UPPER_RIGHT} else { CORNER_UPPER_LEFT}).unwrap())
            } else if self.old_pos.y < pos.y {
                Some(editor.get_outline_char_code(if to_left {CORNER_LOWER_RIGHT} else { CORNER_LOWER_LEFT}).unwrap())
            } else {
                None
            }
        } else if old_char == editor.get_outline_char_code(VERT_LEFT_CHAR).unwrap() || old_char == editor.get_outline_char_code(VERT_RIGHT_CHAR).unwrap()  {
            let cur =editor.get_cur_outline();
            if cur < 4  {
                let ck = editor.get_outline_char_code_from(4, cur);
                Some(ck.unwrap())
            } else { None}
        } else {
            None
        }
    }

    pub fn get_new_vert_char(editor: &std::cell::RefMut<Editor>, new_char: u8, to_left: bool) -> i32
    {
        if new_char == editor.get_outline_char_code(HORIZONTAL_CHAR).unwrap() {
            if to_left { 
                HORIZ_DOWN_CHAR 
            } else { 
                HORIZ_UP_CHAR 
            }
        } else if new_char == editor.get_outline_char_code(CORNER_LOWER_RIGHT).unwrap() || new_char == editor.get_outline_char_code(CORNER_LOWER_LEFT).unwrap() { 
            HORIZ_UP_CHAR
        } else if new_char == editor.get_outline_char_code(CORNER_UPPER_RIGHT).unwrap() || new_char == editor.get_outline_char_code(CORNER_UPPER_LEFT).unwrap() { 
            VERT_RIGHT_CHAR
        } else {
            VERTICAL_CHAR
        }
    }

    pub fn get_old_vert_char(&self, editor: &std::cell::RefMut<Editor>, old_char: u8, to_left: bool) -> Option<u8>
    {
        let pos = editor.cursor.get_position();
        if old_char == editor.get_outline_char_code(HORIZONTAL_CHAR).unwrap() {
            if self.old_pos.x > pos.x {
                Some(editor.get_outline_char_code(if to_left {CORNER_LOWER_LEFT} else { CORNER_UPPER_LEFT}).unwrap())
            } else if self.old_pos.x < pos.x {
                Some(editor.get_outline_char_code(if to_left {CORNER_LOWER_RIGHT} else { CORNER_UPPER_RIGHT}).unwrap())
            } else {
                None
            }
        } else if old_char == editor.get_outline_char_code(HORIZ_UP_CHAR).unwrap() || old_char == editor.get_outline_char_code(HORIZ_DOWN_CHAR).unwrap()  {
            if editor.get_cur_outline() < 4  {
                Some(editor.get_outline_char_code_from(4, editor.get_cur_outline()).unwrap())
            } else { None}
        } else {
            None
        }
    }
}

// block tools:
// copy/moxe
// fill, delete
impl Tool for LineTool {
    fn get_icon_name(&self) -> &'static str {
        "md-tool-line"
    }
    fn use_selection(&self) -> bool { false }

    fn handle_key(
        &mut self,
        editor: Rc<RefCell<Editor>>,
        key: MKey,
        _key_code: MKeyCode,
        modifier: MModifiers,
    ) -> Event {
        let mut e = editor.borrow_mut();
        let old_pos = e.cursor.get_position();
        match key {
            MKey::Down => {
                e.set_cursor(old_pos.x, old_pos.y + 1);
            }
            MKey::Up => {
                e.set_cursor(old_pos.x, old_pos.y - 1);
            }
            MKey::Left => {
                e.set_cursor(old_pos.x - 1, old_pos.y);
            }
            MKey::Right => {
                e.set_cursor(old_pos.x + 1, old_pos.y);
            }

            _ => {
                if modifier.is_shift() || modifier.is_control() {
                    match key {
                        MKey::F1 => {
                            handle_outline_insertion(&mut e, modifier, 0);
                        }
                        MKey::F2 => {
                            handle_outline_insertion(&mut e, modifier, 1);
                        }
                        MKey::F3 => {
                            handle_outline_insertion(&mut e, modifier, 2);
                        }
                        MKey::F4 => {
                            handle_outline_insertion(&mut e, modifier, 3);
                        }
                        MKey::F5 => {
                            handle_outline_insertion(&mut e, modifier, 4);
                        }
                        MKey::F6 => {
                            handle_outline_insertion(&mut e, modifier, 5);
                        }
                        MKey::F7 => {
                            handle_outline_insertion(&mut e, modifier, 6);
                        }
                        MKey::F8 => {
                            handle_outline_insertion(&mut e, modifier, 7);
                        }
                        MKey::F9 => {
                            handle_outline_insertion(&mut e, modifier, 8);
                        }
                        MKey::F10 => {
                            handle_outline_insertion(&mut e, modifier, 9);
                        }
                        _ => {}
                    }
                }
            }
        }

        let new_pos = e.cursor.get_position();
        let new_char = e.buf.get_char(new_pos);
        let old_char = e.buf.get_char(old_pos);

        let b = (new_pos.x - old_pos.x).signum();
        let a = (new_pos.y - old_pos.y).signum();
        if a == 1 || a == -1 {
            let c = LineTool::get_new_vert_char(&e, new_char.char_code, a == -1 );
            let char_code = e.get_outline_char_code(c).unwrap();
            let attribute = e.cursor.get_attribute();
            e.set_char(
                new_pos,
                crate::model::DosChar {
                    char_code,
                    attribute,
                },
            );

            if old_char.is_transparent() {
                let char_code = e.get_outline_char_code(HORIZONTAL_CHAR).unwrap();
                e.set_char(
                    old_pos,
                    crate::model::DosChar {
                        char_code,
                        attribute,
                    },
                );
            } else if let Some(char_code) = self.get_old_vert_char(&e, old_char.char_code, a == -1) {
                e.set_char(
                    old_pos,
                    crate::model::DosChar {
                        char_code,
                        attribute,
                    },
                );   
            }
        }

        
        if b == 1 || b == -1 { // horizontal movement
            let c = LineTool::get_new_horiz_char(&e, new_char.char_code, b == -1 );
            let char_code = e.get_outline_char_code(c).unwrap();
            let attribute = e.cursor.get_attribute();
            e.set_char(
                new_pos,
                crate::model::DosChar {
                    char_code,
                    attribute,
                },
            );

            if old_char.is_transparent() {
                let char_code = e.get_outline_char_code(VERTICAL_CHAR).unwrap();
                e.set_char(
                    old_pos,
                    crate::model::DosChar {
                        char_code,
                        attribute,
                    },
                );
            } else if let Some(char_code) = self.get_old_horiz_char(&e, old_char.char_code, b == -1) {
                e.set_char(
                    old_pos,
                    crate::model::DosChar {
                        char_code,
                        attribute,
                    },
                );   
            }
        }

        self.old_pos = old_pos;
        Event::None
    }

    fn handle_click(&mut self, editor: Rc<RefCell<Editor>>, button: u32, pos: Position) -> Event {
        let mut editor = editor.borrow_mut();
        if button == 1 {
            std::borrow::BorrowMut::borrow_mut(&mut editor)
                .cursor
                .set_position(pos);
        }
        Event::None
    }


    fn handle_drag(&self, editor: Rc<RefCell<Editor>>, start: Position, cur: Position) -> Event
    {
        let mut editor = editor.borrow_mut();
        let attr = editor.cursor.get_attribute();
        if let Some(layer) = editor.get_overlay_layer() {
            layer.clear();
            plot_line(layer, attr, start, cur);
        }
        Event::None
    }

    fn handle_drag_end(&self, editor: Rc<RefCell<Editor>>, start: Position, cur: Position) -> Event {
        let mut editor = editor.borrow_mut();
        if start == cur {
            editor.buf.remove_overlay();
        } else {
            editor.join_overlay();
        }
        Event::None
    }
}


// simple https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
// maybe worth to explore https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
pub fn plot_line(layer: &mut Layer, attribute: TextAttribute, mut pos0: Position, pos1: Position) {
    let dx = (pos1.x - pos0.x).abs();
    let sx = if pos0.x < pos1.x { 1 } else { -1 };
    let dy = -(pos1.y - pos0.y).abs();
    let sy = if pos0.y < pos1.y { 1 } else { -1 };
    let mut error = dx + dy;
    
    loop {
        layer.set_char(pos0, DosChar{ char_code: 219, attribute });
        if pos0.x == pos1.x && pos0.y == pos1.y { break; }
        let e2 = 2 * error;
        if e2 >= dy {
            if pos0.x == pos1.x { break; }
            error += dy;
            pos0.x += sx;
        }
        if e2 <= dx {
            if pos0.y == pos1.y { break; }
            error += dx;
            pos0.y += sy;
        }
    }
}