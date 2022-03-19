use std::{rc::Rc, cell::RefCell};

use glib::subclass::types::ObjectSubclassIsExt;
use gtk4::traits::WidgetExt;

use crate::model::Editor;

use self::gtkattribute_switcher::GtkAttributeSwitcher;

mod gtkattribute_switcher;

glib::wrapper! {
    pub struct AttributeSwitcher(ObjectSubclass<GtkAttributeSwitcher>) @extends gtk4::Widget, gtk4::DrawingArea;
}

impl Default for AttributeSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

impl AttributeSwitcher {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a AnsiEditorArea")
    }

    pub fn get_editor(&self) -> Option<Rc<RefCell<Editor>>> {
        self.imp().editor.borrow().clone()
    }
    
    pub fn set_editor(&self, handle: &Rc<RefCell<Editor>>) {
        self.imp().set_editor(self, handle);
        self.queue_draw();
    }
}
