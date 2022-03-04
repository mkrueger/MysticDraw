use std::{cell::{RefCell}, rc::Rc};
use crate::model::{TextAttribute, Layer, DosChar, Buffer};

use super::{ Tool, Editor, Position, Event};

pub struct FillTool {}

// Fill with 
// Attribute, Fore/Back
// Character 
// Both

impl Tool for FillTool
{
    fn get_icon_name(&self) -> &'static str { "edit-select" }

   
    fn handle_click(&mut self, editor: Rc<RefCell<Editor>>, button: u32, pos: Position) -> Event {
        if button == 1 {
            let mut editor = editor.borrow_mut();
            let attr = editor.cursor.attr;
            let ch = editor.buf.get_char(pos);
            if ch.char_code != b'#' {
                fill(&mut editor.buf, attr, pos, ch, DosChar{ char_code: b'#', attribute: attr });
            }
        }
        Event::None
    }
}

pub fn fill(buffer: &mut Buffer, attribute: TextAttribute, pos: Position, ch: DosChar, new_ch: DosChar) {

    if buffer.get_char(pos) != ch || pos.x < 0 || pos.y < 0 || pos.x >= buffer.width as i32 || pos.y >= buffer.height as i32 {
        return;
    }
    buffer.set_char(0, pos, new_ch);

    fill(buffer, attribute, pos + Position::from(-1, 0), ch, new_ch);
    fill(buffer, attribute, pos + Position::from(1, 0), ch, new_ch);
    fill(buffer, attribute, pos + Position::from(    0, -1), ch, new_ch);
    fill(buffer, attribute, pos + Position::from(0, 1), ch, new_ch);
}