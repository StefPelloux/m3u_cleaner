mod file_manager;
mod gui;
//mod icons;

use eframe::egui;
use eframe::IconData;

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 600.0)),
        icon_data: Some(load_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "M3U Cleaner",
        native_options,
        Box::new(|_cc| Box::new(gui::MyApp::default())),
    );
}

fn load_icon() -> IconData {
    let image_data = include_bytes!("../icons/icon.png").as_ref();
    let image = image::load_from_memory(image_data).expect("Failed to load icon from embedded data").into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    IconData {
        rgba,
        width,
        height,
    }
}
