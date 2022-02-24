use gtk4::gdk::{Key, ModifierType};
use crate::{WORKSPACE, editor::Editor};


pub trait Tool
{
    fn get_icon_name(&self) -> &'static str;
    fn add_tool_page(&self, parent: &mut gtk4::Box);
    fn handle_key(&self, editor: &mut Editor, key: Key, key_code: u32, modifier: ModifierType) -> bool;
}

mod brush_tool;
mod click_tool;
mod draw_shape_tool;
mod erase_tool;
mod fill_tool;
mod font_tool;
mod paint_tool;
mod select_tool;

pub fn init_tools()
{
    unsafe {
        WORKSPACE.tools.push(&click_tool::ClickTool {});
        WORKSPACE.tools.push(&select_tool::SelectTool {});
        WORKSPACE.tools.push(&paint_tool::PaintTool{});
        WORKSPACE.tools.push(&brush_tool::BrushTool{});
        WORKSPACE.tools.push(&erase_tool::EraseTool{});
        WORKSPACE.tools.push(&draw_shape_tool::DrawShapeTool{});
        WORKSPACE.tools.push(&fill_tool::FillTool{});
        WORKSPACE.tools.push(&font_tool::FontTool{});
    }
}