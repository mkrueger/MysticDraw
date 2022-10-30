use super::{Position, TextAttribute};


pub struct Caret {
    pub(super) pos: Position,
    pub(super) attr: TextAttribute,
    pub insert_mode: bool
}

impl Caret {
    pub fn get_attribute(&self) -> TextAttribute
    {
        self.attr
    }

    pub fn get_position(&self) -> Position
    {
        self.pos
    }
}

impl std::fmt::Debug for Caret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cursor").field("pos", &self.pos).field("attr", &self.attr).field("insert_mode", &self.insert_mode).finish()
    }
}

impl Default for Caret {
    fn default() -> Self {
        Self {
            pos: Position::default(),
            attr: TextAttribute::DEFAULT,
            insert_mode: Default::default()
        }
    }
}

impl PartialEq for Caret {
    fn eq(&self, other: &Caret) -> bool {
        self.pos == other.pos && self.attr == other.attr
    }
}
