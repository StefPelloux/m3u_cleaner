use crate::file_manager;
use crate::file_manager::save_m3u;
use eframe::egui;
use eframe::egui::{TextureHandle, Visuals};
use eframe::egui::Context;
use eframe::egui::TextureOptions;
use rfd::FileDialog;
use std::collections::{HashSet, HashMap};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct MyApp {
    pub groups: Arc<Mutex<HashMap<String, Vec<String>>>>,
    pub selected_groups: HashSet<String>,
    pub selected_group_name: Option<String>,
    pub file_path: String,
    pub show_error: bool,
    pub error_message: String,
    pub open_icon: Option<TextureHandle>,
    pub save_icon: Option<TextureHandle>,
    pub delete_icon: Option<TextureHandle>,
    pub is_dark_mode: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            groups: Arc::new(Mutex::new(HashMap::new())),
            selected_groups: HashSet::new(),
            selected_group_name: None,
            file_path: String::new(),
            show_error: false,
            error_message: String::new(),
            open_icon: None,
            save_icon: None,
            delete_icon: None,
            is_dark_mode: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Charger les icônes si elles ne sont pas encore chargées
        if self.open_icon.is_none() {
            self.open_icon = Some(load_icon(ctx, include_bytes!("../icons/open.ico")));
        }
        if self.save_icon.is_none() {
            self.save_icon = Some(load_icon(ctx, include_bytes!("../icons/save.ico")));
        }
        if self.delete_icon.is_none() {
            self.delete_icon = Some(load_icon(ctx, include_bytes!("../icons/delete.ico")));
        }

        let mut show_error = self.show_error;
        if show_error {
            egui::Window::new("Erreur").open(&mut show_error).show(ctx, |ui| {
                ui.label(&self.error_message);
                if ui.button("OK").clicked() {
                    self.show_error = false;
                }
            });
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Fichier", |ui| {
                    if ui.button("Ouvrir").clicked() {
                        self.start_file_loading(ctx.clone());
                    }
                    if ui.button("Sauvegarder").clicked() {
                        if let Some(path) = FileDialog::new().save_file() {
                            if let Err(e) = save_m3u(&self.file_path, &self.groups.lock().unwrap(), &path.display().to_string()) {
                                self.error_message = format!("Erreur lors de la sauvegarde du fichier: {}", e);
                                self.show_error = true;
                            } else {
                                self.error_message.clear();
                            }
                        }
                    }
                    if ui.button("Quitter").clicked() {
                        frame.close();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.menu_button("Thème", |ui| {
                        if ui.button("Clair").clicked() {
                            self.is_dark_mode = false;
                            ctx.set_visuals(Visuals::light());
                        }
                        if ui.button("Sombre").clicked() {
                            self.is_dark_mode = true;
                            ctx.set_visuals(Visuals::dark());
                        }
                    });
                });
            });
        });

        egui::TopBottomPanel::top("action_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(open_icon) = &self.open_icon {
                    if ui.add(egui::ImageButton::new(open_icon.id(), [24.0, 24.0])).on_hover_text("Ouvrir un fichier").clicked() {
                        self.start_file_loading(ctx.clone());
                    }
                }

                if let Some(save_icon) = &self.save_icon {
                    if ui.add(egui::ImageButton::new(save_icon.id(), [24.0, 24.0])).on_hover_text("Sauvegarder le fichier").clicked() {
                        if let Some(path) = FileDialog::new().save_file() {
                            if let Err(e) = save_m3u(&self.file_path, &self.groups.lock().unwrap(), &path.display().to_string()) {
                                self.error_message = format!("Erreur lors de la sauvegarde du fichier: {}", e);
                                self.show_error = true;
                            } else {
                                self.error_message.clear();
                            }
                        }
                    }
                }

                if let Some(delete_icon) = &self.delete_icon {
                    if ui.add(egui::ImageButton::new(delete_icon.id(), [24.0, 24.0])).on_hover_text("Supprimer les groupes sélectionnés").clicked() {
                        let groups_to_remove: Vec<_> = self.selected_groups.iter().cloned().collect();
                        let mut groups = self.groups.lock().unwrap();
                        for group in groups_to_remove {
                            groups.remove(&group);
                        }
                        self.selected_groups.clear();
                    }
                }
            });
        });

        let max_group_name_length = self.groups.lock().unwrap().keys().map(|name| name.len()).max().unwrap_or(0);

        egui::SidePanel::left("group_list").resizable(true).default_width(300.0).show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Groupes disponibles");
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("group_grid")
                        .striped(true)
                        .min_col_width(150.0)
                        .show(ui, |ui| {
                            ui.label("Nom du Groupe");
                            ui.label("Nombre de chaînes");
                            ui.label("Action");
                            ui.end_row();

                            let groups = self.groups.lock().unwrap();
                            for (group, channels) in groups.iter() {
                                ui.horizontal(|ui| {
                                    let mut checked = self.selected_groups.contains(group);
                                    ui.checkbox(&mut checked, "");
                                    if checked {
                                        self.selected_groups.insert(group.clone());
                                    } else {
                                        self.selected_groups.remove(group);
                                    }
                                    let padded_group_name = format!("{:width$}", group, width = max_group_name_length + 5);
                                    ui.label(padded_group_name);
                                    ui.label(&format!("{}", channels.len()));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                                        if ui.button("Voir").clicked() {
                                            self.selected_group_name = Some(group.clone());
                                        }
                                    });
                                });
                                ui.end_row();
                            }
                        });
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Chaînes");
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                        if let Some(selected_group_name) = &self.selected_group_name {
                            if let Some(channels) = self.groups.lock().unwrap().get(selected_group_name) {
                                for channel in channels {
                                    ui.label(channel);
                                }
                            }
                        } else {
                            ui.label("Veuillez sélectionner un groupe pour voir les chaînes.");
                        }
                    });
                });
            });
        });
    }
}

impl MyApp {
    fn start_file_loading(&mut self, ctx: egui::Context) {
        if let Some(path) = FileDialog::new().pick_file() {
            self.file_path = path.display().to_string();
            let groups = Arc::clone(&self.groups);
            let error_message = Arc::new(Mutex::new(None));

            thread::spawn({
                let ctx_clone = ctx.clone();
                let error_message_clone = Arc::clone(&error_message);
                move || {
                    match file_manager::read_groups(&path.display().to_string()) {
                        Ok(file_groups) => {
                            let mut groups = groups.lock().unwrap();
                            *groups = file_groups;
                        }
                        Err(e) => {
                            let mut error_message = error_message_clone.lock().unwrap();
                            *error_message = Some(format!("Erreur lors de la lecture du fichier: {}", e));
                        }
                    }
                    ctx_clone.request_repaint();
                }
            });

            let error_message = Arc::clone(&error_message);
            let ctx_clone = ctx.clone();
            thread::spawn(move || loop {
                if error_message.lock().unwrap().is_some() {
                    ctx_clone.request_repaint();
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            });
        }
    }
}

// Fonction pour charger les icônes à partir des données intégrées
fn load_icon(ctx: &Context, image_data: &[u8]) -> TextureHandle {
    let image = image::load_from_memory(image_data).expect("Failed to load icon").to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();
    ctx.load_texture(
        "icon_texture",
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        TextureOptions::default(),
    )
}
