use eframe::egui;
use regex::Regex;
use rfd::FileDialog;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

#[derive(Default)]
pub struct MyApp {
    groups: Vec<String>,
    selected_groups: HashSet<usize>,
    file_path: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Fichier", |ui| {
                    if ui.button("Ouvrir").clicked() {
                        if let Some(path) = FileDialog::new().pick_file() {
                            self.file_path = path.display().to_string();
                            if let Ok(groups) = read_groups(&self.file_path) {
                                self.groups = groups;
                                self.selected_groups.clear();
                            }
                        }
                    }
                    if ui.button("Sauvegarder").clicked() {
                        if let Some(path) = FileDialog::new().save_file() {
                            std::fs::copy(&self.file_path, path).unwrap();
                        }
                    }
                    if ui.button("Quitter").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("M3U Cleaner");

            if !self.file_path.is_empty() {
                if ui.button("Supprimer les groupes sélectionnés").clicked() {
                    let groups_to_remove: Vec<_> = self.selected_groups.iter().map(|&i| self.groups[i].clone()).collect();
                    for group in groups_to_remove {
                        if let Ok(_) = clean_m3u(&self.file_path, &group) {
                            if let Some(pos) = self.groups.iter().position(|g| g == &group) {
                                self.groups.remove(pos);
                            }
                        }
                    }
                    self.selected_groups.clear();
                }

                ui.label("Groupes disponibles :");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (index, group) in self.groups.iter().enumerate() {
                        ui.horizontal(|ui| {
                            let mut checked = self.selected_groups.contains(&index);
                            if ui.checkbox(&mut checked, group).clicked() {
                                if checked {
                                    self.selected_groups.insert(index);
                                } else {
                                    self.selected_groups.remove(&index);
                                }
                            }
                        });
                    }
                });
            } else {
                ui.label("Veuillez sélectionner un fichier via le menu Fichier pour charger les groupes.");
            }
        });
    }
}

fn read_groups<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let re = Regex::new(r#"group-title="([^"]+)""#).unwrap();
    let mut groups = HashSet::new();
    if let Ok(lines) = read_lines(&filename) {
        for line in lines {
            if let Ok(content) = line {
                if content.starts_with("#EXTINF") {
                    if let Some(caps) = re.captures(&content) {
                        groups.insert(caps[1].to_string());
                    }
                }
            }
        }
    }
    let mut groups_vec: Vec<String> = groups.into_iter().collect();
    groups_vec.sort();
    Ok(groups_vec)
}

fn clean_m3u<P>(filename: P, group_to_remove: &str) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let re = Regex::new(r#"group-title="([^"]+)""#).unwrap();
    let mut output = String::new();
    let mut skip_next = false;
    if let Ok(lines) = read_lines(&filename) {
        for line in lines {
            if let Ok(content) = line {
                if skip_next {
                    skip_next = false;
                    continue;
                }
                if content.starts_with("#EXTINF") {
                    if let Some(caps) = re.captures(&content) {
                        if &caps[1] == group_to_remove {
                            skip_next = true;
                            continue;
                        }
                    }
                }
                output.push_str(&content);
                output.push('\n');
            }
        }
    }
    let mut file = File::create(filename)?;
    file.write_all(output.as_bytes())?;
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
