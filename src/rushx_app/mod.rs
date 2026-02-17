//! ## `rushx_app` <ins>module</ins>: RushX Application Dispatcher
//!
//! Application entry point and CLI dispatcher. Contains the main REPL
//! loop, dispatching to builtins and external commands via
//! the execution engine.
//!
//! ### Metadata
//!
//! - **File**: src/rushx_app/mod.rs
//! - **Module**: rushx_app
//! - **Last Update**: 02/17/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol

/*=============================================================================*/

use crate::rushx_exec;
use crate::rushx_term;
use std::io::{self, Write};

/// Dispatches to shell REPL or terminal GUI based on CLI arguments.
///
/// ### Behavior
/// - If `--rushx-shell` flag is present → launches interactive shell REPL
/// - Otherwise → launches GTK terminal emulator window
/// 
pub fn run() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--rushx-shell") {
        run_shell();
    } else {
        rushx_term::run();
    }
}

/// Main Interactive shell REPL loop.
///
/// ### Behavior
/// - Prints prompt (`$ `)
/// - Reads line from stdin
/// - Parses whitespace-delimited arguments
/// - Dispatches builtins (`exit`, `echo`, `type`) or external commands
///
/// # Exit
/// Terminates on `exit` builtin or EOF.
/// 
fn run_shell() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input_buffer = String::new();
        io::stdin().read_line(&mut input_buffer).unwrap();

        let args: Vec<&str> = input_buffer.split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        match args[0] {
            "exit" => break,
            "echo" => {
                if args.len() > 1 {
                    println!("{}", args[1..].join(" "));
                } else {
                    println!();
                }
            }
            "type" => {
                if args.len() < 2 {
                    println!("type: missing operand");
                } else {
                    rushx_exec::type_command(args[1]);
                }
            }
            _ => rushx_exec::run_external(args[0], &args),
        }
    }
}
