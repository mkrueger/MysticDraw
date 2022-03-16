use std::{rc::Rc, cell::RefCell};

use gtk4::{ traits::{ WidgetExt, BoxExt}, Align };
use libadwaita::{ PreferencesGroup, ActionRow, traits::{PreferencesGroupExt, ActionRowExt} };

use crate::{ model::{SaveOptions, ScreenPreperation}};

pub fn create_settings_page(opt_ref: Rc<RefCell<SaveOptions>>, content_area: &gtk4::Box)
{
    let group = PreferencesGroup::new();
        group.set_hexpand(false);
        group.set_title("Settings");
        
    let preparation_names = [
        "None",
        "Clear Screen",
        "Home Cursor"
    ];
    let options = opt_ref.borrow();
    let prep_dropdown = gtk4::DropDown::from_strings(&preparation_names);
    prep_dropdown.set_valign(Align::Center);
    match options.screen_preparation {
        crate::model::ScreenPreperation::None => prep_dropdown.set_selected(0),
        crate::model::ScreenPreperation::ClearScreen => prep_dropdown.set_selected(1),
        crate::model::ScreenPreperation::Home => prep_dropdown.set_selected(2)
    }

    let row = ActionRow::builder()
        .title("Video Preparation")
        .build();
    row.add_suffix(&prep_dropdown);
    group.add(&row);

    let save_unicode_switch = gtk4::Switch::builder()
        .valign(Align::Center)
        .active(options.modern_terminal_output)
        .build();
    let row = ActionRow::builder()
        .title("Modern terminal format (utf8)")
        .build();
    row.add_suffix(&save_unicode_switch);
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
    prep_dropdown.connect_selected_notify(move |d| {
        opt.borrow_mut().screen_preparation = if d.selected() == 0 {
            ScreenPreperation::None
        } else if d.selected() == 1 {
            ScreenPreperation::ClearScreen
        } else {
            ScreenPreperation::Home
        };
    });

    let opt = opt_ref.clone();
    save_unicode_switch.connect_state_set(move |_, state| {
        opt.borrow_mut().modern_terminal_output = state;
        gtk4::Inhibit(false)
    });

    let opt = opt_ref.clone();
    save_sauce_switch.connect_state_set(move |_, state| {
        opt.borrow_mut().save_sauce = state;
        gtk4::Inhibit(false)
    });

    content_area.append(&group);
}