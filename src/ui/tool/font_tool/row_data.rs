use gtk4::glib;

glib::wrapper! {
    pub struct RowData(ObjectSubclass<super::row_data_imp::RowData>);
}

impl RowData {
    pub fn new(name: &str, count: u32) -> RowData {
        glib::Object::new(&[("name", &name), ("count", &count)]).expect("Failed to create row data")
    }
}
