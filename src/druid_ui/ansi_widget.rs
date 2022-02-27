use std::collections::HashMap;
use std::rc::Rc;

use druid::piet::{ImageFormat, InterpolationMode, CairoImage};
use druid::widget::{prelude::*};
use druid::{
    Rect, Color, MouseButton, Point
};

use crate::model::{ Position, Editor};

use super::AppState;

pub struct AnsiWidget
{
    editor: Rc<Editor>,
    chars: Vec<Vec<u8>>,
    hash: HashMap<(u8, u8), CairoImage>
}

impl AnsiWidget
{
    pub fn new(editor: Rc<Editor>) -> Self {
        AnsiWidget {
            editor,
            chars: Vec::new(),
            hash: HashMap::new()
        }
    }
}

impl AnsiWidget
{
    pub fn initialize(&mut self) {
        let buffer = &self.editor.buf;
        let font_dimensions = buffer.get_font_dimensions();
        for color in 0..16 {
            let fg = buffer.get_rgb(color);
            for ch in 0..=255 {
                let mut result = vec![0; font_dimensions.width * font_dimensions.height * 4];
                let mut i = 0;
                for y in 0..font_dimensions.height {
                    let line = buffer.get_font_scanline(ch, y as usize);
                    for x in 0..font_dimensions.width {
                        if (line & (128 >> x)) != 0 {
                            result[i] = fg.0;
                            i += 1;
                            result[i] = fg.1;
                            i += 1;
                            result[i] = fg.2;
                            i += 1;
                            result[i] = 255;
                            i += 1;
                        } else {
                            result[i] = 0;
                            i += 1;
                            result[i] = 0;
                            i += 1;
                            result[i] = 0;
                            i += 1;
                            result[i] = 0;
                            i += 1;
                        }
                    }
                }
                self.chars.push(result);    
            }
        }
    }

    fn get_buffer_pos(&self, p: Point) -> Option<Position> {
        if p.x < 0.0 || p.y < 0.0 {
            return None;
        }
        let buffer = &self.editor.buf;

        let font_dimensions = buffer.get_font_dimensions();

        let x = (p.x / font_dimensions.width as f64) as usize;
        let y = (p.y / font_dimensions.height as f64) as usize;
        if x >= buffer.width || y >= buffer.height {
            return None;
        }
        Some(Position::from(x as i32, y as i32))
    }

}

impl Widget<AppState> for AnsiWidget {

    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, _data: &mut AppState, _env: &Env)
    {
        match event {
            Event::MouseDown(e) => {
                if e.button == MouseButton::Left {
                    let pos_opt = self.get_buffer_pos(e.pos);
                    if let Some(pos) = pos_opt {
                        
                    }
                }
            }
            Event::MouseMove(_e) => {

            }
            Event::MouseUp(_e) => {

            }
            Event::KeyDown(_e) => {

            }
            _ => {}
        } 
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        _bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        let buffer = &self.editor.buf;
        let font_dimensions = buffer.get_font_dimensions();
        Size::new((buffer.width * font_dimensions.width) as f64, (buffer.height * font_dimensions.height) as f64)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &AppState, _env: &Env) {
        let buffer = &self.editor.buf;
        let font_dimensions = buffer.get_font_dimensions();
        for y in 0..buffer.height {
            for x in 0..buffer.width {
                let rect  = Rect::new(
                    (x * font_dimensions.width) as f64 + 0.5,  
                    (y * font_dimensions.height) as f64 + 0.5, 
                    ((x + 1) * font_dimensions.width) as f64 + 0.5, 
                    ((y + 1) * font_dimensions.height) as f64 + 0.5);
                if !ctx.region().intersects(rect) {
                    continue;
                }
                let ch = buffer.get_char(Position::from(x as i32, y as i32));
                let bg = buffer.get_rgb(ch.attribute.get_background());
                let mut rgba = 0;
                rgba |= bg.0 as u32;
                rgba <<=8;
                rgba |= bg.1 as u32;
                rgba <<=8;
                rgba |= bg.2 as u32;
                rgba <<=8;
                rgba |=  0xFF;
                ctx.fill(rect, &Color::from_rgba32_u32(rgba));

                let key = (ch.char_code, ch.attribute.as_u8());
                if let std::collections::hash_map::Entry::Vacant(e) = self.hash.entry(key) {
                    let image_data = &self.chars[ch.attribute.get_foreground() as usize * 256 + ch.char_code as usize];
                    let image = ctx
                        .make_image(font_dimensions.width, font_dimensions.height, image_data, ImageFormat::RgbaSeparate)
                        .unwrap();
                    e.insert(image);
                }
                ctx.draw_image(self.hash.get(&key).unwrap(), rect, InterpolationMode::Bilinear);
            }
        }
        let x = self.editor.cursor.pos.x as usize;
        let y = self.editor.cursor.pos.y as usize;
        
        let rect  = Rect::new(
            (x * font_dimensions.width) as f64 + 0.5,  
            (y * font_dimensions.height + font_dimensions.height - 4) as f64 + 0.5, 
            ((x + 1) * font_dimensions.width) as f64 + 0.5, 
            ((y + 1) * font_dimensions.height) as f64 + 0.5);
        
        ctx.fill(rect, &Color::WHITE);
    }
}
