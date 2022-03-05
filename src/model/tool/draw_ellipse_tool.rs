use crate::model::{TextAttribute};

use super::{Editor, Event, Position, Tool, DrawMode, Plottable, plot_point};
use std::{
    cell::RefCell,
    cmp::{max, min},
    rc::Rc,
};

pub struct DrawEllipseTool {
    pub draw_mode: DrawMode,

    pub use_fore: bool,
    pub use_back: bool,
    pub fill_mode: bool,
    pub attr: TextAttribute,
    pub char_code: u8
}

impl Plottable for DrawEllipseTool {
    fn get_draw_mode(&self) -> DrawMode { self.draw_mode }

    fn get_use_fore(&self) -> bool { self.use_fore }
    fn get_use_back(&self) -> bool { self.use_back }
    fn get_char_code(&self) -> u8 { self.char_code }
}

impl Tool for DrawEllipseTool {
    fn get_icon_name(&self) -> &'static str {
        "md-tool-circle"
    }

    fn use_caret(&self) -> bool { false }
    fn use_selection(&self) -> bool { false }
    
    fn handle_drag(&self, editor: Rc<RefCell<Editor>>, start: Position, cur: Position) -> Event {
        if let Some(layer) = editor.borrow_mut().get_overlay_layer() {
            layer.clear();
        }

        if start < cur {
            plot_ellipse(&editor, self, start, cur);
        } else {
            plot_ellipse(&editor, self, cur, start);
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
    editor: &Rc<RefCell<Editor>>,
    tool: &DrawEllipseTool,
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
        plot_point(editor, tool, Position::from((x + xc) as i32, (y + yc) as i32));
        plot_point(editor, tool, Position::from((-x + xc) as i32, (y + yc) as i32));
        plot_point(editor, tool, Position::from((x + xc) as i32, (-y + yc) as i32));
        plot_point(editor, tool, Position::from((-x + xc) as i32, (-y + yc) as i32));

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
        plot_point(editor, tool, Position::from((x + xc) as i32, (y + yc) as i32));
        plot_point(editor, tool, Position::from((-x + xc) as i32, (y + yc) as i32));
        plot_point(editor, tool, Position::from((x + xc) as i32, (-y + yc) as i32));
        plot_point(editor, tool, Position::from((-x + xc) as i32, (-y + yc) as i32));

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
