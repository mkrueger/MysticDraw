use std::cell::{Cell, RefCell};

use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
use gtk4::{
    glib::{self, ParamSpec, Value},
    prelude::*,
};

#[derive(Default)]
pub struct RowData {
    name: RefCell<Option<String>>,
    is_visible: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for RowData {
    const NAME: &'static str = "RowData";
    type Type = super::row_data::RowData;
}

impl ObjectImpl for RowData {
    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecString::new(
                    "name",
                    "Name",
                    "Name",
                    None, // Default value
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "isvisible",
                    "IsVisible",
                    "IsVisible",
                    true, // Default value
                    glib::ParamFlags::READWRITE,
                ),
            ]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let name = value.get().unwrap();
                self.name.replace(name);
            }
            "isvisible" => {
                let isvisible = value.get().unwrap();
                self.is_visible.replace(isvisible);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "name" => self.name.borrow().to_value(),
            "isvisible" => self.is_visible.get().to_value(),
            _ => unimplemented!(),
        }
    }
}
