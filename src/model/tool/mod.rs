use std::{rc::Rc, cell::{RefCell, RefMut}};

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
        println!("{:?} {:?} {:?}", key, key_code, modifier);
        let pos = editor.borrow().cursor.get_position();
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
                            return Event::None;
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
                let pos = editor.cursor.get_position();
                for i in pos.x..(editor.buf.width as i32 - 1) {
                    let next = editor.buf.get_char( Position::from(i + 1, pos.y));
                    editor.set_char(Position::from(i, pos.y), next);
                }
                let last_pos = Position::from(editor.buf.width as i32 - 1, pos.y);
                editor.set_char(last_pos, super::DosChar { char_code: b' ', attribute: TextAttribute::DEFAULT });
            }
            MKey::Insert => {
                editor.cursor.insert_mode = !editor.cursor.insert_mode;
            }
            MKey::Backspace => {
                let pos = editor.cursor.get_position();
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
                        editor.cursor.set_position(pos + Position::from(-1, 0));
                    if editor.cursor.insert_mode {
                        for i in pos.x..(editor.buf.width as i32 - 1) {
                            let next = editor.buf.get_char( Position::from(i + 1, pos.y));
                            editor.set_char(Position::from(i, pos.y), next);
                        }
                        let last_pos = Position::from(editor.buf.width as i32 - 1, pos.y);
                        editor.set_char(last_pos, super::DosChar { char_code: b' ', attribute: TextAttribute::DEFAULT });
                    } else  {
                        let pos = editor.cursor.get_position();
                        editor.set_char(pos, super::DosChar { char_code: b' ', attribute: TextAttribute::DEFAULT });
                    } 
                }
            }

            MKey::Character(ch) => { 
                if let MModifiers::Alt = modifier {
                    match key_code { 
                        MKeyCode::KeyI => editor.insert_line(pos.y),
                        MKeyCode::KeyU => editor.pickup_color(pos),
                        MKeyCode::KeyY => editor.delete_line(pos.y),
                        MKeyCode::Unknown => {}
                    }
                    return Event::None;
                }

                editor.type_key(ch);
            }

            MKey::F1 => {
                handle_outline_insertion(&mut editor, modifier, 0);
            }
            MKey::F2 => {
                handle_outline_insertion(&mut editor, modifier, 1);
            }
            MKey::F3 => {
                handle_outline_insertion(&mut editor, modifier, 2);
            }
            MKey::F4 => {
                handle_outline_insertion(&mut editor, modifier, 3);
            }
            MKey::F5 => {
                handle_outline_insertion(&mut editor, modifier, 4);
            }
            MKey::F6 => {
                handle_outline_insertion(&mut editor, modifier, 5);
            }
            MKey::F7 => {
                handle_outline_insertion(&mut editor, modifier, 6);
            }
            MKey::F8 => {
                handle_outline_insertion(&mut editor, modifier, 7);
            }
            MKey::F9 => {
                handle_outline_insertion(&mut editor, modifier, 8);
            }
            MKey::F10 => {
                handle_outline_insertion(&mut editor, modifier, 9);
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

fn handle_outline_insertion(editor: &mut RefMut<Editor>, modifier: MModifiers, outline: i32) {
    if let MModifiers::Control = modifier {
        editor.set_cur_outline(outline);
        return;
    }

    if outline < 5 {
        if let MModifiers::Shift = modifier {
            editor.set_cur_outline(10 + outline);
            return;
        }
    }

    let ch = editor.get_outline_char_code(outline);
    if let Ok(ch) = ch {
        editor.type_key(ch);
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

