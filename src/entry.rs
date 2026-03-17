use crate::utils::{human_age_secs, CLR_BOLD, CLR_CYAN, CLR_GREEN, CLR_RESET, CLR_YELLOW};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Entry {
    pub index: usize,
    pub path: PathBuf,
    pub name: String,
    pub created: u64,
    pub modified: u64,
    pub accessed: u64,
    pub size: u64,
    pub is_dir: bool,
    pub is_empty: bool,
    pub status: String,
}

pub fn format_row(e: &Entry, width_path: usize) -> String {
    let path_s = e.path.to_string_lossy();
    let path_disp = truncate_path_left(&path_s, width_path);
    let sizekb = format!("{:.1}", (e.size as f64) / 1024.0);
    let age = if e.accessed == 0 {
        "unknown".to_string()
    } else {
        human_age_secs(e.accessed) + " ago"
    };
    let empty = if e.is_empty { " Empty" } else { "" };

    let status_colored = match e.status.as_str() {
        "RECENT" => format!("{}RECENT{}{}", CLR_GREEN, CLR_RESET, empty),
        "OLD" => format!("{}OLD{}{}", CLR_YELLOW, CLR_RESET, empty),
        _ => format!("{}{}{}", CLR_CYAN, e.status, CLR_RESET),
    };

    format!(
        "{:>3}  {:<pathw$} {:<20} {:>6} {:>8} {:>8} {:>7}",
        e.index,
        path_disp,
        e.name,
        if e.is_dir { "<DIR>" } else { "" },
        age,
        sizekb,
        status_colored,
        pathw = width_path
    )
}

/// Truncate a path string to `width` characters from the left, adding "..." if truncated
/// UTF-8 safely.
pub fn truncate_path_left(path: &str, width: usize) -> String {
    if path.chars().count() <= width {
        return path.to_string();
    }

    let truncated: String = path
        .chars()
        .rev()
        .take(width.saturating_sub(3))
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    format!("...{}", truncated)
}

pub fn print_header(width_path: usize) {
    println!();
    println!(
        "{}{}{:>3}  {:<pathw$} {:<20} {:<6} {:<8} {:<8} {:<7}{}",
        CLR_BOLD,
        CLR_CYAN,
        "Id",
        "Path",
        "Name",
        "Type",
        "Age",
        "SizeKB",
        "Status",
        CLR_RESET,
        pathw = width_path
    );
    println!("{}", "-".repeat(6 + width_path + 20 + 6 + 8 + 8 + 9));
}

pub fn show_info(e: &Entry) {
    println!();
    println!("{}Entry {}{}", CLR_BOLD, e.index, CLR_RESET);
    println!("  Path     : {}", e.path.to_string_lossy());
    println!("  Name     : {}", e.name);
    println!(
        "  Type     : {}",
        if e.is_dir { "Directory" } else { "File" }
    );
    println!(
        "  Size     : {} bytes ({:.1} KB)",
        e.size,
        (e.size as f64) / 1024.0
    );
    println!(
        "  Created  : {}",
        if e.created == 0 {
            "unknown".to_string()
        } else {
            human_age_secs(e.created) + " ago"
        }
    );
    println!(
        "  Modified : {}",
        if e.modified == 0 {
            "unknown".to_string()
        } else {
            human_age_secs(e.modified) + " ago"
        }
    );
    println!(
        "  Accessed : {}",
        if e.accessed == 0 {
            "unknown".to_string()
        } else {
            human_age_secs(e.accessed) + " ago"
        }
    );
    println!("  Status   : {}", e.status);
    println!("  Empty    : {}", if e.is_empty { "Yes" } else { "No" });
}
