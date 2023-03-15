use chrono::{self};
use utils::format_size;
use std::env;

use fs::items::Item;

use crate::fs::dir_size;

mod fs;
mod utils;

fn print_tree(items: &Vec<Item>, prefix: &str, recursive: bool) {
    for (i, item) in items.iter().enumerate() {
        let metadata = match item {
            Item::File(file) => &file.metadata,
            Item::Directory(dir) => &dir.metadata,
        };

        let mut new_prefix = prefix.to_string();
        if i == items.len() - 1 {
            new_prefix.push_str("└─");
        } else {
            new_prefix.push_str("├─");
        }

        let modified = match metadata.modified {
            Some(modified) => chrono::DateTime::<chrono::Local>::from(modified)
                .format("%b %e %H:%M")
                .to_string(),
            None => "-".to_string(),
        };

        let size = match metadata.size {
            Some(size) => format_size(size),
            None => "-".to_string(),
        };

        match item {
            Item::File(file) => {
                println!(
                    "{:<15} {:<20} {}",
                    size,
                    modified,
                    format!("{} 📄 {}", new_prefix, file.name)
                );
            }
            Item::Directory(dir) => {
                println!(
                    "{:<15} {:<20} {}",
                    format_size(dir_size(dir)),
                    modified,
                    format!("{} 📁 {}", new_prefix, dir.name)
                );

                let new_prefix = if i == items.len() - 1 {
                    format!("{}  ", prefix)
                } else {
                    format!("{}│ ", prefix)
                };

                if recursive {
                    print_tree(&dir.files, &new_prefix, recursive);
                }
            }
        }
    }
}

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let recursive = args.contains(&String::from("--recursive")) || args.contains(&String::from("-r"));

    match fs::read_dir(
        ".",
    ) {
        Ok(vec) => {
            println!("{:<15} {:<20} {}", "Size", "Last Modified", "Tree");
            print_tree(&vec, "", recursive);
        }
        Err(e) => println!("{:?}", e),
    }
}
