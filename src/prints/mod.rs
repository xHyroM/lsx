use crate::{fs::items::Item, Options};

mod grid;
mod long;

pub fn print_long(items: &Vec<Item>, prefix: &str, options: &Options) {
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

    long::print_tree(items, prefix, options);
}

pub fn print_grid(items: &Vec<Item>, options: &Options) {
    grid::print_grid(items, options);
}
