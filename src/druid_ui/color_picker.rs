
use druid::widget::{prelude::*};
use druid::{
    Rect, Color, MouseButton
};

use crate::model::{Buffer, TextAttribute};

use super::AppState;

pub struct ColorPicker {}

impl ColorPicker {
    pub fn new() -> Self {
        ColorPicker {}
    }
}

impl Widget<AppState> for ColorPicker {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, state: &mut AppState, _env: &Env) {
        if let Event::MouseDown(e) = event {
            let width = ctx.size().width as i32;
            let height = ctx.size().height as i32;
    
            let col = (e.pos.x as i32) / (width / 8);
            let row = (e.pos.y as i32) / (height / 2);
            let color = (col + row * 8) as u8;
            if let Some(editor) = state.get_current_editor()  {
                if e.button == MouseButton::Left {
                    editor.borrow_mut().cursor.attr.set_foreground(color);
                } else {
                    editor.borrow_mut().cursor.attr.set_background(color % 8);
                }
            };
    
            ctx.request_paint();
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

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        _bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        Size::new(200.0, 50.0)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, state: &AppState, _env: &Env) {
        let width = ctx.size().width as i32;
        let height = ctx.size().height as i32;

        for y in 0..2 {
            for x in 0..8 {
                let rect = Rect::new(
                    (x * width / 8) as f64 + 0.5,
                    (y * height / 2) as f64 + 0.5,
                    ((x + 1) * width / 8) as f64 + 0.5,
                    ((y + 1) * height / 2) as f64 + 0.5,
                );
                let bg = if let Some(editor) = state.get_current_editor()  {
                    editor.borrow_mut().buf.get_rgba_u32((x + y * 8) as u8)
                } else {
                    let col = Buffer::DOS_DEFAULT_PALETTE[(x + y * 8) as usize];
                    (col.0 as u32) << 24 | (col.1 as u32) << 16 | (col.2 as u32) << 8 | 0xFF
                };

                ctx.fill(rect, &Color::from_rgba32_u32(bg));
            }
        }
        let attr = if let Some(editor) = state.get_current_editor()  {
            editor.borrow_mut().cursor.attr
        } else {
            TextAttribute::DEFAULT
        };

        let marker_width = 6f64;
        let x = (attr.get_foreground() % 8) as i32;
        let y = (attr.get_foreground() / 8) as i32;

        let rect = Rect::new(
            (x * width / 8) as f64 + 0.5,
            (y * height / 2) as f64 + 0.5,
            (x * width / 8) as f64 + 0.5 + marker_width,
            (y * height / 2) as f64 + 0.5+ marker_width,
        );
        ctx.fill(rect, &Color::BLUE);
        
        let x = (attr.get_background() % 8) as i32;
        let y = (attr.get_background() / 8) as i32;

        let rect = Rect::new(
            ((x + 1) * width / 8) as f64 + 0.5 - marker_width,
            ((y + 1) * height / 2) as f64 + 0.5 - marker_width,
            ((x + 1) * width / 8) as f64 + 0.5,
            ((y + 1) * height / 2) as f64 + 0.5,
        );
        ctx.fill(rect, &Color::BLUE);
    }
}
