use super::DosChar;

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
    pub width: usize,
    pub height: usize,
    pub lines: Vec<Line>,
}

impl Layer {
    pub fn new() -> Self {
        Layer {
            width: 0,
            height: 0,
            lines: Vec::new(),
        }
    }
}