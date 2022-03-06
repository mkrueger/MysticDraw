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

    pub size: Size,
    pub lines: Vec<Line>,
}

impl Layer {
    pub fn new() -> Self {
        Layer {
            name: "Background".to_string(),
            is_visible: true,
            lines: Vec::new(),
            size: Size::new(),
        }
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

    pub fn join_overlay(&mut self, layer: &OverlayLayer)
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
        if pos.x < 0 || pos.y < 0 {
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
}




#[derive(Clone, Debug, Default)]
pub struct OverlayLine {
    pub chars: Vec<Option<DosChar>>,
}

impl OverlayLine {
    pub fn new() -> Self {
        OverlayLine { chars: Vec::new() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct OverlayLayer {

    pub lines: Vec<OverlayLine>,
}

impl OverlayLayer {
    pub fn new() -> Self {
        OverlayLayer {
            lines: Vec::new(),
        }
    }

    pub fn clear(&mut self)
    {
        self.lines.clear();
    }

    pub fn set_char(&mut self, pos: Position, dos_char: DosChar) {
        if pos.x < 0 || pos.y < 0 {
            return;
        }

        if pos.y >= self.lines.len() as i32 {
            self.lines.resize(pos.y as usize + 1, OverlayLine::new());
        }

        let cur_line = &mut self.lines[pos.y as usize];
        if pos.x >= cur_line.chars.len() as i32 {
            cur_line.chars.resize(pos.x as usize + 1, None);
        }
        cur_line.chars[pos.x as usize] = Some(dos_char);
    }

    pub fn get_char(&self, pos: Position) -> Option<DosChar> {
        let y = pos.y as usize;
        if self.lines.len() <= y { return None; }

        let cur_line = &self.lines[y];
        if pos.x >= 0 && pos.x < cur_line.chars.len() as i32 {
            cur_line.chars[pos.x as usize]
        } else {
            None
        }
    }
}
