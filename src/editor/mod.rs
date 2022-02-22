use crate::model::{Buffer, Position};

pub struct Editor {
    pub buf: Buffer,
    pub cursor: Position
}

impl Editor {
    pub fn new(buf: Buffer) -> Self {
        Editor { 
            buf, 
            cursor: Position::new() 
        }
    }
}