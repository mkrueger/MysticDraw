use std::{cmp::{max, min}, path::Path, io::{Write, self}, fs::File, ffi::OsStr};
use crate::model::{Buffer, Position, TextAttribute, Rectangle};

use super::{DosChar, UndoSetChar};

pub struct Cursor {
    pos: Position,
    attr: TextAttribute,
    pub insert_mode: bool,
    pub pos_changed: std::boxed::Box<dyn Fn(&Editor, Position)>,
    pub attr_changed: std::boxed::Box<dyn Fn(TextAttribute)>
}

impl Cursor {
    pub fn get_attribute(&self) -> TextAttribute
    {
        self.attr
    }

    pub fn set_attribute(&mut self, attr: TextAttribute)
    {
        if attr == self.attr {
            return;
        }
        self.attr = attr;

        // HACK: FILL tool needs the current editor color, 
        unsafe {
            super::LINE_TOOL.attr = attr;
            super::RECT_TOOL.attr = attr;
            super::FILL_TOOL.attr = attr;
        }
        (self.attr_changed)(attr);
    }
}

impl std::fmt::Debug for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cursor").field("pos", &self.pos).field("attr", &self.attr).field("insert_mode", &self.insert_mode).finish()
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self { pos: Position::default(), attr: TextAttribute::default(), insert_mode: Default::default(), pos_changed: Box::new(|_, _| {}), attr_changed: Box::new(|_| {}) }
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

#[derive(Clone, Debug)]
pub enum Shape {
    Rectangle,
    Elipse
}

#[derive(Clone, Debug)]
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
    pub outline_changed: std::boxed::Box<dyn Fn(&Editor)>,
    pub request_refresh: Box<dyn Fn ()>,
    atomic_undo_stack: Vec<usize>
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
            request_refresh: Box::new(|| {}),
            cur_layer: 0,
            atomic_undo_stack: Vec::new()
        }
    }

    pub fn get_cursor_position(&self) -> Position
    {
        self.cursor.pos
    }

    pub fn set_cursor_position(&mut self, pos: Position)
    {
        let pos = Position::from(
            min(self.buf.width as i32 - 1, max(0, pos.x)),
            min(self.buf.height as i32 - 1, max(0, pos.y))
        );
        self.cursor.pos = pos;
        (self.cursor.pos_changed)(self, pos);
    }

    pub fn get_cur_layer(&mut self) -> Option<&super::Layer>
    {
        self.buf.layers.get(self.cur_layer as usize)
    }

    pub fn get_cur_layer_mut(&mut self) -> Option<&mut super::Layer>
    {
        self.buf.layers.get_mut(self.cur_layer as usize)
    }
    
    pub fn get_overlay_layer(&mut self) -> &mut Option<super::Layer>
    {
        self.buf.get_overlay_layer()
    }
    
    pub fn join_overlay(&mut self)
    {
        self.begin_atomic_undo();
        let opt_layer = self.buf.remove_overlay();

        if let Some(layer) = &opt_layer {
            for y in 0..layer.lines.len() {
                let line = &layer.lines[y];
                for x in 0..line.chars.len() {
                    let ch =line.chars[x];
                    if ch.is_some()  {
                        self.set_char(Position::from(x as i32, y as i32), ch);
                    }
                }
            }
        }
        self.end_atomic_undo();
    }

    pub fn delete_line(&mut self, line: i32)
    {
        // TODO: Undo
        let layer = &mut self.buf.layers[self.cur_layer as usize];
        layer.remove_line(line);
    }

    pub fn insert_line(&mut self, line: i32) {
        // TODO: Undo
        let layer = &mut self.buf.layers[self.cur_layer as usize];
        layer.insert_line(line, super::Line::new());
    }

    pub fn pickup_color(&mut self, pos: Position)
    {
        let ch = self.buf.get_char(pos);
        if let Some(ch) = ch {
            self.cursor.attr = ch.attribute;
        }
    }

    pub fn set_cursor(&mut self, x: i32, y: i32) -> Event
    {
        let old = self.cursor.pos;
        self.set_cursor_position(Position::from(
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

    pub fn save_content(&self, file_name: &Path) -> io::Result<bool>
    {
        let mut f = File::create(file_name)?;

        let content = 
        if let Some(ext) = file_name.extension() {
            let ext = OsStr::to_str(ext).unwrap().to_lowercase();
                self.buf.to_bytes(ext.as_str())?
            } else {
                self.buf.to_bytes("mdf")?
            };
        f.write_all(&content)?;
        Ok(true)
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
    
    pub fn get_outline_char_code_from(outline:i32, i: i32) -> Result<u8, &'static str>
    {
        if outline < 0 || outline >= DEFAULT_OUTLINE_TABLE.len() as i32 {
            return Err("current outline out of range.");
        }
        if !(0..=10).contains(&i) {
            return Err("outline char# out of range.");
        }
        Ok(DEFAULT_OUTLINE_TABLE[outline as usize][i as usize])
    }
    
    pub fn get_char(&self, pos: Position) -> Option<DosChar> {
        self.buf.get_char(pos)
    }

    pub fn get_char_from_cur_layer(&self, pos: Position) -> Option<DosChar> {
        if self.cur_layer >= self.buf.layers.len() as i32 {
            return None;
        }
        self.buf.layers[self.cur_layer as usize].get_char(pos)
    }

    pub fn set_char(&mut self, pos: Position, dos_char: Option<DosChar>) {
        if self.point_is_valid(pos) {
            self.buf.redo_stack.clear();
            let old = self.buf.get_char_from_layer(self.cur_layer as usize, pos);
            self.buf.set_char(self.cur_layer as usize, pos, dos_char);
            self.buf.undo_stack.push(Box::new(UndoSetChar { pos, layer: self.cur_layer as usize, old, new: dos_char } ));
        }
    }
    pub fn begin_atomic_undo(&mut self) {
        self.atomic_undo_stack.push(self.buf.undo_stack.len());
    }

    pub fn end_atomic_undo(&mut self) {
        let base_count = self.atomic_undo_stack.pop().unwrap();
        let count = self.buf.undo_stack.len();
        if base_count == count { return; }

        let mut stack = Vec::new();
        while base_count < self.buf.undo_stack.len() {
            let op = self.buf.undo_stack.pop().unwrap();
            stack.push(op);
        }
        self.buf.undo_stack.push(Box::new(super::AtomicUndo { stack }));
    }

    pub fn undo(&mut self) {
        if let Some(op) = self.buf.undo_stack.pop() {
            op.undo(&mut self.buf);
            self.buf.redo_stack.push(op);
        }
    }

    pub fn redo(&mut self) {
        if let Some(op) = self.buf.redo_stack.pop() {
            op.redo(&mut self.buf);
            self.buf.undo_stack.push(op);
        }
    }

    pub fn fill(&mut self, rect: Rectangle, dos_char: Option<DosChar>) {
        let mut pos = rect.start;
        self.begin_atomic_undo();
        for _ in 0..rect.size.height {
            for _ in 0..rect.size.width {
                self.set_char(pos, dos_char);
                pos.x += 1;
            }
            pos.y += 1;
            pos.x = rect.start.x;
        }
        self.end_atomic_undo();
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
                let next = self.get_char_from_cur_layer( Position::from(i - 1, pos.y));
                self.set_char(Position::from(i, pos.y), next);
            }
        }

        self.set_char(pos, Some(crate::model::DosChar {
            char_code,
            attribute: self.cursor.attr,
        }));
        self.set_cursor(pos.x + 1, pos.y);
    }

    pub fn delete_selection(&mut self) {
        if let Some(selection) = &self.cur_selection.clone() {
            self.begin_atomic_undo();
            let mut pos = selection.rectangle.start;
            for _ in 0..selection.rectangle.size.height {
                for _ in 0..selection.rectangle.size.width {
                    if self.cur_layer == self.buf.layers.len() as i32 - 1 {
                        self.set_char(pos, Some(DosChar::new()));
                    } else {
                        self.set_char(pos, None);
                    }
                    pos.x += 1;
                }
                pos.y += 1;
                pos.x = selection.rectangle.start.x;
            }
            self.end_atomic_undo();
            self.cur_selection = None;
        }
    }

    pub fn clear_cur_layer(&mut self) {
        let b = Box::new(self.buf.clear_layer(self.cur_layer));
        self.buf.undo_stack.push(b);
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