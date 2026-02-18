//! ## `rushx_launcher` <ins>module</ins>: RushX Launcher
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

/*=============================================================================*/

use crate::rushx_term;

/// Launches the terminal emulator.
///
/// ### Behavior
/// - Starts the GTK terminal emulator window
///
pub fn run() {
    rushx_term::run();
}
