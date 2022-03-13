use gtk4::gdk::Texture;
use gtk4::gdk_pixbuf::{Pixbuf, Colorspace};
use gtk4::prelude::TextureExt;
use gtk4::subclass::prelude::*;
use gtk4::traits::WidgetExt;
use gtk4::{glib, gdk, graphene};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::WORKSPACE;
use crate::model::{Editor, Position, Size, Selection};

#[derive(Default)]

pub struct GtkAnsiView {
    pub editor: RefCell<Rc<RefCell<Editor>>>,
    pub textures: RefCell<Vec<Texture>>,
    pub has_editor: RefCell<bool>,
    pub is_minimap: RefCell<bool>,
    pub preview_rectangle: RefCell<Option<crate::model::Rectangle>>,
    pub reference_image_file: RefCell<Option<PathBuf>>,
    pub reference_image: RefCell<Option<gtk4::gdk::Texture>>
}

impl GtkAnsiView {

    pub fn set_mimap_mode(&self, is_minimap: bool)
    {
        self.is_minimap.replace(is_minimap);
    } 

    pub fn set_preview_rectangle(&self, rect: Option<crate::model::Rectangle>)
    {
        self.preview_rectangle.replace(rect);
    } 

    pub fn set_editor_handle(&self, handle: Rc<RefCell<Editor>>) {
        self.has_editor.replace(true);
        let mut textures = Vec::new();
        {   
            let buffer = &handle.borrow().buf;

            let mut font_size = 256;

            if buffer.font.extended_font {
                font_size = 512;
            }

            for col in 0..buffer.palette.colors.len() {
                let fg = buffer.palette.colors[col as usize].get_rgb();
                for u in 0..font_size {
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
            &gdk::RGBA::new(0.6, 0.6, 0.6, 1.0),
            &graphene::Rect::new(0.0, 0.0, widget.width() as f32, widget.height() as f32),
        );
        if !*self.has_editor.borrow() { return; }
        let is_minimap = *self.is_minimap.borrow();
        if !is_minimap {
            let mut y = 0.0; 
            let mut b = true;
            while y < widget.height() as f32 {
                let mut x = if b { 0.0 } else { 8.0 }; 
                b = !b;
                while x  < widget.width() as f32 {
                    snapshot.append_color(
                        &gdk::RGBA::new(0.4, 0.4, 0.4, 1.0),
                        &graphene::Rect::new(x, y, 8.0, 8.0),
                    );
                    x += 16.0;
                }
                y += 8.0;
            }
        } 
        
        let editor = &self.editor.borrow();
        let editor = editor.borrow();
        let buffer = &editor.buf;
        let font_dimensions = buffer.get_font_dimensions();
        let textures = self.textures.borrow();

        let mut font_size = 256;
        if buffer.font.extended_font {
            font_size = 512;
        }

        if is_minimap {
            let full_width = buffer.width as f32 * font_dimensions.width as f32;
            let full_height = buffer.height as f32 * font_dimensions.height as f32;
            let scale = widget.parent().unwrap().width() as f32 / full_width;
            snapshot.scale(scale, scale);
            widget.set_height_request( (full_height * scale) as i32);
        }

        if !self.reference_image_file.borrow().eq(&editor.reference_image) {
            self.reference_image_file.replace(editor.reference_image.clone());

            if let Some(file_name) = &editor.reference_image {
                if let Ok(img) = Texture::from_filename(file_name) {
                    self.reference_image.replace(Some(img));
                } else {
                    eprintln!("Error loading image");
                }
            } else {
                self.reference_image.replace(None);
            }
        }

        let paint_texture = if let Some(texture) = &*self.reference_image.borrow() {
            let full_width = buffer.width as f32 * font_dimensions.width as f32;
            let scale =  full_width / texture.width() as f32 ;
            let bounds = graphene::Rect::new(
                0.0,
                0.0,
                full_width,
                texture.height() as f32 * scale
            );
            snapshot.append_texture(texture, &bounds);
            snapshot.push_opacity(0.7);
            true 
        } else { false };

        for y in 0..buffer.height {
            for x in 0..buffer.width {
                let ch = buffer.get_char(Position::from(x as i32, y as i32));
                if ch.is_none() { continue; }
                let ch = ch.unwrap();
                let mut bg = ch.attribute.get_background() as usize;
                unsafe {
                    if !WORKSPACE.show_bg_color {
                        bg = 0;
                    }      
                }
                let mut char_num = ch.char_code as usize;

                let bg = buffer.palette.colors[bg].get_rgb_f64();
                let mut fg = ch.attribute.get_foreground() as usize;
                unsafe {
                    if !WORKSPACE.show_fg_color {
                        fg = 7;
                    }      
                }
                if buffer.use_512_chars && (fg & 0b_1000) != 0 {
                    char_num += 256;
                    fg &= 0b_0111;
                }

                let bounds = graphene::Rect::new(
                    x as f32 * font_dimensions.width as f32,
                    y as f32 * font_dimensions.height as f32,
                    font_dimensions.width as f32,
                    font_dimensions.height as f32
                );
                snapshot.append_color(&gdk::RGBA::new(bg.0 as f32, bg.1 as f32, bg.2 as f32, 1.0), &bounds);
                snapshot.append_texture(&textures[fg * font_size + char_num], &bounds);
            }
        }

        if paint_texture {
            snapshot.pop();
        }

        if !editor.is_inactive && !is_minimap {
            unsafe {
                if WORKSPACE.cur_tool().use_caret() {
                    draw_caret(editor.get_cursor_position(), snapshot, font_dimensions);
                }
                if WORKSPACE.cur_tool().use_selection() {
                    if let Some(cur_selection) = &editor.cur_selection{
                        draw_selection(cur_selection, snapshot, font_dimensions);
                    }
                }
            }
        }

        if self.preview_rectangle.borrow().is_some() {
            let rect = self.preview_rectangle.borrow().unwrap();
            draw_preview_rectangle(&rect, snapshot, font_dimensions);
        }
    }
}

impl DrawingAreaImpl for GtkAnsiView {}

unsafe fn render_char(buffer: &crate::model::Buffer, ch: u16, fg: (u8, u8, u8)) -> Texture {
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

fn draw_selection(cur_selection: &Selection, snapshot: &gtk4::Snapshot, font_dimensions: Size<u8>)
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

fn draw_preview_rectangle(rect: &crate::model::Rectangle, snapshot: &gtk4::Snapshot, font_dimensions: Size<u8>)
{
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
    cr.set_source_rgb(2.0, 2.0, 6.0);
    cr.set_line_width(3f64);
    cr.stroke_preserve().expect("error while calling stroke.");
}

fn draw_caret(cursor_pos: Position, snapshot: &gtk4::Snapshot, font_dimensions: Size<u8>) {
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

