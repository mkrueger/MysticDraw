use std::cell::{Cell, RefCell};

use glib::subclass::{types::ObjectSubclass, object::ObjectImpl};
use gtk4::{
    glib::{self, ParamSpec, Value},
    prelude::*,
};

#[derive(Default)]
pub struct RowData {
    name: RefCell<Option<String>>,
    count: Cell<u32>,
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
                glib::ParamSpecUInt::new(
                    "count",
                    "Count",
                    "Count",
                    0,
                    u32::MAX,
                    0, // Allowed range and default value
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
            "count" => {
                let count = value.get().unwrap();
                self.count.replace(count);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "name" => {
                self.name.borrow().to_value()
            },
            "count" => self.count.get().to_value(),
            _ => unimplemented!(),
        }
    }
}
