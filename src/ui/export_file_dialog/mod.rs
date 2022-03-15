use std::{rc::Rc, cell::RefCell, path::PathBuf};

use glib::{StaticType, clone };
use gtk4::{ traits::{ WidgetExt, BoxExt, GtkWindowExt, EditableExt, StyleContextExt, ButtonExt, DialogExt, FileChooserExt}, Orientation, Align, PropertyExpression, StringObject, FileChooserAction, ResponseType, prelude::FileExt };
use libadwaita::{ HeaderBar };

use crate::{model::{SaveOptions, Editor}};

use super::MainWindow;

mod ansi;
mod artworx;
mod ascii;
mod avatar;
mod bin;
mod ice_draw;
mod pcboard;
mod tundra_draw;
mod xbin;


pub fn display_export_dialog(main_window: Rc<MainWindow>, editor: Rc<RefCell<Editor>>) 
{
    let main_area = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    let dialog = libadwaita::PreferencesWindow::builder()
        .default_width(680)
        .default_height(360)
        .modal(true)
        .resizable(false)
        .content(&main_area)
        .build();
    dialog.set_transient_for(Some(&main_window.window));

    let hb = HeaderBar::builder()
        .title_widget(&libadwaita::WindowTitle::builder().title("Export").build())
        .show_end_title_buttons(true)
        .build();
    let export_button = gtk4::Button::builder()
        .label("Export")
        .build();
    hb.pack_start(&export_button);
    main_area.append(&hb);

    let content_area = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(20)
        .margin_top(20)
        .margin_end(20)
        .margin_start(20)
        .spacing(8)
        .build();

    let file_row = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .build();

    let file_entry = gtk4::Entry::new();
    if let Some(file_name) = &editor.borrow().buf.file_name {
        file_entry.set_text(file_name.to_str().unwrap());
    }
    file_entry.set_valign(Align::Center);
    file_entry.set_hexpand(true);
    file_row.append(&file_entry);
    
    let set_filename_button = gtk4::Button::builder()
        .valign(Align::Center)
        .icon_name("document-save-symbolic")
        .build();
    set_filename_button.style_context().add_class("flat");
    file_row.append(&set_filename_button);
    let list_model = gtk4::StringList::new(&[]);
    for l in TYPE_DESCRIPTIONS {
        list_model.append(l.0);
    }
    
    let item_string_x = PropertyExpression::new(
        StringObject::static_type(), 
        gtk4::Expression::NONE, 
        "string");

    let type_dropdown = gtk4::DropDown::new(Some(&list_model), Some(item_string_x));
    file_row.append(&type_dropdown);
    content_area.append(&file_row);

    let stack = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(20)
        .margin_top(20)
        .margin_end(20)
        .margin_start(20)
        .spacing(8)
        .build();
    content_area.append(&stack);

    main_area.append(&content_area);

    dialog.show();

    let options = Rc::new(RefCell::new(SaveOptions::new()));
    options.borrow_mut().save_sauce = editor.borrow().buf.get_save_sauce_default(TYPE_DESCRIPTIONS[0].2).0;
    (TYPE_DESCRIPTIONS[0].1)(options.clone(), &stack);
    change_extension(&file_entry, 0);

    let options2 = options.clone();
    type_dropdown.connect_selected_notify(clone!(@weak stack, @weak file_entry, @weak editor => move |x| {
        while let Some(child) = &stack.first_child() {
            stack.remove(child);
        }
        let idx = x.selected() as usize;
        options2.borrow_mut().save_sauce = editor.borrow().buf.get_save_sauce_default(TYPE_DESCRIPTIONS[idx].2).0;
        (TYPE_DESCRIPTIONS[idx].1)(options2.clone(), &stack);
        change_extension(&file_entry, idx);
    }));

    set_filename_button.connect_clicked(clone!(@weak file_entry => move |_| {
        let file_chooser = gtk4::FileChooserDialog::builder()
            .title("Select file name")
            .action(FileChooserAction::Save)
            //.transient_for(&dialog)
            .modal(true)
            .width_request(640)
            .height_request(480)
            .build();

        file_chooser.add_button("OK", ResponseType::Ok);
        file_chooser.add_button("Cancel", ResponseType::Cancel);
        file_chooser.connect_response(/*clone!(@weak file_entry =>*/ move |d, response| {
            if response == ResponseType::Ok {
                let file = d.file().expect("Couldn't get file");
                let filename = file.path().expect("Couldn't get file path");

                file_entry.set_text(filename.as_os_str().to_str().unwrap());
            }
            d.close();
        });
        file_chooser.show();
    }));

    export_button.connect_clicked(clone!(@weak file_entry, @weak main_window => move |_| {
        let filename = PathBuf::from(file_entry.text().as_str());
        main_window.handle_error(editor.borrow().save_content(&filename, &options.borrow()), move || format!("Error saving {}", filename.as_os_str().to_string_lossy()));
        dialog.close();
    }));
}

fn change_extension(file_entry: &gtk4::Entry, arg: usize) 
{
    let mut filename = PathBuf::from(file_entry.text().as_str());
    filename.set_extension(TYPE_DESCRIPTIONS[arg].2);
    file_entry.set_text(filename.to_str().unwrap());
}

const TYPE_DESCRIPTIONS: [(&str, fn(Rc<RefCell<SaveOptions>>, &gtk4::Box), &str); 9] = [
    ("Ansi (.ans)", ansi::create_settings_page, "ans"),
    ("Avatar (.avt)", avatar::create_settings_page, "avt"),
    ("PCBoard (.pcb)", pcboard::create_settings_page, "pcb"),
    ("Ascii (.asc)", ascii::create_settings_page, "asc"),

    ("Artworx (.adf)", artworx::create_settings_page, "adf"),
    ("Ice Draw (.idf)", ice_draw::create_settings_page, "idf"),
    ("Tundra Draw (.tnd)", tundra_draw::create_settings_page, "tnd"),

    ("Bin (.bin)", bin::create_settings_page, "bin"),  
    ("XBin (.xb)", xbin::create_settings_page, "xb"),
];
