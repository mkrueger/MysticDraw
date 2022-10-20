#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use mystic_draw::*;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = mystic_draw::MysticDrawApp::default();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<MysticDrawApp>()
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_system(ui_example)
        .run();
}

fn ui_example(mut egui_ctx: ResMut<EguiContext>, mut ui_state: ResMut<MysticDrawApp>,) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        draw_paint_area(ui, ui_state.as_mut());
    });
}
