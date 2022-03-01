use std::{rc::Rc, cell::RefCell};

use super::TextAttribute;
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
#[derive(Copy, Clone, Debug)]
pub enum MKeyCode
{
    Unknown,
    KeyI,
    KeyU,
    KeyY,
}

pub trait Tool
{
    fn get_icon_name(&self) -> &'static str;/* 
    fn add_tool_page(&self, window: &ApplicationWindow,parent: &mut gtk4::Box);
*/

    fn handle_key(&self, editor: Rc<RefCell<Editor>>, key: MKey, key_code: MKeyCode, modifier: MModifiers) -> Event
    {
        // TODO Keys:

        // ALT+Left Delete current column
        // ALT+Right Insert a colum

        // ALT-F1-10 select char set 1-10
        // CTRL-F1-5 Select char set 11-15

        // Tab - Next tab
        // Shift+Tab - Prev tab

        // ctrl+pgup  - upper left corner
        // ctrl+pgdn  - lower left corner

        let pos = editor.borrow().cursor.pos;
        let mut editor = editor.borrow_mut();
        match key {
            MKey::Down => {
                if let MModifiers::Control = modifier {
                    let fg = (editor.cursor.attr.get_foreground() + 14) % 16;
                    editor.cursor.attr.set_foreground(fg);
                } else {
                    editor.set_cursor(pos.x, pos.y + 1);
                }
            }
            MKey::Up => {
                if let MModifiers::Control = modifier {
                    let fg = (editor.cursor.attr.get_foreground() + 1) % 16;
                    editor.cursor.attr.set_foreground(fg);
                } else {
                    editor.set_cursor(pos.x, pos.y - 1);
                }
            }
            MKey::Left => {
                // TODO: ICE Colors
                if let MModifiers::Control = modifier {
                    let bg = (editor.cursor.attr.get_background() + 7) % 8;
                    editor.cursor.attr.set_background(bg);
                } else {
                    editor.set_cursor(pos.x - 1, pos.y);
                }
            }
            MKey::Right => {
                // TODO: ICE Colors
                if let MModifiers::Control = modifier {
                    let bg = (editor.cursor.attr.get_background() + 1) % 8;
                    editor.cursor.attr.set_background(bg);
                } else {
                    editor.set_cursor(pos.x + 1, pos.y);
                }
            }
            MKey::PageDown => {
                // TODO
                println!("pgdn");
            }
            MKey::PageUp => {
                // TODO
                println!("pgup");
            }
            MKey::Home  => {
                if let MModifiers::Control = modifier {
                    for i in 0..editor.buf.width {
                        if !editor.buf.get_char(pos.with_x(i as i32)).is_transparent() {
                            editor.set_cursor(i as i32, pos.y);
                            return Event::None;
                        }
                    }
                }
                editor.set_cursor(0, pos.y);
            }
            MKey::End => {
                if let MModifiers::Control = modifier {
                    for i in (0..editor.buf.width).rev()  {
                        if !editor.buf.get_char(pos.with_x(i as i32)).is_transparent() {
                            editor.set_cursor(i as i32, pos.y);
                        }
                    }
                }
                let w = editor.buf.width as i32;
                editor.set_cursor(w - 1, pos.y);
            }
            MKey::Return => {
                editor.set_cursor(0,pos.y + 1);
            }
            MKey::Delete => {
                let pos = editor.cursor.pos;
                for i in pos.x..(editor.buf.width as i32 - 1) {
                    let next = editor.buf.get_char( Position::from(i + 1, pos.y));
                    editor.buf.set_char(Position::from(i, pos.y), next);
                }
                let last_pos = Position::from(editor.buf.width as i32 - 1, pos.y);
                editor.buf.set_char(last_pos, super::DosChar { char_code: b' ', attribute: TextAttribute::DEFAULT });
            }
            MKey::Insert => {
                editor.cursor.insert_mode = !editor.cursor.insert_mode;
            }
            MKey::Backspace => {
                let pos = editor.cursor.pos;
                if pos.x> 0 {
                   /* if (caret.fontMode() && FontTyped && cpos > 0)  {
                        caret.getX() -= CursorPos[cpos] - 1;
                        for (a=0;a<=CursorPos[cpos];a++)
                        for (b=0;b<=FontLibrary::getInstance().maxY;b++) {
                            getCurrentBuffer()->getCharacter(caret.getLogicalY() + b, caret.getLogicalX()+a) = getUndoBuffer()->getCharacter(caret.getLogicalY() + b, caret.getLogicalX()+a);
                            getCurrentBuffer()->getAttribute(caret.getLogicalY() + b, caret.getLogicalX()+a) = getUndoBuffer()->getAttribute(caret.getLogicalY() + b, caret.getLogicalX()+a);
                        }
                        cpos--;
                    } else {*/	
                        editor.cursor.pos.x -= 1;
                    if editor.cursor.insert_mode {
                        for i in pos.x..(editor.buf.width as i32 - 1) {
                            let next = editor.buf.get_char( Position::from(i + 1, pos.y));
                            editor.buf.set_char(Position::from(i, pos.y), next);
                        }
                        let last_pos = Position::from(editor.buf.width as i32 - 1, pos.y);
                        editor.buf.set_char(last_pos, super::DosChar { char_code: b' ', attribute: TextAttribute::DEFAULT });
                    } else  {
                        let pos = editor.cursor.pos;
                        editor.buf.set_char(pos, super::DosChar { char_code: b' ', attribute: TextAttribute::DEFAULT });
                    } 
                }
            }
            /*			
            
            if (event->key.keysym.mod & KMOD_SHIFT) {
				switch (event->key.keysym.sym) {   
					case SDLK_F1:
						ActiveCharset=1;
						return true;
					case SDLK_F2:
						ActiveCharset=2;
						return true;
					case SDLK_F3:
						ActiveCharset=3;
						return true;
					case SDLK_F4:
						ActiveCharset=4;
						return true;
					case SDLK_F5:
						ActiveCharset=5;
						return true;
					case SDLK_F6:
						ActiveCharset=6;
						return true;
					case SDLK_F7:
						ActiveCharset=7;
						return true;
					case SDLK_F8:
						ActiveCharset=8;
						return true;
					case SDLK_F9:
						ActiveCharset=9;
						return true;
					case SDLK_F10:
						ActiveCharset=10;
						return true;
					default:
						break;
				}
			}       
 */
            MKey::Character(ch) => { 
                if let MModifiers::Alt = modifier {
                    match key_code { 
                        MKeyCode::KeyI => editor.insert_line(pos.y),
                        MKeyCode::KeyU => editor.pickup_color(pos),
                        MKeyCode::KeyY => editor.delete_line(pos.y),
                        _ => {}
                    }
                    return Event::None;
                }

                let attr = editor.cursor.attr;
                if editor.cursor.insert_mode {
                    for i in (editor.buf.width as i32 - 1)..=pos.x {
                        let next = editor.buf.get_char( Position::from(i - 1, pos.y));
                        editor.buf.set_char(Position::from(i, pos.y), next);
                    }
                }

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

    fn handle_drag_begin(&self, _editor: Rc<RefCell<Editor>>, _start: Position, _cur: Position) -> Event {
        Event::None
    }

    fn handle_drag(&self, _editor: Rc<RefCell<Editor>>, _start: Position, _cur: Position) -> Event {
        Event::None
    }

    fn handle_drag_end(&self, _editor: Rc<RefCell<Editor>>, _start: Position, _cur: Position) -> Event {
        Event::None
    }
}
   

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
