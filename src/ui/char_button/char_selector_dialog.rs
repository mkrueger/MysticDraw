use gtk4::{ResponseType, traits::{DialogExt, CheckButtonExt, WidgetExt, BoxExt}};
use libadwaita::ApplicationWindow;

pub struct CharSelectorDialog {
    pub dialog: gtk4::Dialog,

    pub check_box: gtk4::CheckButton,
    pub name_entry: gtk4::Entry
}

pub fn display_char_selector_dialog() -> CharSelectorDialog
{
    let dialog = gtk4::Dialog::with_buttons(
        Some("Edit Item"),
        Option::<&ApplicationWindow>::None,
        gtk4::DialogFlags::MODAL,
        &[("OK", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
    );
    dialog.set_default_response(ResponseType::Cancel);

    let content_area = dialog.content_area();

    let check_box = gtk4::CheckButton::new();
    check_box.set_label(Some("Visible"));
    content_area.append(&check_box);

    let name_entry = gtk4::Entry::new();
    content_area.append(&name_entry);

    dialog.show();

    CharSelectorDialog {
        dialog,
        check_box,
        name_entry
    }
}
