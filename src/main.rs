use std::env;
use utils::{format_size, readable_systemtime};

use fs::items::{Item};

use crate::fs::dir_size;

mod fs;
mod utils;

pub struct Options {
    recursive: bool,
    print_all: bool,
    show_created_date: bool,
    show_last_accessed_date: bool,
}

fn print_item_info(options: &Options, size: String, modified: String, last_accessed: String, created: String, item: &Item, new_prefix: String) {
    println!(
        "{:<15} {} {}",
        size,
        format!(
            "{:<20}{}{}",
            modified,
            if options.show_last_accessed_date {
                format!("{:<20}", last_accessed)
            } else {
                String::new()
            },
            if options.show_created_date {
                format!("{:<20}", created)
            } else {
                String::new()
            }
        ),

        match item {
            Item::File(file) => format!("{} 📄 {}", new_prefix, file.name),
            Item::Directory(dir) => format!("{} 📁 {}", new_prefix, dir.name),
        }
    );
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
            Some(modified) => readable_systemtime(modified),
            None => "-".to_string(),
        };

        let last_accessed = match metadata.accessed {
            Some(last_accessed) => readable_systemtime(last_accessed),
            None => "-".to_string(),
        };

        let created = match metadata.created {
            Some(created) => readable_systemtime(created),
            None => "-".to_string(),
        };

        let size = match metadata.size {
            Some(size) => format_size(size),
            None => "-".to_string(),
        };

        match item {
            Item::File(_) => {
                print_item_info(options, size, modified, last_accessed, created, item, new_prefix);
            }
            Item::Directory(dir) => {
                print_item_info(options, format_size(if options.recursive { dir_size(dir) } else { 0 }), modified, last_accessed, created, item, new_prefix);

                let new_prefix = if i == items.len() - 1 {
                    format!("{}  ", prefix)
                } else {
                    format!("{}│ ", prefix)
                };

                if options.recursive && options.print_all {
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
    let print_all = args.contains(&String::from("--all")) || args.contains(&String::from("-a"));
    let show_created_date = args.contains(&String::from("--show-created--date"))
        || args.contains(&String::from("-scd"));
    let show_last_accessed_date = args.contains(&String::from("--show-last-accessed-date"))
        || args.contains(&String::from("-slad"));

    let options = &Options {
        recursive: recursive,
        print_all: print_all,
        show_created_date: show_created_date,
        show_last_accessed_date: show_last_accessed_date,
    };

    println!(
        "{:<15} {} {}",
        "Size",
        format!(
            "{:<20}{}{}",
            "Last Modified",
            if options.show_last_accessed_date {
                format!("{:<20}", "Last Accessed")
            } else {
                String::new()
            },
            if options.show_created_date {
                format!("{:<20}", "Created At")
            } else {
                String::new()
            }
        ),
        "Tree"
    );

    match fs::read_dir(".", options) {
        Ok(vec) => {
            print_tree(&vec, "", options);
        }
        Err(e) => println!("{:?}", e),
    }
}
