# DataDriver — File metadata scanner and interactive manager 

DataDriver is a small file scanner written in pure Rust, reporting file metadata, and interactively managing candidate files (old or empty). It also includes a PowerShell GUI (separate) for Windows.

Quick build & run
-----------------

Two ways to build and run the CLI:

- Build with `rustc` (single-file-style invocation that works with the current layout):

```powershell
$ rustc main.rs -O -o datadriver.exe
.\datadriver.exe --path "C:\XXX\XXX" --days 365 --interactive
```

- (Recommended) build with cargo 

```bash
$ cargo build && cargo run -- --path "C:\XXX\XXX" --days 365 --interactive
```

Command-line options
--------------------
- `--path <path>` — root folder to scan (defaults to current directory)
- `--days N` — threshold for what counts as "recent" (default 365 days)
- `--only-candidates` — only list files that are empty or older than the threshold
- `--interactive` — skip the CSV prompt and go straight to the interactive session

Program flow
------------
1. Start program (via `main.rs` → calls `datadriver::run()` which delegates to `app::run()`).
2. The scanner (`scanner::walk_dir`) walks the tree and collects `Entry` items.
3. Results are printed to the terminal as a compact table.
4. You are asked whether to write a CSV report. If you accept, `writer::write_report_with_spinner` writes the CSV in a background thread.
5. The interactive CLI (`cli::run_interactive`) is started and accepts commands to inspect, open, delete, filter, and report on entries.

When the interactive prompt is shown, enter any of these commands. `Id` refers to the `Id` column printed in the table. Use `ls` to reprint the table anytime.

- `ls` — reprint the current list (with `Id` values)
- `o <id>` or `o` — reveal/select entry in Explorer; `o` with no args opens the scan root
	- Example: `o 12` — opens Explorer and selects entry `12`
- `open <id>` — open file with default program (files only)
	- Example: `open 12`
- `p <id> [lines]` — preview first `lines` of file (default 30)
	- Example: `p 5` or `p 5 10`
- `d <id>` — delete single entry (file or directory). Program asks for confirmation
	- Example: `d 7`
- `info <id>` — show full metadata (created, modified, accessed, size, status)
	- Example: `info 7`
- `f <substr>` — filter entries whose path or name contains `substr` (case-insensitive)
	- Example: `f backup`
- `sort <name|size|age>` — sort entries by name (alphabetical), size (largest first), or access time (newest first). Reassigns `Id`s
	- Example: `sort size`
- `clear` — clear the terminal
- `r <file.csv>` — write current entries to CSV at `file.csv` via `writer::write_csv_report`.
	- Example: `r output.csv`
- `help` or `h` — show the colored help screen
- `q` or `quit` — exit the interactive mode and end the program

Notes about `Id` values
-----------------------
- `Id` is a 1-based index assigned to entries when the list is printed.
- After deletion or sorting, the program reassigns `Id` values so use `ls` to refresh if you lose track.

CSV format
----------
The CSV written by `writer::write_csv_report` has columns:

```
Path,Name,Created,Modified,LastAccess,Age,SizeKB,Status,Empty
```

- Values are CSV-quoted and double quotes inside fields are escaped by doubling them.
- `Age` and created/modified/lastaccess fields use human-readable durations (e.g., `3d ago`) when available; otherwise `unknown`.

Color and terminal compatibility
--------------------------------
- Colors are implemented with ANSI escape codes. On Windows the program attempts to enable ANSI processing via `utils::enable_ansi()`.
- If colors don't appear:
	- Ensure you run in a terminal that supports ANSI (Windows Terminal, modern PowerShell, cmd in recent Windows builds). Git Bash/MinGW may require different handling.
	- You can disable color output by modifying the color constants in `utils.rs` (set them to empty strings).

Performance notes
-----------------
- `scanner::walk_dir` captures the current time once and reuses it to compute ages (faster than calling the system clock repeatedly).
- The program intentionally only skips `node_modules` to avoid many noisy entries; otherwise it scans all files.
- For very large trees you can:
	- Use `--only-candidates` to reduce the listing to likely candidates.

Troubleshooting
---------------
- Compiler errors about missing `main`:
	- Build `main.rs` (not `datadriver.rs`) because `main.rs` contains the actual `main()` function which calls `datadriver::run()`.
		```bash
		rustc main.rs -O -o datadriver.exe
		```
- If `stdout().flush()` prompts an error about the `Write` trait, ensure `use std::io::Write;` is present (it's used in the project in `datadriver.rs` and `utils.rs`).