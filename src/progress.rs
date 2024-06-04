use eframe::egui;

pub struct Progress {
    pub progress: f32,
    pub visible: bool,
}

impl Progress {
    pub fn new() -> Self {
        Self {
            progress: 0.0,
            visible: false,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if self.visible {
            egui::Window::new("Chargement en cours")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.add(egui::ProgressBar::new(self.progress).show_percentage());
                    if self.progress >= 1.0 {
                        ui.label("Chargement terminé!");
                        if ui.button("Fermer").clicked() {
                            self.visible = false;
                        }
                    }
                });
        }
    }
}
