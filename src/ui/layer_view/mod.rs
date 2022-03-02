mod model;
use gtk4::{traits::{WidgetExt, DialogExt, GtkWindowExt, EntryExt, BoxExt, EditableExt, CheckButtonExt}, ResponseType};
pub use model::*;
mod model_imp;

mod row_data;
pub use row_data::*;
mod row_data_imp;

mod list_box_row;
pub use list_box_row::*;

use crate::model::Layer;
mod list_box_row_imp;


pub struct EditLayerDialog {
    pub dialog: gtk4::Dialog,

    check_box: gtk4::CheckButton,
    name_entry: gtk4::Entry
}

impl EditLayerDialog {
    pub fn set_layer_values(&self, layer: &mut Layer)
    {
        layer.name = self.name_entry.text().to_string();
        layer.is_visible = self.check_box.is_active();
    } 
}

pub fn display_edit_layer_dialog(window: &libadwaita::ApplicationWindow, layer: &Layer) -> EditLayerDialog
{
    let dialog = gtk4::Dialog::with_buttons(
        Some("Edit Item"),
        Some(window),
        gtk4::DialogFlags::MODAL,
        &[("OK", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
    );
    dialog.set_default_response(ResponseType::Cancel);

    let content_area = dialog.content_area();

    let check_box = gtk4::CheckButton::new();
    check_box.set_label(Some("Visible"));
    check_box.set_active(layer.is_visible);
    content_area.append(&check_box);

    let name_entry = gtk4::Entry::new();
    name_entry.set_text(layer.name.as_str());
    content_area.append(&name_entry);

    dialog.show();

    EditLayerDialog {
        dialog,
        check_box,
        name_entry
    }
}