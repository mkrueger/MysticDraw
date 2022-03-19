use std::cell::RefCell;
use std::rc::Rc;

use gtk4::gdk::Texture;
use gtk4::{glib, gdk, graphene};
use gtk4::subclass::prelude::*;
use gtk4::traits::{GestureSingleExt, WidgetExt};

use crate::model::{Editor, TextAttribute};

#[derive(Default)]

pub struct GtkAttributeSwitcher {
    pub editor: RefCell<Option<Rc<RefCell<Editor>>>>,
}

impl GtkAttributeSwitcher {
    pub fn set_editor(&self, _obj: &super::AttributeSwitcher, handle: &Rc<RefCell<Editor>>) {
        self.editor.replace(Some(handle.clone()));
    }
}

#[glib::object_subclass]
impl ObjectSubclass for GtkAttributeSwitcher {
    const NAME: &'static str = "GtkAttributeSwitcher";
    type Type = super::AttributeSwitcher;
    type ParentType = gtk4::DrawingArea;
}

impl ObjectImpl for GtkAttributeSwitcher {
    fn constructed(&self, obj: &Self::Type) {
        obj.set_can_focus(true);
        obj.set_focusable(true);
        obj.set_focus_on_click(true);
        obj.set_size_request(PICKER_WIDTH as i32, PICKER_HEIGHT as i32);
    }
}

impl WidgetImpl for GtkAttributeSwitcher {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);
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
                if let Some(editor) = this.get_editor() {
                    let mut editor = editor.borrow_mut();
                    let s = PADDING + DEFAULT_COLOR_RECTSIZE * 1.5;
                    if ((x as f32) < s) && y as f32 > PICKER_HEIGHT - s {
                        editor.cursor.set_attribute(TextAttribute::DEFAULT);
                    } else if ((x as f32) > PICKER_WIDTH - s) && (y as f32) < s  {
                        editor.switch_fg_bg_color();
                    }
                    this.queue_draw();
                }
            }),
        );
        widget.add_controller(&gesture);
    }


    fn snapshot(&self, _widget: &Self::Type, snapshot: &gtk4::Snapshot) 
    {
        if let Some(editor) = &*self.editor.borrow() {
            let editor = editor.borrow_mut();
            let fg = editor.buf.palette.colors[editor.cursor.get_attribute().get_foreground() as usize].get_rgb_f64();
            let bg = editor.buf.palette.colors[editor.cursor.get_attribute().get_background() as usize].get_rgb_f64();

            draw_color_rectangle(snapshot, PICKER_WIDTH - RECTANGLE_SIZE - PADDING, PICKER_HEIGHT - RECTANGLE_SIZE - PADDING, bg);
            draw_color_rectangle(snapshot, PADDING, PADDING, fg);

            // bg
            let bounds = graphene::Rect::new(
                PADDING + DEFAULT_COLOR_RECTSIZE / 2.0 - 1.0,
                PICKER_HEIGHT - PADDING - DEFAULT_COLOR_RECTSIZE,
                DEFAULT_COLOR_RECTSIZE,DEFAULT_COLOR_RECTSIZE
            );
            let color = editor.buf.palette.colors[0].get_rgb_f64();
            snapshot.append_color(&gdk::RGBA::new(color.0 as f32, color.1 as f32, color.2 as f32, 1.0), &bounds);

            // fg
            let bounds = graphene::Rect::new(
                PADDING - 1.0,
                PICKER_HEIGHT - PADDING - DEFAULT_COLOR_RECTSIZE - DEFAULT_COLOR_RECTSIZE / 2.0 + 1.0,
                DEFAULT_COLOR_RECTSIZE,DEFAULT_COLOR_RECTSIZE
            );
            let color = editor.buf.palette.colors[7].get_rgb_f64();
            snapshot.append_color(&gdk::RGBA::new(color.0 as f32, color.1 as f32, color.2 as f32, 1.0), &bounds);


            let texture = Texture::from_resource("/com/github/mkrueger/MysticDraw/icons/scalable/apps/md-switch-color.svg");
            let s = 18.0;
            snapshot.append_texture(&texture,  &graphene::Rect::new(
                PICKER_WIDTH - s - 2.0,
                2.0,
                s,s
            ));
        }
    }
}

const PICKER_WIDTH: f32  = 48.0;
const PICKER_HEIGHT: f32  = 48.0;
const PADDING: f32 = 4.0;
const RECTANGLE_SIZE: f32 = 24.0;
const DEFAULT_COLOR_RECTSIZE: f32 = 10.0;

fn draw_color_rectangle(snapshot: &gtk4::Snapshot, x: f32, y: f32, color: (f64, f64, f64))
{
    let bounds = graphene::Rect::new(
        x,
        y,
        RECTANGLE_SIZE,
        RECTANGLE_SIZE
    );
    snapshot.append_color(&gdk::RGBA::new(0.0, 0.0,  0.0, 1.0), &bounds);

    let bounds = graphene::Rect::new(
        x + 1.0,
        y + 1.0,
        RECTANGLE_SIZE - 2.0,
        RECTANGLE_SIZE - 2.0
    );

    snapshot.append_color(&gdk::RGBA::new(1.0, 1.0,  1.0, 1.0), &bounds);

    let bounds = graphene::Rect::new(
        x + 2.0,
        y + 2.0,
        RECTANGLE_SIZE - 4.0,
        RECTANGLE_SIZE - 4.0
    );

    snapshot.append_color(&gdk::RGBA::new(color.0 as f32, color.1 as f32, color.2 as f32, 1.0), &bounds);
}

impl DrawingAreaImpl for GtkAttributeSwitcher {}
