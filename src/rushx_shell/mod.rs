//!
//! ## `rushx_shell` <ins>module</ins>: Shell REPL & Dispatch
//!
//! Interactive shell loop and builtin/external dispatch.
//!
//! ## Metadata
//!
//! - **File**: src/rushx_shell/mod.rs
//! - **Module**: rushx_shell
//! - **Last Update**: 02/18/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol
//!

/*=============================================================================*/

pub mod core;
pub mod exec;
pub mod expand;
pub mod parser;

use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::fs::PermissionsExt;

use nix::sys::termios::{self, Termios, SetArg, LocalFlags, InputFlags};

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     build_prompt() -> String
/// ```
/// Builds the shell prompt string: `user@hostname:cwd$ `.
///
/// ### Behavior
/// - Reads username from `$USER` env var (fallback: `"rushx"`)
/// - Reads hostname from `/etc/hostname` (fallback: `"localhost"`)
/// - Reads current directory via `std::env::current_dir()`
/// - Replaces `$HOME` prefix with `~` in the displayed path
///
fn build_prompt() -> String {
    let user = env::var("USER").unwrap_or_else(|_| "rushx".to_string());

    let hostname = fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "localhost".to_string());

    let cwd = env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "?".to_string());

    let home = env::var("HOME").unwrap_or_default();
    let display_cwd = if !home.is_empty() && cwd.starts_with(&home) {
        format!("~{}", &cwd[home.len()..])
    } else {
        cwd
    };

    format!("{}@{}:{}$ ", user, hostname, display_cwd)
}

/*--  Builtin command names eligible for tab autocompletion --*/
const BUILTIN_COMMANDS: &[&str] = &["echo", "exit", "type", "pwd", "cd"];

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     enable_raw_mode(fd: i32) -> Termios
/// ```
/// Switches the terminal to raw mode and returns the original settings.
///
/// ### Behavior
/// - Saves current termios attributes
/// - Disables `ICANON` (line buffering), `ECHO` (kernel echo), `ISIG` (Ctrl-C/Z signals)
/// - Disables `ICRNL` (CR→NL mapping) in input flags
/// - Applies changes immediately via `TCSANOW`
///
fn enable_raw_mode(fd: i32) -> Termios {
    let original = termios::tcgetattr(fd).expect("tcgetattr failed");
    let mut raw = original.clone();
    raw.local_flags.remove(LocalFlags::ICANON | LocalFlags::ECHO | LocalFlags::ISIG);
    raw.input_flags.remove(InputFlags::ICRNL);
    termios::tcsetattr(fd, SetArg::TCSANOW, &raw).expect("tcsetattr failed");
    original
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     restore_mode(fd: i32, termios: &Termios) -> ()
/// ```
/// Restores previously saved terminal attributes.
///
fn restore_mode(fd: i32, original: &Termios) {
    termios::tcsetattr(fd, SetArg::TCSANOW, original).ok();
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     find_completions(prefix: &str) -> Vec<String>
/// ```
/// Finds all command names (builtins + PATH executables) matching a prefix.
///
/// ### Behavior
/// - Matches `prefix` against `BUILTIN_COMMANDS` and executables in `$PATH`
/// - Returns a sorted, deduplicated list of matching command names
/// - Returns empty vec if prefix is empty or no matches found
///
fn find_completions(prefix: &str) -> Vec<String> {
    if prefix.is_empty() {
        return Vec::new();
    }

    let mut candidates: Vec<String> = Vec::new();

    for cmd in BUILTIN_COMMANDS {
        if cmd.starts_with(prefix) && *cmd != prefix {
            candidates.push(cmd.to_string());
        }
    }

    if let Ok(path_var) = env::var("PATH") {
        for dir in env::split_paths(&path_var) {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    if name_str.starts_with(prefix) && name_str != prefix {
                        if let Ok(meta) = entry.metadata() {
                            if meta.is_file() && meta.permissions().mode() & 0o111 != 0 {
                                if !candidates.contains(&name_str.to_string()) {
                                    candidates.push(name_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    candidates.sort();
    candidates
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     find_file_completions(prefix: &str) -> Vec<String>
/// ```
/// Finds files and directories matching a prefix for argument completion.
///
/// ### Behavior
/// - If prefix contains `/`, splits into directory and partial name components
/// - Otherwise searches the current working directory
/// - Appends `/` to directory entries
/// - Returns a sorted list of matching filenames
///
fn find_file_completions(prefix: &str) -> Vec<String> {
    let (search_dir, partial) = if let Some(pos) = prefix.rfind('/') {
        let dir_part = &prefix[..=pos];
        let name_part = &prefix[pos + 1..];
        (
            if dir_part.starts_with('~') {
                let home = env::var("HOME").unwrap_or_default();
                format!("{}{}", home, &dir_part[1..])
            } else {
                dir_part.to_string()
            },
            name_part.to_string(),
        )
    } else {
        (".".to_string(), prefix.to_string())
    };

    let mut candidates: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(&search_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy().to_string();
            if name_str.starts_with(&partial) {
                let full = if prefix.contains('/') {
                    let dir_prefix = &prefix[..=prefix.rfind('/').unwrap()];
                    if entry.path().is_dir() {
                        format!("{}{}/", dir_prefix, name_str)
                    } else {
                        format!("{}{}", dir_prefix, name_str)
                    }
                } else if entry.path().is_dir() {
                    format!("{}/", name_str)
                } else {
                    name_str
                };
                if !candidates.contains(&full) {
                    candidates.push(full);
                }
            }
        }
    }

    candidates.sort();
    candidates
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     longest_common_prefix(candidates: &[String]) -> String
/// ```
/// Computes the longest common prefix shared by all candidate strings.
///
/// ### Behavior
/// - Returns empty string if candidates is empty
/// - Iterates character-by-character through the first candidate, checking
///   that all other candidates share the same character at each position
///
fn longest_common_prefix(candidates: &[String]) -> String {
    if candidates.is_empty() {
        return String::new();
    }
    let first = &candidates[0];
    let mut len = first.len();
    for other in &candidates[1..] {
        len = len.min(other.len());
        for (i, (a, b)) in first.chars().zip(other.chars()).enumerate() {
            if a != b {
                len = len.min(i);
                break;
            }
        }
    }
    first[..len].to_string()
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     read_line_with_completion() -> Option<String>
/// ```
/// Reads a line of input character-by-character with tab autocompletion.
///
/// ### Behavior
/// - Reads one byte at a time from stdin (requires raw mode)
/// - Handles printable characters, backspace, tab completion, and enter
/// - Returns `Some(line)` on Enter, `None` on Ctrl-D (EOF) with empty buffer
///
fn read_line_with_completion() -> Option<String> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let mut byte = [0u8; 1];
    let mut last_was_tab = false;

    loop {
        if stdin.read(&mut byte).ok()? == 0 {
            return None;
        }

        match byte[0] {
            b'\r' | b'\n' => {
                last_was_tab = false;
                io::stdout().write_all(b"\n").ok();
                io::stdout().flush().ok();
                return Some(buffer);
            }
            0x04 => {
                last_was_tab = false;
                if buffer.is_empty() {
                    return None;
                }
            }
            b'\t' => {
                let (candidates, word_start) = if let Some(space_pos) = buffer.rfind(' ') {
                    let word = &buffer[space_pos + 1..];
                    (find_file_completions(word), space_pos + 1)
                } else {
                    (find_completions(&buffer), 0)
                };

                match candidates.len() {
                    0 => {
                        /*-- No matches: ring bell --*/
                        if !buffer.is_empty() {
                            io::stdout().write_all(b"\x07").ok();
                            io::stdout().flush().ok();
                        }
                        last_was_tab = false;
                    }
                    1 => {
                        /*-- Single match: complete it --*/
                        let word = &buffer[word_start..];
                        let suffix = &candidates[0][word.len()..];
                        let is_dir = candidates[0].ends_with('/');
                        let completion = if is_dir {
                            suffix.to_string()
                        } else {
                            format!("{} ", suffix)
                        };
                        io::stdout().write_all(completion.as_bytes()).ok();
                        io::stdout().flush().ok();
                        buffer.push_str(&completion);
                        last_was_tab = false;
                    }
                    _ => {
                        /*-- Multiple matches: compute longest common prefix --*/
                        let lcp = longest_common_prefix(&candidates);
                        let word = &buffer[word_start..];

                        if lcp.len() > word.len() {
                            /*-- LCP extends beyond current word: complete to LCP --*/
                            let suffix = &lcp[word.len()..];
                            io::stdout().write_all(suffix.as_bytes()).ok();
                            io::stdout().flush().ok();
                            buffer.push_str(suffix);
                            last_was_tab = false;
                        } else if last_was_tab {
                            /*-- Second TAB: list all matches, re-show prompt --*/
                            let listing = candidates.join("  ");
                            let prompt = build_prompt();
                            let output = format!("\n{}\n{}{}", listing, prompt, buffer);
                            io::stdout().write_all(output.as_bytes()).ok();
                            io::stdout().flush().ok();
                            last_was_tab = false;
                        } else {
                            /*-- First TAB, no LCP progress: ring bell --*/
                            io::stdout().write_all(b"\x07").ok();
                            io::stdout().flush().ok();
                            last_was_tab = true;
                        }
                    }
                }
                continue;
            }
            0x7f => {
                last_was_tab = false;
                if !buffer.is_empty() {
                    buffer.pop();
                    io::stdout().write_all(b"\x08 \x08").ok();
                    io::stdout().flush().ok();
                }
            }
            0x1b => {
                last_was_tab = false;
                let mut seq = [0u8; 1];
                if stdin.read(&mut seq).unwrap_or(0) > 0 && seq[0] == b'[' {
                    let _ = stdin.read(&mut seq);
                }
            }
            c if c >= 0x20 => {
                last_was_tab = false;
                buffer.push(c as char);
                io::stdout().write_all(&byte).ok();
                io::stdout().flush().ok();
            }
            _ => { last_was_tab = false; }
        }
    }
}

///
/// #### **<ins>Function</ins>** 
/// ```Rust
///     run_rushx_shell() -> ()
/// ```
/// Main interactive shell REPL loop.
///
/// ### Behavior
/// - Prints prompt (`user@hostname:cwd$ `) with username, hostname, and current directory
/// - Reads line from stdin
/// - Parses whitespace-delimited arguments
/// - Dispatches builtins (`exit`, `echo`, `type`, `pwd`, `cd`) or external commands
///
/// ### Exit
/// Terminates on `exit` builtin or EOF.
///
pub fn run_rushx_shell() -> () {
    let stdin_fd = io::stdin().as_raw_fd();
    let original_termios = enable_raw_mode(stdin_fd);

    let mut oldpwd: Option<String> = None;
    loop {
        let prompt = build_prompt();
        io::stdout().write_all(prompt.as_bytes()).ok();
        io::stdout().flush().unwrap();

        let input_buffer = match read_line_with_completion() {
            Some(line) => line,
            None => break,
        };

        let raw_args = parser::parse_args(input_buffer.trim());

        if raw_args.is_empty() {
            continue;
        }

        /*-- Parse redirections (>, 1>) from the argument list --*/
        let parsed = parser::parse_redirections(raw_args);
        let args = parsed.args;
        let stdout_append = parsed.stdout_redirect.as_ref().map_or(false, |r| r.append);
        let stderr_append = parsed.stderr_redirect.as_ref().map_or(false, |r| r.append);
        let stdout_file = parsed.stdout_redirect.map(|r| r.target);
        let stderr_file = parsed.stderr_redirect.map(|r| r.target);

        if args.is_empty() {
            continue;
        }

        /*-- Helper: get a writer : either a file or stdout --*/
        /*-- Returns Box<dyn Write> so builtins can write transparently --*/
        let mut out: Box<dyn Write> = match &stdout_file {
            Some(path) => {
                let file_result = if stdout_append {
                    std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(path)
                } else {
                    File::create(path)
                };
                match file_result {
                    Ok(f) => Box::new(f),
                    Err(e) => {
                        eprintln!("rushx: {}: {}", path, e);
                        continue;
                    }
                }
            }
            None => Box::new(io::stdout()),
        };

        /*-- Helper: get an error writer — either a file or stderr --*/
        let mut err: Box<dyn Write> = match &stderr_file {
            Some(path) => {
                let file_result = if stderr_append {
                    std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(path)
                } else {
                    File::create(path)
                };
                match file_result {
                    Ok(f) => Box::new(f),
                    Err(e) => {
                        eprintln!("rushx: {}: {}", path, e);
                        continue;
                    }
                }
            }
            None => Box::new(io::stderr()),
        };

        match args[0].as_str() {
            "exit" => break,
            "echo" => {
                if args.len() > 1 {
                    writeln!(out, "{}", args[1..].join(" ")).ok();
                } else {
                    writeln!(out).ok();
                }
            }
            "type" => {
                if args.len() < 2 {
                    writeln!(out, "type: missing operand").ok();
                } else {
                    if exec::is_builtin(&args[1]) {
                        writeln!(out, "{} is a shell builtin", args[1]).ok();
                    } else {
                        match exec::find_executable_in_path(&args[1]) {
                            Some(path) => { writeln!(out, "{} is {}", args[1], path.display()).ok(); }
                            None => { writeln!(out, "{}: not found", args[1]).ok(); }
                        }
                    }
                }
            }
            "pwd" => {
                match env::current_dir() {
                    Ok(path) => { writeln!(out, "{}", path.display()).ok(); }
                    Err(e) => { writeln!(err, "pwd: {}", e).ok(); }
                }
            }
            "cd" => {
                let home = env::var("HOME").unwrap_or_default();
                let resolved = if args.len() < 2 || args[1] == "~" {
                    home.clone()
                } else if args[1] == "-" {
                    match &oldpwd {
                        Some(prev) => {
                            writeln!(out, "{}", prev).ok();
                            prev.clone()
                        }
                        None => {
                            writeln!(err, "cd: OLDPWD not set").ok();
                            continue;
                        }
                    }
                } else if args[1].starts_with("~/") {
                    format!("{}/{}", home, &args[1][2..])
                } else {
                    args[1].to_string()
                };
                let path = std::path::Path::new(&resolved);
                if path.is_dir() {
                    let current = env::current_dir()
                        .ok()
                        .map(|p| p.to_string_lossy().to_string());
                    if let Err(e) = env::set_current_dir(path) {
                        writeln!(err, "cd: {}: {}", resolved, e).ok();
                    } else {
                        oldpwd = current;
                    }
                } else {
                    writeln!(err, "cd: {}: No such file or directory", resolved).ok();
                }
            }
            _ => {
                /*-- Restore cooked mode before running external commands --*/
                /*-- so they get normal terminal behavior --*/
                restore_mode(stdin_fd, &original_termios);

                let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                exec::run_external(
                    &args[0],
                    &str_args,
                    stdout_file.as_deref(),
                    stderr_file.as_deref(),
                    stdout_append,
                    stderr_append,
                );

                enable_raw_mode(stdin_fd);
            }
        }
    }

    restore_mode(stdin_fd, &original_termios);
}
