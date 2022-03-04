use crate::model::{DosChar, Layer, TextAttribute};

use super::{Editor, Event, Position, Tool};
use std::{
    cell::RefCell,
    cmp::{max, min},
    rc::Rc,
};

pub struct DrawEllipseTool {}

impl Tool for DrawEllipseTool {
    fn get_icon_name(&self) -> &'static str {
        "md-tool-circle"
    }

    fn use_caret(&self) -> bool { false }
    fn use_selection(&self) -> bool { false }
    
    fn handle_drag(&self, editor: Rc<RefCell<Editor>>, start: Position, cur: Position) -> Event {
        let mut editor = editor.borrow_mut();
        let attr = editor.cursor.attr;
        if let Some(layer) = editor.get_overlay_layer() {
            layer.clear();

            if start < cur {
                plot_ellipse(layer, attr, start, cur);
            } else {
                plot_ellipse(layer, attr, cur, start);
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
        if start == cur {
            editor.buf.remove_overlay();
        } else {
            editor.join_overlay();
        }
        Event::None
    }
}

pub fn plot_ellipse(
    layer: &mut Layer,
    attribute: TextAttribute,
    pos0: Position,
    pos1: Position,
) {
    let x1 = min(pos0.x, pos1.x) as f64;
    let x2 = max(pos0.x, pos1.x) as f64;
    let y1 = min(pos0.y, pos1.y) as f64;
    let y2 = max(pos0.y, pos1.y) as f64;

    let rx = (x2 - x1) / 2.0;
    let ry = (y2 - y1) / 2.0;
    let xc = x1 + (x2 - x1) / 2.0;
    let yc = y1 + (y2 - y1) / 2.0;

    let mut x = 0.0;
    let mut y = ry;

    let mut d1 = (ry * ry) - (rx * rx * ry) + (0.25 * rx * rx);
    let mut dx = 2.0 * ry * ry * x;
    let mut dy = 2.0 * rx * rx * y;

    while dx < dy {
        layer.set_char(
            Position::from((x + xc) as i32, (y + yc) as i32),
            DosChar {
                char_code: 219,
                attribute,
            },
        );
        layer.set_char(
            Position::from((-x + xc) as i32, (y + yc) as i32),
            DosChar {
                char_code: 219,
                attribute,
            },
        );
        layer.set_char(
            Position::from((x + xc) as i32, (-y + yc) as i32),
            DosChar {
                char_code: 219,
                attribute,
            },
        );
        layer.set_char(
            Position::from((-x + xc) as i32, (-y + yc) as i32),
            DosChar {
                char_code: 219,
                attribute,
            },
        );

        if d1 < 0.0 {
            x += 1.0;
            dx += 2.0 * ry * ry;
            d1 += dx + (ry * ry);
        } else {
            x += 1.0;
            y -= 1.0;
            dx += 2.0 * ry * ry;
            dy -= 2.0 * rx * rx;
            d1 += dx - dy + (ry * ry);
        }
    }

    let mut d2 = ((ry * ry) * ((x + 0.5) * (x + 0.5))) + ((rx * rx) * ((y - 1.0) * (y - 1.0)))
        - (rx * rx * ry * ry);

    while y >= 0.0 {
        layer.set_char(
            Position::from((x + xc) as i32, (y + yc) as i32),
            DosChar {
                char_code: 219,
                attribute,
            },
        );
        layer.set_char(
            Position::from((-x + xc) as i32, (y + yc) as i32),
            DosChar {
                char_code: 219,
                attribute,
            },
        );
        layer.set_char(
            Position::from((x + xc) as i32, (-y + yc) as i32),
            DosChar {
                char_code: 219,
                attribute,
            },
        );
        layer.set_char(
            Position::from((-x + xc) as i32, (-y + yc) as i32),
            DosChar {
                char_code: 219,
                attribute,
            },
        );

        if d2 > 0.0 {
            y -= 1.0;
            dy -= 2.0 * rx * rx;
            d2 += (rx * rx) - dy;
        } else {
            y -= 1.0;
            x += 2.0;
            dx += 2.0 * ry * ry;
            dy -= 2.0 * rx * rx;
            d2 += dx - dy + (rx * rx);
        }
    }
}
