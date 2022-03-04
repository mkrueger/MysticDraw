use crate::model::{DosChar, Layer, TextAttribute};

use super::{Editor, Event, Position, Tool};
use std::{
    cell::RefCell,
    cmp::{max, min},
    rc::Rc,
};

pub struct DrawRectangleTool {}

impl Tool for DrawRectangleTool {
    fn get_icon_name(&self) -> &'static str {
        "md-tool-rectangle"
    }

    fn handle_drag(&self, editor: Rc<RefCell<Editor>>, start: Position, cur: Position) -> Event {
        let mut editor = editor.borrow_mut();
        let attr = editor.cursor.attr;
        if let Some(layer) = editor.get_overlay_layer() {
            layer.clear();

            if start < cur {
                plot_rectangle(layer, attr, start, cur);
            } else {
                plot_rectangle(layer, attr, cur, start);
            }
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
        println!("{:?}, {:?}", start, cur);
        if start == cur {
            editor.buf.remove_overlay();
        } else {
            editor.join_overlay();
        }
        Event::None
    }
}

pub fn plot_rectangle(
    layer: &mut Layer,
    attribute: TextAttribute,
    pos0: Position,
    pos1: Position,
) {
    let x1 = min(pos0.x, pos1.x);
    let x2 = max(pos0.x, pos1.x);
    let y1 = min(pos0.y, pos1.y);
    let y2 = max(pos0.y, pos1.y);

    for x in x1..=x2 {
        layer.set_char(
            Position::from(x, y1),
            DosChar {
                char_code: 219,
                attribute,
            },
        );
        layer.set_char(
            Position::from(x, y2),
            DosChar {
                char_code: 219,
                attribute,
            },
        );
    }

    for y in (y1 + 1)..y2 {
        layer.set_char(
            Position::from(x1, y),
            DosChar {
                char_code: 219,
                attribute,
            },
        );
        layer.set_char(
            Position::from(x2, y),
            DosChar {
                char_code: 219,
                attribute,
            },
        );
    }
}
