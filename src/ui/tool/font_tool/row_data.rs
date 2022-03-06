use gtk4::glib;

glib::wrapper! {
    pub struct FontRowData(ObjectSubclass<super::row_data_imp::FontRowDataImpl>);
}

impl FontRowData {
    pub fn new(name: &str, count: u32) -> FontRowData {
        glib::Object::new(&[("name", &name), ("count", &count)]).expect("Failed to create row data")
    }
}
