use gtk4::{traits::BoxExt};
use crate::{editor::{Editor, EditorEvent}, model::Position};
use super::Tool;

pub struct BrushTool {}

impl Tool for BrushTool
{
    fn get_icon_name(&self) -> &'static str { "edit-select" }
    fn add_tool_page(&self, parent: &mut gtk4::Box)
    {
        parent.append(&gtk4::Label::builder().label("BrushTool").build());
    }
    
    fn handle_click(&self, editor: &mut Editor, _button: u32, x: i32, y: i32) -> EditorEvent
    {
        editor.cursor.pos = Position::from(x, y);
        EditorEvent::None
    }

    fn handle_drag(&self, _editor: &mut Editor, _start: Position, _cur: Position) -> EditorEvent
    {
        EditorEvent::None
    }
}