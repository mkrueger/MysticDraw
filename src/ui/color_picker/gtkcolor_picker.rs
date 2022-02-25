use glib::{ParamSpec, ParamFlags, Value, ParamSpecUChar};
use gtk4::prelude::DrawingAreaExtManual;
use gtk4::subclass::prelude::*;
use gtk4::traits::WidgetExt;
use gtk4::{glib};
use once_cell::sync::Lazy;
use std::cell::{ Cell };

use crate::model::{ DOS_DEFAULT_PALETTE };

#[derive(Default)]

pub struct GtkColorPicker {
    pub color: Cell<u8>
}

impl GtkColorPicker {
}

#[glib::object_subclass]
impl ObjectSubclass for GtkColorPicker {
    const NAME: &'static str = "GtkColorPicker";
    type Type = super::ColorPicker;
    type ParentType = gtk4::DrawingArea;
}

impl ObjectImpl for GtkColorPicker {

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecUChar::new(
                // Name
                "color",
                // Nickname
                "color",
                // Short description
                "color",
                // Minimum value
                0,
                // Maximum value
                255,
                // Default value
                7,
                // The property can be read and written to
                ParamFlags::READWRITE,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "color" => {
                let input_number = value.get().expect("The value needs to be of type `i32`.");
                self.color.set(input_number);
            }
            _ => unimplemented!(),
        }
    }
    
    fn constructed(&self, obj: &Self::Type) {
        obj.set_can_focus(true);
        obj.set_focusable(true);
        obj.set_focus_on_click(true);
        obj.set_size_request(200, 50);
            
        let gesture = gtk4::GestureClick::new();
        gesture.connect_pressed(glib::clone!(@strong self as this => move |_, _clicks, x, y| {
     /*      let color = (x as i32 / 200 / 8 + 8 * (y as i32 / 50 / 2)) as u8;
            obj.set_property("color", color.to_value());*/
        }));

        obj.add_controller(&gesture);

        obj.set_draw_func(move | _, cr, width, height| {
            for y in 0..2 {
                for x in 0..8 {
                    cr.rectangle(
                        (x * (width / 8)) as f64, 
                        (y * height / 2) as f64, 
                        (width / 8) as f64, 
                        (height / 2) as f64);
                    let color = DOS_DEFAULT_PALETTE[(x + y * 8) as usize];

                    cr.set_source_rgb((color.0 as f64) / 255.0, 
                    (color.1 as f64) / 255.0,
                    (color.2 as f64) / 255.0);

                    cr.fill().expect("error while calling fill");
                }
            }
        });
    }
}

impl WidgetImpl for GtkColorPicker {
      fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);
      }
}

impl DrawingAreaImpl for GtkColorPicker {}
