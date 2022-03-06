use std::cell::RefCell;

use gtk4::{
    glib::{self, ParamSpec, ParamSpecObject, Value},
    prelude::*,
    subclass::prelude::*,
};

use super::FontRowData;

#[derive(Default, Debug)]
pub struct FontListBoxRowImpl {
    pub row_data: RefCell<Option<FontRowData>>,
}

#[glib::object_subclass]
impl ObjectSubclass for FontListBoxRowImpl {
    const NAME: &'static str = "FontListBoxRow";
    type ParentType = gtk4::ListBoxRow;
    type Type = super::list_box_row::FontListBoxRow;
}

impl ObjectImpl for FontListBoxRowImpl {
    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::new(
                "row-data",
                "Row Data",
                "Row Data",
                FontRowData::static_type(),
                glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT_ONLY,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "row-data" => {
                let row_data = value.get().unwrap();
                self.row_data.replace(row_data);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "row-data" => self.row_data.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        let item = self.row_data.borrow();
        let item = item.as_ref().cloned().unwrap();

        let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
        let label = gtk4::Label::new(None);
        item.bind_property("name", &label, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        hbox.append(&label);
        obj.set_child(Some(&hbox));
    }
}

impl WidgetImpl for FontListBoxRowImpl {}
impl ListBoxRowImpl for FontListBoxRowImpl {}
