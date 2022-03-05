use std::{cell::{RefCell}, rc::Rc};
use crate::model::TextAttribute;

use super::{ Tool, Editor, Position};

pub enum EraseType {
    Shade,
    Solid
}

pub struct EraseTool {
    pub size: i32,
    pub brush_type: EraseType

}
impl EraseTool {

    fn paint_brush(&self, editor: &Rc<RefCell<Editor>>, pos: Position)
    {
        let mid = Position::from(-(self.size / 2), -(self.size / 2));

        let center = pos + mid;
        let gradient = [ 219, 178, 177, 176, b' '];
        let mut editor = editor.borrow_mut();
        for y in 0..self.size {
            for x in 0..self.size {
                match self.brush_type {
                    EraseType::Shade => {    
                        let ch = editor.buf.get_char(center + Position::from(x, y));
                       
                        let mut attribute= ch.attribute;

                        let mut char_code = gradient[0];
                        let mut found = false;
                        if ch.char_code == gradient[gradient.len() -1] {
                            char_code = gradient[gradient.len() -1];
                            attribute = TextAttribute::DEFAULT;
                            found = true;
                        } else {
                            for i in 0..gradient.len() - 1 {
                                if ch.char_code == gradient[i] {
                                    char_code = gradient[i + 1];
                                    found = true;
                                    break;
                                }
                            }
                        }

                        if found {
                            editor.set_char(center + Position::from(x, y), crate::model::DosChar { 
                                char_code, 
                                attribute
                            });
                        }
                    },
                    EraseType::Solid => {
                        editor.set_char(center + Position::from(x, y), crate::model::DosChar { char_code: b' ', attribute: TextAttribute::DEFAULT });
                    }
                }
            }                
        }
    }
    
}

impl Tool for EraseTool
{
    fn get_icon_name(&self) -> &'static str { "md-tool-erase" }
   
    fn use_caret(&self) -> bool { false }

    fn handle_click(&mut self, editor: Rc<RefCell<Editor>>, button: u32, pos: Position) -> super::Event {
        if button == 1 {
            self.paint_brush(&editor, pos);
        }
        super::Event::None
    }

    fn handle_drag(&self, editor: Rc<RefCell<Editor>>, _start: Position, cur: Position) -> super::Event
    {
        self.paint_brush(&editor, cur);
        super::Event::None
    }
}
