use std::{
    cell::RefCell,
    rc::Rc,
};

use glib::subclass::types::ObjectSubclassIsExt;
use gtk4::{
    glib
};

use crate::{
    model::{Editor},
};

use self::gtkansi_view::GtkMinimapAnsiView;
mod gtkansi_view;

glib::wrapper! {
    pub struct MinimapAnsiView(ObjectSubclass<GtkMinimapAnsiView>) @extends gtk4::Widget, gtk4::DrawingArea;
}

impl Default for MinimapAnsiView {
    fn default() -> Self {
        Self::new()
    }
}

impl MinimapAnsiView {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a AnsiEditorArea")
    }

    pub fn get_editor(&self) -> Rc<RefCell<Editor>> {
        self.imp().editor.borrow().clone()
    }
   
    pub fn set_editor_handle(&self, handle: Rc<RefCell<Editor>>) {
        self.imp().set_editor_handle(handle);
    }
}
