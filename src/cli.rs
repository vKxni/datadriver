use crate::entry::{format_row, print_header, show_info, Entry};
use crate::explorer::{open_in_explorer, open_with_default, preview_file};
use crate::utils::{clear_screen, hot_reload, prompt_confirm, CLR_BOLD, CLR_CYAN, CLR_RESET};
use crate::writer::write_csv_report;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::cli_commands::{
    delete_file, filter_entries, open_scan_root, open_with_id, preview_content, reprint, display_info, save_csv, showcase_stats
};

pub fn interactive_help() {
    println!();
    println!("{CLR_BOLD}Interactive help{CLR_RESET}");

    const COMMANDS: &[(&str, &str)] = &[
        ("ls", "Reprint the current list"),
        ("o <id> | o", "Reveal entry in Explorer or open scan root"),
        ("open <id>", "Open file with default program (files only)"),
        ("p <id> [lines]", "Preview first N lines (default 30)"),
        ("d <id>", "Delete single entry (asks confirmation)"),
        ("info <id>", "Show full metadata for entry"),
        ("f <substr>", "Filter entries by substring (path or name)"),
        ("sort <name|size|age>", "Sort entries (reassigns Ids)"),
        ("clear", "Clear the terminal"),
        ("w <file.csv>", "Write CSV report"),
        ("stats", "Show summary statistics about the scan"),
        ("help", "Show this help"),
        ("q", "Quit interactive mode"),
    ];

    for (cmd, desc) in COMMANDS {
        println!(
            "  {CLR_CYAN}{:<22}{CLR_RESET} - {CLR_BOLD}{}{CLR_RESET}",
            cmd, desc
        );
    }

    println!();
}

pub fn run_interactive(mut entries: Vec<Entry>, root: PathBuf, _width_path: usize) {
    let width_path = std::cmp::min(
        std::cmp::max(
            30,
            entries
                .iter()
                .map(|e| e.path.to_string_lossy().len())
                .max()
                .unwrap_or(30),
        ),
        100,
    );

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
            "ls" => reprint(&entries, width_path),
            "help" | "h" => interactive_help(),
            "o" => open_scan_root(&entries, &parts, &root),
            "open" => open_with_id(&entries, &parts, &root),
            "p" => preview_content(&entries, &parts),
            "d" => delete_file(&entries, &parts),
            "info" => display_info(&entries, &parts),
            "f" => filter_entries(&entries, &parts),
            "stats" => showcase_stats(&entries),
            "w" => save_csv(&entries, &parts),
            "rc" => hot_reload(),
            "clear" => clear_screen(),
            _ => println!("Unknown command. Type 'hhhhh' for list."),
        }
        // reassign indices after modifications
        for (i, e) in entries.iter_mut().enumerate() {
            e.index = i + 1;
        }
    }
}
