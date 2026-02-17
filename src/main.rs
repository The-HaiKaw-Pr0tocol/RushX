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

mod rushx_app;
mod rushx_core;
mod rushx_exec;
mod rushx_expand;
mod rushx_parser;
mod rushx_term;

fn main() {
    rushx_app::run();
}
