
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new() -> Self {
        Position { x: 0, y: 0 }
    }
    pub fn from(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}
