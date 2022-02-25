use gtk4::{traits::BoxExt};
use super::{ Tool};


pub struct EraseTool {}

impl Tool for EraseTool
{
    fn get_icon_name(&self) -> &'static str { "edit-select" }
    fn add_tool_page(&self, parent: &mut gtk4::Box)
    {
        parent.append(&gtk4::Label::builder().label("EraseTool").build());
    }
}
