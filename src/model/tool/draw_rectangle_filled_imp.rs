use crate::model::{TextAttribute, Rectangle};

use super::{Editor, Event, Position, Tool, DrawMode, Plottable, plot_point, ScanLines};
use std::{
    cell::{RefCell},
    rc::Rc,
};

pub struct DrawRectangleFilledTool {
    pub draw_mode: DrawMode,

    pub use_fore: bool,
    pub use_back: bool,
    pub attr: TextAttribute,
    pub char_code: u8
}

impl Plottable for DrawRectangleFilledTool {
    fn get_draw_mode(&self) -> DrawMode { self.draw_mode }

    fn get_use_fore(&self) -> bool { self.use_fore }
    fn get_use_back(&self) -> bool { self.use_back }
    fn get_char_code(&self) -> u8 { self.char_code }
}

impl Tool for DrawRectangleFilledTool {
    fn get_icon_name(&self) -> &'static str {
        "md-tool-rectangle-filled"
    }

    fn use_caret(&self) -> bool { false }
    fn use_selection(&self) -> bool { false }

    fn handle_drag(&self, editor: Rc<RefCell<Editor>>,  mut start: Position, mut cur: Position) -> Event {
        if let Some(layer) = editor.borrow_mut().get_overlay_layer() {
            layer.clear();
        }

        let mut lines = ScanLines::new(1);
        lines.add_rectangle(Rectangle::from_pt(start, cur));

        let draw = move |rect: Rectangle| {
            for y in 0..rect.size.height {
                for x in 0..rect.size.width {
                    plot_point(&editor, self, Position::from(rect.start.x + x, rect.start.y + y));
                }
            }
        };
        lines.fill(draw);

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
