use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use druid::piet::{ImageFormat, InterpolationMode };
use druid::widget::{prelude::*};
use druid::{
    Rect, Color, MouseButton, Point, KbKey
};

use crate::model::{ Position, Editor, TOOLS, MKey, MModifiers};

use super::AppState;

pub struct AnsiWidget
{
    editor: Rc<RefCell<Editor>>,
    chars: Vec<Vec<u8>>,
    #[cfg(target_os = "linux")]
    hash: HashMap<(u8, u8), druid::piet::CairoImage>,
    #[cfg(target_os = "macos")]
    hash: HashMap<(u8, u8), druid::piet::CoreGraphicsImage>,
}

impl AnsiWidget
{
    pub fn new(editor: Rc<RefCell<Editor>>) -> Self {
        AnsiWidget {
            editor,
            chars: Vec::new(),
            hash: HashMap::new()
        }
    }

    pub fn initialize(&mut self) {
        let buffer = &(self.editor.borrow_mut()).buf;
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
        let buffer = &(self.editor.borrow_mut()).buf;

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
                let button = match e.button {
                    MouseButton::Left => 1,
                    MouseButton::Middle => 2,
                    MouseButton::Right => 3,
                    _ => 0
                };
                if button > 0 {
                    let pos_opt = self.get_buffer_pos(e.pos);
                    if let Some(pos) = pos_opt {
                        unsafe {
                            TOOLS[_data.cur_tool].handle_click(self.editor.clone(), button, pos);
                        }
                    }
                }
                _ctx.request_focus();
                _ctx.request_paint();
            }
            Event::MouseMove(_e) => {

            }
            Event::MouseUp(_e) => {

            }
            Event::KeyDown(e) => {
                unsafe {
                    println!("key down!");
                    if let Some(key) = transform_key(&e.key) {
                        println!("handle!");
                        TOOLS[_data.cur_tool].handle_key(self.editor.clone(), key,   transform_mod(e.mods));
                    }
                    _ctx.request_paint();

                }
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
        let buffer = &(self.editor.borrow_mut()).buf;
        let font_dimensions = buffer.get_font_dimensions();
        Size::new((buffer.width * font_dimensions.width) as f64, (buffer.height * font_dimensions.height) as f64)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &AppState, _env: &Env) {
        let editor = self.editor.borrow_mut();
        let buffer = &editor.buf;
        let font_dimensions = buffer.get_font_dimensions();
        let mut rects = Vec::new();
        {
            let paint_rects = ctx.region().rects();
            if paint_rects.is_empty() {
                return;
            }
            // for some reason this gets filled with duplicates - sometimes >200, so draw only the first rect.
            rects.push(paint_rects[0]);
            /* 
            for r in  paint_rects {
                rects.push(*r);
            }*/
        }

        for r in rects {
            let x1 = (r.x0 as usize) / font_dimensions.width;
            let x2 = (r.x1 as usize) / font_dimensions.width + 1;
            let y1 = (r.y0 as usize) / font_dimensions.height;
            let y2 = (r.y1 as usize) / font_dimensions.height + 1;

            for y in y1..=y2 {
                for x in x1..=x2 {
                    let rect  = Rect::new(
                        (x * font_dimensions.width) as f64 + 0.5,  
                        (y * font_dimensions.height) as f64 + 0.5, 
                        ((x + 1) * font_dimensions.width) as f64 + 0.5, 
                        ((y + 1) * font_dimensions.height) as f64 + 0.5);
                    let ch = buffer.get_char(Position::from(x as i32, y as i32));
                    let bg = buffer.get_rgba_u32(ch.attribute.get_background());
                    ctx.fill(rect, &Color::from_rgba32_u32(bg));

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
        }
        let x = editor.cursor.pos.x as usize;
        let y = editor.cursor.pos.y as usize;
        
        let rect  = Rect::new(
            (x * font_dimensions.width) as f64 + 0.5,  
            (y * font_dimensions.height + font_dimensions.height - 4) as f64 + 0.5, 
            ((x + 1) * font_dimensions.width) as f64 + 0.5, 
            ((y + 1) * font_dimensions.height) as f64 + 0.5);
        
        ctx.fill(rect, &Color::WHITE);
    }

    fn id(&self) -> Option<WidgetId> {
        None
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn short_type_name(&self) -> &'static str {
        let name = self.type_name();
        name.split('<')
            .next()
            .unwrap_or(name)
            .split("::")
            .last()
            .unwrap_or(name)
    }

    fn debug_state(&self, data: &AppState) -> druid::debug_state::DebugState {
        #![allow(unused_variables)]
        druid::debug_state::DebugState {
            display_name: self.short_type_name().to_string(),
            ..Default::default()
        }
    }
}

fn transform_mod(mods: druid::Modifiers) -> MModifiers {
    match mods {
        druid::Modifiers::ALT => MModifiers::Alt,
        druid::Modifiers::SHIFT => MModifiers::Shift,
        druid::Modifiers::CONTROL => MModifiers::Control,
        _ => { MModifiers::None }
    }
}

fn transform_key(key: &KbKey) -> Option<MKey>
{
    match key {
        druid::keyboard_types::Key::Character(c) => {
            let bytes  = c.as_bytes();
            if bytes.len() != 1 { return None; }
            Some(MKey::Character(bytes[0]))
        },
        druid::keyboard_types::Key::Enter => Some(MKey::Return),
        druid::keyboard_types::Key::Tab => Some(MKey::Tab),
        druid::keyboard_types::Key::ArrowDown => Some(MKey::Down),
        druid::keyboard_types::Key::ArrowLeft => Some(MKey::Left),
        druid::keyboard_types::Key::ArrowRight => Some(MKey::Right),
        druid::keyboard_types::Key::ArrowUp => Some(MKey::Up),
        druid::keyboard_types::Key::End => Some(MKey::End),
        druid::keyboard_types::Key::Home => Some(MKey::Home),
        druid::keyboard_types::Key::PageDown => Some(MKey::PageDown),
        druid::keyboard_types::Key::PageUp => Some(MKey::PageUp),
        druid::keyboard_types::Key::Backspace => Some(MKey::Backspace),
        druid::keyboard_types::Key::Delete => Some(MKey::Delete),
        druid::keyboard_types::Key::Insert => Some(MKey::Insert),
        druid::keyboard_types::Key::Escape => Some(MKey::Escape),

        druid::keyboard_types::Key::F1 => Some(MKey::F1),
        druid::keyboard_types::Key::F2 => Some(MKey::F2),
        druid::keyboard_types::Key::F3 => Some(MKey::F3),
        druid::keyboard_types::Key::F4 => Some(MKey::F4),
        druid::keyboard_types::Key::F5 => Some(MKey::F5),
        druid::keyboard_types::Key::F6 => Some(MKey::F6),
        druid::keyboard_types::Key::F7 => Some(MKey::F7),
        druid::keyboard_types::Key::F8 => Some(MKey::F8),
        druid::keyboard_types::Key::F9 => Some(MKey::F9),
        druid::keyboard_types::Key::F10 => Some(MKey::F10),
        druid::keyboard_types::Key::F11 => Some(MKey::F11),
        druid::keyboard_types::Key::F12 => Some(MKey::F12),
        
        _ => None
    }
}
