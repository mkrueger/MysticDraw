use std::{ rc::Rc, cell::RefCell };

use gtk4::{ glib, traits::{WidgetExt}};

use crate::{model::{Editor}};

use self::gtkcolor_picker::GtkColorPicker;

mod gtkcolor_picker;

glib::wrapper! {
    pub struct ColorPicker(ObjectSubclass<GtkColorPicker>) @extends gtk4::Widget, gtk4::DrawingArea;
}

impl Default for ColorPicker {
    fn default() -> Self {
         Self::new()
    }
}
struct Dialog {
    payload: Editor,
}

impl ColorPicker {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a AnsiEditorArea")
    }

    pub fn set_editor(&self, editor: Editor)
    {
        let buffer = &editor.buf;
        let font_dimensions = buffer.get_font_dimensions();
        self.set_size_request(buffer.width as i32 * font_dimensions.x, buffer.height as i32 * font_dimensions.y);

        let dialog = Dialog { payload: editor };
        let handle = Rc::new(RefCell::new(dialog));

        let handle1 = handle;
    }
}