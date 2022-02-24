
use std::{ str::FromStr, rc::Rc, cell::RefCell, cmp::{max, min} };

use gtk4::{ glib, traits::{WidgetExt}, gdk::{Paintable, self}, prelude::{DrawingAreaExtManual, GdkCairoContextExt}, cairo::Operator};

use crate::{editor::Editor, model::Position};

use self::gtkchar_editor_view::GtkCharEditorView;

mod gtkchar_editor_view;


glib::wrapper! {
    pub struct CharEditorView(ObjectSubclass<GtkCharEditorView>) @extends gtk4::Widget, gtk4::DrawingArea, @implements Paintable;
}

impl Default for CharEditorView {
    fn default() -> Self {
         Self::new()
    }
}
struct Dialog {
    payload: Editor,
 }

impl CharEditorView {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a AnsiEditorArea")
    }

    pub fn set_editor(&self, mut editor: Editor)
    {
        let buffer = &editor.buf;
        let font_dimensions = buffer.get_font_dimensions();
        self.set_size_request(buffer.width as i32 * font_dimensions.x, buffer.height as i32 * font_dimensions.y);

        let rgba = gdk::RGBA::from_str("black").unwrap();
        let mut char_img =
        gtk4::cairo::ImageSurface::create(gtk4::cairo::Format::ARgb32, 8, 16).unwrap();
        let dialog = Dialog { payload: editor };

        let handle = Rc::new(RefCell::new(dialog));

        let gesture = gtk4::GestureClick::new();

        let handle1 = handle.clone();
        gesture.connect_pressed(glib::clone!(@strong self as this => move |_, _clicks, x, y| {
            let font_dimensions = handle1.borrow().payload.buf.get_font_dimensions();
            let x = min(handle1.borrow().payload.buf.width as i32, max(0, x as i32 / font_dimensions.x));
            let y = min(handle1.borrow().payload.buf.height as i32, max(0, y as i32 / font_dimensions.y));
            println!("goto {}, {}", x, y);
            handle1.borrow_mut().payload.cursor.pos = Position::from(x, y);
            this.queue_draw();
        }));
        self.add_controller(&gesture);



        let handle1 = handle.clone();
        let key = gtk4::EventControllerKey::new();
        key.connect_key_pressed(glib::clone!(@strong self as this => move |_, key, key_code, modifier| {
            {
                handle1.borrow_mut().payload.handle_key(key, key_code, modifier);
                this.queue_draw();
            }
            glib::signal::Inhibit(true)
        }));
        self.add_controller(&key);


        let handle1 = handle.clone();
        self.set_draw_func(move |_, cr, _width, _height| {
            GdkCairoContextExt::set_source_rgba(cr, &rgba);
            cr.paint().expect("Invalid cairo surface state");

            let cursor_pos = handle1.borrow().payload.cursor.pos;

            let buffer =&handle1.borrow_mut().payload.buf;
            let font_dimensions = buffer.get_font_dimensions();
    
            for y in 0..buffer.height {
                for x in 0..buffer.width {
                    let ch = buffer.get_char(Position::from(x as i32, y as i32));
                    
                    cr.rectangle(
                        x as f64 * font_dimensions.x as f64, 
                        y as f64 * font_dimensions.y as f64, 
                        font_dimensions.x as f64,
                        font_dimensions.y as f64);
                    let bg = buffer.get_rgb_f64(ch.attribute.get_background());
                    cr.set_source_rgba(bg.0, bg.1, bg.2, 1f64);
                    cr.fill().expect("error while calling fill.");
                    
                    let fg = buffer.get_rgb(ch.attribute.get_foreground());
                    unsafe {
                        let mut data = char_img.data().expect("Can't lock image");
                        let ptr = data.as_mut_ptr();
                        render_char(buffer, ch.char_code, ptr, fg);
                    }
                    cr.set_source_surface(&char_img, (x as i32 * font_dimensions.x) as f64, (y as i32 * font_dimensions.y) as f64)
                        .expect("error while calling fill.");
                        cr.paint()
                        .expect("error while calling fill.");
                }
            }

            let x = cursor_pos.x;
            let y = cursor_pos.y;

            cr.rectangle((x as i32 * font_dimensions.x) as f64, (y as i32 * font_dimensions.y) as f64, font_dimensions.x as f64, font_dimensions.y as f64);
            cr.set_source_rgb(0x7F as f64 / 255.0, 0x7F as f64 / 255.0, 0x7F as f64 / 255.0);
            cr.set_operator(Operator::Difference);
            cr.fill().expect("error while calling fill.");
        });
            
    }

}

unsafe fn render_char(buffer: &crate::model::Buffer, ch: u8, ptr: *mut u8, fg: (u8, u8, u8)) {
    let font_dimensions = buffer.get_font_dimensions();
    let mut i = 0;
    for y in 0..font_dimensions.y {
        let line = buffer.get_font_scanline(ch, y as usize);
        for x in 0..font_dimensions.x {
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