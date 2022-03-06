use gtk4::gdk::Texture;
use gtk4::gdk_pixbuf::{Pixbuf, Colorspace};
use gtk4::subclass::prelude::*;
use gtk4::traits::WidgetExt;
use gtk4::{glib, gdk, graphene};
use std::cell::RefCell;
use std::rc::Rc;

use crate::WORKSPACE;
use crate::model::{Editor, Position, Size, Selection};

#[derive(Default)]

pub struct GtkAnsiView {
    pub editor: RefCell<Rc<RefCell<Editor>>>,
    pub textures: RefCell<Vec<Texture>>,
}

impl GtkAnsiView {

    pub fn set_editor_handle(&self, handle: Rc<RefCell<Editor>>) {
        let mut textures = Vec::new();
        {   
            let buffer = &handle.borrow().buf;
            for col in 0..=15_u8 {
                let fg = buffer.get_rgb(col);
                for u in 0..=255_u8 {
                    unsafe {
                        textures.push(render_char(buffer, u, fg));
                    }
                }
            }
        }
        self.textures.replace(textures);
        self.editor.replace(handle);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for GtkAnsiView {
    const NAME: &'static str = "GtkCharEditorView";
    type Type = super::AnsiView;
    type ParentType = gtk4::DrawingArea;
}

impl ObjectImpl for GtkAnsiView {
    fn constructed(&self, obj: &Self::Type) {
        obj.set_can_focus(true);
        obj.set_focusable(true);
        obj.set_focus_on_click(true);
    }
}

impl WidgetImpl for GtkAnsiView {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);
    }

    fn snapshot(&self, widget: &Self::Type, snapshot: &gtk4::Snapshot) {
        snapshot.append_color(
            &gdk::RGBA::WHITE,
            &graphene::Rect::new(0.0, 0.0, widget.width() as f32, widget.height() as f32),
        );
        let editor = &self.editor.borrow();
        let editor = editor.borrow();
        let buffer = &editor.buf;
        let font_dimensions = buffer.get_font_dimensions();
        let textures = self.textures.borrow();
        for y in 0..buffer.height {
            for x in 0..buffer.width {
                let ch = buffer.get_char(Position::from(x as i32, y as i32));
                let bg = buffer.get_rgb_f64(ch.attribute.get_background());
                let fg = ch.attribute.get_foreground() as usize;
                let bounds = graphene::Rect::new(
                    x as f32 * font_dimensions.width as f32,
                    y as f32 * font_dimensions.height as f32,
                    font_dimensions.width as f32,
                    font_dimensions.height as f32
                );
                snapshot.append_color(&gdk::RGBA::new(bg.0 as f32, bg.1 as f32, bg.2 as f32, 1.0), &bounds);
                snapshot.append_texture(&textures[fg * 256 + (ch.char_code as usize)], &bounds);
            }
        }
        if !editor.is_inactive {
            unsafe {
                if WORKSPACE.cur_tool().use_caret() {
                    draw_caret(editor.cursor.get_position(), snapshot, font_dimensions);
                }
                if WORKSPACE.cur_tool().use_selection() {
                    if let Some(cur_selection) = &editor.cur_selection{
                        draw_selection(cur_selection, snapshot, font_dimensions);
                    }
                }
            }
        }
    }
}

impl DrawingAreaImpl for GtkAnsiView {}

unsafe fn render_char(buffer: &crate::model::Buffer, ch: u8, fg: (u8, u8, u8)) -> Texture {

    let font_dimensions = buffer.get_font_dimensions();
    let pix_buf = Pixbuf::new(Colorspace::Rgb, true, 8, font_dimensions.width as i32, font_dimensions.height as i32).unwrap();
    let pixels = pix_buf.pixels();

    let mut i = 0;
    for y in 0..font_dimensions.height {
        let line = buffer.get_font_scanline(ch, y as usize);
        for x in 0..font_dimensions.width {
            if (line & (128 >> x)) != 0 {
                pixels[i] = fg.0;
                i += 1;
                pixels[i] = fg.1;
                i += 1;
                pixels[i] = fg.2;
                i += 1;
                pixels[i] = 255;
                i += 1;
            } else {
                pixels[i] = 0;
                i += 1;
                pixels[i] = 0;
                i += 1;
                pixels[i] = 0;
                i += 1;
                pixels[i] = 0;
                i += 1;
            }
        }
    }

    Texture::for_pixbuf(&pix_buf)
}

fn draw_selection(cur_selection: &Selection, snapshot: &gtk4::Snapshot, font_dimensions: Size)
{
    let rect = &cur_selection.rectangle;

    let bounds = graphene::Rect::new(
        rect.start.x as f32 * font_dimensions.width as f32,
        rect.start.y as f32 * font_dimensions.height as f32,
        rect.size.width as f32 * font_dimensions.width as f32,
        rect.size.height as f32 * font_dimensions.height as f32
    );

    let cr = snapshot.append_cairo(&bounds);

    cr.rectangle(bounds.x() as f64,
                 bounds.y() as f64,
              bounds.width() as f64,
             bounds.height() as f64);
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.set_line_width(3f64);
    cr.stroke_preserve().expect("error while calling stroke.");
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.set_line_width(1f64);
    cr.stroke().expect("error while calling stroke.");
}

fn draw_caret(cursor_pos: Position, snapshot: &gtk4::Snapshot, font_dimensions: Size) {
    let x = cursor_pos.x;
    let y = cursor_pos.y;

    let bounds = graphene::Rect::new(
        x as f32 * font_dimensions.width as f32,
        y as f32 * font_dimensions.height as f32,
        font_dimensions.width as f32,
        font_dimensions.height as f32
    );

    let cr = snapshot.append_cairo(&bounds);
    
    cr.rectangle(
        (x as i32 * font_dimensions.width as i32) as f64,
        (y as i32 * font_dimensions.height as i32) as f64,
        font_dimensions.width as f64,
        font_dimensions.height as f64,
    );
    cr.set_source_rgb(
        0xF7 as f64 / 255.0,
        0xF7 as f64 / 255.0,
        0xF7 as f64 / 255.0,
    );
    cr.set_operator(gtk4::cairo::Operator::Difference);
    cr.fill().expect("error while calling fill.");
}

