
use std::{rc::Rc, cell::RefCell};

use super::{Editor, Event, Position, Tool};


pub struct ClickTool {}

impl Tool for ClickTool
{
    fn get_icon_name(&self) -> &'static str { "md-tool-click" }
/* 
    fn add_tool_page(&self, window: &ApplicationWindow,parent: &mut gtk4::Box)
    {
        parent.append(&gtk4::Label::builder().label("Click").build());
    }
*/

fn handle_click(&mut self, editor: Rc<RefCell<Editor>>, button: u32, pos: Position) -> Event {
    if button == 1 {
        editor.borrow_mut().cursor.set_position(pos);
    }
    Event::None
}

    fn handle_drag(&self, editor: Rc<RefCell<Editor>>, start: Position, cur: Position) -> Event
    {
        let mut editor = editor.borrow_mut();
        let mut cur = cur;
        if start < cur {
            cur = cur + Position::from(1, 1);
        }
        editor.cur_selection.rectangle = crate::model::Rectangle::from_pt(start, cur);
        editor.cur_selection.is_preview = true;
        editor.cur_selection.is_active = true;
        Event::None
    }

    fn handle_drag_end(&self, editor: Rc<RefCell<Editor>>, _start: Position, _cur: Position) -> Event {
        let mut editor = editor.borrow_mut();
        editor.cur_selection.is_preview = false;
        editor.cur_selection.is_active = true;
        Event::None
    }
}
