use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

pub fn read_groups<P>(filename: P) -> io::Result<HashMap<String, Vec<String>>>
where
    P: AsRef<Path>,
{
    let re_group = Regex::new(r#"group-title="([^"]+)""#).unwrap();
    let re_name = Regex::new(r#"tvg-name="([^"]+)""#).unwrap();
    let mut groups = HashMap::new();
    if let Ok(lines) = read_lines(&filename) {
        let mut current_group = None;
        for line in lines {
            let line = line?;
            if line.starts_with("#EXTINF") {
                if let Some(caps) = re_group.captures(&line) {
                    current_group = Some(caps[1].to_string());
                }
                if let Some(caps) = re_name.captures(&line) {
                    let channel_name = caps[1].to_string();
                    if let Some(group) = &current_group {
                        groups.entry(group.clone()).or_insert_with(Vec::new).push(channel_name);
                    }
                }
            }
        }
    }
    Ok(groups)
}

pub fn save_m3u<P>(original_filename: &str, groups: &HashMap<String, Vec<String>>, save_path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let re = Regex::new(r#"group-title="([^"]+)""#).unwrap();
    let mut output = String::new();
    let lines = read_lines(original_filename)?;
    let mut current_group = None;
    for line in lines {
        let line = line?;
        if line.starts_with("#EXTINF") {
            if let Some(caps) = re.captures(&line) {
                current_group = Some(caps[1].to_string());
            }
        }
        if let Some(group) = &current_group {
            if groups.contains_key(group) {
                output.push_str(&line);
                output.push('\n');
            }
        } else if groups.values().any(|channels| channels.contains(&line)) {
            output.push_str(&line);
            output.push('\n');
        }
    }
    let mut file = File::create(save_path)?;
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
