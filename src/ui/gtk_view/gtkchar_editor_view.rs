use gtk4::subclass::prelude::*;
use gtk4::traits::WidgetExt;
use gtk4::{gdk, glib};
use std::cell::{RefCell};
use std::rc::Rc;

use crate::model::Editor;

#[derive(Default)]

pub struct GtkCharEditorView {
    pub editor: RefCell<Rc<Editor>>
}

impl GtkCharEditorView {
}

#[glib::object_subclass]
impl ObjectSubclass for GtkCharEditorView {
    const NAME: &'static str = "GtkCharEditorView";
    type Type = super::CharEditorView;
    type ParentType = gtk4::DrawingArea;
    type Interfaces = (gdk::Paintable,);
}

impl ObjectImpl for GtkCharEditorView {
    fn constructed(&self, obj: &Self::Type) {
        obj.set_can_focus(true);
        obj.set_focusable(true);
        obj.set_focus_on_click(true);
    }
}

impl WidgetImpl for GtkCharEditorView {
      fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);
      }
}


impl DrawingAreaImpl for GtkCharEditorView {}

impl PaintableImpl for GtkCharEditorView {
    /*
    fn flags(&self, _paintable: &Self::Type) -> gdk::PaintableFlags {
        // Fixed size
        gdk::PaintableFlags::CONTENTS
    }

    fn intrinsic_width(&self, _paintable: &Self::Type) -> i32 {
        let borrow = self.editor.borrow();
        let font_dimensions = borrow.buf.get_font_dimensions();
        borrow.buf.width as i32 * font_dimensions.x
    }

    fn intrinsic_height(&self, _paintable: &Self::Type) -> i32 {
        let borrow = self.editor.borrow();
        let font_dimensions = borrow.buf.get_font_dimensions();
        borrow.buf.height as i32 * font_dimensions.y
    }*/

    fn snapshot(&self, _paintable: &Self::Type, _snapshot: &gdk::Snapshot, _width: f64, _height: f64) {
       /* println!("get snapshot!");
        let snapshot = snapshot.downcast_ref::<gtk4::Snapshot>().unwrap();
        snapshot.append_linear_gradient(
            &graphene::Rect::new(0_f32, 0_f32, width as f32, height as f32),
            &graphene::Point::new(0f32, 0f32),
            &graphene::Point::new(width as f32, height as f32),
            &[
                gsk::ColorStop::new(0.0, gdk::RGBA::RED),
                gsk::ColorStop::new(0.15, gdk::RGBA::new(1.0, 127_f32 / 255_f32, 0.0, 1.0)),
                gsk::ColorStop::new(0.3, gdk::RGBA::new(1.0, 1.0, 0.0, 1.0)),
                gsk::ColorStop::new(0.45, gdk::RGBA::GREEN),
                gsk::ColorStop::new(0.6, gdk::RGBA::BLUE),
                gsk::ColorStop::new(
                    0.75,
                    gdk::RGBA::new(75_f32 / 255_f32, 0.0, 130_f32 / 255_f32, 1.0),
                ),
                gsk::ColorStop::new(0.9, gdk::RGBA::new(143_f32 / 255_f32, 0.0, 1.0, 1.0)),
            ],
        );
        
        let editor = self.editor.borrow();
        let buffer = &editor.buf;

        gtk4::cairo::ImageSurface::create(gtk4::cairo::Format::ARgb32, 8, 16).unwrap();

        let font_dimensions = buffer.get_font_dimensions();
        let char_img =  Pixbuf::new(Colorspace::Rgb, false, 8, font_dimensions.x, font_dimensions.y).unwrap();

        for y in 0..buffer.height {
            for x in 0..buffer.width {
                let ch = buffer.get_char(Position::from(x as i32, y as i32));
                
                let bg = buffer.get_rgb(ch.attribute.get_background());
                let fg = buffer.get_rgb(ch.attribute.get_foreground());

                unsafe {
                    let ptr = char_img.pixels();
                    let mut i = 0;
                    for y in 0..font_dimensions.y {
                        let line = buffer.get_font_scanline(ch.char_code, y as usize);
                        for x in 0..font_dimensions.x {
                            if (line & (128 >> x)) != 0 {
                                ptr[i] = fg.2;
                                i += 1;
                                ptr[i] = fg.1;
                                i += 1;
                                ptr[i] = fg.0;
                                i += 1;
                            } else {
                                ptr[i] = bg.2;
                                i += 1;
                                ptr[i] = bg.1;
                                i += 1;
                                ptr[i] = bg.0;
                                i += 1;
                            }
                        }
                    }
                }
                let texture = gdk::Texture::for_pixbuf(&char_img);
                snapshot.append_texture(&texture, &graphene::Rect::new(
                    (x * 8) as f32, 
                    (y * 16) as f32, 
                    font_dimensions.x as f32, 
                    font_dimensions.y as f32))
            }
        }*/
    }
}
 