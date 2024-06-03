mod test_ui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "M3U Cleaner",
        native_options,
        Box::new(|_cc| Box::new(test_ui::MyApp::default())),
    );
}
