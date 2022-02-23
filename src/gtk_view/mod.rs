
use glib::subclass::types::ObjectSubclassIsExt;
use gtk4::{ glib, traits::{WidgetExt}};

use self::gtkchar_editor_view::GtkCharEditorView;

mod gtkchar_editor_view;


glib::wrapper! {
    pub struct CharEditorView(ObjectSubclass<GtkCharEditorView>) @extends gtk4::Widget, gtk4::DrawingArea;
}

impl Default for CharEditorView {
    fn default() -> Self {
         Self::new()
    }
}

impl CharEditorView {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a AnsiEditorArea")
    }

    pub fn set_buffer(&self, buffer_id: usize)
    {
        let imp = self.imp();
        imp.buf.set(Some(buffer_id));
        
        let buffer = &crate::Workspace::get_editor(buffer_id).buf;
        let font_dimensions = buffer.get_font_dimensions();

        self.set_size_request(buffer.width as i32 * font_dimensions.x, buffer.height as i32 * font_dimensions.y);
    }
}