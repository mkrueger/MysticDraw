use glib::ObjectExt;
use gtk4::{ traits::{ WidgetExt, BoxExt, GtkWindowExt}, SpinButton, Orientation, Align, prelude::DisplayExt};
use libadwaita::{ PreferencesGroup, ActionRow, traits::{PreferencesGroupExt, ActionRowExt}, HeaderBar};

use super::MainWindow;

pub struct NewFileDialog {
    pub dialog: libadwaita::Window,
    pub open_button: gtk4::Button,

    pub width_spin_button: SpinButton,
    pub height_spin_button: SpinButton,
    pub type_dropdown: gtk4::DropDown
}

pub fn display_newfile_dialog(main_window: &MainWindow) -> NewFileDialog
{
    let main_area = gtk4::Box::builder()
    .orientation(Orientation::Vertical)
    .build();

    let dialog = libadwaita::Window::builder()
        .default_width(480)
        .default_height(240)
        .modal(true)
        .resizable(false)
        .content(&main_area)
        .build();
        dialog.set_transient_for(Some(&main_window.window));
        let hb = HeaderBar::builder()
        .title_widget(&libadwaita::WindowTitle::builder().title("New file").build())
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
    group.set_title("Set size");

    let mut width = 80.0;
    let mut height = 100.0;
    let display = gtk4::gdk::Display::default().unwrap();
    let clipboard = display.clipboard();
    unsafe {
        if let Some(data) = clipboard.data::<super::ClipboardLayer>("MysticDraw.Layer") {
            let layer = data.as_ref();
            width = layer.size.width as f64;
            height = layer.size.height as f64;
        }
    }

    let width_spin_button = SpinButton::with_range(0.0, 10000.0, 10.0);
    width_spin_button.set_valign(Align::Center);
    width_spin_button.set_value(width);
    let row = ActionRow::builder()
        .title("Width")
        .build();
    row.add_suffix(&width_spin_button);
    group.add(&row);

    let height_spin_button = SpinButton::with_range(0.0, 10000.0, 10.0);
    height_spin_button.set_valign(Align::Center);
    height_spin_button.set_value(height);
    let row = ActionRow::builder()
        .title("Height")
        .build();
    row.add_suffix(&height_spin_button);
    group.add(&row);

    let type_names = [
        "VGA",
        "iCE",
        "2 fonts",
        "2 fonts + iCE",
        "RGB"
    ];

    let description = [
        "16 fore, 8 back, blink mode",
        "16 colors, no blink",
        "8 fore, 8 back, blink mode, 2 fonts",
        "8 fore, 16 back, no blink, 2 fonts",
        "unlimited colors and blink"
    ];
    
    let type_dropdown = gtk4::DropDown::from_strings(&type_names);
    type_dropdown.set_valign(Align::Center);

    let row = ActionRow::builder()
        .title("Type")
        .build();
    row.set_subtitle(description[0]);
    row.add_suffix(&type_dropdown);
    group.add(&row);
    content_area.append(&group);
    main_area.append(&content_area);
    dialog.show();

    type_dropdown.connect_selected_notify(move |d| {
        row.set_subtitle(description[d.selected() as usize]);
    });

    NewFileDialog {
        dialog,
        open_button,
        width_spin_button,
        height_spin_button,
        type_dropdown
    }
}
