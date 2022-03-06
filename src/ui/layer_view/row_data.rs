use gtk4::glib;

glib::wrapper! {
    pub struct RowData(ObjectSubclass<super::row_data_imp::RowData>);
}

impl RowData {
    pub fn new(name: &str, is_visible: bool) -> RowData {
        glib::Object::new(&[("name", &name), ("isvisible", &is_visible)]).expect("Failed to create row data")
    }
}