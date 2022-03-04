use std::{cell::{RefCell, RefMut}, rc::Rc};
use crate::model::{TextAttribute, DosChar};

use super::{ Tool, Editor, Position, Event};

pub struct FillTool {}

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
            let attr = editor.cursor.attr;
            let ch = editor.buf.get_char(pos);
            if ch.char_code != b'#' {
                fill(&mut editor, attr, pos, ch, DosChar{ char_code: b'#', attribute: attr });
            }
        }
        Event::None
    }
}

pub fn fill(editor: &mut RefMut<Editor>, attribute: TextAttribute, pos: Position, ch: DosChar, new_ch: DosChar) {
    if editor.buf.get_char(pos) != ch || 
        !editor.point_is_valid(pos) {
        return;
    }
    editor.set_char(pos, new_ch);

    fill(editor, attribute, pos + Position::from(-1, 0), ch, new_ch);
    fill(editor, attribute, pos + Position::from(1, 0), ch, new_ch);
    fill(editor, attribute, pos + Position::from(    0, -1), ch, new_ch);
    fill(editor, attribute, pos + Position::from(0, 1), ch, new_ch);
}