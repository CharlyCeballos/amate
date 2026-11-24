// No console window behind the editor in release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

const APP_NAME: &str = "Amate";
const INITIAL_WINDOW_SIZE: [f32; 2] = [960.0, 640.0];

struct Amate;

impl eframe::App for Amate {
    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(APP_NAME)
            .with_inner_size(INITIAL_WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(APP_NAME, options, Box::new(|_cc| Ok(Box::new(Amate))))
}
