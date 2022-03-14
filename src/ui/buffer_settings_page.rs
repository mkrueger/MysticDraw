use std::{rc::Rc, cell::RefCell, fs::File, io::Write};

use glib::{clone, StaticType };
use gtk4::{ traits::{ WidgetExt, BoxExt, EditableExt, EntryExt, TextViewExt, TextBufferExt, StyleContextExt, ButtonExt, DialogExt, FileChooserExt, GtkWindowExt}, SpinButton, Orientation, Align, prelude::{ FileExt, EntryBufferExtManual}, pango::EllipsizeMode, FileChooserAction, ResponseType, StringObject, PropertyExpression };
use libadwaita::{ PreferencesGroup, ActionRow, traits::{PreferencesGroupExt, ActionRowExt}};

use crate::model::{Editor, SauceString, BitFont, BitFontType};

use super::MainWindow;

pub struct BufferSettingsPage {
    pub content_area: gtk4::Box,

    title_entry: gtk4::Entry,
    group_entry: gtk4::Entry,
    author_entry: gtk4::Entry,
    comment_textview: gtk4::TextView,

    font_dropdown: Rc<gtk4::DropDown>,
    save_sauce_switcher: gtk4::Switch,

    width_spin_button: SpinButton,
    height_spin_button: SpinButton,
    custom_font: Option<BitFont>,

    editor:Rc<RefCell<Editor>>
}

impl BufferSettingsPage {
    pub fn sync_back(&self) {
        let mut editor = self.editor.borrow_mut();

        editor.buf.title = SauceString::from(&self.title_entry.buffer().text());

        editor.buf.group = SauceString::from(&self.group_entry.buffer().text());
        editor.buf.author = SauceString::from(&self.author_entry.buffer().text());
        editor.buf.comments.clear();
        let b  =self.comment_textview.buffer();
        let str  = b.text(&b.start_iter(), &b.end_iter(), true);
        let comments: Vec<&str> = str.split('\n').collect();

        for comment in comments {
            editor.buf.comments.push(SauceString::from(comment));
        }
        editor.buf.write_sauce = self.save_sauce_switcher.is_active();
        editor.buf.width = self.width_spin_button.value() as u16;
        editor.buf.height = self.height_spin_button.value() as u16;
        let mut row  = self.font_dropdown.selected();
        if let Some(custom_font) = &self.custom_font {
            if row == 0 {
                editor.buf.font = custom_font.clone();
                return;
            }
            row -= 1;
        }
        if let Some(font) = BitFont::from_name( &BitFont::get_font_list()[row as usize]) {
            editor.buf.font = font
        }
    }
}

pub fn get_settings_page(main_window: Rc<MainWindow>, editor_ref: Rc<RefCell<Editor>>) -> BufferSettingsPage
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
    let s = editor.buf.title.to_string();
    title_entry.set_text(s.as_str());
    let row = ActionRow::builder()
        .title("Title")
        .build();
    row.add_suffix(&title_entry);
    group.add(&row);

    let group_entry = gtk4::Entry::new();
    group_entry.set_valign(Align::Center);
    group_entry.set_max_length(editor.buf.group.max_len() as i32);
    let s = editor.buf.group.to_string();
    group_entry.set_text(s.as_str());
    let row = ActionRow::builder()
        .title("Group")
        .build();
    row.add_suffix(&group_entry);
    group.add(&row);

    let author_entry = gtk4::Entry::new();
    author_entry.set_valign(Align::Center);
    author_entry.set_max_length(editor.buf.author.max_len() as i32);
    let s = editor.buf.author.to_string();
    author_entry.set_text(s.as_str());
    let row = ActionRow::builder()
        .title("Author")
        .build();
    row.add_suffix(&author_entry);
    group.add(&row);
    let list_model = gtk4::StringList::new(&[]);
    if editor.buf.font.font_type() == BitFontType::Custom {
        list_model.append("(Custom)");
    }

    for font in BitFont::get_font_list() {
        list_model.append(font);
    }
    // TODO: Filter with substring.
    let item_string_x = PropertyExpression::new(
        StringObject::static_type(), 
        gtk4::Expression::NONE, 
        "string");

    let font_dropdown = gtk4::DropDown::new(Some(&list_model), Some(item_string_x));
    font_dropdown.set_enable_search(true);
    font_dropdown.set_valign(Align::Center);
    let font_list = BitFont::get_font_list();

    if editor.buf.font.font_type() == BitFontType::Custom {
        font_dropdown.set_selected(0);
    } else {
        #[allow(clippy::needless_range_loop)]
        for i in 0..font_list.len() {
            if font_list[i] == editor.buf.font.name.to_string() {
                font_dropdown.set_selected(i as u32);
            }
        }
    }
    let import_font_button = gtk4::Button::builder()
    .valign(Align::Center)
    .icon_name("document-open-symbolic")
    .build();
    import_font_button.style_context().add_class("flat");

    let row = ActionRow::builder()
        .title("Font")
        .build();
    row.add_suffix(&font_dropdown);
    row.add_suffix(&import_font_button);
    
    let export_font_button = gtk4::Button::builder()
        .valign(Align::Center)
        .icon_name("document-save-symbolic")
        .build();
    export_font_button.style_context().add_class("flat");
    
    if editor.buf.font.font_type() == BitFontType::Custom {
        row.add_suffix(&export_font_button);
    }
    
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

    let width_spin_button = SpinButton::with_range(0.0, 10000.0, 10.0);
    width_spin_button.set_valign(Align::Center);
    width_spin_button.set_value(editor.buf.width as f64);
    let row = ActionRow::builder()
        .title("Width")
        .build();
    row.add_suffix(&width_spin_button);
    group.add(&row);

    let height_spin_button = SpinButton::with_range(0.0, 10000.0, 10.0);
    height_spin_button.set_valign(Align::Center);
    height_spin_button.set_value(editor.buf.height as f64);
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

    let open_ref_image_button = gtk4::Button::builder()
        .valign(Align::Center)
        .child(&open_button_content)
        .build();
    let clear_ref_image_button = gtk4::Button::builder()
        .valign(Align::Center)
        .icon_name("user-trash-symbolic")
        .build();
    clear_ref_image_button.style_context().add_class("flat");

    let row = ActionRow::builder()
        .title("Reference image")
        .build();
    row.add_suffix(&open_ref_image_button);
    row.add_suffix(&clear_ref_image_button);
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
    clear_ref_image_button.connect_clicked(clone!(@weak bg_image_file_name => move |_| {
        editor_ref2.borrow_mut().reference_image = None;
        update_reference_file_name(&bg_image_file_name, &editor_ref2.borrow());
    }));

    let editor_ref = editor_ref.clone();
    let editor_ref2 = editor_ref.clone();
    let dialog = &main_window.window;
    open_ref_image_button.connect_clicked(clone!(@strong dialog, @weak bg_image_file_name => move |_| {
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
    let font_dropdown = Rc::new(font_dropdown);
    let font_dropdown2 = font_dropdown.clone();
    import_font_button.connect_clicked(clone!(@strong dialog, @strong main_window, => move |_| {
        let file_chooser = gtk4::FileChooserDialog::builder()
            .title("Import font")
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
        file_chooser.connect_response(clone!( @strong main_window, @strong list_model, @strong font_dropdown2 =>move |d, response| {
            if response == ResponseType::Ok {
                let file = d.file().expect("Couldn't get file");
                let file_name = file.path().expect("Couldn't get file path");
                let font = BitFont::import(&file_name);
                d.close();

                if font.is_err() {
                    main_window.handle_error(font, || "Error while importing font…".to_string());
                    return;
                }

                list_model.append(&font.unwrap());
                font_dropdown2.set_selected((BitFont::get_font_list().len() - 1) as u32);
            } else {
                d.close();
            }
        }));
        file_chooser.present();
    }));


    let custom_font = editor.buf.font.clone();
    export_font_button.connect_clicked(clone!(@strong dialog, @strong main_window, => move |_| {
        let file_chooser = gtk4::FileChooserDialog::builder()
            .title("Export font")
            .action(FileChooserAction::Save)
            .transient_for(&dialog)
            .modal(true)
            .width_request(640)
            .height_request(480)
            .vexpand(false)
            .hexpand(false)
            .build();

        file_chooser.add_button("Export", ResponseType::Ok);
        file_chooser.add_button("Cancel", ResponseType::Cancel);
        file_chooser.connect_response(clone!(@strong main_window, @strong custom_font =>move |d, response| {
            if response == ResponseType::Ok {
                let file = d.file().expect("Couldn't get file");
                let file_name = file.path().expect("Couldn't get file path");
                d.close();

                let mut f = File::create(file_name).unwrap();
                let mut result = Vec::new();
                custom_font.push_u8_data(&mut result);
                f.write_all(&result).expect("can't write file");
            
                /* 
                if font.is_err() {
                    main_window.handle_error(font, || "Error while importing font…".to_string());
                    return;
                }*/

            } else {
                d.close();
            }
        }));
        file_chooser.present();
    }));

    let custom_font = if editor.buf.font.font_type() == BitFontType::Custom {
        Some(editor.buf.font.clone())
    } else {
        None
    };

    BufferSettingsPage {
        content_area,

        title_entry,
        group_entry,
        author_entry,
        comment_textview,
        font_dropdown,
        save_sauce_switcher,
        width_spin_button,
        height_spin_button,
        custom_font,
        editor: editor_ref2,
    }
}

fn update_reference_file_name(label: &gtk4::Label, editor: &std::cell::Ref<Editor>)  {
    if let Some(p) = &editor.reference_image {
        label.set_label(p.file_name().unwrap().to_str().unwrap());
    } else {
        label.set_label("(None)");
    }
}
