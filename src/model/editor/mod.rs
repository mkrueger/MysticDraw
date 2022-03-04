use std::{cmp::{max, min}, path::Path, io::Write, fs::File, ffi::OsStr};
use crate::model::{Buffer, Position, TextAttribute, Rectangle, convert_to_ans, convert_to_asc, convert_to_avt, convert_to_binary, convert_to_pcb, convert_to_xb};

use super::{DosChar, Layer};

pub struct Cursor {
    pos: Position,
    attr: TextAttribute,
    pub insert_mode: bool,
    pub changed: std::boxed::Box<dyn Fn(Position)>
}

impl Cursor {
    pub fn get_position(&self) -> Position
    {
        self.pos
    }

    pub fn set_position(&mut self, pos: Position)
    {
        self.pos = pos;
        (self.changed)(pos);
    }

    pub fn get_attribute(&self) -> TextAttribute
    {
        self.attr
    }

    pub fn set_attribute(&mut self, attr: TextAttribute)
    {
        self.attr = attr;

        // HACK: FILL tool needs the current editor color, 
        unsafe {
            super::FILL_TOOL.attr = attr;
        }
    }
}

impl std::fmt::Debug for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cursor").field("pos", &self.pos).field("attr", &self.attr).field("insert_mode", &self.insert_mode).finish()
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self { pos: Position::default(), attr: TextAttribute::default(), insert_mode: Default::default(), changed: Box::new(|_| {}) }
    }
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Cursor) -> bool {
        self.pos == other.pos && self.attr == other.attr
    }
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
}

impl Selection {
    pub fn new() -> Self
    {
        Selection {
            shape: Shape::Rectangle,
            rectangle:  Rectangle::from(-1, -1, 0, 0),
            is_preview: true,
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Editor {
    pub id: usize,
    pub buf: Buffer,
    
    pub cursor: Cursor,
    pub cur_selection: Option<Selection>,

    cur_outline: i32,
    pub is_inactive: bool,

    pub cur_layer: i32,
    pub outline_changed: std::boxed::Box<dyn Fn(&Editor)>
}

impl std::fmt::Debug for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Editor").field("id", &self.id).field("buf", &self.buf).field("cursor", &self.cursor).field("cur_selection", &self.cur_selection).field("cur_outline", &self.cur_outline).field("is_inactive", &self.is_inactive).finish()
    }
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
            cursor: Cursor::default(),
            cur_selection: None,
            cur_outline: 0,
            is_inactive: false,
            outline_changed: Box::new(|_| {}),
            cur_layer: 0
        }
    }
    
    pub fn get_overlay_layer(&mut self) -> &mut Option<Layer>
    {
        self.buf.get_overlay_layer()
    }
    
    pub fn join_overlay(&mut self)
    {
        self.buf.join_overlay(self.cur_layer);
    }

    pub fn delete_line(&mut self, line: i32)
    {
        let layer = &mut self.buf.layers[0];
        assert!(!(line < 0 || line >= self.buf.height as i32), "line out of range");
        layer.lines.remove(line as usize);
    }

    pub fn insert_line(&mut self, line: i32) {
        let layer = &mut self.buf.layers[0];
        assert!(!(line < 0 || line >= self.buf.height as i32), "line out of range");
        layer.lines.insert(line as usize, super::Line::new());
    }

    pub fn pickup_color(&mut self, pos: Position)
    {
        let ch = self.buf.get_char(pos);
        self.cursor.attr = ch.attribute;
    }

    pub fn set_cursor(&mut self, x: i32, y: i32) -> Event
    {
        let old = self.cursor.pos;
        self.cursor.set_position(Position::from(
            min(max(0, x), self.buf.width as i32 - 1),
            min(max(0, y), self.buf.height as i32 - 1)));
        Event::CursorPositionChange(old, self.cursor.pos)
    }
    
    pub fn get_cur_outline(&self) -> i32
    {
        self.cur_outline
    }

    pub fn set_cur_outline(&mut self, outline: i32)
    {
        self.cur_outline = outline;
        (self.outline_changed)(self);
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

    pub fn get_outline_char_code(&self, i: i32) -> Result<u8, &str>
    {
        if self.cur_outline < 0 || self.cur_outline >= DEFAULT_OUTLINE_TABLE.len() as i32 {
            return Err("current outline out of range.");
        }
        if !(0..=10).contains(&i) {
            return Err("outline char# out of range.");
        }
        
        Ok(DEFAULT_OUTLINE_TABLE[self.cur_outline as usize][i as usize])
    }
    
    pub fn get_outline_char_code_from(&self, outline:i32, i: i32) -> Result<u8, &str>
    {
        if outline < 0 || outline >= DEFAULT_OUTLINE_TABLE.len() as i32 {
            return Err("current outline out of range.");
        }
        if !(0..=10).contains(&i) {
            return Err("outline char# out of range.");
        }
        Ok(DEFAULT_OUTLINE_TABLE[outline as usize][i as usize])
    }
    
    pub fn set_char(&mut self, pos: Position, dos_char: DosChar) {
        if self.point_is_valid(pos) {
            self.buf.set_char(self.cur_layer as usize, pos, dos_char);
        }
    }

    pub fn point_is_valid(&self, pos: Position) -> bool {
        if let Some(selection) = &self.cur_selection {
            return selection.rectangle.is_inside(pos);
        }

        pos.x >= 0 &&
        pos.y >= 0 && 
        pos.x < self.buf.width as i32 &&
        pos.y < self.buf.height as i32
    }

    pub fn type_key(&mut self, char_code: u8) {
        let pos = self.cursor.pos;
        if self.cursor.insert_mode {
            for i in (self.buf.width as i32 - 1)..=pos.x {
                let next = self.buf.get_char( Position::from(i - 1, pos.y));
                self.set_char(Position::from(i, pos.y), next);
            }
        }

        self.set_char(pos, crate::model::DosChar {
            char_code,
            attribute: self.cursor.attr,
        });
        self.set_cursor(pos.x + 1, pos.y);
    }
}

const DEFAULT_OUTLINE_TABLE: [[u8;10];15] = [
    [218, 191, 192, 217, 196, 179, 195, 180, 193, 194 ],
    [201, 187, 200, 188, 205, 186, 204, 185, 202, 203 ],
    [213, 184, 212, 190, 205, 179, 198, 181, 207, 209 ],
    [214, 183, 211, 189, 196, 186, 199, 182, 208, 210 ],
    [197, 206, 216, 215, 232, 233, 155, 156, 153, 239 ],
    [176, 177, 178, 219, 223, 220, 221, 222, 254, 250 ],
    [1, 2, 3, 4, 5, 6, 240, 127, 14, 15 ],
    [24, 25, 30, 31, 16, 17, 18, 29, 20, 21 ],
    [174, 175, 242, 243, 169, 170, 253, 246, 171, 172 ],
    [227, 241, 244, 245, 234, 157, 228, 248, 251, 252 ],
    [224, 225, 226, 229, 230, 231, 235, 236, 237, 238 ],
    [128, 135, 165, 164, 152, 159, 247, 249, 173, 168 ],
    [131, 132, 133, 160, 166, 134, 142, 143, 145, 146 ],
    [136, 137, 138, 130, 144, 140, 139, 141, 161, 158 ],
    [147, 148, 149, 162, 167, 150, 129, 151, 163, 154 ]
];