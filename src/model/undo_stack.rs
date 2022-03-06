use super::{Buffer, DosChar, Position};

pub trait UndoOperation {
    fn undo(&self, buffer: &mut Buffer);
    fn redo(&self, buffer: &mut Buffer);
}

pub struct UndoSetChar {
    pub pos: Position,
    pub layer: usize,
    pub old: DosChar,
    pub new: DosChar
}

impl UndoOperation for UndoSetChar {
    fn undo(&self, buffer: &mut Buffer)
    {
        buffer.set_char(self.layer, self.pos, self.old);
    }

    fn redo(&self, buffer: &mut Buffer)
    {
        buffer.set_char(self.layer, self.pos, self.new);
    }
}

pub struct AtomicUndo {
    pub stack: Vec<Box<dyn UndoOperation>>,
}

impl UndoOperation for AtomicUndo {

    fn undo(&self, buffer: &mut Buffer)
    {
        for op in &self.stack {
            op.undo(buffer);
        }
    }

    fn redo(&self, buffer: &mut Buffer)
    {
        for op in self.stack.iter().rev() {
            op.redo(buffer);
        }
    }
}
