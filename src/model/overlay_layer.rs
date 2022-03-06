use super::{DosChar, Position};

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
