use std::env;

use prints::{print_grid, print_long};

mod fs;
mod prints;
mod utils;

pub struct Options {
    long: bool,
    recursive: bool,
    print_all: bool,
    show_created_date: bool,
    show_last_accessed_date: bool,
}

fn main() -> () {
    let args: Vec<String> = env::args().collect();

    let long = args.contains(&String::from("--long")) || args.contains(&String::from("-l"));
    let recursive =
        args.contains(&String::from("--recursive")) || args.contains(&String::from("-r"));
    let print_all = args.contains(&String::from("--all")) || args.contains(&String::from("-a"));
    let show_created_date = args.contains(&String::from("--show-created--date"))
        || args.contains(&String::from("-scd"));
    let show_last_accessed_date = args.contains(&String::from("--show-last-accessed-date"))
        || args.contains(&String::from("-slad"));

    let options = &Options {
        long: long,
        recursive: recursive,
        print_all: print_all,
        show_created_date: show_created_date,
        show_last_accessed_date: show_last_accessed_date,
    };

    match fs::read_dir(".", options) {
        Ok(vec) => {
            if options.long {
                print_long(&vec, "", options);
                return;
            }

            print_grid(&vec, options);
        }
        Err(e) => println!("{:?}", e),
    }
}
