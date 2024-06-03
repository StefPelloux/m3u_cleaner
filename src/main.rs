use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use regex::Regex;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let path = "exemple.m3u";
    let re = Regex::new(r#"group-title="([^"]+)""#).unwrap();
    
    loop {
        let groups = read_groups(&path, &re)?;
        if groups.is_empty() {
            println!("Aucun groupe trouvé.");
            break;
        }

        println!("Groupes trouvés :");
        for (index, group) in groups.iter().enumerate() {
            println!("{}: {}", index + 1, group);
        }

        // Demander à l'utilisateur de spécifier un groupe à supprimer
        let mut input = String::new();
        println!("Entrez le numéro du groupe à supprimer (0 pour sortir) : ");
        io::stdin().read_line(&mut input)?;
        let input: usize = input.trim().parse().unwrap_or(0);

        if input == 0 {
            println!("Sortie du programme.");
            break;
        } else if input > 0 && input <= groups.len() {
            let group_to_remove = &groups[input - 1];
            clean_m3u(&path, &re, group_to_remove)?;
            println!("Groupe '{}' supprimé.", group_to_remove);
        } else {
            println!("Numéro invalide, veuillez réessayer.");
        }
    }

    Ok(())
}

fn read_groups<P>(filename: P, re: &Regex) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
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

fn clean_m3u<P>(filename: P, re: &Regex, group_to_remove: &str) -> io::Result<()>
where P: AsRef<Path>, {
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
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
