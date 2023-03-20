use std::env;

use prints::{print_grid, print_tree, PrintShow};

mod fs;
mod prints;
mod utils;

pub struct Options {
    show: PrintShow,
    recursive: bool,
    print_all: bool,
    show_created_date: bool,
    show_last_accessed_date: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let show = if args.iter().any(|x| x.contains("--show")) {
        let index = args.iter().position(|x| x.starts_with("--show")).unwrap();
        if args[index].contains('=') {
            args[index].split('=').collect::<Vec<&str>>()[1].to_string()
        } else {
            args[index + 1].clone()
        }
    } else {
        String::from("grid")
    };

    let recursive =
        args.contains(&String::from("--recursive")) || args.contains(&String::from("-r"));
    let print_all = args.contains(&String::from("--all")) || args.contains(&String::from("-a"));
    let show_created_date = args.contains(&String::from("--show-created--date"))
        || args.contains(&String::from("-scd"));
    let show_last_accessed_date = args.contains(&String::from("--show-last-accessed-date"))
        || args.contains(&String::from("-slad"));

    let options = &Options {
        show: match show.as_str() {
            "grid" => PrintShow::Grid,
            "tree" => PrintShow::Tree,
            _ => PrintShow::Grid,
        },
        recursive,
        print_all,
        show_created_date,
        show_last_accessed_date,
    };

    match fs::read_dir(".", options) {
        Ok(vec) => {
            match options.show {
                PrintShow::Tree => {
                    print_tree(&vec, "", options);
                }
                PrintShow::Grid => {
                    print_grid(&vec, options);
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
