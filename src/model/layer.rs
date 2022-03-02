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
    pub name: String,
    pub is_visible: bool,

    pub lines: Vec<Line>,
}

impl Layer {
    pub fn new() -> Self {
        Layer {
            name: "Background".to_string(),
            is_visible: true,
            lines: Vec::new(),
        }
    }
}