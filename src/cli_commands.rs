use crate::entry::{format_row, show_info, Entry};
use crate::explorer::open_with_default;
use crate::explorer::{open_in_explorer, preview_file};
use crate::utils::{prompt_confirm, CLR_BOLD, CLR_CYAN, CLR_RESET};
use crate::writer::write_csv_report;
use std::path::PathBuf;

pub fn reprint(entries: &[Entry], width_path: usize) {
    for e in entries {
        println!("{}", format_row(e, width_path));
    }
}

pub fn open_scan_root(entries: &[Entry], parts: &[&str], root: &PathBuf) {
    let target = parts
        .get(1)
        .and_then(|id| id.parse::<usize>().ok())
        .and_then(|id| entries.iter().find(|e| e.index == id))
        .map(|e| e.path.clone())
        .unwrap_or_else(|| root.clone());

    if let Err(err) = open_in_explorer(&target) {
        eprintln!("Failed to open explorer: {}", err);
    }
}

pub fn open_with_id(entries: &[Entry], parts: &[&str], root: &PathBuf) {
    let Some(id) = parts.get(1).and_then(|s| s.parse::<usize>().ok()) else {
        println!("Invalid id.");
        return;
    };

    let Some(e) = entries.iter().find(|x| x.index == id) else {
        println!("No entry {}", id);
        return;
    };

    if e.is_dir {
        println!("Cannot open directory with default program; use 'o' instead.");
        return;
    }

    if let Err(err) = open_with_default(&e.path) {
        eprintln!("Failed to open file: {}", err);
    }
}

pub fn preview_content(entries: &[Entry], parts: &[&str]) {
    let Some(id) = parts.get(1).and_then(|s| s.parse::<usize>().ok()) else {
        println!("Invalid id.");
        return;
    };

    let lines = parts
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(30);

    let Some(e) = entries.iter().find(|x| x.index == id) else {
        println!("No entry {}", id);
        return;
    };

    if e.is_dir {
        println!("Preview not available for directories.");
        return;
    }

    if let Err(err) = preview_file(&e.path, lines) {
        eprintln!("Preview error: {}", err);
    }
}

pub fn delete_file(entries: &[Entry], parts: &[&str]) {
    if parts.len() >= 2 {
        if let Ok(id) = parts[1].parse::<usize>() {
            if let Some(e) = entries.iter().find(|x| x.index == id) {
                if prompt_confirm(&format!("Delete {}?", e.path.to_string_lossy())) {
                    let res = if e.path.is_dir() {
                        std::fs::remove_dir_all(&e.path)
                    } else {
                        std::fs::remove_file(&e.path)
                    };
                    match res {
                        Ok(_) => println!("Deleted {}", e.path.to_string_lossy()),
                        Err(err) => eprintln!("Failed to delete: {}", err),
                    }
                }
            } else {
                println!("No entry {}", id);
            }
        } else {
            println!("Invalid id.");
        }
    }
}

pub fn display_info(entries: &[Entry], parts: &[&str]) {
    let Some(id) = parts.get(1).and_then(|s| s.parse::<usize>().ok()) else {
        println!("Usage: info <id>");
        return;
    };

    let Some(e) = entries.iter().find(|x| x.index == id) else {
        println!("No entry {}", id);
        return;
    };

    show_info(e);
}

pub fn filter_entries(entries: &[Entry], parts: &[&str]) {
    let q = parts[1].to_lowercase();
    let filtered: Vec<_> = entries
        .iter()
        .filter(|e| {
            e.path.to_string_lossy().to_lowercase().contains(&q)
                || e.name.to_lowercase().contains(&q)
        })
        .cloned()
        .collect();

    if filtered.is_empty() {
        println!("No entries match '{}'", q);
    } else {
        println!("Matches:");
        for e in filtered {
            println!("{}", format_row(&e, 80));
        }
    }
}

pub fn save_csv(entries: &[Entry], parts: &[&str]) {
    if parts.len() >= 2 {
        let out = parts[1];
        if let Err(err) = write_csv_report(out, entries) {
            eprintln!("Failed to write report: {}", err);
        } else {
            println!("Wrote {}", out);
        }
    } else {
        println!("Usage: w <output.csv>");
    }
}

pub fn showcase_stats(entries: &[Entry]) {
    let total = entries.len();

    let (num_dirs, total_size) = entries.iter().fold((0usize, 0u64), |(dirs, size), e| {
        (dirs + e.is_dir as usize, size + e.size)
    });

    let num_files = total - num_dirs;

    let largest = entries.iter().filter(|e| !e.is_dir).max_by_key(|e| e.size);

    let oldest = entries.iter().min_by_key(|e| e.accessed);
    let newest = entries.iter().max_by_key(|e| e.accessed);

    println!();
    println!("{}Scan Summary{}", CLR_BOLD, CLR_RESET);
    println!("Total entries: {}", total);
    println!("Files: {}, Directories: {}", num_files, num_dirs);
    println!("Total file size: {:.1} KB", total_size as f64 / 1024.0);

    println!(
        "Largest file: {}",
        largest
            .map(|e| e.path.to_string_lossy().to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );

    println!(
        "Oldest accessed: {}",
        oldest
            .map(|e| e.name.clone())
            .unwrap_or_else(|| "N/A".to_string())
    );

    println!(
        "Newest accessed: {}",
        newest
            .map(|e| e.name.clone())
            .unwrap_or_else(|| "N/A".to_string())
    );
}
