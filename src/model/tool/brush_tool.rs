use std::{cell::{RefCell}, rc::Rc};
use super::{ Tool, Editor, Position};

pub enum BrushType {
    Gradient,
    Character(u8),
    Color(bool, bool)
}

pub struct BrushTool {
    pub size: i32,
    pub brush_type: BrushType
}

impl BrushTool {

    fn paint_brush(&self, editor: &Rc<RefCell<Editor>>, pos: Position)
    {
        let mid = Position::from(-(self.size / 2), -(self.size / 2));

        let center = pos + mid;
        let gradient = [176, 177, 178, 219];
        let mut editor = editor.borrow_mut();
        for y in 0..self.size {
            for x in 0..self.size {
                match self.brush_type {
                    BrushType::Gradient => {    
                        let ch = editor.buf.get_char(center + Position::from(x, y));
                       
                        let attribute= editor.cursor.get_attribute();

                        let mut char_code = gradient[0];
                        if ch.char_code == gradient[gradient.len() -1] {
                            char_code = gradient[gradient.len() -1];
                        } else {
                            for i in 0..gradient.len() - 1 {
                                if ch.char_code == gradient[i] {
                                    char_code = gradient[i + 1];
                                    break;
                                }
                            }
                        }
                        editor.set_char(center + Position::from(x, y), crate::model::DosChar { 
                            char_code, 
                            attribute
                        });

                    },
                    BrushType::Character(char_code) => {
                        let attribute= editor.cursor.get_attribute();
                        editor.set_char(center + Position::from(x, y), crate::model::DosChar { char_code, attribute });
                    },
                    BrushType::Color(use_fore, use_back) => {
                        let ch = editor.buf.get_char(center + Position::from(x, y));
                        let mut attribute = ch.attribute;

                        if use_fore {
                            attribute.set_foreground(editor.cursor.get_attribute().get_foreground());
                        }
                        if use_back {
                            attribute.set_background_ice(editor.cursor.get_attribute().get_background_ice());
                        }

                        editor.set_char(center + Position::from(x, y), crate::model::DosChar { 
                            char_code:ch.char_code, 
                            attribute
                        });
                    },
                }
            }                
        }
    }
    
}

impl Tool for BrushTool
{
    fn get_icon_name(&self) -> &'static str { "md-tool-draw" }
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