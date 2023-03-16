use ansi_term::Color::{Blue, Yellow};

use crate::{
    fs::items::{Directory, Item},
    Options,
};

pub fn print_grid(items: &Vec<Item>, options: &Options) {
    let length = items.len();
    let size = (length as f32).sqrt().ceil() as usize;
    let max_len = items
        .iter()
        .map(|item| match item {
            Item::File(file) => file.name.len(),
            Item::Directory(dir) => dir.name.len() + 1,
        })
        .max()
        .unwrap_or(0);

    let mut next_items: Vec<&Directory> = Vec::new();

    for i in 0..size {
        for j in 0..size {
            let index = i * size + j;
            if index < length {
                let item = &items[index];

                match item {
                    Item::File(file) => {
                        let padding = max_len - file.name.len();
                        print!("{}{}", file.name, " ".repeat(padding));
                    }
                    Item::Directory(dir) => {
                        let name = dir.name.to_owned() + "/";
                        let padding = max_len - name.len();
                        print!("{}{}", Blue.paint(name), " ".repeat(padding));

                        if options.recursive {
                            next_items.push(&dir);
                        }
                    }
                }

                print!("  ");
            }
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
