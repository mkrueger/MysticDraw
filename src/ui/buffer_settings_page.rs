use std::{rc::Rc, cell::RefCell};

use glib::ObjectExt;
use gtk4::{ traits::{ WidgetExt, BoxExt, EditableExt, EntryExt, TextViewExt, TextBufferExt}, SpinButton, Orientation, Align, prelude::DisplayExt};
use libadwaita::{ PreferencesGroup, ActionRow, traits::{PreferencesGroupExt, ActionRowExt}};

use crate::model::Editor;

use super::MainWindow;

pub struct BufferSettingsPage {
    pub dialog: libadwaita::Window,
    pub open_button: gtk4::Button,

    pub width_spin_button: SpinButton,
    pub height_spin_button: SpinButton
}


pub fn get_settings_page(_main_window: &MainWindow, editor: Rc<RefCell<Editor>>) -> gtk4::Box
{
    let content_area = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(20)
        .margin_top(20)
        .margin_end(20)
        .margin_start(20)
        .spacing(8)
        .build();

    let editor = editor.borrow();
    let group = PreferencesGroup::new();
    group.set_hexpand(false);
    group.set_title("Sauce");


    let title_entry = gtk4::Entry::new();
    title_entry.set_valign(Align::Center);
    title_entry.set_max_length(editor.buf.title.max_len() as i32);
    title_entry.set_text(editor.buf.title.to_string().as_str());
    let row = ActionRow::builder()
        .title("Title")
        .build();
    row.add_suffix(&title_entry);
    group.add(&row);

    let group_entry = gtk4::Entry::new();
    group_entry.set_valign(Align::Center);
    group_entry.set_max_length(editor.buf.group.max_len() as i32);
    group_entry.set_text(editor.buf.group.to_string().as_str());
    let row = ActionRow::builder()
        .title("Group")
        .build();
    row.add_suffix(&group_entry);
    group.add(&row);

    let author_entry = gtk4::Entry::new();
    author_entry.set_valign(Align::Center);
    author_entry.set_max_length(editor.buf.author.max_len() as i32);
    author_entry.set_text(editor.buf.author.to_string().as_str());
    let row = ActionRow::builder()
        .title("Author")
        .build();
    row.add_suffix(&author_entry);
    group.add(&row);

    let save_sauce_switcher = gtk4::Switch::builder()
    .active(editor.buf.write_sauce)
    .valign(Align::Center)
    .build();
    let row = ActionRow::builder()
        .title("Save Sauce")
        .build();
    row.add_suffix(&save_sauce_switcher);
    group.add(&row);
    content_area.append(&group);

    let comment_textview = gtk4::TextView::new();
    for cmt in &editor.buf.comments {
        let b = &comment_textview.buffer();
        b.insert_at_cursor(cmt.to_string().as_str());
        b.insert_at_cursor("\n");
    }
    let scroller = gtk4::ScrolledWindow::builder()
    .child(&comment_textview)
    .build();

    content_area.append(&gtk4::Label::builder().label("Comment").build());
    content_area.append(&scroller);

    let group = PreferencesGroup::new();
    group.set_hexpand(false);
    group.set_title("Buffer size");

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

    content_area.append(&group);
    content_area
}
