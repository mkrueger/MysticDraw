use gio::subclass::prelude::*;
use gtk4::{
    gio::{self, ListModel},
    glib,
    prelude::*,
};

use std::cell::RefCell;

use super::row_data::FontRowData;

#[derive(Debug, Default)]
pub struct FontModel(pub(super) RefCell<Vec<FontRowData>>);

#[glib::object_subclass]
impl ObjectSubclass for FontModel {
    const NAME: &'static str = "FontModel";
    type Type = super::model::FontModel;
    type Interfaces = (ListModel,);
}

impl ObjectImpl for FontModel {}

impl ListModelImpl for FontModel {
    fn item_type(&self, _list_model: &Self::Type) -> glib::Type {
        FontRowData::static_type()
    }
    fn n_items(&self, _list_model: &Self::Type) -> u32 {
        self.0.borrow().len() as u32
    }
    fn item(&self, _list_model: &Self::Type, position: u32) -> Option<glib::Object> {
        self.0
            .borrow()
            .get(position as usize)
            .map(|o| o.clone().upcast::<glib::Object>())
    }
}
