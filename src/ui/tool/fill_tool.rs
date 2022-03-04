use std::{rc::Rc, cell::RefCell};

use gtk4::{traits::{BoxExt, CheckButtonExt}, CheckButton};

use crate::{model::{FILL_TOOL, Buffer, Editor, Position, DosChar, TextAttribute}, ui::AnsiView};

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

pub unsafe fn get_preview_char() -> DosChar
{
    let mut color = TextAttribute::DEFAULT;
    
    if FILL_TOOL.use_back {
        color.set_background_ice(FILL_TOOL.attr.get_background_ice());
    }

    if FILL_TOOL.use_fore {
        color.set_foreground(FILL_TOOL.attr.get_foreground());
    }
    
    crate::model::DosChar {
        char_code: FILL_TOOL.char_code, 
        attribute: color 
    }
}

pub fn add_fill_tool_page(content_box: &mut gtk4::Box)
{
    unsafe {
        let mut fg_button = CheckButton::builder()
            .label("Foreground")
            .active(FILL_TOOL.use_fore)
            .build();
        content_box.append(&fg_button);

        let bg_button = CheckButton::builder()
            .label("Background")
            .active(FILL_TOOL.use_back)
            .build();
        content_box.append(&bg_button);
    
        let char_button = CheckButton::builder()
            .label("Character")
            .active(FILL_TOOL.use_char)
            .build();
        content_box.append(&char_button);

        let (ansi_view, editor) = create_char_view();
        content_box.append(&ansi_view);

        editor.borrow_mut().buf.set_char(0, Position::new(), get_preview_char());
        let editor2 = editor.clone();
        fg_button.connect_toggled(move |x| {
            FILL_TOOL.use_fore = x.is_active();
            editor2.borrow_mut().buf.set_char(0, Position::new(), get_preview_char());
        });

        let editor2 = editor;
        bg_button.connect_toggled(move |x| {
            FILL_TOOL.use_back = x.is_active();
            editor2.borrow_mut().buf.set_char(0, Position::new(), get_preview_char());
        });

        char_button.connect_toggled(|x| {
            FILL_TOOL.use_char = x.is_active();
        });
    }
}
