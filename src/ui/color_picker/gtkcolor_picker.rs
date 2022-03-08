use std::cell::RefCell;
use std::rc::Rc;

use gtk4::cairo::Operator;
use gtk4::glib;
use gtk4::prelude::DrawingAreaExtManual;
use gtk4::subclass::prelude::*;
use gtk4::traits::{GestureSingleExt, WidgetExt};

use crate::model::{Editor};
use crate::WORKSPACE;

#[derive(Default)]

pub struct GtkColorPicker {
    pub editor: RefCell<Rc<RefCell<Editor>>>,
}

impl GtkColorPicker {

    pub fn set_editor(&self, handle: &Rc<RefCell<Editor>>) {
        self.editor.replace(handle.clone());
    }
}

#[glib::object_subclass]
impl ObjectSubclass for GtkColorPicker {
    const NAME: &'static str = "GtkColorPicker";
    type Type = super::ColorPicker;
    type ParentType = gtk4::DrawingArea;
}

impl ObjectImpl for GtkColorPicker {
    fn constructed(&self, obj: &Self::Type) {
        obj.set_can_focus(true);
        obj.set_focusable(true);
        obj.set_focus_on_click(true);
        obj.set_size_request(200, 50);
        

        obj.set_draw_func(
            glib::clone!(@strong obj as this => move | _, cr, width, height| {
               cr.set_operator(Operator::Source);
                for y in 0..2 {
                    for x in 0..8 {
                        cr.rectangle(
                            (x * (width / 8)) as f64,
                            (y * height / 2) as f64,
                            (width / 8) as f64,
                            (height / 2) as f64);
                        let color = this.get_editor().borrow().buf.palette.colors[(x + y * 8) as usize].get_rgb();
                        cr.set_source_rgb((color.0 as f64) / 255.0,
                        (color.1 as f64) / 255.0,
                        (color.2 as f64) / 255.0);
                        cr.fill().expect("error while calling fill");
                    }
                }

                if width <= 0 || height <= 0 {
                    eprintln!("invalid size for the color picker.");
                    return;
                }

                unsafe {
                    cr.set_operator(Operator::Difference);
                    let marker_width = 6f64;
                    let x = (WORKSPACE.selected_attribute.get_foreground() % 8) as i32;
                    let y = (WORKSPACE.selected_attribute.get_foreground() / 8) as i32;

                    cr.rectangle(
                        (x * (width / 8)) as f64,
                        (y * (height / 2)) as f64,
                        marker_width,
                        marker_width);
                    cr.set_source_rgb(1.0, 1.0, 1.0);
                    cr.fill().expect("error while calling fill");

                    let x = (WORKSPACE.selected_attribute.get_background() % 8) as i32;
                    let y = (WORKSPACE.selected_attribute.get_background() / 8) as i32;
                    cr.rectangle(
                        ((1 + x) * width / 8) as f64 - marker_width,
                        ((1 + y) * height / 2) as f64 - marker_width,
                        marker_width,
                        marker_width);
                    cr.set_source_rgb(1.0, 1.0, 1.0);
                    cr.fill().expect("error while calling fill");
                }
            }),
        );
    }
}

impl WidgetImpl for GtkColorPicker {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);

        // TODO: Remove code duplication.
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(1);
        gesture.connect_pressed(
            glib::clone!(@strong widget as this => move |_, _clicks, x, y| {
                let x = x as i32;
                let y = y as i32;

                let width = this.width();
                let height = this.height();
                if width <= 0 || height <= 0 {
                    eprintln!("invalid size for the color picker.");
                    return;
                }
                let col = x / (width / 8);
                let row = y / (height / 2);
                let color = (col + row * 8) as u8;
                unsafe {
                    WORKSPACE.selected_attribute.set_foreground(color);
                    this.queue_draw();
                }
            }),
        );
        widget.add_controller(&gesture);

        let gesture = gtk4::GestureClick::new();
        gesture.set_button(3);
        gesture.connect_pressed(
            glib::clone!(@strong widget as this => move |_, _clicks, x, y| {
                let x = x as i32;
                let y = y as i32;

                let width = this.width();
                let height = this.height();
                if width <= 0 || height <= 0 {
                    eprintln!("invalid size for the color picker.");
                    return;
                }
                let col = x / (width / 8);
                let row = y / (height / 2);
                let color = (col + row * 8) as u8;
                unsafe {
                    WORKSPACE.selected_attribute.set_background(color);
                    this.queue_draw();
                }
            }),
        );
        widget.add_controller(&gesture);
    }
}

impl DrawingAreaImpl for GtkColorPicker {}
