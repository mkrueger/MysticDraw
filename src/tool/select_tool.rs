use gtk4::{traits::BoxExt, gdk::{Key, ModifierType}};
use crate::{editor::{Editor, EditorEvent}, model::Position};
use super::Tool;

pub struct SelectTool {}

impl Tool for SelectTool
{
    fn get_icon_name(&self) -> &'static str { "edit-select" }
    fn add_tool_page(&self, parent: &mut gtk4::Box)
    {
        parent.append(&gtk4::Label::builder().label("Select").build());
    }

    fn handle_key(&self, editor: &mut Editor, key: Key, _key_code: u32, _modifier: ModifierType) -> EditorEvent
    {
        match key {
            Key::Down => {
                if editor.cur_selection.is_active {
                    editor.cur_selection.rectangle.start.y += 1;
                }
            }
            Key::Up => {
                if editor.cur_selection.is_active {
                    editor.cur_selection.rectangle.start.y -= 1;
                }
            }
            Key::Left => {
                if editor.cur_selection.is_active {
                    editor.cur_selection.rectangle.start.x -= 1;
                }
            }
            Key::Right => {
                if editor.cur_selection.is_active {
                    editor.cur_selection.rectangle.start.x += 1;
                }
            }
            Key::Escape => {
                editor.cur_selection.is_active = false;
            }
            _ => {}
        }
        EditorEvent::None
    }

    fn handle_click(&self, editor: &mut Editor, _button: u32, x: i32, y: i32) -> EditorEvent
    {
        editor.cursor.pos = Position::from(x, y);
        EditorEvent::None
    }

    fn handle_drag(&self, editor: &mut Editor, start: Position, cur: Position) -> EditorEvent
    {
        editor.cur_selection.rectangle = crate::model::Rectangle::from_pt(start, cur);
        editor.cur_selection.is_preview = true;
        editor.cur_selection.is_active = true;

        EditorEvent::None
    }

    fn handle_drag_end(&self, editor: &mut Editor, _start: Position, _cur: Position) -> EditorEvent {
        editor.cur_selection.is_preview = false;
        editor.cur_selection.is_active = true;

        EditorEvent::None
    }
}
