//! ## RushX - Main Entry Point
//!
//! Binary entry point for RushX.
//!
//! ## Metadata
//!
//! - **File**: src/main.rs
//! - **Module**: main
//! - **Last Update**: 02/17/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol

/*=============================================================================*/

mod rushx_launcher;
mod rushx_shell;
mod rushx_term;

fn main() {
    rushx_launcher::run();
}
