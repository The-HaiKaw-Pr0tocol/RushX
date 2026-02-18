//! ## `rushx_shell::core` <ins>module</ins>: Core Types & Shared Data Structures
//!
//! Shared types and core data structures. Contains AST definitions for parsed
//! commands, error types for the execution engine, and shell state management
//! types. Provides foundational types used across parser, expander, and executor.
//!
//! ## Metadata
//!
//! - **File**: src/rushx_shell/core/mod.rs
//! - **Module**: rushx_shell::core
//! - **Last Update**: 02/17/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol

/*=============================================================================*/

pub mod ast;
pub mod error;
pub mod state;
