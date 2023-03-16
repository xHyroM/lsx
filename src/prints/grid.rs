use term_size;
use ansi_term::Color::{Blue, Yellow};

use crate::{
    fs::items::{Directory, Item},
    Options,
};

pub fn print_grid(items: &Vec<Item>, options: &Options) {
    let (width, _) = term_size::dimensions().unwrap_or((80, 20));
    let max_width = items
        .iter()
        .map(|item| match item {
            Item::File(file) => file.name.len(),
            Item::Directory(dir) => dir.name.len() + 1,
        })
        .max()
        .unwrap_or(0);

    let spacing = 4;

    let num_cols = width / (max_width + 2).max(1);
    let num_rows = (items.len() as f64 / num_cols as f64).ceil() as usize;
    let num_elements_last_row = items.len() - (num_rows - 1) * num_cols;
    let last_row_has_fewer_elements = num_elements_last_row < num_cols;
    
    let mut next_items: Vec<&Directory> = Vec::new();

    let mut index = 0;
    for row in 0..num_rows {
        let mut num_cols_this_row = num_cols;
        if last_row_has_fewer_elements && row == num_rows - 1 {
            num_cols_this_row = num_elements_last_row;
        }
        for col in 0..num_cols_this_row {
            if index > items.len() {
                break;
            }

            let item = &items[index];
            match item {
                Item::File(file) => {
                    print!("{:<width$}", file.name, width = max_width);
                }
                Item::Directory(dir) => {
                    let name = format!("{}/", dir.name);
                    let padded_name = format!("{:<width$}", name, width = max_width);
                    print!("{}", Blue.paint(padded_name));

                    if options.recursive {
                        next_items.push(&dir);
                    }
                }
            }

            if col < num_cols_this_row - 1 {
                print!("{:width$}", "", width = spacing);
            }

            index += 1;
        }

        println!();
    }

    for next_item in next_items {
        println!();
        print!("{}:\n", Yellow.paint(&next_item.path));

        if options.print_all {
            print_grid(&next_item.files, options);
        }
    }
}
