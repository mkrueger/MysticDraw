mod status_bar;

glib::wrapper! {
    pub struct AnsiStatusBar(ObjectSubclass<status_bar::GtkAnsiStatusBar>) @extends gtk4::Widget, gtk4::Box;
}

impl Default for AnsiStatusBar {
    fn default() -> Self {
         Self::new()
    }
}

impl AnsiStatusBar {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a AnsiEditorArea")
    }
}