use std::{rc::Rc, cell::{RefCell, RefMut}, cmp::{max, min}};

use super::{TextAttribute, DosChar};
pub use super::{Editor, Event, Position};

pub mod brush_imp;
mod click_imp;
pub mod draw_rectangle_imp;
mod draw_ellipse_imp;
pub mod erase_imp;
mod fill_imp;
mod font_imp;
mod pipette_imp;
mod line_imp;
mod flip_imp;
mod move_layer_imp;

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

impl MModifiers
{
    pub fn is_shift(self) -> bool 
    {
        matches!(self, MModifiers::Shift)
    }

    pub fn is_alt(self) -> bool 
    {
        matches!(self, MModifiers::Alt)
    }

    pub fn is_control(self) -> bool 
    {
        matches!(self, MModifiers::Control)
    }
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
    fn get_icon_name(&self) -> &'static str;
    
    fn use_caret(&self) -> bool { true }
    
    fn use_selection(&self) -> bool { true }
    
    fn handle_key(&mut self, editor: Rc<RefCell<Editor>>, key: MKey, key_code: MKeyCode, modifier: MModifiers) -> Event
    {
        // TODO Keys:

        // Tab - Next tab
        // Shift+Tab - Prev tab

        // ctrl+pgup  - upper left corner
        // ctrl+pgdn  - lower left corner
        let pos = editor.borrow().get_cursor_position();
        let mut editor = editor.borrow_mut();
        match key {
            MKey::Down => {
                if let MModifiers::Control = modifier {
                    let fg = (editor.cursor.get_attribute().get_foreground() + 14) % 16;
                    editor.cursor.get_attribute().set_foreground(fg);
                } else {
                    editor.set_cursor(pos.x, pos.y + 1);
                }
            }
            MKey::Up => {
                if let MModifiers::Control = modifier {
                    let fg = (editor.cursor.get_attribute().get_foreground() + 1) % 16;
                    editor.cursor.get_attribute().set_foreground(fg);
                } else {
                    editor.set_cursor(pos.x, pos.y - 1);
                }
            }
            MKey::Left => {
                // TODO: ICE Colors
                if let MModifiers::Control = modifier {
                    let bg = (editor.cursor.get_attribute().get_background() + 7) % 8;
                    editor.cursor.get_attribute().set_background(bg);
                } else {
                    editor.set_cursor(pos.x - 1, pos.y);
                }
            }
            MKey::Right => {
                // TODO: ICE Colors
                if let MModifiers::Control = modifier {
                    let bg = (editor.cursor.get_attribute().get_background() + 1) % 8;
                    editor.cursor.get_attribute().set_background(bg);
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

            MKey::Tab => {
                let tab_size = unsafe { crate::WORKSPACE.settings.tab_size } ;
                if let MModifiers::Control = modifier {
                    let tabs = max(0, (pos.x / tab_size) - 1);
                    let next_tab = tabs * tab_size;
                    editor.set_cursor(next_tab, pos.y);
                } else {
                    let tabs = 1 + pos.x / tab_size;
                    let next_tab = min(editor.buf.width as i32 - 1, tabs * tab_size);
                    editor.set_cursor(next_tab, pos.y);
                }
            }
            MKey::Home  => {
                if let MModifiers::Control = modifier {
                    for i in 0..editor.buf.width {
                        if !editor.get_char_from_cur_layer(pos.with_x(i as i32)).unwrap_or_default().is_transparent() {
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
                        if !editor.get_char_from_cur_layer(pos.with_x(i as i32)).unwrap_or_default().is_transparent() {
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
                if editor.cur_selection.is_some() {
                    editor.delete_selection(); 
                } else {
                    let pos = editor.get_cursor_position();
                    for i in pos.x..(editor.buf.width as i32 - 1) {
                        let next = editor.get_char_from_cur_layer( Position::from(i + 1, pos.y));
                        editor.set_char(Position::from(i, pos.y), next);
                    }
                    let last_pos = Position::from(editor.buf.width as i32 - 1, pos.y);
                    editor.set_char(last_pos, None);
                }
            }
            MKey::Insert => {
                editor.cursor.insert_mode = !editor.cursor.insert_mode;
            }
            MKey::Backspace => {
                editor.cur_selection = None;
                let pos = editor.get_cursor_position();
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
                        editor.set_cursor_position(pos + Position::from(-1, 0));
                    if editor.cursor.insert_mode {
                        for i in pos.x..(editor.buf.width as i32 - 1) {
                            let next = editor.get_char_from_cur_layer( Position::from(i + 1, pos.y));
                            editor.set_char(Position::from(i, pos.y), next);
                        }
                        let last_pos = Position::from(editor.buf.width as i32 - 1, pos.y);
                        editor.set_char(last_pos, None);
                    } else  {
                        let pos = editor.get_cursor_position();
                        editor.set_char(pos, None);
                    } 
                }
            }

            MKey::Character(ch) => { 
                editor.cur_selection = None;
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
            MKey::Escape => {
                editor.cur_selection = None;
            }
            _ => {}
        }
        Event::None
    }

    fn handle_click(&mut self, _editor: Rc<RefCell<Editor>>, _button: u32, _pos: Position) -> Event {
        Event::None
    }

    fn handle_drag_begin(&mut self, _editor: Rc<RefCell<Editor>>, _start: Position, _cur: Position) -> Event {
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
    editor.cur_selection = None;
    let ch = editor.get_outline_char_code(outline);
    if let Ok(ch) = ch {
        editor.type_key(ch);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DrawMode {
    Line,
    Char,
    Shade,
    Colorize
}

trait Plottable {
    fn get_draw_mode(&self) -> DrawMode;

    fn get_use_fore(&self) -> bool;
    fn get_use_back(&self) -> bool;
    fn get_char_code(&self) -> u8;
}

fn plot_point(editor: &Rc<RefCell<Editor>>, tool: &dyn Plottable, pos: Position)
{
    let ch = editor.borrow().get_char_from_cur_layer(pos).unwrap_or_default();
    let editor_attr = editor.borrow().cursor.get_attribute();
    let mut attribute= ch.attribute;
    if tool.get_use_back() {
        attribute.set_background(editor_attr.get_background());
    }
    if tool.get_use_fore() {
        attribute.set_foreground(editor_attr.get_foreground());
    }

    match tool.get_draw_mode() {
        DrawMode::Line => {
            if let Some(layer) = editor.borrow_mut().get_overlay_layer() {
                layer.set_char(
                    pos,
                    Some(DosChar {
                        char_code: 219,
                        attribute,
                    }),
                );
            }
        },
        DrawMode::Char => {
            if let Some(layer) = editor.borrow_mut().get_overlay_layer() {
                layer.set_char(
                    pos,
                    Some(DosChar {
                        char_code: tool.get_char_code(),
                        attribute,
                    }),
                );
            }
        },
        DrawMode::Shade => {
            let mut char_code = SHADE_GRADIENT[0];
            if ch.char_code == SHADE_GRADIENT[SHADE_GRADIENT.len() -1] {
                char_code = SHADE_GRADIENT[SHADE_GRADIENT.len() -1];
            } else {
                for i in 0..SHADE_GRADIENT.len() - 1 {
                    if ch.char_code == SHADE_GRADIENT[i] {
                        char_code = SHADE_GRADIENT[i + 1];
                        break;
                    }
                }
            }
            if let Some(layer) = editor.borrow_mut().get_overlay_layer() {
                layer.set_char(
                    pos,
                    Some(DosChar {
                        char_code,
                        attribute,
                    }),
                );
            }
        }
        DrawMode::Colorize => {
            if let Some(layer) = editor.borrow_mut().get_overlay_layer() {
                layer.set_char(
                    pos,
                    Some(DosChar {
                        char_code: ch.char_code,
                        attribute,
                    }),
                );
            }
        }
    }
}

pub static SHADE_GRADIENT: [u8;4] = [176, 177, 178, 219];

pub static mut TOOLS: Vec<&mut dyn Tool> = Vec::new();

pub static mut CLICK_TOOL: click_imp::ClickTool = click_imp::ClickTool { };
pub static mut FONT_TOOL: font_imp::FontTool = font_imp::FontTool { fonts: Vec::new(), selected_font: -1, sizes: Vec::new() };

pub static mut LINE_TOOL: line_imp::LineTool = line_imp::LineTool {
    draw_mode: DrawMode::Line, 
    use_fore: true, 
    use_back: true, 
    attr: TextAttribute::DEFAULT, 
    char_code: b'#',
    old_pos: Position { x: 0, y: 0 }
};

pub static mut RECT_TOOL: draw_rectangle_imp::DrawRectangleTool = draw_rectangle_imp::DrawRectangleTool { 
    draw_mode: DrawMode::Line, 
    use_fore: true, 
    use_back: true, 
    fill_mode: false, 
    attr: TextAttribute::DEFAULT, 
    char_code: b'#'
};

pub static mut ELLIPSE_TOOL: draw_ellipse_imp::DrawEllipseTool = draw_ellipse_imp::DrawEllipseTool {
    draw_mode: DrawMode::Line, 
    use_fore: true, 
    use_back: true, 
    fill_mode: false, 
    attr: TextAttribute::DEFAULT, 
    char_code: b'#'
};

pub static mut BRUSH_TOOL: brush_imp::BrushTool = brush_imp::BrushTool {
    size: 3, 
    use_back: true,
    use_fore: true,
    brush_type: brush_imp::BrushType::Shade,
    char_code: b'#',
 };

pub static mut ERASE_TOOL: erase_imp::EraseTool = erase_imp::EraseTool { size: 3, brush_type: erase_imp::EraseType::Shade };
pub static mut PIPETTE_TOOL: pipette_imp::PipetteTool = pipette_imp::PipetteTool { };
pub static mut FILL_TOOL: fill_imp::FillTool = fill_imp::FillTool {
    use_char: true,
    use_fore: true,
    use_back: true,
    char_code: b'#',
    attr: TextAttribute::DEFAULT
};
pub static mut FLIP_TOOL: flip_imp::FlipTool = flip_imp::FlipTool { };
pub static mut MOVE_LAYER_TOOL: move_layer_imp::MoveLayer = move_layer_imp::MoveLayer { pos: Position { x: 0, y: 0 } };

pub fn init_tools()
{
    unsafe {
        FONT_TOOL.load_fonts();
        TOOLS.push(&mut CLICK_TOOL);
        
        TOOLS.push(&mut BRUSH_TOOL);
        TOOLS.push(&mut ERASE_TOOL);
        TOOLS.push(&mut PIPETTE_TOOL);

        TOOLS.push(&mut LINE_TOOL);
        TOOLS.push(&mut RECT_TOOL);
        TOOLS.push(&mut ELLIPSE_TOOL);
        
        TOOLS.push(&mut FILL_TOOL);
        TOOLS.push(&mut FLIP_TOOL);
        TOOLS.push(&mut FONT_TOOL);
        TOOLS.push(&mut MOVE_LAYER_TOOL);
    }
}