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

impl ColorPicker {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a AnsiEditorArea")
    }
}
