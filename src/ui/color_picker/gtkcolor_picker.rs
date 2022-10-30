use std::cell::RefCell;
use std::rc::Rc;

use gtk4::{glib, graphene, gdk};
use gtk4::subclass::prelude::*;
use gtk4::traits::{GestureSingleExt, WidgetExt};

use crate::model::{Editor, BufferType};

#[derive(Default)]

pub struct GtkColorPicker {
    pub editor: RefCell<Option<Rc<RefCell<Editor>>>>,
}

impl GtkColorPicker {

    pub fn set_editor(&self, obj: &super::ColorPicker, handle: &Rc<RefCell<Editor>>) {
        self.editor.replace(Some(handle.clone()));


        match handle.borrow().buf.buffer_type {
            BufferType::LegacyDos => {
                obj.set_size_request(200, 50);
            }
            crate::model::BufferType::LegacyIce => {
                obj.set_size_request(200, 50);
            }
            crate::model::BufferType::ExtFont => {
                obj.set_size_request(200, 25);
            }
            crate::model::BufferType::ExtFontIce => {
                obj.set_size_request(200, 50);
            }
            crate::model::BufferType::NoLimits => {
                obj.set_size_request(200, 50);
            }
        }
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
    }
}

impl WidgetImpl for GtkColorPicker {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);

        // TODO: Remove code duplication.
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(1);
        
        gesture.connect_pressed(
            glib::clone!(@weak widget => move |_, _clicks, x, y| {
                let x = x as i32;
                let y = y as i32;

                let width = widget.width();
                let height = widget.height();
                if width <= 0 || height <= 0 {
                    eprintln!("invalid size for the color picker.");
                    return;
                }
                if let Some(editor) = widget.get_editor() {
                    let mut editor = editor.borrow_mut();

                    let col = x / (width / 8);
                    let row =  if editor.buf.buffer_type == BufferType::ExtFont {
                        0
                    } else {
                        y / (height / 2)
                    };
                    let color = (col + row * 8) as u8;
    
                    let mut attr = editor.caret.get_attribute();
                    attr.set_foreground(color);
                    editor.set_caret_attribute(attr);
                    widget.queue_draw();
                }
            }),
        );
        widget.add_controller(&gesture);

        let gesture = gtk4::GestureClick::new();
        gesture.set_button(3);
        gesture.connect_pressed(
            glib::clone!(@weak widget => move |_, _clicks, x, y| {
                let x = x as i32;
                let y = y as i32;

                let width = widget.width();
                let height = widget.height();
                if width <= 0 || height <= 0 {
                    eprintln!("invalid size for the color picker.");
                    return;
                }
                let col = x / (width / 8);
                let row = y / (height / 2);
                let color = (col + row * 8) as u8;

                if let Some(editor) = widget.get_editor() {
                    let mut editor = editor.borrow_mut();
                    let mut attr = editor.caret.get_attribute();
                    attr.set_background(color);
                    editor.set_caret_attribute(attr);
                    widget.queue_draw();
                }
            }),
        );
        widget.add_controller(&gesture);
    }

    fn snapshot(&self, widget: &Self::Type, snapshot: &gtk4::Snapshot) 
    {
        if let Some(editor) = &*self.editor.borrow() {
            let editor = editor.borrow_mut();

            let width = widget.width();
            let height = widget.height();

            match editor.buf.buffer_type {

                crate::model::BufferType::ExtFont => {
                    for x in 0..8 {
                        let color = editor.buf.palette.colors[x as usize].get_rgb_f32();
                        let bounds = graphene::Rect::new(
                            (x * (width / 8)) as f32,
                            0.0,
                            (width / 8) as f32,
                            height as f32
                        );
                        snapshot.append_color(&gdk::RGBA::new(color.0, color.1, color.2, 1.0), &bounds);
                    }
                }

                _ => {
                    for y in 0..2 {
                        for x in 0..8 {
                            let color = editor.buf.palette.colors[(x + y * 8) as usize].get_rgb_f32();
                            let bounds = graphene::Rect::new(
                                (x * (width / 8)) as f32,
                                (y * height / 2) as f32,
                                (width / 8) as f32,
                                (height / 2) as f32
                            );
                            snapshot.append_color(&gdk::RGBA::new(color.0, color.1, color.2, 1.0), &bounds);
                        }
                    }
                }
            };
            let attribute = editor.caret.get_attribute();
            let marker_width = 6.0;
            let x = (attribute.get_foreground() % 8) as i32;
            let y = (attribute.get_foreground() / 8) as i32;
            let bounds = graphene::Rect::new(
                (x * (width / 8)) as f32,
                (y * height / 2) as f32,
                marker_width,
                marker_width
            );
            snapshot.append_color(&gdk::RGBA::new(0.0, 0.0, 0.0, 1.0), &bounds);

            let bounds = graphene::Rect::new(
                (x * (width / 8)) as f32 + 1.0,
                (y * height / 2) as f32 + 1.0,
                marker_width - 2.0,
                marker_width - 2.0
            );

            snapshot.append_color(&gdk::RGBA::new(1.0, 1.0, 1.0, 1.0), &bounds);

            let x = (attribute.get_background() % 8) as i32;
            let y = (attribute.get_background() / 8) as i32;
            let bounds = graphene::Rect::new(
                ((1 + x) * width / 8) as f32 - marker_width,
                ((1 + y) * height / 2) as f32 - marker_width,
                marker_width,
                marker_width
            );
            snapshot.append_color(&gdk::RGBA::new(0.0, 0.0, 0.0, 1.0), &bounds);

            let x = (attribute.get_background() % 8) as i32;
            let y = (attribute.get_background() / 8) as i32;
            let bounds = graphene::Rect::new(
                ((1 + x) * width / 8) as f32 - marker_width + 1.0,
                ((1 + y) * height / 2) as f32 - marker_width + 1.0,
                marker_width - 2.0,
                marker_width - 2.0
            );
            snapshot.append_color(&gdk::RGBA::new(1.0, 1.0, 1.0, 1.0), &bounds);
        }
    }
}

impl DrawingAreaImpl for GtkColorPicker {}
