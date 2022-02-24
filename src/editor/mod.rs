use std::cmp::{max, min};

use crate::{model::{Buffer, Position, DEFAULT_ATTRIBUTE, TextAttribute}};

#[derive(Debug, Default)]
pub struct Cursor {
    pub pos: Position,
    pub attr: TextAttribute
}

pub enum EditorEvent {
    None,
    CursorPositionChange(Position, Position)
}

//#[derive(Debug, Default)]
pub struct Editor {
    pub id: usize,
    pub buf: Buffer,
    pub cursor: Cursor
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
            cursor: Cursor { pos: Position::new(), attr: DEFAULT_ATTRIBUTE }
        }
    }


    pub fn handle_key(&mut self, key: gtk4::gdk::Key, key_code: u32, modifier: gtk4::gdk::ModifierType) -> bool
    {
        unsafe {
            crate::WORKSPACE.cur_tool().handle_key( self, key, key_code, modifier)
        }
    }

    pub fn set_cursor(&mut self, x: i32, y: i32) -> EditorEvent
    {
        let old = self.cursor.pos;
        self.cursor.pos.x = min(max(0, x), self.buf.width as i32);
        self.cursor.pos.y = min(max(0, y), self.buf.height as i32);
        EditorEvent::CursorPositionChange(old, self.cursor.pos)
    }
}