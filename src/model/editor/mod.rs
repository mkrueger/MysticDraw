use std::{cmp::{max, min}, path::Path, io::Write, fs::File, ffi::OsStr};

use crate::model::{Buffer, Position, TextAttribute, Rectangle, convert_to_ans, convert_to_asc, convert_to_avt, convert_to_binary, convert_to_pcb, convert_to_xb};

#[derive(Debug, Default)]
pub struct Cursor {
    pub pos: Position,
    pub attr: TextAttribute
}

pub enum Event {
    None,
    CursorPositionChange(Position, Position)
}

#[derive(Debug)]
pub enum Shape {
    Rectangle,
    Elipse
}

#[derive(Debug)]
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

#[derive(Debug)]
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
/* 
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
    */
    pub fn set_cursor(&mut self, x: i32, y: i32) -> Event
    {
        let old = self.cursor.pos;
        self.cursor.pos.x = min(max(0, x), self.buf.width as i32);
        self.cursor.pos.y = min(max(0, y), self.buf.height as i32);
        Event::CursorPositionChange(old, self.cursor.pos)
    }

    pub fn save_content(&self, file_name: &Path)
    {
        let mut f = File::create(file_name).expect("Can't create file.");

        let content = 
            if let Some(ext) = file_name.extension() {
                let ext = OsStr::to_str(ext).unwrap().to_lowercase();
                self.get_file_content(ext.as_str())
            } else {
                self.get_file_content("")
            };
        
        f.write_all(&content).expect("Can't write file.");
    }

    pub fn get_file_content(&self, extension: &str) -> Vec<u8>
    {
        match extension {
            "bin" => convert_to_binary(&self.buf),
            "xb" => convert_to_xb(&self.buf),
            "ans" => convert_to_ans(&self.buf),
            "avt" => convert_to_avt(&self.buf),
            "pcb" => convert_to_pcb(&self.buf),
            _ => convert_to_asc(&self.buf)
        }
    }
}