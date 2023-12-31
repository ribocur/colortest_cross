#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{
    egui::{self, CursorIcon},
    epaint::Color32,
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let mut options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        always_on_top: true,
        ..Default::default()
    };
    options.maximized = true; // Maximize the window
    options.decorated = false; // Remove window decorations
    eframe::run_native("ColorTest", options, Box::new(|cc| Box::<MyApp>::default()))
}

struct MyApp {
    vis: egui::Visuals,
    colors: Vec<Color32>,
    current: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            vis: egui::Visuals {
                panel_fill: egui::Color32::RED,
                ..egui::Visuals::default()
            },
            colors: vec![
                // RED is default color
                egui::Color32::GREEN,
                egui::Color32::BLUE,
                egui::Color32::WHITE,
                egui::Color32::BLACK,
            ],
            current: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Destructure self to get individual components
        let Self {
            vis,
            colors,
            current,
        } = self;

        // Disable cursor icon
        // Not sure if can't be disabled outside loop
        ctx.set_cursor_icon(egui::CursorIcon::None);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Check for SPACE key to switch colors
            if ui.input(|i| i.key_pressed(egui::Key::Space)) {
                *vis = egui::Visuals {
                    panel_fill: colors[current.to_owned()],
                    ..egui::Visuals::default()
                };
                // Panic will close the app
                *current += 1;
            }
            // Update visuals
            ctx.set_visuals(vis.to_owned());
            // Check for Esc to quit app
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                _frame.close();
            }
        });
    }
}
