use gtk4::prelude::{DrawingAreaExtManual, GdkCairoContextExt};
use gtk4::subclass::prelude::*;
use gtk4::traits::WidgetExt;
use gtk4::{gdk, glib};
use std::cell::Cell;
use std::str::FromStr;

use crate::model::Position;

#[derive(Default)]
pub struct GtkCharEditorView {
    pub buf: Cell<Option<usize>>,
}

impl GtkCharEditorView {
    pub fn set_editor(&self, editor_id: usize) {
        self.buf.set(Some(editor_id));
    }
}

#[glib::object_subclass]
impl ObjectSubclass for GtkCharEditorView {
    const NAME: &'static str = "GtkCharEditorView";
    type Type = super::CharEditorView;
    type ParentType = gtk4::DrawingArea;
}

impl ObjectImpl for GtkCharEditorView {
    fn constructed(&self, obj: &Self::Type) {
        obj.set_can_focus(true);
        obj.set_focusable(true);
        obj.set_focus_on_click(true);
        /*
        let gesture = gtk4::GestureClick::new();
        // Trigger a transition on click
        gesture.connect_pressed(glib::clone!(@strong obj as this => move |_, clicks, x, y| {
            println!("gesture click {}, {}, {}", clicks ,x, y);
        }));
        obj.add_controller(&gesture);
        let id = self.buf.get();
        let key = gtk4::EventControllerKey::new();
        key.connect_key_pressed(glib::clone!(@strong obj as this => move |_, key, key_code, modifier| {
            {
                crate::Workspace::get_editor(id).handle_key(key, key_code, modifier);
            }
            glib::signal::Inhibit(true)
        }));
        obj.add_controller(&key);*/
    }
}

impl WidgetImpl for GtkCharEditorView {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);

        let rgba = gdk::RGBA::from_str("black").unwrap();
        println!("1");
        let id = self.buf.get();
        if id.is_none() {
            return;
        }
        let editor = crate::Workspace::get_editor(id.unwrap());
        let buffer = &editor.buf;

        let mut char_img =
        gtk4::cairo::ImageSurface::create(gtk4::cairo::Format::ARgb32, 8, 16).unwrap();

        widget.set_draw_func(move |_, cr, _width, _height| {
            GdkCairoContextExt::set_source_rgba(cr, &rgba);
            cr.paint().expect("Invalid cairo surface state");
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
                        let mut data = char_img.data().unwrap();
                        let ptr = data.as_mut_ptr();
                        let mut i = 0;
                        for y in 0..font_dimensions.y {
                            let line = buffer.get_font_scanline(ch.char_code, y as usize);
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
                    cr.set_source_surface(&char_img, (x * 8) as f64, (y * 16) as f64)
                        .expect("error while calling fill.");
                    cr.paint()
                        .expect("error while calling fill.");
                }
            }
        });
    }
}

impl DrawingAreaImpl for GtkCharEditorView {}
