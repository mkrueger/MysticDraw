use gtk4::subclass::prelude::*;

use super::row_data::RowData;
use gtk4::{gio, glib, prelude::*};

glib::wrapper! {
    pub struct Model(ObjectSubclass<super::model_imp::Model>) @implements gio::ListModel;
}

impl Model {
    pub fn new() -> Model {
        glib::Object::new(&[]).expect("Failed to create Model")
    }

    pub fn append(&self, obj: &RowData) {
        let imp = self.imp();
        let index = {
            let mut data = imp.0.borrow_mut();
            data.push(obj.clone());
            data.len() - 1
        };
        self.items_changed(index as u32, 0, 1);
    }

    pub fn remove(&self, index: u32) {
        let imp = self.imp();
        imp.0.borrow_mut().remove(index as usize);

        self.items_changed(index, 1, 0);
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
