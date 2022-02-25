use std::cmp::{max, min};

use crate::model::{Buffer, Position, TextAttribute, Rectangle};

#[derive(Debug, Default)]
pub struct Cursor {
    pub pos: Position,
    pub attr: TextAttribute
}

pub enum Event {
    None,
    CursorPositionChange(Position, Position)
}

pub enum Shape {
    Rectangle,
    Elipse
}

pub struct Selection
{
    pub shape: Shape,
    pub rectangle: Rectangle,
    pub is_preview: bool,
    pub is_active: bool
}

impl Selection {
    pub fn new() -> Self
    {
        Selection {
            shape: Shape::Rectangle,
            rectangle:  Rectangle::from(-1, -1, 0, 0),
            is_preview: true,
            is_active: false
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}

//#[derive(Debug, Default)]
pub struct Editor {
    pub id: usize,
    pub buf: Buffer,
    pub cursor: Cursor,

    pub cur_selection: Selection
}

impl Default for Editor 
{
    fn default() -> Self
    {
        Editor::new(0, Buffer::new())
    }
}

impl Editor 
{
    pub fn new(id: usize, buf: Buffer) -> Self 
    {
        Editor {
            id,
            buf, 
            cursor: Cursor { pos: Position::new(), attr: TextAttribute::DEFAULT },
            cur_selection: Selection::new()
        }
    }

    pub fn handle_key(&mut self, key: gtk4::gdk::Key, key_code: u32, modifier: gtk4::gdk::ModifierType) -> Event
    {
        unsafe {
            crate::WORKSPACE.cur_tool().handle_key( self, key, key_code, modifier)
        }
    }

    pub fn handle_click(&mut self, button: u32, x: i32, y: i32) -> Event
    {
        unsafe {
            crate::WORKSPACE.cur_tool().handle_click( self, button, x, y)
        }
    }

    pub fn handle_drag_begin(&mut self, start: Position, cur: Position) -> Event
    {
        unsafe {
            crate::WORKSPACE.cur_tool().handle_drag_begin( self, start, cur)
        }
    }

    pub fn handle_drag(&mut self, start: Position, cur: Position) -> Event
    {
        unsafe {
            crate::WORKSPACE.cur_tool().handle_drag( self, start, cur)
        }
    }

    pub fn handle_drag_end(&mut self, start: Position, cur: Position) -> Event
    {
        unsafe {
            crate::WORKSPACE.cur_tool().handle_drag_end( self, start, cur)
        }
    }
    
    pub fn set_cursor(&mut self, x: i32, y: i32) -> Event
    {
        let old = self.cursor.pos;
        self.cursor.pos.x = min(max(0, x), self.buf.width as i32);
        self.cursor.pos.y = min(max(0, y), self.buf.height as i32);
        Event::CursorPositionChange(old, self.cursor.pos)
    }
}