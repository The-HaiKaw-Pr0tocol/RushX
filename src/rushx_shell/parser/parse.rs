//
// ## Recursive Descent Parser
//
// Recursive descent parser for shell command syntax. Consumes token
// stream from lexer and constructs AST nodes. Handles operator
// precedence, associativity, and nested command structures.
//
// ## Metadata
//
// - **File**: src/rushx_shell/parser/parse.rs
// - **Module**: rushx_shell::parser::parse
// - **Last Update**: 02/17/2026
// - **Last Updated By**: sch0penheimer
// - **Version**: 0.1.0
// - **Copyright**: © 2026 The HaiKaw Pr0tocol
//

/*=============================================================================*/

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     parse_args(input: &str) -> Vec<String>
/// ```
/// Splits a raw input line into arguments with POSIX-style quote handling.
///
/// ### Arguments
/// - `input`: Raw input string from the user
///
/// ### Returns
/// A vector of parsed argument strings, respecting single quotes,
/// double quotes, and backslash escapes.
///
/// ### Quote Rules
/// - **Single quotes**: preserve literal content, no escape processing
/// - **Double quotes**: allow `\\`, `\"`, `\$`, `` \` ``, `\newline` escapes
/// - **Unquoted backslash**: escapes the next character
///
pub fn parse_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut has_content = false;

    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if in_single_quote {
            if c == '\'' {
                in_single_quote = false;
            } else {
                current.push(c);
            }
        } else if in_double_quote {
            if c == '"' {
                in_double_quote = false;
            } else if c == '\\' {
                if i + 1 < chars.len() {
                    let next = chars[i + 1];
                    if next == '\\' || next == '"' || next == '$' || next == '`' || next == '\n' {
                        current.push(next);
                        i += 1;
                    } else {
                        current.push(c);
                    }
                } else {
                    current.push(c);
                }
            } else {
                current.push(c);
            }
        } else {
            match c {
                '\\' => {
                    // Backslash outside quotes: escape next character
                    i += 1;
                    if i < chars.len() {
                        current.push(chars[i]);
                        has_content = true;
                    }
                }
                '\'' => {
                    in_single_quote = true;
                    has_content = true;
                }
                '"' => {
                    in_double_quote = true;
                    has_content = true;
                }
                ' ' | '\t' => {
                    if has_content || !current.is_empty() {
                        args.push(current.clone());
                        current.clear();
                        has_content = false;
                    }
                }
                _ => {
                    current.push(c);
                    has_content = true;
                }
            }
        }
        i += 1;
    }

    if has_content || !current.is_empty() {
        args.push(current);
    }

    args
}
