use gtk4::{traits::BoxExt};
use libadwaita::ApplicationWindow;

use super::{Editor, Event, Position, Tool};


pub struct DrawShapeTool {}

impl Tool for DrawShapeTool
{
    fn get_icon_name(&self) -> &'static str { "edit-select" }
    fn add_tool_page(&self, window: &ApplicationWindow,parent: &mut gtk4::Box)
    {
        parent.append(&gtk4::Label::builder().label("DrawShapeTool").build());
    }

    fn handle_click(&self, editor: &mut Editor, _button: u32, x: i32, y: i32) -> Event
    {
        editor.cursor.pos = Position::from(x, y);
        Event::None
    }

}