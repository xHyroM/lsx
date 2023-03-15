use chrono::{DateTime, Local};
use std::time::SystemTime;

pub fn format_size(size: u64) -> String {
    let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let mut size = size as f64;
    let mut i = 0;
    while size >= 1024.0 && i < units.len() - 1 {
        size /= 1024.0;
        i += 1;
    }
    format!("{:.1} {}", size, units[i])
}

pub fn readable_systemtime(time: SystemTime) -> String {
    DateTime::<Local>::from(time)
        .format("%b %e %H:%M")
        .to_string()
}
