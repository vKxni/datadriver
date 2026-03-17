use std::io::Write;
use std::path::PathBuf;
use std::thread;

pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut threshold_days: u64 = 365;
    let mut only_candidates = false;
    let mut interactive_flag = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--path" => {
                if i + 1 < args.len() {
                    path = PathBuf::from(&args[i + 1]);
                    i += 1;
                }
            }
            "--days" => {
                if i + 1 < args.len() {
                    threshold_days = args[i + 1].parse().unwrap_or(threshold_days);
                    i += 1;
                }
            }
            "--only-candidates" => only_candidates = true,
            "--interactive" => interactive_flag = true,
            _ => {}
        }
        i += 1;
    }

    println!("Loading... please wait");
    thread::sleep(std::time::Duration::from_millis(400));
    // try enable ANSI escapes on Windows terminals
    super::utils::enable_ansi();
    println!("Scanning: {}", path.to_string_lossy());

    match super::scanner::walk_dir(&path, threshold_days, only_candidates) {
        Ok(entries) => {
            if entries.is_empty() {
                println!("No items found.");
                return;
            }

            let max_path_len = entries
                .iter()
                .map(|e| e.path.to_string_lossy().len())
                .max()
                .unwrap_or(40);
            let width_path = std::cmp::min(std::cmp::max(30, max_path_len), 80);

            super::entry::print_header(width_path);
            for e in &entries {
                println!("{}", super::entry::format_row(e, width_path));
            }

            // ask about writing CSV
            print!("Write CSV report? [Y / N]: ");
            std::io::stdout().flush().ok();
            let mut resp = String::new();
            let _ = std::io::stdin().read_line(&mut resp);
            let resp_trim = resp.trim().to_lowercase();
            if resp_trim == "" || resp_trim.starts_with('y') {
                let report_name = "datadriver_report.csv".to_string();
                if let Err(err) = super::writer::write_report_with_spinner(&report_name, &entries) {
                    eprintln!("Failed to write report: {}", err);
                }
            } else {
                if !super::utils::prompt_confirm("Enter interactive mode to manage results?") {
                    println!("Exiting.");
                    return;
                }
            }

            // run interactive
            if interactive_flag || true {
                super::cli::run_interactive(entries, path, width_path);
            }
        }
        Err(e) => eprintln!("Scan error: {}", e),
    }
}
