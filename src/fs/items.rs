use std::time::SystemTime;

#[derive(Debug,Clone)]
pub struct File {
    pub name: String,
    pub path: String,
    pub metadata: ItemMetadata,
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub name: String,
    pub path: String,
    pub metadata: ItemMetadata,
    pub files: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct ItemMetadata {
    pub size: Option<u64>,
    pub modified: Option<SystemTime>,
    pub accessed: Option<SystemTime>,
    pub created: Option<SystemTime>,
}

#[derive(Debug, Clone)]
pub enum Item {
    File(File),
    Directory(Directory),
}
