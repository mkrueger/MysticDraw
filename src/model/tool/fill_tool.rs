use std::{cell::{RefCell, RefMut}, rc::Rc};
use crate::model::{TextAttribute, DosChar};

use super::{ Tool, Editor, Position, Event, FILL_TOOL};

pub struct FillTool {
    pub use_char : bool,
    pub use_fore : bool,
    pub use_back : bool,

    pub attr: TextAttribute,
    pub char_code: u8
}

// Fill with 
// Attribute, Fore/Back
// Character 
// Both

impl Tool for FillTool
{
    fn get_icon_name(&self) -> &'static str { "md-tool-fill" }
    fn use_caret(&self) -> bool { false }

    fn handle_click(&mut self, editor: Rc<RefCell<Editor>>, button: u32, pos: Position) -> Event {
        if button == 1 {
            let mut editor = editor.borrow_mut();
            let attr = editor.cursor.get_attribute();
            let ch = editor.buf.get_char(pos);
            if self.use_back || self.use_fore || self.use_char {
                fill(&mut editor, attr, pos, ch,  DosChar{ char_code: self.char_code, attribute: attr });
            }
        }
        Event::None
    }
}

pub fn fill(editor: &mut RefMut<Editor>, attribute: TextAttribute, pos: Position, old_ch: DosChar, new_ch: DosChar) {
    if !editor.point_is_valid(pos) {
        return;
    }
    let cur_char = editor.buf.get_char(pos);
    unsafe {
        if FILL_TOOL.use_char && FILL_TOOL.use_fore && FILL_TOOL.use_back {
            if cur_char != old_ch || cur_char == new_ch {
                return;
            }
        } else if FILL_TOOL.use_fore && FILL_TOOL.use_back {
            if cur_char.attribute != old_ch.attribute || cur_char.attribute == new_ch.attribute {
                return;
            }
        } else if FILL_TOOL.use_char && FILL_TOOL.use_fore  {
            if cur_char.char_code != old_ch.char_code && cur_char.attribute.get_foreground() != old_ch.attribute.get_foreground() || 
               cur_char.char_code == new_ch.char_code && cur_char.attribute.get_foreground() == new_ch.attribute.get_foreground() {
                return;
            }
        } else if FILL_TOOL.use_char && FILL_TOOL.use_back  {
            if cur_char.char_code != old_ch.char_code && cur_char.attribute.get_background_ice() != old_ch.attribute.get_background_ice() || 
               cur_char.char_code == new_ch.char_code && cur_char.attribute.get_background_ice() == new_ch.attribute.get_background_ice() {
                return;
            }
        } else if FILL_TOOL.use_char {
            if cur_char.char_code != old_ch.char_code || cur_char.char_code == new_ch.char_code {
                return;
            }
        } else if FILL_TOOL.use_fore  {
            if cur_char.attribute.get_foreground() != old_ch.attribute.get_foreground() || cur_char.attribute.get_foreground() == new_ch.attribute.get_foreground() {
                return;
            }
        } else if FILL_TOOL.use_back {
            if cur_char.attribute.get_background_ice() != old_ch.attribute.get_background_ice()  || cur_char.attribute.get_background_ice() == new_ch.attribute.get_background_ice() {
                return;
            }
        } else {
            panic!("should never happen!");
        }
        let mut repl_ch = cur_char;
        if FILL_TOOL.use_char { repl_ch.char_code = new_ch.char_code; }
        if FILL_TOOL.use_fore { repl_ch.attribute.set_foreground(new_ch.attribute.get_foreground()) }
        if FILL_TOOL.use_back { repl_ch.attribute.set_background_ice(new_ch.attribute.get_background_ice()) }

        editor.set_char(pos, repl_ch);
    }
    fill(editor, attribute, pos + Position::from(-1, 0), old_ch, new_ch);
    fill(editor, attribute, pos + Position::from(1, 0), old_ch, new_ch);
    fill(editor, attribute, pos + Position::from(    0, -1), old_ch, new_ch);
    fill(editor, attribute, pos + Position::from(0, 1), old_ch, new_ch);
}