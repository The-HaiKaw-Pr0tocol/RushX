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
/// - Dispatches builtins (`exit`, `echo`, `type`) or external commands
///
/// ### Exit
/// Terminates on `exit` builtin or EOF.
///
pub fn run_rushx_shell() -> () {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input_buffer = String::new();
        if io::stdin().read_line(&mut input_buffer).is_err() {
            break;
        }

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
                    exec::type_command(args[1]);
                }
            }
            _ => exec::run_external(args[0], &args),
        }
    }
}
