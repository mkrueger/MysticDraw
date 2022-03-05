use gtk4::{ResponseType, traits::{DialogExt,  WidgetExt, BoxExt, GtkWindowExt}, SpinButton};
use libadwaita::{ PreferencesGroup, ActionRow, traits::{PreferencesGroupExt, ActionRowExt}};

use super::MainWindow;

pub struct NewFileDialog {
    pub dialog: gtk4::Dialog,

    pub width_spin_button: SpinButton,
    pub height_spin_button: SpinButton
}

pub fn display_newfile_dialog(main_window: &MainWindow) -> NewFileDialog
{
    let dialog = gtk4::Dialog::with_buttons(
        Some("New File"),
        Some(&main_window.window),
        gtk4::DialogFlags::MODAL,
        &[("OK", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
    );
    dialog.set_width_request(280);
    dialog.set_height_request(240);
    dialog.set_resizable(false);
    dialog.set_default_response(ResponseType::Cancel);
    let content_area = dialog.content_area();
    content_area.set_margin_top(20);
    content_area.set_margin_bottom(20);
    content_area.set_margin_start(20);
    content_area.set_margin_end(20);
    

    let group = PreferencesGroup::new();
    group.set_title("New file size");

    let width_spin_button = SpinButton::with_range(0.0, 500.0, 10.0);
    width_spin_button.set_value(80.0);

    let row = ActionRow::builder()
    .title("Width")
    .build();
    row.add_suffix(&width_spin_button);
    group.add(&row);

    let height_spin_button = SpinButton::with_range(0.0, 10000.0, 10.0);
    height_spin_button.set_value(100.0);
    let row = ActionRow::builder()
    .title("Height")
    .build();
    row.add_suffix(&height_spin_button);
    group.add(&row);

    content_area.append(&group);

    dialog.show();

    NewFileDialog {
        dialog,
        width_spin_button,
        height_spin_button
    }
}
