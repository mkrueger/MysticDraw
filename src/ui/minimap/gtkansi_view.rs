use gtk4::gdk_pixbuf::{Pixbuf, Colorspace};
use gtk4::subclass::prelude::*;
use gtk4::traits::WidgetExt;
use gtk4::{glib, gdk, graphene};

use std::cell::RefCell;
use std::rc::Rc;

use crate::model::{Editor, Position};

#[derive(Default)]
pub struct GtkMinimapAnsiView {
    pub editor: RefCell<Rc<RefCell<Editor>>>,

    pub textures: RefCell<Vec<Vec<f32>>>,
    pub has_editor: RefCell<bool>,

    pub pix_buf: RefCell<Option<Pixbuf>>,
    pub minimap_image : RefCell<Option<gtk4::gdk::Texture>>,
}


impl GtkMinimapAnsiView {

    pub fn set_editor_handle(&self, handle: Rc<RefCell<Editor>>) {
        self.has_editor.replace(true);
        let mut new_textures = Vec::new();
        {   
            let buffer = &handle.borrow().buf;

            let block_size = if buffer.width >= 160 { 4 } else { 2 };
            let font_size = 255;
            let full_block = (block_size * block_size) as f32;
            for u in 0..=font_size {
                let mut fg = Vec::new();
                for y in 0..(buffer.font.size.height / block_size) {
                    for x in 0..(buffer.font.size.width / block_size) {

                        let mut i = 0;
                        for y2 in 0..block_size { 
                            let line = buffer.font.get_scanline(u, (y * block_size + y2) as usize);
                            for x2 in 0..block_size {
                                if (line & (128 >> (x * block_size + x2))) != 0 {
                                    i += 1;
                                }
                            }
                        }
                        fg.push(i as f32 / full_block);
                    }
                }
                new_textures.push(fg);
            }
        }
        self.textures.replace(new_textures);
        self.editor.replace(handle);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for GtkMinimapAnsiView {
    const NAME: &'static str = "GtkMinimapAnsiView";
    type Type = super::MinimapAnsiView;
    type ParentType = gtk4::DrawingArea;
}

impl ObjectImpl for GtkMinimapAnsiView {
    fn constructed(&self, obj: &Self::Type) {
        obj.set_can_focus(true);
        obj.set_focusable(true);
        obj.set_focus_on_click(true);
    }
}

impl WidgetImpl for GtkMinimapAnsiView {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);
    }

    fn snapshot(&self, widget: &Self::Type, snapshot: &gtk4::Snapshot) {
        snapshot.append_color(
            &gdk::RGBA::new(0.6, 0.6, 0.6, 1.0),
            &graphene::Rect::new(0.0, 0.0, widget.width() as f32, widget.height() as f32),
        );
        if !*self.has_editor.borrow() { return; }
        
        let editor = &self.editor.borrow();
        let editor = editor.borrow();
        let buffer = &editor.buf;
        let block_size = if buffer.width >= 160 { 4 } else { 2 };

        let block_w = buffer.font.size.width as u16 / block_size as u16;
        let block_h = buffer.font.size.height as u16 / block_size as u16;

        widget.set_width_request((buffer.width * block_w) as i32);
        widget.set_height_request((buffer.height * block_h) as i32);
        let t = self.textures.borrow();
        
        let mut new_pixbuf = self.pix_buf.borrow().is_none();
        
        if let Some(pb) = &*self.pix_buf.borrow() {
            new_pixbuf |= pb.width() != buffer.width as i32 * block_w as i32 || pb.height() != buffer.height  as i32 * block_h as i32;
        }

        if new_pixbuf  {
            self.pix_buf.replace(Some(Pixbuf::new(Colorspace::Rgb, true, 8, buffer.width as i32 * block_w as i32, buffer.height as i32 * block_h as i32).unwrap()));
        } 

        if let Some(pix_buf) = &*self.pix_buf.borrow_mut() {
            unsafe {
                let pixels = pix_buf.pixels();
                let mut i = 0;
                for y in 0..(buffer.height * block_h) {
                    for x in 0..(buffer.width * block_w) {

                        let ch = buffer.get_char(Position::from((x / block_w)  as i32, (y / block_h) as i32)).unwrap_or_default();
                        let bg = ch.attribute.get_background() as usize;
                        let bg = buffer.palette.colors[bg].get_rgb_f32();
                        let fg = ch.attribute.get_foreground() as usize;
                        let fg = buffer.palette.colors[fg].get_rgb_f32();

                        let f_fac = &t[(ch.char_code & 0xFF) as usize];
                        let f_fac = f_fac[(x % block_w + (y % block_h) * block_w) as usize];
                        let b_fac = 1.0 - f_fac;
                        pixels[i] = ((fg.0 * f_fac + bg.0 * b_fac) * 255.0) as u8;
                        i += 1;
                        pixels[i] = ((fg.1 * f_fac + bg.1 * b_fac) * 255.0) as u8;
                        i += 1;
                        pixels[i] = ((fg.2 * f_fac + bg.2 * b_fac) * 255.0) as u8;
                        i += 1;
                        pixels[i] = 255;
                        i += 1;
                    }
                }
            }

            let texture = gdk::Texture::for_pixbuf(pix_buf);
            snapshot.append_texture(&texture, &graphene::Rect::new(0.0, 0.0, (buffer.width * block_w) as f32, (buffer.height * block_h) as f32));
        }
    }
}

impl DrawingAreaImpl for GtkMinimapAnsiView {}
