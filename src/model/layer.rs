use super::{DosChar, Position, Size};

#[derive(Clone, Debug, Default)]
pub struct Line {
    pub chars: Vec<DosChar>,
}

impl Line {
    pub fn new() -> Self {
        Line { chars: Vec::new() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Layer {
    pub name: String,
    pub is_visible: bool,
    pub is_locked: bool,
    pub is_position_locked: bool,

    offset: Position,
    pub size: Size,
    lines: Vec<Line>,
}

impl Layer {
    pub fn new() -> Self {
        Layer {
            name: "Background".to_string(),
            is_visible: true,
            is_locked: false,
            is_position_locked: false,
            lines: Vec::new(),
            offset: Position::new(),
            size: Size::new(),
        }
    }

    pub fn get_offset(&self) -> Position {
        self.offset
    }

    pub fn set_offset(&mut self, pos: Position) {
        if self.is_position_locked {
            return;
        }
        self.offset = pos;
    }

    pub fn join(&mut self, layer: &Layer)
    {  
        for y in 0..layer.lines.len() {
            let line = &layer.lines[y];
            for x in 0..line.chars.len() {
                let ch = line.chars[x];
                if ch.is_transparent() { continue; }
                self.set_char(Position::from(x as i32, y as i32), ch);
            }
        }
    }

    pub fn join_overlay(&mut self, layer: &super::OverlayLayer)
    {  
        for y in 0..layer.lines.len() {
            let line = &layer.lines[y];
            for x in 0..line.chars.len() {
                if let Some(ch) = line.chars[x] {
                    self.set_char(Position::from(x as i32, y as i32), ch);
                }
            }
        }
    }

    pub fn clear(&mut self)
    {
        self.lines.clear();
    }

    pub fn set_char(&mut self, pos: Position, dos_char: DosChar) {
        let pos = pos - self.offset;
        if pos.x < 0 || pos.y < 0 || self.is_locked || !self.is_visible {
            return;
        }

        if pos.y >= self.lines.len() as i32 {
            self.lines.resize(pos.y as usize + 1, Line::new());
        }

        let cur_line = &mut self.lines[pos.y as usize];
        if pos.x >= cur_line.chars.len() as i32 {
            cur_line.chars.resize(pos.x as usize + 1, DosChar::new());
        }
        cur_line.chars[pos.x as usize] = dos_char;
    }

    pub fn get_char(&self, pos: Position) -> DosChar {
        let pos = pos - self.offset;
        let y = pos.y as usize;
        if self.lines.len() <= y { return DosChar::new(); }
        
        let cur_line = &self.lines[y];
        if pos.x >= 0 && pos.x < cur_line.chars.len() as i32 {
            let ch = cur_line.chars[pos.x as usize];
            if !ch.is_transparent() {
                return ch;
            }
        }
        DosChar::new()
    }

    pub fn remove_line(&mut self, index: i32)
    {
        if self.is_locked || !self.is_visible {
            return;
        }
        assert!(!(index < 0 || index >= self.lines.len() as i32), "line out of range");
        self.lines.remove(index as usize);
    }

    pub fn insert_line(&mut self, index: i32, line: Line)
    {
        if self.is_locked || !self.is_visible {
            return;
        }
        assert!(!(index < 0 || index > self.lines.len() as i32), "line out of range");
        self.lines.insert(index as usize, line);
    }
}
