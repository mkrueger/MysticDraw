use std::{rc::Rc, cell::RefCell};

use gtk4::{ traits::{ WidgetExt, BoxExt}, Align };
use libadwaita::{ PreferencesGroup, ActionRow, traits::{PreferencesGroupExt, ActionRowExt} };

use crate::{ model::{SaveOptions}};

pub fn create_settings_page(opt_ref: Rc<RefCell<SaveOptions>>, content_area: &gtk4::Box) 
{
    let group = PreferencesGroup::new();
    group.set_hexpand(false);
    group.set_title("Settings");

    let compression_names = [
        "Off",
        "Medium",
        "High"
    ];
    let options = opt_ref.borrow();
    
    let comp_dropdown = gtk4::DropDown::from_strings(&compression_names);
    comp_dropdown.set_valign(Align::Center);
    match options.compression_level {
        crate::model::CompressionLevel::Off => comp_dropdown.set_selected(0),
        crate::model::CompressionLevel::Medium => comp_dropdown.set_selected(1),
        crate::model::CompressionLevel::High => comp_dropdown.set_selected(2)
    }

    let row = ActionRow::builder()
        .title("Compression level")
        .build();
    row.add_suffix(&comp_dropdown);
    group.add(&row);

    let save_sauce_switch = gtk4::Switch::builder()
        .valign(Align::Center)
        .active(options.save_sauce)
        .build();
    let row = ActionRow::builder()
        .title("Save sauce")
        .build();
    row.add_suffix(&save_sauce_switch);
    group.add(&row);

    let opt = opt_ref.clone();
    comp_dropdown.connect_selected_notify(move |d| {
        opt.borrow_mut().compression_level = if d.selected() == 0 {
            crate::model::CompressionLevel::Off
        } else if d.selected() == 1 {
            crate::model::CompressionLevel::Medium
        } else {
            crate::model::CompressionLevel::High
        };
    });

    let opt = opt_ref.clone();
    save_sauce_switch.connect_state_set(move |_, state| {
        opt.borrow_mut().save_sauce = state;
        println!("{}", state);
        gtk4::Inhibit(false)
    });
    content_area.append(&group);
}
