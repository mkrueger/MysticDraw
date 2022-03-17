use std::{rc::Rc, cell::{RefCell, Ref}};

use gtk4::{ traits::{ WidgetExt, BoxExt, GtkWindowExt, GestureSingleExt, ButtonExt}, Orientation};
use libadwaita::{ HeaderBar };

use crate::{ui::{AnsiView, MainWindow}, model::{Buffer, Editor, Position, DosChar, TextAttribute, Rectangle, BitFont}};

pub struct CharSelectorDialog {
    pub dialog: libadwaita::Window,
    pub open_button: gtk4::Button,
    pub char_code: Rc<RefCell<u16>>,

    // storing that here is hack to bypass some insane gtk related limitations
    pub result_char: Rc<RefCell<u16>>,
}

const CHARS_PER_LINE : u16 = 32;

pub fn display_select_char_dialog(parent: &libadwaita::ApplicationWindow, main_window: Ref<Rc<MainWindow>>, char_code: Rc<RefCell<u16>>, result_char: Rc<RefCell<u16>>) -> CharSelectorDialog
{
    if let Some(editor) = main_window.get_current_editor() {
        display_select_char_dialog2(parent, &editor.borrow().buf.font, &editor.borrow().buf.extended_font, char_code, result_char)
    } else {
        display_select_char_dialog2(parent, &BitFont::default(), &None, char_code, result_char)
    }
}

pub fn display_select_char_dialog2(parent: &libadwaita::ApplicationWindow, font: &BitFont, ext_font: &Option<BitFont>, char_code: Rc<RefCell<u16>>, result_char: Rc<RefCell<u16>>) -> CharSelectorDialog
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
    let char_label = gtk4::Label::new(None);

    let code  = *char_code.borrow();

    let stack = gtk4::Stack::new();
    let charset_view = create_char_table(font, 0, char_code.clone(), &char_label);
    stack.add_child(&charset_view);

    if let Some(ext_font) = &ext_font {
        let charset_view = create_char_table(ext_font, 1, char_code.clone(), &char_label);
        stack.add_child(&charset_view);
        if code >> 8 == 1 {
            stack.set_visible_child(&stack.last_child().unwrap());
        }
    }
    content_area.append(&stack);
    content_area.append(&char_label);

    if ext_font.is_some() {
        let switch_button = gtk4::Button::builder().label("Switch fonts").build();
        content_area.append(&switch_button);

        switch_button.connect_clicked(move |_| {
            if stack.visible_child() == stack.first_child() {
                stack.set_visible_child(&stack.last_child().unwrap());
            } else {
                stack.set_visible_child(&stack.first_child().unwrap());
            }
        });
    }

    main_area.append(&content_area);
    dialog.show();

    CharSelectorDialog {
        dialog,
        open_button,
        char_code,
        result_char
    }
}

fn create_char_table(font: &BitFont, font_number: u16, char_code: Rc<RefCell<u16>>, char_label: &gtk4::Label) -> AnsiView
{
    let mut buffer = Buffer::new();
    buffer.font = font.clone();
    buffer.width = CHARS_PER_LINE;
    buffer.height = 256 / CHARS_PER_LINE;

    for y in 0..buffer.height {
        for x in 0..buffer.width {
            buffer.set_char(0, Position::from(x as i32, y as i32), Some(DosChar {
                char_code: (y * CHARS_PER_LINE + x) as u16,
                attribute: TextAttribute::DEFAULT
            }));
        }
    }

    let mut editor = Editor::new(0, buffer);
    editor.is_inactive = true;
    let key_handle = Rc::new(RefCell::new(editor));

    let charset_view = AnsiView::new();
    charset_view.set_mimap_mode(true);
    charset_view.set_width_request((CHARS_PER_LINE * font.size.width as u16 * 2) as i32);
    charset_view.set_height_request((256 / CHARS_PER_LINE * font.size.height as u16 * 2) as i32);
    
    charset_view.set_valign(gtk4::Align::Center);
    charset_view.set_editor_handle(key_handle);

    let gesture = gtk4::GestureClick::new();
    gesture.set_button(1);

    let font_width  = font.size.width as u16;
    let font_height = font.size.height as u16;

    let code  = *char_code.borrow();
    if code >> 8 == font_number {
        set_selected_char(&charset_view, char_label, code & 0xFF);
    }

    gesture.connect_pressed(glib::clone!(@strong charset_view as this, @weak char_label => move |_, _clicks, x, y| {
        let x = (x / 2.0) as u16;
        let y = (y / 2.0) as u16;

        let my_char = x / font_width + CHARS_PER_LINE * (y / font_height);
        set_selected_char(&this, &char_label, my_char);
        char_code.replace(my_char | (font_number << 8));
        this.queue_draw();
        this.grab_focus();
    }));
    charset_view.add_controller(&gesture);

    charset_view
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
