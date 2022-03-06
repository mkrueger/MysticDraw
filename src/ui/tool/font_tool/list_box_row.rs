use glib::ObjectExt;
use gtk4::glib;

use super::{list_box_row_imp, FontRowData};

glib::wrapper! {
    pub struct FontListBoxRow(ObjectSubclass<list_box_row_imp::FontListBoxRowImpl>)
        @extends gtk4::Widget, gtk4::ListBoxRow;
}

impl FontListBoxRow {
    pub fn new(row_data: &FontRowData) -> Self {
        glib::Object::new(&[("row-data", &row_data)]).unwrap()
    }

    pub fn index(&self) -> FontRowData {
        self.property_value("row-data").get::<FontRowData>().unwrap()
    }
}
