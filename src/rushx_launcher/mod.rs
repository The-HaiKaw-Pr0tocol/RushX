//!
//!  ## `rushx_launcher` <ins>module</ins>: RushX Launcher
//!
//! Application entry point and launcher. The terminal emulator owns
//! shell spawning via PTY, keeping shell logic separate.
//!
//! ### Metadata
//!
//! - **File**: src/rushx_launcher/mod.rs
//! - **Module**: rushx_launcher
//! - **Last Update**: 02/17/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol
//! 

/*=============================================================================*/

use crate::rushx_shell;
use crate::rushx_term;

///
/// #### **<ins>Function</ins>** 
/// ```Rust
///     run() -> ()
/// ```
/// Launches the terminal emulator.
///
/// ### Behavior
/// - If `--rushx-shell` flag is present → launches interactive shell REPL
/// - Otherwise → starts the GTK terminal emulator window
///
pub fn run() -> () {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--rushx-shell") {
        rushx_shell::run_rushx_shell();
    } else {
        rushx_term::run_rushx_terminal();
    }
}
