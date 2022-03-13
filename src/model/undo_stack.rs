use super::{Buffer, DosChar, Position, Layer, Size};

pub trait UndoOperation {
    fn undo(&self, buffer: &mut Buffer);
    fn redo(&self, buffer: &mut Buffer);
}

pub struct UndoSetChar {
    pub pos: Position,
    pub layer: usize,
    pub old: Option<DosChar>,
    pub new: Option<DosChar>
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


pub struct UndoSwapChar {
    pub layer: usize,
    pub pos1: Position,
    pub pos2: Position
}
impl UndoOperation for UndoSwapChar {

    fn undo(&self, buffer: &mut Buffer)
    {
        buffer.layers[self.layer as usize].swap_char(self.pos1, self.pos2);
    }

    fn redo(&self, buffer: &mut Buffer)
    {
        buffer.layers[self.layer as usize].swap_char(self.pos1, self.pos2);
    }
}

pub struct UndoReplaceLayers {
    pub old_layer: Vec<Layer>, 
    pub new_layer: Vec<Layer>,
    pub old_size: Size<u16>, 
    pub new_size: Size<u16>
}

impl UndoOperation for UndoReplaceLayers {
    fn undo(&self, buffer: &mut Buffer)
    {
        buffer.layers = self.old_layer.clone();
        buffer.width = self.old_size.width;
        buffer.height = self.old_size.height;
    }

    fn redo(&self, buffer: &mut Buffer)
    {
        buffer.layers = self.new_layer.clone();
        buffer.width = self.new_size.width;
        buffer.height = self.new_size.height;
    }
}


pub struct ClearLayerOperation {
    pub layer_num: i32,
    pub lines: Vec<super::Line>,
}

impl UndoOperation for ClearLayerOperation {

    fn undo(&self, buffer: &mut Buffer)
    {
        buffer.layers[self.layer_num as usize].lines.clear();
        buffer.layers[self.layer_num as usize].lines.extend(self.lines.clone());
    }

    fn redo(&self, buffer: &mut Buffer)
    {
        buffer.layers[self.layer_num as usize].lines.clear();
    }
}
