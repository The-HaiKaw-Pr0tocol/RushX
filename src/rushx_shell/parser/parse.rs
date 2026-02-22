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

///
/// #### **<ins>Struct</ins>**
/// ```Rust
///     Redirection { fd: u32, target: String, append: bool }
/// ```
/// Represents a single output redirection.
///
/// ### Fields
/// - `fd`: File descriptor number (1 = stdout, 2 = stderr)
/// - `target`: Path to the target file
/// - `append`: If true, append to file (`>>`); if false, truncate (`>`)
///
#[derive(Debug, Clone)]
pub struct Redirection {
    pub fd: u32,
    pub target: String,
    pub append: bool,
}

///
/// #### **<ins>Struct</ins>**
/// ```Rust
///     ParsedCommand { args: Vec<String>, stdout_redirect: Option<Redirection>, stderr_redirect: Option<Redirection> }
/// ```
/// A parsed command with its arguments and optional redirections.
///
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub args: Vec<String>,
    pub stdout_redirect: Option<Redirection>,
    pub stderr_redirect: Option<Redirection>,
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     parse_redirections(args: Vec<String>) -> ParsedCommand
/// ```
/// Scans parsed argument tokens for output redirections (`>`, `1>`, `2>`).
///
/// ### Arguments
/// - `args`: Token vector from `parse_args`
///
/// ### Returns
/// A `ParsedCommand` with redirections separated from command arguments.
///
/// ### Supported Operators
/// - `>` — redirect stdout (fd 1) to file, truncate (shorthand for `1>`)
/// - `1>` — redirect stdout (fd 1) to file, truncate (explicit)
/// - `>>` — redirect stdout (fd 1) to file, append (shorthand for `1>>`)
/// - `1>>` — redirect stdout (fd 1) to file, append (explicit)
/// - `2>` — redirect stderr (fd 2) to file, truncate
/// - `2>>` — redirect stderr (fd 2) to file, append
///
pub fn parse_redirections(args: Vec<String>) -> ParsedCommand {
    let mut cmd_args: Vec<String> = Vec::new();
    let mut stdout_redirect: Option<Redirection> = None;
    let mut stderr_redirect: Option<Redirection> = None;
    let mut i = 0;

    while i < args.len() {
        let token = &args[i];

        if token == ">>" || token == "1>>" {
            // Append stdout to file
            if i + 1 < args.len() {
                stdout_redirect = Some(Redirection {
                    fd: 1,
                    target: args[i + 1].clone(),
                    append: true,
                });
                i += 2;
            } else {
                eprintln!("syntax error near unexpected token `newline'");
                i += 1;
            }
        } else if token == ">" || token == "1>" {
            // Truncate stdout to file
            if i + 1 < args.len() {
                stdout_redirect = Some(Redirection {
                    fd: 1,
                    target: args[i + 1].clone(),
                    append: false,
                });
                i += 2;
            } else {
                eprintln!("syntax error near unexpected token `newline'");
                i += 1;
            }
        } else if token == "2>" {
            // Truncate stderr to file
            if i + 1 < args.len() {
                stderr_redirect = Some(Redirection {
                    fd: 2,
                    target: args[i + 1].clone(),
                    append: false,
                });
                i += 2;
            } else {
                eprintln!("syntax error near unexpected token `newline'");
                i += 1;
            }
        } else if token == "2>>" {
            // Append stderr to file
            if i + 1 < args.len() {
                stderr_redirect = Some(Redirection {
                    fd: 2,
                    target: args[i + 1].clone(),
                    append: true,
                });
                i += 2;
            } else {
                eprintln!("syntax error near unexpected token `newline'");
                i += 1;
            }
        } else if token.ends_with(">>") && token.len() > 2 {
            // Handle cases like "1>>" or "2>>" attached
            let prefix = &token[..token.len() - 2];
            if let Ok(fd) = prefix.parse::<u32>() {
                if fd == 1 {
                    if i + 1 < args.len() {
                        stdout_redirect = Some(Redirection {
                            fd: 1,
                            target: args[i + 1].clone(),
                            append: true,
                        });
                        i += 2;
                    } else {
                        eprintln!("syntax error near unexpected token `newline'");
                        i += 1;
                    }
                } else if fd == 2 {
                    if i + 1 < args.len() {
                        stderr_redirect = Some(Redirection {
                            fd: 2,
                            target: args[i + 1].clone(),
                            append: true,
                        });
                        i += 2;
                    } else {
                        eprintln!("syntax error near unexpected token `newline'");
                        i += 1;
                    }
                } else {
                    cmd_args.push(token.clone());
                    i += 1;
                }
            } else {
                cmd_args.push(token.clone());
                i += 1;
            }
        } else if token.ends_with(">") && token.len() > 1 {
            // Handle cases like "1>" or "2>" attached
            let prefix = &token[..token.len() - 1];
            if let Ok(fd) = prefix.parse::<u32>() {
                if fd == 1 {
                    if i + 1 < args.len() {
                        stdout_redirect = Some(Redirection {
                            fd: 1,
                            target: args[i + 1].clone(),
                            append: false,
                        });
                        i += 2;
                    } else {
                        eprintln!("syntax error near unexpected token `newline'");
                        i += 1;
                    }
                } else if fd == 2 {
                    if i + 1 < args.len() {
                        stderr_redirect = Some(Redirection {
                            fd: 2,
                            target: args[i + 1].clone(),
                            append: false,
                        });
                        i += 2;
                    } else {
                        eprintln!("syntax error near unexpected token `newline'");
                        i += 1;
                    }
                } else {
                    cmd_args.push(token.clone());
                    i += 1;
                }
            } else {
                cmd_args.push(token.clone());
                i += 1;
            }
        } else {
            cmd_args.push(token.clone());
            i += 1;
        }
    }

    ParsedCommand {
        args: cmd_args,
        stdout_redirect,
        stderr_redirect,
    }
}
