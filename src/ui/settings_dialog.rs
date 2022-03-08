use gtk4::{ traits::{ WidgetExt, BoxExt, GtkWindowExt, EditableExt}, SpinButton, Orientation, Align };
use libadwaita::{ PreferencesGroup, ActionRow, traits::{PreferencesGroupExt, ActionRowExt}, HeaderBar };

use crate::WORKSPACE;

use super::MainWindow;

pub struct SettingsDialog {
    pub dialog: libadwaita::PreferencesWindow,
    pub open_button: gtk4::Button,
}

pub fn display_settings_dialog(main_window: &MainWindow) -> SettingsDialog
{
    let main_area = gtk4::Box::builder()
    .orientation(Orientation::Vertical)
    .build();
    let dialog = libadwaita::PreferencesWindow::builder()
        .default_width(480)
        .default_height(440)
        .modal(true)
        .resizable(false)
        .content(&main_area)
        .build();
    dialog.set_transient_for(Some(&main_window.window));

    let hb = HeaderBar::builder()
        .title_widget(&libadwaita::WindowTitle::builder().title("Preferences").build())
        .show_end_title_buttons(true)
        .build();
    let open_button = gtk4::Button::builder()
        .label("OK")
        .build();
    hb.pack_start(&open_button);
    main_area.append(&hb);

    let content_area = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(20)
        .margin_top(20)
        .margin_end(20)
        .margin_start(20)
        .spacing(8)
        .build();

    let group = PreferencesGroup::new();
    group.set_title("Settings");

    let tab_size_spin_button = SpinButton::with_range(0.0, 10000.0, 10.0);
    unsafe {
        tab_size_spin_button.set_value(WORKSPACE.settings.tab_size as f64);
    }

    let row = ActionRow::builder()
        .title("Tab size")
        .build();
    row.add_suffix(&tab_size_spin_button);
    group.add(&row);

    let name_entry = gtk4::Entry::new();
    name_entry.set_valign(Align::Center);
    unsafe {
        if let Some(path) = &WORKSPACE.settings.font_path  {
            name_entry.set_text(path.to_str().unwrap());
        }
    }
    let row = ActionRow::builder()
        .title("TDF font path")
        .build();
    row.add_suffix(&name_entry);
    group.add(&row);

    content_area.append(&group);
    main_area.append(&content_area);

    dialog.show();
    SettingsDialog {
        dialog,
        open_button,
    }
}
