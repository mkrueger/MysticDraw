use std::{rc::Rc, cell::RefCell};

use gtk4::{ traits::{ WidgetExt, BoxExt, GtkWindowExt, GestureSingleExt}, Orientation};
use libadwaita::{ HeaderBar };

use crate::{ui::AnsiView, model::{Buffer, Editor, Position, DosChar, TextAttribute, Rectangle}};

pub struct CharSelectorDialog {
    pub dialog: libadwaita::Window,
    pub open_button: gtk4::Button,
    pub char_code: Rc<RefCell<u16>>,

    // storing that here is hack to bypass some insane gtk related limitations
    pub result_char: Rc<RefCell<u16>>,
}

const CHARS_PER_LINE : u16 = 32;

pub fn display_select_char_dialog(parent: &libadwaita::ApplicationWindow, font_size: crate::model::Size, char_code: Rc<RefCell<u16>>, result_char: Rc<RefCell<u16>>) -> CharSelectorDialog
{
    let main_area = gtk4::Box::builder()
    .orientation(Orientation::Vertical)
    .build();
    let dialog = libadwaita::Window::builder()
        .default_width(280)
        .default_height(240)
        .modal(true)
        .resizable(false)
        .content(&main_area)
        .build();
    dialog.set_transient_for(Some(parent));
    let hb = HeaderBar::builder()
        .title_widget(&libadwaita::WindowTitle::builder().title("Select character").build())
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

    let mut key_preview_buf = Buffer::new();
    key_preview_buf.width = CHARS_PER_LINE;
    key_preview_buf.height = 256 / CHARS_PER_LINE;

    println!("{}x{}", key_preview_buf.width, key_preview_buf.height);

    for y in 0..key_preview_buf.height {
        for x in 0..key_preview_buf.width {
            key_preview_buf.set_char(0, Position::from(x as i32, y as i32), Some(DosChar {
                char_code: (y * CHARS_PER_LINE + x) as u8,
                attribute: TextAttribute::DEFAULT
            }));
        }
    }

    let mut key_preview_editor = Editor::new(0, key_preview_buf);
    key_preview_editor.is_inactive = true;
    let key_handle = Rc::new(RefCell::new(key_preview_editor));


    let key_set_view = AnsiView::new();
    key_set_view.set_mimap_mode(true);
    key_set_view.set_width_request((CHARS_PER_LINE * font_size.width as u16 * 2) as i32);
    key_set_view.set_height_request((256 / CHARS_PER_LINE * font_size.height as u16 * 2) as i32);
    
    key_set_view.set_valign(gtk4::Align::Center);
    key_set_view.set_editor_handle(key_handle);
    content_area.append(&key_set_view);

    let char_label = gtk4::Label::new(None);

    content_area.append(&char_label);

    let gesture = gtk4::GestureClick::new();
    gesture.set_button(1);
    let code = char_code.clone();

    let font_width  = font_size.width as u16;
    let font_height = font_size.height as u16;
    set_selected_char(&key_set_view, &char_label, *char_code.borrow());
    gesture.connect_pressed(glib::clone!(@strong key_set_view as this, @weak char_label => move |_, _clicks, x, y| {
        let x = (x / 2.0) as u16;
        let y = (y / 2.0) as u16;

        let my_char = x / font_width + CHARS_PER_LINE * (y / font_height);
        set_selected_char(&this, &char_label, my_char);
        code.replace(my_char);
        this.queue_draw();
        this.grab_focus();
    }));
    key_set_view.add_controller(&gesture);


    main_area.append(&content_area);
    dialog.show();

    CharSelectorDialog {
        dialog,
        open_button,
        char_code,
        result_char
    }
}

fn set_selected_char(view: &AnsiView, label: &gtk4::Label, char_code: u16)
{
    view.set_preview_rectangle(Some(
        Rectangle::from(
            (char_code % CHARS_PER_LINE) as i32, 
            (char_code / CHARS_PER_LINE) as i32, 
            1, 
            1
        )
    ));
    label.set_text(format!("Char: {}, (0x{0:>04X})", char_code).as_str());
}
