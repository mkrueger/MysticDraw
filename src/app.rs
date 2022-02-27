use std::{collections::HashMap, slice::SliceIndex};

use eframe::{egui::{self, ScrollArea, Sense}, epi, epaint::{Rect, Pos2, Rounding, Color32, image, ColorImage, Shape, TextureHandle}, emath};

use crate::model::{Buffer, Editor, Position};


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
    editor: Editor,
    chars: Vec<Vec<u8>>,
    hash : HashMap<(u8, u8), TextureHandle>
}

impl Default for TemplateApp {


    fn default() -> Self {
        let buffer = Buffer::load_buffer(std::path::Path::new("/home/mkrueger/Downloads/r5-25b.xb")).unwrap();
        let editor = crate::model::Editor::new(0, buffer);
        let mut chars = Vec::new();
        let font_dimensions = editor.buf.get_font_dimensions();
        for color in 0..16 {
            let fg = editor.buf.get_rgb(color);
            for ch in 0..=255 {
                let mut result = vec![0; font_dimensions.width * font_dimensions.height * 4];
                let mut i = 0;
                for y in 0..font_dimensions.height {
                    let line = editor.buf.get_font_scanline(ch, y as usize);
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
                chars.push(result);    
            }
        }

        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            editor,
            chars,
            hash: HashMap::new()
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { label, value, editor, chars, hash } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

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

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            draw_paint_area(ui, editor, chars, hash);

        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }

}

fn draw_paint_area(ui: &mut egui::Ui, editor: &Editor, chars: &Vec<Vec<u8>>, hash: &mut HashMap<(u8, u8), TextureHandle>) {
    let buffer = &editor.buf;
    let font_dimensions = buffer.get_font_dimensions();
    let width = (buffer.width * font_dimensions.width) as f32;
    let height = (buffer.height * font_dimensions.height) as f32;


    ScrollArea::both()
        .auto_shrink([false; 2])
        .show_viewport(ui, move |ui, viewport| {
            ui.set_height(width);
            ui.set_width(height);

            let mut used_rect = Rect::NOTHING;
            
            let x1 = (viewport.min.x as usize) / font_dimensions.width + 1;
            let x2 = (viewport.max.x as usize) / font_dimensions.width + 1;
            let y1 = (viewport.min.y as usize) / font_dimensions.height;
            let y2 = (viewport.max.y as usize) / font_dimensions.height + 1;
            let (mut response, painter) = ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

            let to_screen = emath::RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
                response.rect,
            );
            let from_screen = to_screen.inverse();
        
            for y in y1..=y2 {
                for x in x1..=x2 {
                    let rect  = Rect::from_two_pos(Pos2 {
                        x:((x * font_dimensions.width) as f32 - viewport.min.x).floor(),  
                        y:((y * font_dimensions.height) as f32 - viewport.min.y).floor()},
                        Pos2 {
                        x: (((x + 1) * font_dimensions.width) as f32 - viewport.min.x).floor(), 
                        y: (((y + 1) * font_dimensions.height) as f32  - viewport.min.y).floor()});
                    let ch = buffer.get_char(Position::from(x as i32, y as i32));
                    let bg = buffer.get_rgb(ch.attribute.get_background());
                    painter.rect_filled(rect, Rounding::none(), Color32::from_rgb(bg.0,  bg.1, bg.2));

                    let key = (ch.char_code, ch.attribute.as_u8());
                    let tex = hash.entry(key).or_insert_with(|| {
                        let image_data = &chars[ch.attribute.get_foreground() as usize * 256 + ch.char_code as usize];

                        let pixels = image_data
                        .chunks_exact(4)
                        .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                        .collect();
                        let image = ColorImage { size: [8, 16], pixels };
                        let handle = painter.ctx().load_texture("name", image);
                        handle
                    });
                    // Now I've a texture what to do with it?
                }
            }

            ui.allocate_rect(used_rect, Sense::hover()); // make sure it is visible!
    });

}

