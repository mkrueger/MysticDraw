use std::{rc::Rc, cell::RefCell};

use gtk4::{ traits::{ WidgetExt, BoxExt}, Align };
use libadwaita::{ PreferencesGroup, ActionRow, traits::{PreferencesGroupExt, ActionRowExt} };

use crate::{ model::{SaveOptions}};


pub fn create_settings_page(opt_ref: Rc<RefCell<SaveOptions>>, content_area: &gtk4::Box) 
{
    let group = PreferencesGroup::new();
    group.set_hexpand(false);
    group.set_title("Settings");
    let options = opt_ref.borrow();

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
    save_sauce_switch.connect_state_set(move |_, state| {
        opt.borrow_mut().save_sauce = state;
        println!("{}", state);
        gtk4::Inhibit(false)
    });
    content_area.append(&group);
}
