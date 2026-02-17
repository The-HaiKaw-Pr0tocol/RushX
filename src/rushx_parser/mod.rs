//! ## `rushx_parser` <ins>module</ins>: Command Parser & Tokenizer
//!
//! Command parsing and tokenization. Transforms raw input strings into
//! structured AST nodes. Handles POSIX syntax elements: quotes, escapes,
//! operators, pipelines, redirections, and control structures.
//!
//! ## Metadata
//!
//! - **File**: src/rushx_parser/mod.rs
//! - **Module**: rushx_parser
//! - **Last Update**: 02/17/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol

/*=============================================================================*/

pub mod lexer;
pub mod parse;
