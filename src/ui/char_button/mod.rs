
use gtk4::traits::GtkWindowExt;
use gtk4::{
     traits::ButtonExt,
};

use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use gtk4::prelude::DrawingAreaExtManual;
use gtk4::{gdk};
use gtk4::{traits::{WidgetExt}};
use gtk4::{
    prelude::{ GdkCairoContextExt},
};

use crate::WORKSPACE;
use crate::ui::MainWindow;

mod char_selector_dialog;

pub struct CharButton {
    pub button: gtk4::Button,
    pub char_code: Rc<RefCell<u16>>
}

pub fn create_char_button(main_window: &MainWindow, char_code: u16, callback: Box<&'static dyn Fn(u16)>) -> CharButton
{
    let res = CharButton {
        button: gtk4::Button::builder()
        .hexpand(false)
        .vexpand(false)
        .width_request(32)
        .height_request(32)
        .build(),
        char_code: Rc::new(RefCell::new(char_code))
    };

    let font_size = if let Some(editor) = main_window.get_current_editor()  {
        editor.borrow().buf.get_font_dimensions()
    } else { 
        crate::model::Size::DEFAULT
    };

    let drawing_area = gtk4::DrawingArea::builder()
        .content_height(24)
        .content_width(24)
        .build();
    let mut char_img = gtk4::cairo::ImageSurface::create(gtk4::cairo::Format::ARgb32, font_size.width as i32, font_size.height as i32).unwrap();
    let background_rgba = gdk::RGBA::from_str("black").unwrap();
    let ch = res.char_code.clone();
    drawing_area.set_draw_func( move |_, cr, width, height| {
        GdkCairoContextExt::set_source_rgba(cr, &background_rgba);
        cr.paint().expect("Invalid cairo surface state");

        unsafe {
            let mut data = char_img.data().expect("Can't lock image");
            let ptr = data.as_mut_ptr();
            render_char(*ch.borrow(), ptr, (255, 255, 255));
        }

        cr.translate(width as f64  / 2.0, height as f64 / 2.0);
        cr.scale(1.8, 1.8);
        cr.set_source_surface(
            &char_img,
            -char_img.width() as f64 / 2.0,
            -char_img.height() as f64 / 2.0,
        ).expect("error while calling fill.");

        cr.paint().expect("error while calling fill.");
    });

    res.button.set_child(Some(&drawing_area));
    let dialog = &main_window.window;
    let dialog_char = Rc::new(RefCell::new(char_code));
    let result_char = res.char_code.clone();
   
    res.button.connect_clicked(glib::clone!(@weak dialog, @strong callback => move |_| {
        let char_sel_dialog = char_selector_dialog::display_select_char_dialog(&dialog, font_size, dialog_char.clone(), result_char.clone());

        char_sel_dialog.open_button.connect_clicked(glib::clone!(@weak dialog, @weak drawing_area, @strong callback => move |_| {
            char_sel_dialog.dialog.close();
            let b = *char_sel_dialog.char_code.borrow();
            char_sel_dialog.result_char.replace(b);
            (callback)(b);
            drawing_area.queue_draw();
        }));

    }));

    res
}

unsafe fn render_char(ch: u16, ptr: *mut u8, fg: (u8, u8, u8)) {
    let font_dimensions = WORKSPACE.get_font_dimensions();
    let mut i = 0;
    for y in 0..font_dimensions.height {
        let line = WORKSPACE.get_font_scanline(ch , y as usize);
        for x in 0..font_dimensions.width {
            if (line & (128 >> x)) != 0 {
                *ptr.add(i) = fg.2;
                i += 1;
                *ptr.add(i) = fg.1;
                i += 1;
                *ptr.add(i) = fg.0;
                i += 1;
                *ptr.add(i) = 255;
                i += 1;
            } else {
                *ptr.add(i) = 0;
                i += 1;
                *ptr.add(i) = 0;
                i += 1;
                *ptr.add(i) = 0;
                i += 1;
                *ptr.add(i) = 0;
                i += 1;
            }
        }
    }
}
