use glib::ObjectExt;
use gtk4::glib;

use super::{list_box_row_imp, RowData};

glib::wrapper! {
    pub struct ListBoxRow(ObjectSubclass<list_box_row_imp::ListBoxRow>)
    @extends gtk4::Widget, gtk4::ListBoxRow;
}

impl ListBoxRow {
    pub fn new(row_data: &RowData) -> Self {
        glib::Object::new(&[("row-data", &row_data)]).unwrap()
    }

    pub fn get_data(&self) -> RowData {
        self.property_value("row-data").get::<RowData>().unwrap()
    }
}
