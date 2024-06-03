use crate::file_manager;
use eframe::egui;
use eframe::egui::{TextureHandle, TextureOptions};
use rfd::FileDialog;
use std::collections::{HashSet, HashMap};

pub struct MyApp {
    pub groups: HashMap<String, Vec<String>>,
    pub selected_groups: HashSet<String>,
    pub selected_group_name: Option<String>,
    pub file_path: String,
    pub show_error: bool,
    pub error_message: String,
    pub open_icon: Option<TextureHandle>,
    pub save_icon: Option<TextureHandle>,
    pub delete_icon: Option<TextureHandle>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            groups: HashMap::new(),
            selected_groups: HashSet::new(),
            selected_group_name: None,
            file_path: String::new(),
            show_error: false,
            error_message: String::new(),
            open_icon: None,
            save_icon: None,
            delete_icon: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.open_icon.is_none() {
            self.open_icon = load_icon(ctx, "icons/open.ico");
        }
        if self.save_icon.is_none() {
            self.save_icon = load_icon(ctx, "icons/save.ico");
        }
        if self.delete_icon.is_none() {
            self.delete_icon = load_icon(ctx, "icons/delete.ico");
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
                        if let Some(path) = FileDialog::new().pick_file() {
                            self.file_path = path.display().to_string();
                            match file_manager::read_groups(&self.file_path) {
                                Ok(groups) => {
                                    self.groups = groups;
                                    self.selected_groups.clear();
                                    self.error_message.clear();
                                },
                                Err(e) => {
                                    self.error_message = format!("Erreur lors de la lecture du fichier: {}", e);
                                    self.show_error = true;
                                }
                            }
                        }
                    }
                    if ui.button("Sauvegarder").clicked() {
                        if let Some(path) = FileDialog::new().save_file() {
                            if let Err(e) = file_manager::save_m3u(&self.file_path, &self.groups, &path) {
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
            });
        });

        egui::TopBottomPanel::top("action_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(open_icon) = &self.open_icon {
                    if ui.add(egui::ImageButton::new(open_icon.id(), [24.0, 24.0])).on_hover_text("Ouvrir un fichier").clicked() {
                        if let Some(path) = FileDialog::new().pick_file() {
                            self.file_path = path.display().to_string();
                            match file_manager::read_groups(&self.file_path) {
                                Ok(groups) => {
                                    self.groups = groups;
                                    self.selected_groups.clear();
                                    self.error_message.clear();
                                },
                                Err(e) => {
                                    self.error_message = format!("Erreur lors de la lecture du fichier: {}", e);
                                    self.show_error = true;
                                }
                            }
                        }
                    }
                }

                if let Some(save_icon) = &self.save_icon {
                    if ui.add(egui::ImageButton::new(save_icon.id(), [24.0, 24.0])).on_hover_text("Sauvegarder le fichier").clicked() {
                        if let Some(path) = FileDialog::new().save_file() {
                            if let Err(e) = file_manager::save_m3u(&self.file_path, &self.groups, &path) {
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
                        for group in groups_to_remove {
                            self.groups.remove(&group);
                        }
                        self.selected_groups.clear();
                    }
                }
            });
        });

        egui::SidePanel::left("group_list").resizable(true).default_width(300.0).show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Groupes disponibles");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Nom du Groupe");
                    ui.label("Nombre de chaînes");
                    ui.label("Action");
                });
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (group, channels) in &self.groups {
                        ui.horizontal(|ui| {
                            ui.group(|ui| {
                                let mut checked = self.selected_groups.contains(group);
                                if ui.checkbox(&mut checked, group).clicked() {
                                    if checked {
                                        self.selected_groups.insert(group.clone());
                                    } else {
                                        self.selected_groups.remove(group);
                                    }
                                }
                                ui.label(&format!("{}", channels.len()));
                                if ui.button("Voir").clicked() {
                                    self.selected_group_name = Some(group.clone());
                                }
                            });
                        });
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Chaînes");
                ui.separator();
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        if let Some(selected_group_name) = &self.selected_group_name {
                            if let Some(channels) = self.groups.get(selected_group_name) {
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

fn load_icon(ctx: &egui::Context, path: &str) -> Option<TextureHandle> {
    let image = image::open(path).ok()?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();
    Some(ctx.load_texture(
        path,
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        TextureOptions::default(),
    ))
}
