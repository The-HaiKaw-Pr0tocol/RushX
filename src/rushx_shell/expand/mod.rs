//! ## `rushx_shell::expand` <ins>module</ins>: Command Expansion & Resolution
//!
//! Command expansion and resolution. Performs variable substitution, globbing,
//! tilde expansion, and PATH resolution. Transforms parsed AST nodes into
//! fully-resolved command vectors ready for execution.
//!
//! ## Metadata
//!
//! - **File**: src/rushx_shell/expand/mod.rs
//! - **Module**: rushx_shell::expand
//! - **Last Update**: 02/17/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol

/*=============================================================================*/

pub mod glob;
pub mod path;
pub mod vars;
