use gtk4::{traits::BoxExt, gdk::{Key, ModifierType}};
use crate::editor::Editor;
use super::Tool;

pub struct BrushTool {}

impl Tool for BrushTool
{
    fn get_icon_name(&self) -> &'static str { "edit-select" }
    fn add_tool_page(&self, parent: &mut gtk4::Box)
    {
        parent.append(&gtk4::Label::builder().label("BrushTool").build());
    }
    
    fn handle_key(&self, _editor: &mut Editor, _key: Key, _key_code: u32, _modifier: ModifierType) -> bool
    {
        false
    }
}