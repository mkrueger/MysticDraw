use std::cmp::Ordering;


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

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.y < other.y { return Some(Ordering::Less); }
        if self.y > other.y { return Some(Ordering::Greater); }
        if self.x < other.x { return Some(Ordering::Less); }
        if self.x > other.x { return Some(Ordering::Greater); }
        Some(Ordering::Equal)
    }
}