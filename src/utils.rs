use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(windows)]
pub fn enable_ansi() {
    use std::ffi::c_void;
    type BOOL = i32;
    type DWORD = u32;
    type HANDLE = *mut c_void;
    type LPDWORD = *mut DWORD;

    const STD_OUTPUT_HANDLE: DWORD = (-11i32) as DWORD;
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;

    extern "system" {
        fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
        fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;
        fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
    }

    unsafe {
        let h = GetStdHandle(STD_OUTPUT_HANDLE);
        if h.is_null() {
            return;
        }
        let mut mode: DWORD = 0;
        if GetConsoleMode(h, &mut mode as LPDWORD) == 0 {
            return;
        }
        let _ = SetConsoleMode(h, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    }
}

#[cfg(not(windows))]
pub fn enable_ansi() { /* no-op on non-windows */
}

pub const CLR_RESET: &str = "\x1B[0m";
pub const CLR_BOLD: &str = "\x1B[1m";
pub const CLR_GREEN: &str = "\x1B[32m";
pub const CLR_YELLOW: &str = "\x1B[33m";
pub const CLR_CYAN: &str = "\x1B[36m";
pub const CLR_RED: &str = "\x1B[31m";

pub fn sys_time_to_secs(t: SystemTime) -> u64 {
    match t.duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => 0,
    }
}

pub fn human_age_secs(secs: u64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    if now <= secs || secs == 0 {
        return "0s".to_string();
    }
    let diff = now - secs;

    match diff {
        0..=59 => format!("{}s", diff),
        60..=3599 => format!("{}m", diff / 60),
        3600..=86399 => format!("{}h", diff / 3600),
        _ => format!("{}d", diff / 86400),
    }
}

pub fn clear_screen() {
    // ANSI clear
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().ok();
}

pub fn prompt_confirm(msg: &str) -> bool {
    print!("{} [y/N]: ", msg);
    io::stdout().flush().unwrap();

    let mut s = String::new();
    if io::stdin().read_line(&mut s).is_ok() {
        let t = s.trim().to_lowercase();
        return t == "y" || t == "yes";
    }
    false
}
