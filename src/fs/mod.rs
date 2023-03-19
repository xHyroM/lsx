use std::{
    fs::{self},
    io
};

use crate::{Options, prints::PrintShow};

use self::items::{Directory, File, Item, ItemMetadata};

pub mod items;

pub fn dir_size(directory: &Directory) -> u64 {
    let mut size = 0;

    for item in &directory.files {
        match item {
            Item::File(file) => {
                if let Some(file_size) = file.metadata.size {
                    size += file_size;
                }
            }
            Item::Directory(dir) => {
                size += dir_size(dir);
            }
        }
    }

    size
}

pub fn read_dir(read_path: &str, options: &Options) -> io::Result<Vec<Item>> {
    let mut vec = Vec::new();

    for file in fs::read_dir(read_path)? {
        let file = file?;

        if let Ok(file_type) = file.file_type() {
            let name = file.file_name().into_string().unwrap();
            let path = file.path().into_os_string().into_string().unwrap();
            let metadata = if let Ok(metadata) = file.metadata() {
                ItemMetadata {
                    size: if options.show == PrintShow::Tree { Some(metadata.len()) } else { None },
                    modified: if options.show == PrintShow::Tree { metadata.modified().ok() } else { None },
                    accessed: if options.show_last_accessed_date { metadata.accessed().ok() } else { None },
                    created: if options.show_created_date { metadata.created().ok() } else { None },
                }
            } else {
                ItemMetadata {
                    size: None,
                    modified: None,
                    accessed: None,
                    created: None,
                }
            };

            if file_type.is_dir() {
                vec.push(Item::Directory({
                    Directory {
                        name: name.to_owned(),
                        path,
                        metadata,
                        files: if options.recursive {
                            if let Ok(mut files) = read_dir(
                                format!("{}\\{}", read_path, name).as_str(),
                                options,
                            ) {
                                files.sort_by(|a, b| match (a, b) {
                                    (Item::Directory(a), Item::Directory(b)) => a.name.cmp(&b.name),
                                    (Item::File(a), Item::File(b)) => a.name.cmp(&b.name),
                                    (Item::Directory(_), Item::File(_)) => std::cmp::Ordering::Less,
                                    (Item::File(_), Item::Directory(_)) => {
                                        std::cmp::Ordering::Greater
                                    }
                                });

                                files
                            } else {
                                Vec::new()
                            }
                        } else {
                            Vec::new()
                        },
                    }
                }));

                continue;
            }

            vec.push(Item::File({
                File {
                    name,
                    path,
                    metadata,
                }
            }));
        }
    }

    vec.sort_by(|a, b| match (a, b) {
        (Item::Directory(a), Item::Directory(b)) => a.name.cmp(&b.name),
        (Item::File(a), Item::File(b)) => a.name.cmp(&b.name),
        (Item::Directory(_), Item::File(_)) => std::cmp::Ordering::Less,
        (Item::File(_), Item::Directory(_)) => std::cmp::Ordering::Greater,
    });

    Ok(vec)
}
