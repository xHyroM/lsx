use crate::{fs::items::Item, Options};

mod grid;
mod tree;

#[derive(Debug, PartialEq)]
pub enum PrintShow {
    Grid,
    Tree,
}

pub fn print_tree(items: &Vec<Item>, prefix: &str, options: &Options) {
    println!(
        "{:<15} {} Tree",
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
        )
    );

    tree::print_tree(items, prefix, options);
}

pub fn print_grid(items: &Vec<Item>, options: &Options) {
    grid::print_grid(items, options);
}
