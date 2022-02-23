
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new() -> Self {
        Position { x: 0, y: 0 }
    }
    pub fn from(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}
