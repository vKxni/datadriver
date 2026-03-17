use std::io::{self, Write};
use std::path::PathBuf;
use std::fs;
use crate::entry::{Entry, format_row, print_header, show_info};
use crate::explorer::{open_in_explorer, open_with_default, preview_file};
use crate::writer::write_csv_report;
use crate::utils::{CLR_BOLD, CLR_RESET, CLR_CYAN, CLR_GREEN, CLR_YELLOW, prompt_confirm, clear_screen};

fn interactive_help() {
    println!();
    println!("{}Interactive help{}", CLR_BOLD, CLR_RESET);
    println!("  {}ls{}                - {}Reprint{} the current list", CLR_CYAN, CLR_RESET, CLR_BOLD, CLR_RESET);
    println!("  {}o <id>{} or {}o{}   - {}Reveal{} entry in Explorer; no-arg {}o{} opens scan root", CLR_CYAN, CLR_RESET, CLR_CYAN, CLR_RESET, CLR_BOLD, CLR_RESET, CLR_CYAN, CLR_RESET);
    println!("  {}open <id>{}         - {}Open{} file with default program (files only)", CLR_CYAN, CLR_RESET, CLR_BOLD, CLR_RESET);
    println!("  {}p <id> [lines]{}    - {}Preview{} first N lines (default 30)", CLR_CYAN, CLR_RESET, CLR_BOLD, CLR_RESET);
    println!("  {}d <id>{}            - {}Delete{} single entry (asks confirmation)", CLR_CYAN, CLR_RESET, CLR_BOLD, CLR_RESET);
    println!("  {}info <id>{}         - {}Show{} full metadata for entry", CLR_CYAN, CLR_RESET, CLR_BOLD, CLR_RESET);
    println!("  {}f <substr>{}        - {}Filter{} entries by substring (path or name)", CLR_CYAN, CLR_RESET, CLR_BOLD, CLR_RESET);
    println!("  {}sort <name|size|age>{} - {}Sort{} entries (reassigns Ids)", CLR_CYAN, CLR_RESET, CLR_BOLD, CLR_RESET);
    println!("  {}clear{}             - {}Clear{} the terminal", CLR_CYAN, CLR_RESET, CLR_BOLD, CLR_RESET);
    println!("  {}r <file.csv>{}      - {}Write{} CSV report to given path", CLR_CYAN, CLR_RESET, CLR_BOLD, CLR_RESET);
    println!("  {}help{}              - {}Show{} this help", CLR_CYAN, CLR_RESET, CLR_BOLD, CLR_RESET);
    println!("  {}q{}                 - {}Quit{} interactive mode", CLR_CYAN, CLR_RESET, CLR_BOLD, CLR_RESET);
}

pub fn run_interactive(mut entries: Vec<Entry>, root: PathBuf, mut width_path: usize) {
    width_path = std::cmp::min(std::cmp::max(30, entries.iter().map(|e| e.path.to_string_lossy().len()).max().unwrap_or(30)), 100);

    loop {
        println!();
        println!("{}Commands (type 'h' for details){}", CLR_BOLD, CLR_RESET);
        print!("cmd> ");
        io::stdout().flush().unwrap();
        let mut cmd = String::new();
        if io::stdin().read_line(&mut cmd).is_err() {
            break;
        }
        let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "q" | "quit" => break,
            "ls" => {
                print_header(width_path);
                for e in &entries {
                    println!("{}", format_row(e, width_path));
                }
            }
            "help" | "h" => interactive_help(),
            // "o" with no args -> open scan root; with id -> open entry
            "o" => {
                if parts.len() >= 2 {
                    if let Ok(id) = parts[1].parse::<usize>() {
                        if let Some(e) = entries.iter().find(|x| x.index == id) {
                            if let Err(err) = open_in_explorer(&e.path) {
                                eprintln!("Failed to open explorer: {}", err);
                            }
                        } else {
                            println!("No entry {}", id);
                        }
                    } else {
                        println!("Invalid id.");
                    }
                } else {
                    if let Err(err) = open_in_explorer(&root) {
                        eprintln!("Failed to open explorer: {}", err);
                    }
                }
            }
            "open" if parts.len() >= 2 => {
                if let Ok(id) = parts[1].parse::<usize>() {
                    if let Some(e) = entries.iter().find(|x| x.index == id) {
                        if e.is_dir {
                            println!("Cannot open directory with default program; use 'o' to open in Explorer.");
                        } else if let Err(err) = open_with_default(&e.path) {
                            eprintln!("Failed to open file: {}", err);
                        }
                    } else {
                        println!("No entry {}", id);
                    }
                }
            }
            // preview content of file 
            "p" if parts.len() >= 2 => {
                if let Ok(id) = parts[1].parse::<usize>() {
                    let lines = if parts.len() >= 3 {
                        parts[2].parse::<usize>().unwrap_or(30)
                    } else {
                        30
                    };
                    if let Some(e) = entries.iter().find(|x| x.index == id) {
                        if e.is_dir {
                            println!("Preview not available for directories.");
                        } else if let Err(err) = preview_file(&e.path, lines) {
                            eprintln!("Preview error: {}", err);
                        }
                    } else {
                        println!("No entry {}", id);
                    }
                }
            }
            // delete file or directory with confirmation
            "d" if parts.len() >= 2 => {
                if let Ok(id) = parts[1].parse::<usize>() {
                    if let Some(pos) = entries.iter().position(|x| x.index == id) {
                        let e = &entries[pos];
                        if prompt_confirm(&format!("Delete {}?", e.path.to_string_lossy())) {
                            let res = if e.path.is_dir() {
                                fs::remove_dir_all(&e.path)
                            } else {
                                fs::remove_file(&e.path)
                            };
                            match res {
                                Ok(_) => {
                                    println!("Deleted {}", e.path.to_string_lossy());
                                    entries.remove(pos);
                                }
                                Err(err) => eprintln!("Failed to delete: {}", err),
                            }
                        }
                    } else {
                        println!("No entry {}", id);
                    }
                }
            }
            // show full metadata info
            "info" if parts.len() >= 2 => {
                if let Ok(id) = parts[1].parse::<usize>() {
                    if let Some(e) = entries.iter().find(|x| x.index == id) {
                        show_info(e);
                    } else {
                        println!("No entry {}", id);
                    }
                }
            }
            // filter entries by substring in path or name (case-insensitive)
            "f" if parts.len() >= 2 => {
                let q = parts[1].to_lowercase();
                let filtered: Vec<_> = entries.iter().filter(|e| {
                    e.path.to_string_lossy().to_lowercase().contains(&q) ||
                    e.name.to_lowercase().contains(&q)
                }).cloned().collect();
                if filtered.is_empty() {
                    println!("No matches for '{}'", q);
                } else {
                    println!("Matches:");
                    for e in filtered {
                        println!("{}", format_row(&e, width_path));
                    }
                }
            }
            // sort entries by name, size or age (access time); reassign indices after sorting
            "sort" if parts.len() >= 2 => {
                match parts[1] {
                    "name" => entries.sort_by_key(|e| e.name.to_lowercase()),
                    "size" => entries.sort_by_key(|e| std::cmp::Reverse(e.size)),
                    "age" => entries.sort_by_key(|e| std::cmp::Reverse(e.accessed)),
                    _ => println!("Unknown sort key. Use name|size|age"),
                }
                // reassign indices
                for (i, e) in entries.iter_mut().enumerate() {
                    e.index = i + 1;
                }
                println!("Sorted by {}", parts[1]);
            }
            "clear" => clear_screen(),
            "r" if parts.len() >= 2 => {
                let out = parts[1];
                if let Err(err) = write_csv_report(out, &entries) {
                    eprintln!("Failed to write report: {}", err);
                } else {
                    println!("Wrote {}", out);
                }
            }
            _ => println!("Unknown command. Type 'help' for list."),
        }
        // reassign indices after modifications
        for (i, e) in entries.iter_mut().enumerate() {
            e.index = i + 1;
        }
    }
}