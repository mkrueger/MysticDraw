use gtk4::gdk::Texture;
use gtk4::gdk_pixbuf::{Pixbuf, Colorspace};
use gtk4::prelude::TextureExt;
use gtk4::subclass::prelude::*;
use gtk4::traits::WidgetExt;
use gtk4::{glib, gdk, graphene};
use core::time;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::WORKSPACE;
use crate::model::{Editor, Position, Size, Selection, SauceString, BitFont};

#[derive(Default)]
pub struct GtkAnsiView {
    pub editor: RefCell<Rc<RefCell<Editor>>>,
    pub textures: RefCell<Vec<Texture>>,
    pub ext_font_textures: RefCell<Vec<Texture>>,
    pub has_editor: RefCell<bool>,
    pub is_minimap: RefCell<bool>,
    pub font_name: RefCell<SauceString<22, 0>>,
    pub ext_font_name: RefCell<SauceString<22, 0>>,
    pub preview_rectangle: RefCell<Option<crate::model::Rectangle>>,
    pub reference_image_file: RefCell<Option<PathBuf>>,
    pub reference_image: RefCell<Option<gtk4::gdk::Texture>>,

    blink: Rc<RefCell<bool>>
}

impl GtkAnsiView {
    pub fn get_start_pos(&self, widget: &super::AnsiView) -> (f32, f32)
    {
        if !*self.has_editor.borrow() { return (0.0, 0.0); }

        let editor = &self.editor.borrow();
        let editor = editor.borrow();
        if editor.is_inactive { return (0.0, 0.0); }
        let buffer = &editor.buf;

        let font_dimensions = buffer.get_font_dimensions();
        let full_width = buffer.width as f32 * font_dimensions.width as f32;
        let full_height = buffer.height as f32 * font_dimensions.height as f32;
        
        let start_x = if full_width < widget.width() as f32 { ((widget.width() as f32 - full_width) / 2.0).floor() } else { 0.0 };
        let start_y = if full_height < widget.height() as f32 { ((widget.height() as f32 - full_height) / 2.0).floor() } else { 0.0 };

        (start_x, start_y)
    }

    pub fn set_mimap_mode(&self, is_minimap: bool)
    {
        self.is_minimap.replace(is_minimap);
    } 

    pub fn set_preview_rectangle(&self, rect: Option<crate::model::Rectangle>)
    {
        self.preview_rectangle.replace(rect);
    } 

    fn update_font_textures(&self)
    {
        let b = self.editor.borrow();
        let buffer = &b.borrow().buf;
        if *self.font_name.borrow() == buffer.font.name && !self.textures.borrow().is_empty() {
            return;
        }

        let mut textures = Vec::new();

        let font_size = 255;

        self.font_name.replace(buffer.font.name.clone());

        for col in 0..buffer.palette.colors.len() {
            let fg = buffer.palette.colors[col as usize].get_rgb();
            for u in 0..=font_size {
                unsafe {
                    textures.push(render_char(&buffer.font, u, fg));
                }
            }
        }
        self.textures.replace(textures);

        if let Some(ext_font) = &buffer.extended_font {
            let mut ext_font_textures = Vec::new();

            let font_size = 255;

            self.ext_font_name.replace(buffer.font.name.clone());

            for col in 0..buffer.palette.colors.len() {
                let fg = buffer.palette.colors[col as usize].get_rgb();
                for u in 0..=font_size {
                    unsafe {
                        ext_font_textures.push(render_char(ext_font, u, fg));
                    }
                }
            }
            self.ext_font_textures.replace(ext_font_textures);
        }
    }

    pub fn set_editor_handle(&self, handle: Rc<RefCell<Editor>>) {
        self.has_editor.replace(true);
        self.editor.replace(handle);
        self.update_font_textures();
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

        let blink = self.blink.clone();
        let blink_cb = glib::clone!(@weak obj => move || {
            let b = *blink.borrow();
            blink.replace(!b);
            obj.queue_draw();
        });
        let blink_timer = glib::timeout_add_local(time::Duration::from_millis(550), 
        move || {
                blink_cb();
                glib::Continue(true)
            }
        );

        // removing is not easy - SourceId is missing the Copy & Clone trait.
        let raw_blink_timer = unsafe { blink_timer.as_raw() };
        obj.connect_destroy(move |_| {
            unsafe { glib::ffi::g_source_remove(raw_blink_timer); }
        });
    }
}

impl WidgetImpl for GtkAnsiView {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);
    }

    fn focus(&self, widget: &Self::Type, direction_type: gtk4::DirectionType) -> bool {
        println!("focus {}" , direction_type);
        self.parent_focus(widget, direction_type)
    }

    fn snapshot(&self, widget: &Self::Type, snapshot: &gtk4::Snapshot) {
        self.update_font_textures();
        snapshot.append_color(
            &gdk::RGBA::new(0.2, 0.2, 0.2, 1.0),
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
                        &gdk::RGBA::new(0.1, 0.1, 0.1, 1.0),
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
        let ext_textures = self.ext_font_textures.borrow();
        let full_width = buffer.width as f32 * font_dimensions.width as f32;
        let full_height = buffer.height as f32 * font_dimensions.height as f32;
        
        let start_x = if !editor.is_inactive && full_width < widget.width() as f32 { ((widget.width() as f32 - full_width) / 2.0).floor() } else { 0.0 };
        let start_y = if !editor.is_inactive && full_height < widget.height() as f32 { ((widget.height() as f32 - full_height) / 2.0).floor() } else { 0.0 };

        let font_size = 256;

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
        let is_blink = *self.blink.borrow();
        let should_draw_caret = unsafe { WORKSPACE.cur_tool().use_caret() && widget.has_focus() && is_blink };
        let caret_pos = editor.get_cursor_position();
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
                let char_num = ch.char_code as usize;

                let bg = buffer.palette.colors[bg].get_rgb_f64();
                let mut fg = ch.attribute.get_foreground() as usize;
                unsafe {
                    if !WORKSPACE.show_fg_color {
                        fg = 7;
                    }      
                }

                let bounds = graphene::Rect::new(
                    start_x + x as f32 * font_dimensions.width as f32,
                    start_y + y as f32 * font_dimensions.height as f32,
                    font_dimensions.width as f32,
                    font_dimensions.height as f32
                );

                if should_draw_caret && y as i32 == caret_pos.y && x as i32 == caret_pos.x {
                    snapshot.push_blend(gtk4::gsk::BlendMode::Difference);
                }
                snapshot.append_color(&gdk::RGBA::new(bg.0 as f32, bg.1 as f32, bg.2 as f32, 1.0), &bounds);
                let idx = fg * font_size + (char_num & 0xFF);
                if !ch.attribute.is_blink() || !is_blink {
                    if char_num > 0xFF { 
                        if idx < ext_textures.len() {
                            snapshot.append_texture(&ext_textures[idx], &bounds);
                        }
                    } else if idx < textures.len() {
                        snapshot.append_texture(&textures[idx], &bounds);
                    }
                }

                if should_draw_caret && y as i32 == caret_pos.y && x as i32 == caret_pos.x {
                    snapshot.pop();
                    let fg = buffer.palette.colors[fg].get_rgb_f64();
                    let fg_rgb = if fg.0 + fg.1 + fg.2 == 0.0 { 
                        gdk::RGBA::new(1.0, 1.0, 1.0, 1.0)
                    } else {
                        gdk::RGBA::new(fg.0 as f32, fg.1 as f32, fg.2 as f32, 1.0)
                    };
                    draw_caret(start_x, start_y, caret_pos, snapshot, font_dimensions, editor.cursor.insert_mode, &fg_rgb);
                    snapshot.pop();
                }
            }
        }

        if paint_texture {
            snapshot.pop();
        }

        if !editor.is_inactive && !is_minimap {
            unsafe {
                if let Some(grid) = WORKSPACE.get_grid_size() {
                    let mut x = 0;
                    let w = buffer.width as f32 * font_dimensions.width as f32;
                    let h = buffer.height as f32 * font_dimensions.height as f32;
                    while x < buffer.width {
                        snapshot.append_color(
                            &gdk::RGBA::new(1.0, 1.0, 1.0, 0.5),
                            &graphene::Rect::new(
                                start_x + x as f32 * font_dimensions.width as f32, 
                                start_y + 0.0, 
                                1.0, 
                                h),
                        );
                        x += grid.width as u16;
                    }
                    let mut y = 0;
                    while y < buffer.height {
                        snapshot.append_color(
                            &gdk::RGBA::new(1.0, 1.0, 1.0, 0.5),
                            &graphene::Rect::new(
                                start_x + 0.0, 
                                start_y + y as f32 * font_dimensions.height as f32, 
                                w,
                                1.0
                                ),
                        );
                        y += grid.height as u16;
                    }
                }

                if let Some(guide) = WORKSPACE.get_guide_size() {
                    let mut x = 0;
                    let w = buffer.width as f32 * font_dimensions.width as f32;
                    let h = buffer.height as f32 * font_dimensions.height as f32;
                    while x < buffer.width {
                        snapshot.append_color(
                            &gdk::RGBA::new(1.0, 1.0, 0.0, 0.5),
                            &graphene::Rect::new(
                                start_x + x as f32 * font_dimensions.width as f32, 
                                start_y + 0.0, 
                                1.0, 
                                h),
                        );
                        x += guide.width as u16;
                    }
                    let mut y = 0;
                    while y < buffer.height {
                        snapshot.append_color(
                            &gdk::RGBA::new(1.0, 1.0, 0.0, 0.5),
                            &graphene::Rect::new(
                                start_x + 0.0, 
                                start_y + y as f32 * font_dimensions.height as f32, 
                                w,
                                1.0
                                ),
                        );
                        y += guide.height as u16;
                    }
                }
                if WORKSPACE.cur_tool().use_selection() {
                    if let Some(cur_selection) = &editor.cur_selection{
                        draw_selection(start_x, start_y, cur_selection, snapshot, font_dimensions);
                    }
                }
            }
        }

        if self.preview_rectangle.borrow().is_some() {
            let rect = self.preview_rectangle.borrow().unwrap();
            draw_preview_rectangle(start_x, start_y, &rect, snapshot, font_dimensions);
        }
    }
}

impl DrawingAreaImpl for GtkAnsiView {}

unsafe fn render_char(font: &BitFont, ch: u8, fg: (u8, u8, u8)) -> Texture {
    let font_dimensions = font.size;
    let pix_buf = Pixbuf::new(Colorspace::Rgb, true, 8, font_dimensions.width as i32, font_dimensions.height as i32).unwrap();
    let pixels = pix_buf.pixels();

    let mut i = 0;
    for y in 0..font_dimensions.height {
        let line = font.get_scanline(ch, y as usize);
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

fn draw_selection(start_x: f32, start_y: f32, cur_selection: &Selection, snapshot: &gtk4::Snapshot, font_dimensions: Size<u8>)
{
    let rect = &cur_selection.rectangle;

    let bounds = graphene::Rect::new(
        start_x + rect.start.x as f32 * font_dimensions.width as f32,
        start_y + rect.start.y as f32 * font_dimensions.height as f32,
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

fn draw_preview_rectangle(start_x: f32, start_y: f32, rect: &crate::model::Rectangle, snapshot: &gtk4::Snapshot, font_dimensions: Size<u8>)
{
    let bounds = graphene::Rect::new(
        start_x + rect.start.x as f32 * font_dimensions.width as f32,
        start_y + rect.start.y as f32 * font_dimensions.height as f32,
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

const CARET_HEIGHT: f32 = 3.0;

fn draw_caret(start_x: f32, start_y: f32, cursor_pos: Position, snapshot: &gtk4::Snapshot, font_dimensions: Size<u8>, insert_mode: bool, color: &gdk::RGBA) {
    let x = cursor_pos.x;
    let y = cursor_pos.y;
    
    let bounds = if insert_mode { graphene::Rect::new(
        start_x + x as f32 * font_dimensions.width as f32,
        start_y + y as f32 * font_dimensions.height as f32,
        font_dimensions.width as f32,
        font_dimensions.height as f32
    ) } else {
        graphene::Rect::new(
            start_x + x as f32 * font_dimensions.width as f32,
            start_y + (y + 1) as f32 * font_dimensions.height as f32 - CARET_HEIGHT,
            font_dimensions.width as f32,
            CARET_HEIGHT
        ) 
    };

    snapshot.append_color(color, &bounds);
}