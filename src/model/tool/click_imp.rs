
use std::{rc::Rc, cell::RefCell};

use crate::model::Selection;

use super::{Editor, Event, Position, Tool};

pub struct ClickTool {}

impl Tool for ClickTool
{
    fn get_icon_name(&self) -> &'static str { "md-tool-click" }

    fn handle_click(&mut self, editor: Rc<RefCell<Editor>>, button: u32, pos: Position) -> Event {
        if button == 1 {
            editor.borrow_mut().cursor.set_position(pos);
        }
        let mut editor = editor.borrow_mut();
        editor.cur_selection = None;
        Event::None
    }

    fn handle_drag(&self, editor: Rc<RefCell<Editor>>, start: Position, cur: Position) -> Event
    {
        let mut editor = editor.borrow_mut();
        let mut cur = cur;
        if start < cur {
            cur = cur + Position::from(1, 1);
        }
        if start == cur { 
            editor.cur_selection = None;
        } else {
            editor.cur_selection = Some(Selection { 
                rectangle: crate::model::Rectangle::from_pt(start, cur),
                is_preview: true,
                shape: crate::model::Shape::Rectangle
            });
        }
        editor.cursor.set_position(cur);
        Event::None
    }

    fn handle_drag_end(&self, editor: Rc<RefCell<Editor>>, start: Position, cur: Position) -> Event {
        let mut editor = editor.borrow_mut();
        let mut cur = cur;
        if start < cur {
            cur = cur + Position::from(1, 1);
        }

        if start == cur { 
            editor.cur_selection = None;
        } else {
            editor.cur_selection = Some(Selection { 
                rectangle: crate::model::Rectangle::from_pt(start, cur),
                is_preview: false,
                shape: crate::model::Shape::Rectangle
            });
        }
        editor.cursor.set_position(cur);

        Event::None
    }
}
