use eframe::egui;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

#[derive(Default)]
struct MyApp {
    groups: Vec<String>,
    selected_group_index: Option<usize>,
    file_path: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("M3U Cleaner");

            if ui.button("Charger les groupes").clicked() {
                if let Ok(groups) = read_groups(&self.file_path) {
                    self.groups = groups;
                }
            }

            ui.label("Groupes disponibles :");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (index, group) in self.groups.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.selected_group_index, Some(index), group);
                    });
                }
            });

            if let Some(selected_index) = self.selected_group_index {
                if ui.button("Supprimer le groupe sélectionné").clicked() {
                    let group_to_remove = &self.groups[selected_index];
                    if let Ok(_) = clean_m3u(&self.file_path, group_to_remove) {
                        self.groups.remove(selected_index);
                        self.selected_group_index = None;
                    }
                }
            }

            ui.label("Chemin du fichier M3U :");
            ui.text_edit_singleline(&mut self.file_path);

            if ui.button("Quitter").clicked() {
                _frame.close();
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "M3U Cleaner",
        native_options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
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
