use crate::model::{DosChar, TextAttribute};

use super::{Editor, Event, Position, Tool, SHADE_GRADIENT, DrawMode, Plottable, plot_point};
use std::{
    cell::{RefCell},
    cmp::{max, min},
    rc::Rc,
};

pub struct DrawRectangleTool {
    pub draw_mode: DrawMode,

    pub use_fore: bool,
    pub use_back: bool,
    pub fill_mode: bool,
    pub attr: TextAttribute,
    pub char_code: u8
}

impl Plottable for DrawRectangleTool {
    fn get_draw_mode(&self) -> DrawMode { self.draw_mode }

    fn get_use_fore(&self) -> bool { self.use_fore }
    fn get_use_back(&self) -> bool { self.use_back }
    fn get_char_code(&self) -> u8 { self.char_code }
}

impl Tool for DrawRectangleTool {
    fn get_icon_name(&self) -> &'static str {
        "md-tool-rectangle"
    }

    fn use_caret(&self) -> bool { false }
    fn use_selection(&self) -> bool { false }

    fn handle_drag(&self, editor: Rc<RefCell<Editor>>, start: Position, cur: Position) -> Event {
        if let Some(layer) = editor.borrow_mut().get_overlay_layer() {
            layer.clear();
        }
        
        if self.fill_mode {
            fill_rectangle(&editor, self, start, cur);
        } else {
            plot_rectangle(&editor, self, start, cur);
        }

        Event::None
    }

    fn handle_drag_end(
        &self,
        editor: Rc<RefCell<Editor>>,
        start: Position,
        cur: Position,
    ) -> Event {
        let mut editor = editor.borrow_mut();
        if start == cur {
            editor.buf.remove_overlay();
        } else {
            editor.join_overlay();
        }
        Event::None
    }
}



pub fn plot_rectangle(
    editor: &Rc<RefCell<Editor>>,
    tool: &DrawRectangleTool,
    pos0: Position,
    pos1: Position,
) {
    let x1 = min(pos0.x, pos1.x);
    let x2 = max(pos0.x, pos1.x);
    let y1 = min(pos0.y, pos1.y);
    let y2 = max(pos0.y, pos1.y);

    for x in x1..=x2 {
        plot_point(editor, tool, Position::from(x, y1));
        plot_point(editor, tool, Position::from(x, y2));
    }

    for y in (y1 + 1)..y2 {
        plot_point(editor, tool, Position::from(x1, y));
        plot_point(editor, tool, Position::from(x2, y));
    }
}

pub fn fill_rectangle(
    editor: &Rc<RefCell<Editor>>,
    tool: &DrawRectangleTool,
    pos0: Position,
    pos1: Position,
) {
    let x1 = min(pos0.x, pos1.x);
    let x2 = max(pos0.x, pos1.x);
    let y1 = min(pos0.y, pos1.y);
    let y2 = max(pos0.y, pos1.y);

    for y in (y1 + 1)..y2 {
        for x in x1..=x2 {
            plot_point(editor, tool, Position::from(x, y));
        }
    }

}
