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
use std::fs::File;
use std::io::{self, Write};

///
/// #### **<ins>Function</ins>** 
/// ```Rust
///     run_rushx_shell() -> ()
/// ```
/// Main interactive shell REPL loop.
///
/// ### Behavior
/// - Prints prompt (`$ `)
/// - Reads line from stdin
/// - Parses whitespace-delimited arguments
/// - Dispatches builtins (`exit`, `echo`, `type`, `pwd`, `cd`) or external commands
///
/// ### Exit
/// Terminates on `exit` builtin or EOF.
///
pub fn run_rushx_shell() -> () {
    let mut oldpwd: Option<String> = None;
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input_buffer = String::new();
        if io::stdin().read_line(&mut input_buffer).is_err() {
            break;
        }

        let raw_args = parser::parse_args(input_buffer.trim());

        if raw_args.is_empty() {
            continue;
        }

        /*-- Parse redirections (>, 1>) from the argument list --*/
        let parsed = parser::parse_redirections(raw_args);
        let args = parsed.args;
        let stdout_append = parsed.stdout_redirect.as_ref().map_or(false, |r| r.append);
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
            Some(path) => match File::create(path) {
                Ok(f) => Box::new(f),
                Err(e) => {
                    eprintln!("rushx: {}: {}", path, e);
                    continue;
                }
            },
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
                let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                exec::run_external(
                    &args[0],
                    &str_args,
                    stdout_file.as_deref(),
                    stderr_file.as_deref(),
                    stdout_append,
                );
            }
        }
    }
}
