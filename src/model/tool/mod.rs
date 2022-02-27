use std::{rc::Rc, cell::RefCell};

pub use super::{Editor, Event, Position};

mod brush_tool;
mod click_tool;
mod draw_shape_tool;
mod erase_tool;
mod fill_tool;
mod font_tool;
mod paint_tool;
mod select_tool;
#[derive(Copy, Clone, Debug)]
 pub enum MKey {
    Character(u8),
    Down,
    Up,
    Left,
    Right,
    PageDown,
    PageUp,
    Home,
    End,
    Return,
    Delete,
    Insert,
    Backspace,
    Tab,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12
}

#[derive(Copy, Clone, Debug)]
pub enum MModifiers
{
    None,
    Shift,
    Alt,
    Control
}

pub trait Tool
{
    fn get_icon_name(&self) -> &'static str;/* 
    fn add_tool_page(&self, window: &ApplicationWindow,parent: &mut gtk4::Box);
*/
    fn handle_key(&self, _editor: Rc<RefCell<Editor>>, key: MKey, _modifier: MModifiers) -> Event
    {
        let pos = _editor.borrow().cursor.pos;
        let attr = _editor.borrow().cursor.attr;
        let mut editor = _editor.borrow_mut();

        match key {
            MKey::Down => {
                editor.set_cursor(pos.x, pos.y + 1);
            }
            MKey::Up => {
                editor.set_cursor(pos.x, pos.y - 1);
            }
            MKey::Left => {
                editor.set_cursor(pos.x - 1, pos.y);
            }
            MKey::Right => {
                editor.set_cursor(pos.x + 1, pos.y);
            }
            
            MKey::PageDown |
            
            MKey::PageUp => {
                // TODO
            }
            MKey::Home  => {
                editor.set_cursor(0, pos.y);
            }
            
            MKey::End => {
                let w = editor.buf.width as i32;
                editor.set_cursor(w - 1, pos.y);
            }

            MKey::Return => {
                editor.set_cursor(0,pos.y + 1);
            }
/*

                            case SDLK_DELETE:
                    for (int i = caret.getLogicalX(); i < getCurrentBuffer()->getWidth(); ++i) {
                        getCurrentBuffer()->getCharacter(caret.getLogicalY(), i) = getCurrentBuffer()->getCharacter(caret.getLogicalY(), i + 1);
                        getCurrentBuffer()->getAttribute(caret.getLogicalY(), i) = getCurrentBuffer()->getAttribute(caret.getLogicalY(), i + 1);
                    }
                    getCurrentBuffer()->getCharacter(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = ' ';
                    getCurrentBuffer()->getAttribute(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = 7;
                    break;
                case SDLK_INSERT:
                    caret.insertMode() = !caret.insertMode();
                    break;
                case SDLK_BACKSPACE:
                    if (caret.getLogicalX()>0){
                        if (caret.fontMode() && FontTyped && cpos > 0)  {
                            caret.getX() -= CursorPos[cpos] - 1;
                            for (a=0;a<=CursorPos[cpos];a++)
                            for (b=0;b<=FontLibrary::getInstance().maxY;b++) {
                                getCurrentBuffer()->getCharacter(caret.getLogicalY() + b, caret.getLogicalX()+a) = getUndoBuffer()->getCharacter(caret.getLogicalY() + b, caret.getLogicalX()+a);
                                getCurrentBuffer()->getAttribute(caret.getLogicalY() + b, caret.getLogicalX()+a) = getUndoBuffer()->getAttribute(caret.getLogicalY() + b, caret.getLogicalX()+a);
                            }
                            cpos--;
                        } else {	
                            cpos=0;
                            caret.getX()--;
                            if (caret.insertMode()) {
                                for (int i = caret.getLogicalX(); i < getCurrentBuffer()->getWidth(); ++i) {
                                    getCurrentBuffer()->getCharacter(caret.getLogicalY(), i) = getCurrentBuffer()->getCharacter(caret.getLogicalY(), i + 1);
                                    getCurrentBuffer()->getAttribute(caret.getLogicalY(), i) = getCurrentBuffer()->getAttribute(caret.getLogicalY(), i + 1);
                                }
                                getCurrentBuffer()->getCharacter(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = ' ';
                                getCurrentBuffer()->getAttribute(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = 7;
                            } else  {
                                getCurrentBuffer()->getCharacter(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = ' ';
                                getCurrentBuffer()->getAttribute(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = 7;
                            } 
                        }
                    }
                    break;
*/
            MKey::Character(ch) => { 
                let attr = editor.cursor.attr;
                editor.buf.set_char(pos, crate::model::DosChar {
                    char_code: ch,
                    attribute: attr,
                });
                editor.set_cursor(pos.x + 1, pos.y);
            }
            _ => {}
        }
        Event::None
    }

    fn handle_click(&self, _editor: Rc<RefCell<Editor>>, _button: u32, _pos: Position) -> Event {
        Event::None
    }
}
/* 
    fn handle_drag_begin(&self, _editor: &mut Editor, _start: Position, _cur: Position) -> Event {
        Event::None
    }

    fn handle_drag(&self, _editor: &mut Editor, _start: Position, _cur: Position) -> Event {
        Event::None
    }

    fn handle_drag_end(&self, _editor: &mut Editor, _start: Position, _cur: Position) -> Event {
        Event::None
    }*/


pub static mut FONT_TOOL: font_tool::FontTool = font_tool::FontTool { fonts: Vec::new(), selected_font: -1  };
pub static mut TOOLS: Vec<&dyn Tool> = Vec::new();

pub fn init_tools()
{
    unsafe {
        // FONT_TOOL.load_fonts();
        TOOLS.push(&click_tool::ClickTool {});
        TOOLS.push(&select_tool::SelectTool {});
        TOOLS.push(&paint_tool::PaintTool{});
        TOOLS.push(&brush_tool::BrushTool{});
        TOOLS.push(&erase_tool::EraseTool{});
        TOOLS.push(&draw_shape_tool::DrawShapeTool{});
        TOOLS.push(&fill_tool::FillTool{});
        TOOLS.push(&FONT_TOOL);
    }
}