/*use std::{cell::RefCell, rc::Rc};

use gtk4::{traits::{BoxExt, WidgetExt, GestureSingleExt}, Align };
use crate::{model::{Buffer, BitFont}, ui::{MainWindow, minimap::MinimapAnsiView}};
use crate::{ui::{AnsiView,}, model::{ Editor, Position, DosChar, TextAttribute}};


const CHARS_PER_LINE : u16 = 12;*/

pub fn add_click_tool_page(_main_window: std::rc::Rc<crate::ui::MainWindow>, _content_box: &mut gtk4::Box)
{/* 
    content_box.set_margin_top(20);
    content_box.set_margin_start(20);
    content_box.set_margin_end(20);
    content_box.set_margin_bottom(20);
    content_box.set_spacing(20);*/
    
/* 
    let font = BitFont::default();
    
    let mut key_preview_buf = Buffer::new();
    key_preview_buf.font = font.clone();
    key_preview_buf.width = CHARS_PER_LINE;
    key_preview_buf.height = 256 / CHARS_PER_LINE;

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

    let key_set_view = MinimapAnsiView::new();
    key_set_view.set_valign(Align::Start);
//    key_set_view.set_width_request((CHARS_PER_LINE * font.size.width as u16 ) as i32);
//key_set_view.set_height_request((256 / CHARS_PER_LINE * font.size.height as u16 ) as i32);
    key_set_view.set_height_request(100);
   key_set_view.set_halign(gtk4::Align::Center);
   key_set_view.set_editor_handle(key_handle);
   content_box.append(&key_set_view);
   let char_label = gtk4::Label::new(None);


    content_box.append(&char_label);

    let gesture = gtk4::GestureClick::new();
    gesture.set_button(1);
    //let code = char_code.clone();

    let font_width  = font.size.width as u16;
    let font_height = font.size.height as u16;
  //  set_selected_char(&key_set_view, &char_label, *char_code.borrow());
    gesture.connect_pressed(glib::clone!(@strong key_set_view as this, @weak char_label => move |_, _clicks, x, y| {
        let x = (x / 2.0) as u16;
        let y = (y / 2.0) as u16;

        let my_char = x / font_width + CHARS_PER_LINE * (y / font_height);
   //     set_selected_char(&this, &char_label, my_char);
    //    code.replace(my_char);
        this.queue_draw();
        this.grab_focus();
    }));
    key_set_view.add_controller(&gesture);*/

}
