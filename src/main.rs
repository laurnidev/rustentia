#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
use app::*;

mod database;
mod deck;
mod flashcard;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Floating window",
        options,
        Box::new(|_cc| Box::<RunApp>::default()),
    )
}
