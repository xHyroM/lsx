use chrono::{self};
use std::env;
use utils::format_size;

use fs::items::Item;

use crate::fs::dir_size;

mod fs;
mod utils;

pub struct Options {
    recursive: bool,
    disable_dir_size: bool,
}

fn print_tree(items: &Vec<Item>, prefix: &str, options: &Options) {
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
                    format_size(if !options.disable_dir_size {
                        dir_size(dir)
                    } else {
                        0
                    }),
                    modified,
                    format!("{} 📁 {}", new_prefix, dir.name)
                );

                let new_prefix = if i == items.len() - 1 {
                    format!("{}  ", prefix)
                } else {
                    format!("{}│ ", prefix)
                };

                if options.recursive {
                    print_tree(&dir.files, &new_prefix, &options);
                }
            }
        }
    }
}

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let recursive =
        args.contains(&String::from("--recursive")) || args.contains(&String::from("-r"));
    let disable_dir_size = (args.contains(&String::from("--disable-dir-size"))
        || args.contains(&String::from("-d")))
        && !recursive;
    let options = &Options {
        recursive: recursive,
        disable_dir_size: disable_dir_size,
    };

    match fs::read_dir(".", options) {
        Ok(vec) => {
            println!("{:<15} {:<20} {}", "Size", "Last Modified", "Tree");
            print_tree(&vec, "", options);
        }
        Err(e) => println!("{:?}", e),
    }
}
