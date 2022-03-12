use std::{rc::Rc, cell::RefCell};

use glib::{ObjectExt, clone};
use gtk4::{ traits::{ WidgetExt, BoxExt, EditableExt, EntryExt, TextViewExt, TextBufferExt, StyleContextExt, ButtonExt, DialogExt, FileChooserExt, GtkWindowExt}, SpinButton, Orientation, Align, prelude::{DisplayExt, FileExt}, pango::EllipsizeMode, FileChooserAction, ResponseType};
use libadwaita::{ PreferencesGroup, ActionRow, traits::{PreferencesGroupExt, ActionRowExt}};

use crate::model::Editor;

use super::MainWindow;

pub struct BufferSettingsPage {
    pub dialog: libadwaita::Window,
    pub open_button: gtk4::Button,

    pub width_spin_button: SpinButton,
    pub height_spin_button: SpinButton
}


pub fn get_settings_page(main_window: &MainWindow, editor_ref: Rc<RefCell<Editor>>) -> gtk4::Box
{
    let content_area = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(20)
        .margin_top(20)
        .margin_end(20)
        .margin_start(20)
        .spacing(8)
        .build();

    let editor = editor_ref.borrow();
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
        .title("Sauce settings")
        .build();
    row.add_suffix(&save_sauce_switcher);
    group.add(&row);
    content_area.append(&group);

    let group = PreferencesGroup::new();
    group.set_hexpand(false);
    group.set_title("Buffer settings");

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

    
    let open_button_content =  gtk4::Box::builder()
        .spacing(6)
        .build();
    open_button_content.append(&gtk4::Image::from_icon_name("document-open-symbolic"));

    let bg_image_file_name= gtk4::Label::builder()
        .ellipsize(EllipsizeMode::Middle)
        .build();

    update_reference_file_name(&bg_image_file_name, &editor);
    open_button_content.append(&bg_image_file_name);

    let open_button = gtk4::Button::builder()
        .valign(Align::Center)
        .child(&open_button_content)
        .build();
    let trash_button = gtk4::Button::builder()
        .valign(Align::Center)
        .icon_name("user-trash-symbolic")
        .build();
    trash_button.style_context().add_class("flat");

    let row = ActionRow::builder()
        .title("Reference image")
        .build();
    row.add_suffix(&open_button);
    row.add_suffix(&trash_button);
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
        .hexpand(true)
        .vexpand(true)
        .build();

    content_area.append(&gtk4::Label::builder().label("Comment").build());
    content_area.append(&scroller);

    let editor_ref2 = editor_ref.clone();
    trash_button.connect_clicked(clone!(@weak bg_image_file_name => move |_| {
        editor_ref2.borrow_mut().reference_image = None;
        update_reference_file_name(&bg_image_file_name, &editor_ref2.borrow());
    }));

    let editor_ref = editor_ref.clone();
    let dialog = &main_window.window;
    open_button.connect_clicked(clone!(@strong dialog, @weak bg_image_file_name => move |_| {
        let file_chooser = gtk4::FileChooserDialog::builder()
            .title("Open file")
            .action(FileChooserAction::Open)
            .transient_for(&dialog)
            .modal(true)
            .width_request(640)
            .height_request(480)
            .vexpand(false)
            .hexpand(false)
            .build();

        file_chooser.add_button("Open", ResponseType::Ok);
        file_chooser.add_button("Cancel", ResponseType::Cancel);

        file_chooser.connect_response(clone!(@strong editor_ref => move |d, response| {
            if response == ResponseType::Ok {
                let file = d.file().expect("Couldn't get file");
                let file_name = file.path().expect("Couldn't get file path");
                editor_ref.borrow_mut().reference_image = Some(file_name);
                update_reference_file_name(&bg_image_file_name, &editor_ref.borrow());
            }
            d.close();
        }));
        file_chooser.present();
    }));

    content_area
}

fn update_reference_file_name(label: &gtk4::Label, editor: &std::cell::Ref<Editor>)  {
    if let Some(p) = &editor.reference_image {
        label.set_label(p.file_name().unwrap().to_str().unwrap());
    } else {
        label.set_label("(None)");
    }
}
