mod file_manager;
mod gui;

use eframe::egui;
use eframe::IconData;

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        icon_data: Some(load_icon("icons/icon.png")),
        ..Default::default()
    };
    eframe::run_native(
        "M3U Cleaner",
        native_options,
        Box::new(|_cc| Box::new(gui::MyApp::default())),
    );
}

fn load_icon(path: &str) -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path).expect("Failed to open icon path").into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}