use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::str::FromStr;

use glib::{ParamSpec, Value, ToValue};
use gtk4::prelude::DrawingAreaExtManual;
use gtk4::subclass::prelude::*;
use gtk4::{glib, gdk};
use gtk4::{traits::{WidgetExt, ButtonExt}};
use gtk4::{
    prelude::{ GdkCairoContextExt},
};

use crate::WORKSPACE;
use crate::ui::MainWindow;

use super::char_selector_dialog::display_char_selector_dialog;

#[derive(Default)]

pub struct GtkCharButton {
    pub main_window: Rc<RefCell<Option<MainWindow>>>,
    pub char_code: Cell<u8>,
}

impl GtkCharButton {}

#[glib::object_subclass]
impl ObjectSubclass for GtkCharButton {
    const NAME: &'static str = "GtkCharButton";
    type Type = super::CharButton;
    type ParentType = gtk4::Button;
}

impl ObjectImpl for GtkCharButton {
    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecUChar::new(
                    "charcode",
                    "CharCode",
                    "CharCode",
                    u8::MIN,
                    u8::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                ),
            ]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "charcode" => {
                self.char_code.set(value.get().unwrap());
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "charcode" => self.char_code.get().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for GtkCharButton {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);
        widget.set_size_request(32, 32);
        widget.set_hexpand(false);
        widget.set_vexpand(false);

        let drawing_area = gtk4::DrawingArea::builder()
            .content_height(24)
            .content_width(24)
            .build();
        let font_dimensions = unsafe { WORKSPACE.get_font_dimensions() };
        let mut char_img =
        gtk4::cairo::ImageSurface::create(gtk4::cairo::Format::ARgb32, font_dimensions.width as i32, font_dimensions.height as i32).unwrap();
        let background_rgba = gdk::RGBA::from_str("black").unwrap();

        drawing_area.set_draw_func(glib::clone!(@weak widget => move |_, cr, width, height| {
            GdkCairoContextExt::set_source_rgba(cr, &background_rgba);
            cr.paint().expect("Invalid cairo surface state");

            unsafe {
                let mut data = char_img.data().expect("Can't lock image");
                let ptr = data.as_mut_ptr();

                render_char(widget.imp().char_code.get(), ptr, (255, 255, 255));
            }

            cr.translate(width as f64  / 2.0, height as f64 / 2.0);
            cr.scale(1.8, 1.8);
            cr.set_source_surface(
                &char_img,
                -char_img.width() as f64 / 2.0,
                -char_img.height() as f64 / 2.0,
            ).expect("error while calling fill.");

            cr.paint().expect("error while calling fill.");

        }));
        widget.set_child(Some(&drawing_area));

        widget.connect_clicked(move |b| {
            b.imp().char_code.set(b'A');
            display_char_selector_dialog();
            drawing_area.queue_draw();
        });
    }
}

impl ButtonImpl for GtkCharButton {}

unsafe fn render_char(ch: u8, ptr: *mut u8, fg: (u8, u8, u8)) {
    let font_dimensions = WORKSPACE.get_font_dimensions();
    let mut i = 0;
    for y in 0..font_dimensions.height {
        let line = WORKSPACE.get_font_scanline(ch, y as usize);
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
