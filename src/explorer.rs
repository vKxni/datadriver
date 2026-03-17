use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::Command;

pub fn open_in_explorer(path: &Path) -> io::Result<()> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Path does not exist: {}", path.display()),
        ));
    }

    let arg = if path.is_file() {
        format!("/select,\"{}\"", path.display())
    } else {
        format!("\"{}\"", path.display())
    };

    Command::new("explorer")
        .arg(arg)
        .spawn()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to launch Explorer: {}", e),
            )
        })
        .map(|_| ())
}

pub fn open_with_default(path: &Path) -> io::Result<()> {
    let p = path.to_string_lossy().to_string();
    Command::new("cmd")
        .args(&["/C", "start", "", &p])
        .spawn()
        .map(|_| ())
}

pub fn preview_file(path: &Path, lines: usize) -> io::Result<()> {
    let f = fs::File::open(path)?;
    let reader = BufReader::new(f);
    for (i, line) in reader.lines().enumerate() {
        if i >= lines {
            break;
        }
        if let Ok(l) = line {
            println!("{:4} {}", i + 1, l);
        }
    }
    Ok(())
}
