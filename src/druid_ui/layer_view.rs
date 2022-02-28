use std::{rc::Rc, cell::RefCell};

use druid::kurbo::BezPath;
use druid::piet::{ImageFormat, InterpolationMode, Text, TextLayoutBuilder };
use druid::widget::{prelude::*};
use druid::{
    Rect, Color, MouseButton, Point, KbKey, TextLayout, FontFamily, FontDescriptor, Lens
};

use crate::model::Editor;

use super::AppState;

pub struct LayerView
{
}

impl LayerView
{
    pub fn new() -> Self {
        LayerView {
        }
    }
}

impl Widget<AppState> for LayerView {
    
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
                    
                }
                _ctx.request_focus();
                _ctx.request_paint();
            }
            Event::KeyDown(e) => {
               println!("key down!");
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

        let mut height = 100.0;

        if let Some(editor) = _data.get_current_editor()  {
            height = (1 + editor.borrow().buf.layers.len()) as f64 * 50.0;
        }

        Size::new(300.0, height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &AppState, _env: &Env) {
        let paint_rects = ctx.region().rects()[0];
        ctx.fill(paint_rects, &Color::WHITE);

        if let Some(editor) = _data.get_current_editor()  {
            let layers = &editor.borrow().buf.layers;

            let fill_color = Color::rgba8(0x00, 0x00, 0x00, 0x7F);
            let mut layout = TextLayout::<String>::from_text("");
            layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(24.0));
            layout.set_text_color(fill_color);

            for i in 0..layers.len() {
                let text = ctx.text();
                let layout = text
                .new_text_layout(layers[i].name.clone())
                .font(FontFamily::SANS_SERIF, 14.0)
                .text_color(Color::rgb8(0, 0, 0))
                .build()
                .unwrap();
                 ctx.draw_text(&layout, (100.0, i as f64 * 50.0 + 15.0));
                 
                 let mut path = BezPath::new();
                 let y = (i + 1) as f64 * 50.0;
                 path.move_to((0.0, y));
                 path.line_to((ctx.size().width, y));

                 let stroke_color = Color::rgb8(128, 0, 0);
                 ctx.stroke(path, &stroke_color, 3.0);
            }
        }
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
