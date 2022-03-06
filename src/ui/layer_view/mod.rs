mod model;
use gtk4::{traits::{WidgetExt, BoxExt, EditableExt, GtkWindowExt}, Orientation, Switch, SpinButton, Align};
use libadwaita::{PreferencesGroup, traits::{PreferencesGroupExt, ActionRowExt}, HeaderBar, ActionRow};
pub use model::*;
mod model_imp;

mod row_data;
pub use row_data::*;
mod row_data_imp;

mod list_box_row;
pub use list_box_row::*;

use crate::model::{Layer, Position};
mod list_box_row_imp;

pub struct EditLayerDialog {
    pub dialog: libadwaita::Window,

    pub open_button: gtk4::Button,
    name_entry: gtk4::Entry,
    is_visible_switch: Switch,
    is_locked_switch: Switch,
    is_position_locked_switch: Switch,
    xoffset_spin_button : SpinButton,
    yoffset_spin_button : SpinButton,
}

impl EditLayerDialog {
    pub fn set_layer_values(&self, layer: &mut Layer)
    {
        layer.name = self.name_entry.text().to_string();
        layer.is_visible = self.is_visible_switch.is_active();
        layer.is_locked = self.is_locked_switch.is_active();
        layer.is_position_locked = self.is_position_locked_switch.is_active();
        layer.set_offset(Position::from(self.xoffset_spin_button.value() as i32, self.yoffset_spin_button.value() as i32));
    } 
}

pub fn display_edit_layer_dialog(window: &libadwaita::ApplicationWindow, layer: &Layer) -> EditLayerDialog
{
    let main_area = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let dialog = libadwaita::Window::builder()
        .default_width(400)
        .default_height(420)
        .modal(true)
        .resizable(false)
        .content(&main_area)
        .build();
    dialog.set_transient_for(Some(window));
    let hb = HeaderBar::builder()
        .title_widget(&libadwaita::WindowTitle::builder().title("Layer Preferences").build())
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
    group.set_title("Layer settings");

    let name_entry = gtk4::Entry::new();
    name_entry.set_valign(Align::Center);
    name_entry.set_text(layer.name.as_str());
    let row = ActionRow::builder()
        .title("Name")
        .build();
    row.add_suffix(&name_entry);
    group.add(&row);

    let is_visible_switch = Switch::builder()
        .active(layer.is_visible)
        .valign(Align::Center)
        .build();
    let row = ActionRow::builder()
        .title("Visible")
        .build();
    row.add_suffix(&is_visible_switch);
    group.add(&row);

    let is_locked_switch = Switch::builder()
        .active(layer.is_locked)
        .valign(Align::Center)
        .build();
    let row = ActionRow::builder()
        .title("Locked")
        .build();
    row.add_suffix(&is_locked_switch);
    group.add(&row);

    let is_position_locked_switch = Switch::builder()
        .active(layer.is_position_locked)
        .valign(Align::Center)
        .build();
    let row = ActionRow::builder()
        .title("Lock position and size")
        .build();
    row.add_suffix(&is_position_locked_switch);
    group.add(&row);
    content_area.append(&group);

    let group = PreferencesGroup::new();
    group.set_title("Layer offset");

    let xoffset_spin_button = SpinButton::with_range(0.0, 10000.0, 1.0);
    xoffset_spin_button.set_valign(Align::Center);
    xoffset_spin_button.set_value(layer.get_offset().x as f64);
    let row = ActionRow::builder()
        .title("X")
        .build();
    row.add_suffix(&xoffset_spin_button);
    group.add(&row);

    let yoffset_spin_button = SpinButton::with_range(0.0, 10000.0, 1.0);
    yoffset_spin_button.set_valign(Align::Center);
    yoffset_spin_button.set_value(layer.get_offset().y as f64);
    let row = ActionRow::builder()
        .title("Y")
        .build();
    row.add_suffix(&yoffset_spin_button);
     group.add(&row);
   content_area.append(&group);
    main_area.append(&content_area);
    dialog.show();

    EditLayerDialog {
        dialog,
        open_button,
        name_entry,
        is_visible_switch,
        is_locked_switch,
        is_position_locked_switch,
        xoffset_spin_button,
        yoffset_spin_button
    }
}