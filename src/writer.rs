use crate::entry::Entry;
use crate::utils::clear_screen;
use crate::utils::human_age_secs;
use crate::utils::CLR_GREEN;
use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn write_csv_report<P: AsRef<str>, E: AsRef<[Entry]>>(path: P, entries: E) -> io::Result<()> {
    let entries = entries.as_ref();
    let total = entries.len();

    let file = File::create(path.as_ref())?;
    let mut writer = BufWriter::new(file);

    // CSV header
    writeln!(
        writer,
        "Path,Name,Created,Modified,LastAccess,Age,SizeKB,Status,Empty"
    )?;

    // Helpers
    fn human_or_unknown(ts: u64) -> String {
        if ts == 0 {
            "unknown".into()
        } else {
            format!("{} ago", human_age_secs(ts))
        }
    }

    fn escape_csv(s: &str) -> String {
        s.replace('"', "\"\"")
    }

    for (i, e) in entries.iter().enumerate() {
        let created = human_or_unknown(e.created);
        let modified = human_or_unknown(e.modified);
        let accessed = human_or_unknown(e.accessed);
        let size_kb = format!("{:.1}", e.size as f64 / 1024.0);

        let path_esc = escape_csv(&e.path.to_string_lossy());
        let name_esc = escape_csv(&e.name);

        writeln!(
            writer,
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
            path_esc,
            name_esc,
            created,
            modified,
            accessed,
            accessed, // age (same as last access)
            size_kb,
            e.status,
            if e.is_empty { "Yes" } else { "No" }
        )?;

        // Print progress for large files
        if total > 100 && i % 200 == 0 {
            print!("\rWriting CSV: {}/{}", i, total);
            io::stdout().flush().ok();
        }
    }

    if total > 100 {
        print!("\rWriting CSV: {}/{}\n", total, total);
    }

    writer.flush()?;
    Ok(())
}

pub fn write_report_with_spinner(path: &str, entries: &Vec<Entry>) -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    let entries_clone = entries.clone();
    let path_str = path.to_string();
    thread::spawn(move || {
        let res = write_csv_report(&path_str, &entries_clone);
        let _ = tx.send(res);
    });

    let spinner = ['|', '/', '-', '\\'];
    let mut idx = 0usize;
    print!("Writing CSV report ");
    io::stdout().flush().ok();
    loop {
        match rx.recv_timeout(Duration::from_millis(120)) {
            Ok(res) => {
                // finished
                clear_screen();
                match res {
                    Ok(_) => {
                        println!("{}Wrote \"{}\"{}", CLR_GREEN, path, "");
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                print!("{}", spinner[idx % spinner.len()]);
                io::stdout().flush().ok();
                print!("\x08");
                idx += 1;
            }
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "writer thread error")),
        }
    }
}
