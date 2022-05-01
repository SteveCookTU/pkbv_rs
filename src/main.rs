#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use pkbv_rs::my_app::MyApp;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 300.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native("pkBV-rs", options, Box::new(|cc| Box::new(MyApp::new(cc))));
}
