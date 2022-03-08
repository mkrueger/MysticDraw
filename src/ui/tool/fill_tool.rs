use std::{rc::Rc, cell::RefCell};

use gtk4::{traits::{BoxExt, CheckButtonExt, WidgetExt, StyleContextExt, ToggleButtonExt}, CheckButton, ToggleButton, Orientation, Align};

use crate::{model::{FILL_TOOL, Buffer, Editor, DosChar, TextAttribute}, ui::{AnsiView}};

pub fn create_char_view() -> (AnsiView, Rc<RefCell<Editor>>)
{
    let mut buf = Buffer::new();
    buf.width = 1;
    buf.height = 1;
    let mut editor = Editor::new(0, buf);
    editor.is_inactive = true;
    
    let editor_handle = Rc::new(RefCell::new(editor));
    let ansi_view = AnsiView::new();
    ansi_view.set_editor_handle(editor_handle.clone());
    
    (ansi_view, editor_handle)
}

pub fn get_preview_char() -> DosChar
{
    unsafe {
        let mut color = TextAttribute::DEFAULT;
        
        if FILL_TOOL.use_back {
            color.set_background(FILL_TOOL.attr.get_background());
        }

        if FILL_TOOL.use_fore {
            color.set_foreground(FILL_TOOL.attr.get_foreground());
        }
        
        crate::model::DosChar {
            char_code: FILL_TOOL.char_code, 
            attribute: color 
        }
    }
}

pub fn add_fill_tool_page(content_box: &mut gtk4::Box)
{
    unsafe {
        content_box.set_margin_top(20);
        content_box.set_margin_bottom(20);
        content_box.set_margin_start(20);
        content_box.set_spacing(20);
        
        let color_box = gtk4::Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(Align::Center)
            .build();
        color_box.style_context().add_class("linked");

        let fg_button = ToggleButton::builder()
            .label("Fg")
            .active(FILL_TOOL.use_fore)
            .build();
        color_box.append(&fg_button);

        let bg_button = ToggleButton::builder()
            .label("Bg")
            .active(FILL_TOOL.use_back)
            .build();
        color_box.append(&bg_button);

        content_box.append(&color_box);
        

        let char_container = gtk4::Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(Align::Start)
            .build();

        let char_checkbox = CheckButton::builder()
            .label("Character")
            .active(FILL_TOOL.use_char)
            .build();
        char_container.append(&char_checkbox);

        let button = crate::ui::CharButton::new(FILL_TOOL.char_code);
        char_container.append(&button);
        content_box.append(&char_container);

        //let (ansi_view, editor) = create_char_view();
        //content_box.append(&ansi_view);

        //editor.borrow_mut().buf.set_char(0, Position::new(), get_preview_char());


        //let editor2 = editor.clone();
        fg_button.connect_toggled(move |x| {
            FILL_TOOL.use_fore = x.is_active();
          //  editor2.borrow_mut().buf.set_char(0, Position::new(), get_preview_char());
        });

//        let editor2 = editor;
        bg_button.connect_toggled(move |x| {
            FILL_TOOL.use_back = x.is_active();
  //          editor2.borrow_mut().buf.set_char(0, Position::new(), get_preview_char());
        });

        char_checkbox.connect_toggled(|x| {
            FILL_TOOL.use_char = x.is_active();
        });
    }
}
