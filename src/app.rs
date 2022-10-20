use std::{collections::HashMap};
use eframe::{egui::{self, ScrollArea, Sense}, epaint::{Rect, Pos2, Rounding, Color32, ColorImage, Shape, TextureHandle, Vec2, Mesh, pos2}, App, Frame};
use crate::model::{Buffer, Editor, Position};

pub struct MysticDrawApp {
    editor: Editor,
    chars: Vec<Vec<u8>>,
    hash : HashMap<u16, TextureHandle>
}

impl Default for MysticDrawApp {

    fn default() -> Self {
        let buffer = Buffer::load_buffer(std::path::Path::new("/home/mkrueger/work/test.xb")).unwrap();
        let editor = crate::model::Editor::new(0, buffer);
        let mut chars = Vec::new();
        let font_dimensions = editor.buf.get_font_dimensions();
        for ch in 0..=255 {
            let mut result = vec![0; font_dimensions.width * font_dimensions.height * 4];
            let mut i = 0;
            for y in 0..font_dimensions.height {
                let line = editor.buf.get_font_scanline(ch, y as usize);
                for x in 0..font_dimensions.width {
                    if (line & (128 >> x)) != 0 {
                        result[i] = 255;
                        i += 1;
                        result[i] = 255;
                        i += 1;
                        result[i] = 255;
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
            chars.push(result);    
        }

        Self {
            editor,
            chars,
            hash: HashMap::new()
        }
    }
}

impl eframe::App for MysticDrawApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        let Self { editor, chars, hash } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            draw_paint_area(ui, editor, chars, hash);
        });

    }

}

fn draw_paint_area(ui: &mut egui::Ui, editor: &Editor, chars: &Vec<Vec<u8>>, hash: &mut HashMap<u16, TextureHandle>) {
    let buffer = &editor.buf;
    let font_dimensions = buffer.get_font_dimensions();
    let width = (buffer.width * font_dimensions.width) as f32;
    let height = (buffer.height * font_dimensions.height) as f32;

    ScrollArea::both()
        .auto_shrink([false; 2])
        .show_viewport(ui, move |ui, viewport| {
            ui.set_height(width);
            ui.set_width(height);

            let used_rect = Rect::NOTHING;
            
            let x1 = (viewport.min.x as usize) / font_dimensions.width;
            let x2 = (viewport.max.x / font_dimensions.width as f32).ceil() as usize + 1;
            let y1 = (viewport.min.y as usize) / font_dimensions.height;
            let y2 = (viewport.max.y / font_dimensions.height as f32).ceil() as usize + 1;

            let left = ui.min_rect().left();
            let top =  ui.min_rect().top();

            let (_, painter) = ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());


            let size = Vec2 {
                x: font_dimensions.width as f32, 
                y: font_dimensions.height as f32
            };
            let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
            for y in y1..=y2 {
                for x in x1..=x2 {
                    let rect  = Rect::from_min_size(Pos2 {
                        x: (left + (x * font_dimensions.width) as f32).floor() + 0.5,  
                        y: (top + (y * font_dimensions.height) as f32).floor() + 0.5},
                        size
                    );
                    let ch = buffer.get_char(Position::from(x as i32, y as i32));
                    let fg = buffer.get_rgb(ch.attribute.get_foreground());
                    let bg = buffer.get_rgb(ch.attribute.get_background());

                    painter.add(Shape::rect_filled(rect, Rounding::none(), Color32::from_rgb(bg.0,  bg.1, bg.2)));

                    let tex = hash.entry(ch.char_code as u16).or_insert_with(|| {
                        let image_data = &chars[ch.char_code as usize];
                        let pixels = image_data
                            .chunks_exact(4)
                            .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                            .collect();
                        let image = ColorImage { size: [8, 16], pixels };
                        let handle = painter.ctx().load_texture("name", image, egui::TextureFilter::Linear);
                        handle
                    });

                    let mut mesh = Mesh::with_texture(tex.id());
                    mesh.add_rect_with_uv(rect, uv, Color32::from_rgb(fg.0, fg.1, fg.2));

                    painter.add(Shape::mesh(mesh));
                }
            }

            ui.allocate_rect(used_rect, Sense::hover());
    });
}