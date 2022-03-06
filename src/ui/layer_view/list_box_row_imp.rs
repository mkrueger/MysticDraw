use std::cell::RefCell;

use glib::{subclass::Signal, clone};
use gtk4::{
    glib::{self, ParamSpec, ParamSpecObject, Value},
    prelude::*,
    subclass::prelude::*,
};

use super::RowData;

#[derive(Default, Debug)]
pub struct ListBoxRow {
    pub row_data: RefCell<Option<RowData>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ListBoxRow {
    const NAME: &'static str = "ExListBoxRow";
    type ParentType = gtk4::ListBoxRow;
    type Type = super::list_box_row::ListBoxRow;
}

impl ObjectImpl for ListBoxRow {
    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::new(
                "row-data",
                "Row Data",
                "Row Data",
                RowData::static_type(),
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
    
    fn signals() -> &'static [Signal] {
        use once_cell::sync::Lazy;
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("isvisiblechanged", &[bool::static_type().into()], <()>::static_type().into()).build()]
        });
        SIGNALS.as_ref()
    }

    fn constructed(&self, obj: &Self::Type) {
        let item = self.row_data.borrow();
        let item = item.as_ref().cloned().unwrap();

        let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);

        let check_button = gtk4::CheckButton::new();
        item.bind_property("isvisible", &check_button, "active")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            .build();
        hbox.append(&check_button);

        let label = gtk4::Label::new(None);
        item.bind_property("name", &label, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        hbox.append(&label);
        
        obj.set_child(Some(&hbox));
        obj.index();

        check_button.connect_toggled(clone!(@weak obj => move |x| {
            obj.emit_by_name::<()>("isvisiblechanged", &[&x.is_active()]);
        }));
    }
}

impl WidgetImpl for ListBoxRow {}
impl ListBoxRowImpl for ListBoxRow {}
