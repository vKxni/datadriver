use crate::entry::Entry;
use crate::utils::sys_time_to_secs;
use std::fs;
use std::io;
use std::path::Path;
use std::time::SystemTime;

pub fn walk_dir(root: &Path, threshold_days: u64, only_candidates: bool) -> io::Result<Vec<Entry>> {
    let mut stack = vec![root.to_path_buf()];
    let mut collected: Vec<Entry> = Vec::new();

    while let Some(p) = stack.pop() {
        let read = fs::read_dir(&p);
        if read.is_err() {
            continue;
        }
        for entry in read.unwrap().filter_map(Result::ok) {
            let path = entry.path();

            // fast node_modules check
            let path_lc = path.to_string_lossy().to_lowercase();
            if path_lc.contains("node_modules") {
                continue;
            }

            match entry.metadata() {
                Ok(meta) => {
                    let name = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string();
                    let created = meta.created().map(sys_time_to_secs).unwrap_or(0);
                    let modified = meta.modified().map(sys_time_to_secs).unwrap_or(0);
                    let accessed = meta.accessed().map(sys_time_to_secs).unwrap_or(0);
                    let size = if meta.is_file() { meta.len() } else { 0 };

                    // use single now timestamp
                    let now_secs = SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let age_days = if accessed == 0 {
                        u64::MAX
                    } else {
                        (now_secs - accessed) / 86400
                    };
                    let status = if age_days <= threshold_days {
                        "RECENT".to_string()
                    } else {
                        "OLD".to_string()
                    };
                    let is_empty = meta.is_file() && size == 0;
                    let is_candidate = is_empty || age_days > threshold_days;

                    if meta.is_dir() {
                        stack.push(path.clone());
                        if !only_candidates {
                            collected.push(Entry {
                                index: 0,
                                path,
                                name,
                                created,
                                modified,
                                accessed,
                                size,
                                is_dir: true,
                                is_empty,
                                status,
                            });
                        }
                    } else {
                        if only_candidates && !is_candidate {
                            continue;
                        }
                        collected.push(Entry {
                            index: 0,
                            path,
                            name,
                            created,
                            modified,
                            accessed,
                            size,
                            is_dir: false,
                            is_empty,
                            status,
                        });
                    }
                }
                Err(_) => continue,
            }
        }
    }

    for (i, e) in collected.iter_mut().enumerate() {
        e.index = i + 1;
    }
    Ok(collected)
}
