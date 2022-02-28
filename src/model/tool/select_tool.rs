
use std::{rc::Rc, cell::RefCell};

use super::{ Tool, MKey, MModifiers, Editor, Event, Position};

pub struct SelectTool {}

impl Tool for SelectTool
{
    fn get_icon_name(&self) -> &'static str { "edit-select" }

    fn handle_key(&self, editor: Rc<RefCell<Editor>>, key: MKey, _modifier: MModifiers) -> Event
    {
        let mut editor = editor.borrow_mut();
        match key {
            MKey::Down => {
                if editor.cur_selection.is_active {
                    editor.cur_selection.rectangle.start.y += 1;
                }
            }
            MKey::Up => {
                if editor.cur_selection.is_active {
                    editor.cur_selection.rectangle.start.y -= 1;
                }
            }
            MKey::Left => {
                if editor.cur_selection.is_active {
                    editor.cur_selection.rectangle.start.x -= 1;
                }
            }
            MKey::Right => {
                if editor.cur_selection.is_active {
                    editor.cur_selection.rectangle.start.x += 1;
                }
            }
            MKey::Escape => {
                editor.cur_selection.is_active = false;
            }
            _ => {}
        }
        Event::None
    }

    fn handle_click(&self, editor: Rc<RefCell<Editor>>, button: u32, cur: Position) -> Event
    {
        let mut editor = editor.borrow_mut();
        if button == 3 {
            editor.cur_selection.is_active = false;
        }
        Event::None
    }

    fn handle_drag(&self, editor: Rc<RefCell<Editor>>, start: Position, mut cur: Position) -> Event
    {
        let mut editor = editor.borrow_mut();
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
