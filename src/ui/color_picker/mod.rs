use std::{rc::Rc, cell::RefCell};

use glib::subclass::types::ObjectSubclassIsExt;
use gtk4::traits::WidgetExt;

use crate::model::Editor;

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

    pub fn get_editor(&self) -> Rc<RefCell<Editor>> {
        self.imp().editor.borrow().clone()
    }
    
    pub fn set_editor(&self, handle: &Rc<RefCell<Editor>>) {
        self.imp().set_editor(handle);
        self.queue_draw();
    }
}
