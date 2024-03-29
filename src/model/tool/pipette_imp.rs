use std::{rc::Rc, cell::RefCell};

use super::{Tool, Editor, Position, Event};
pub struct PipetteTool {}

impl Tool for PipetteTool
{
    fn get_icon_name(&self) -> &'static str { "md-tool-pipette" }
    fn use_caret(&self) -> bool { false }
    fn use_selection(&self) -> bool { false }
    
    fn handle_click(&mut self, editor: Rc<RefCell<Editor>>, button: u32, pos: Position) -> Event {
        if button == 1 {
            let ch = editor.borrow().get_char(pos).unwrap_or_default();
            editor.borrow_mut().set_caret_attribute(ch.attribute);
        }
        Event::None
    }
}