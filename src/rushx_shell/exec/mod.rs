//!
//! ## `rushx_shell::exec` <ins>module</ins>: Execution Engine
//!
//! Command execution engine. Handles fork/exec for external commands, builtin
//! detection and dispatch, PATH resolution with permission checking. Manages
//! process lifecycle from spawn to waitpid, with proper exit status reporting.
//!
//! Syscall functions like `execvp(const char *file, char * const argv[])` require C strings.
//! Direct Rust strings would cause undefined behavior (reading past memory). We convert
//! at FFI boundaries using `CString::new()` to ensure null termination (**CString**: Rust's owned, null-terminated C string wrapper)
//!
//!
//! ## Metadata
//!
//! - **File**: src/rushx_shell/exec/mod.rs
//! - **Module**: rushx_shell::exec
//! - **Last Update**: 02/17/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol
//!

/*=============================================================================*/

use std::env;
use std::ffi::CString;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use nix::fcntl::{OFlag, open};
use nix::sys::stat::Mode;
use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::{ForkResult, close, dup2, execvp, fork};

///
/// #### **<ins>Function</ins>** 
/// ```Rust
///     run_external(cmd: &str, args: &[&str]) -> ()
/// ```
/// Executes an external command via fork/exec.
///
/// ### Arguments
/// - `cmd`: Command name (resolved via PATH)
/// - `args`: Argument vector
///
/// ### Behavior
/// - Searches PATH for executable
/// - Forks child process
/// - Child: overlays with `execvp`
/// - Parent: waits for child termination via `waitpid`
///
/// ### Output
/// - Prints error to stderr on non-zero exit or command not found
///
/// ### Some FFI Notes
/// The `execvp` C syscall function expects null-terminated strings. We convert Rust strings
/// to `CString` (owned, null-terminated) to satisfy this.
///
pub fn run_external(cmd: &str, args: &[&str], stdout_file: Option<&str>, stderr_file: Option<&str>, stdout_append: bool) -> () {
    match find_executable_in_path(cmd) {
        Some(path) => {
            let path_cstr = CString::new(path.to_str().unwrap()).unwrap();
            let mut c_args: Vec<CString> = Vec::with_capacity(args.len());

            //** argv[0] must be the command name (POSIX requirement) **//
            c_args.push(CString::new(cmd).unwrap());

            for &arg in &args[1..] {
                c_args.push(CString::new(arg).unwrap());
            }

            match unsafe { fork() } {
                Ok(ForkResult::Child) => {
                    /*-- If stdout redirection is requested, open target file --*/ 
                    /*-- and dup2 it onto fd 1 (stdout). --*/
                    if let Some(file_path) = stdout_file {
                        let flags = if stdout_append {
                            OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND
                        } else {
                            OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC
                        };
                        let fd = open(
                            file_path,
                            flags,
                            Mode::from_bits_truncate(0o644),
                        )
                        .expect("failed to open redirect target");
                        dup2(fd, 1).expect("dup2 failed");
                        close(fd).ok();
                    }

                    /*-- If stderr redirection is requested, open target file --*/
                    /*-- and dup2 it onto fd 2 (stderr). --*/
                    if let Some(file_path) = stderr_file {
                        let fd = open(
                            file_path,
                            OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC,
                            Mode::from_bits_truncate(0o644),
                        )
                        .expect("failed to open stderr redirect target");
                        dup2(fd, 2).expect("dup2 stderr failed");
                        close(fd).ok();
                    }
                    execvp(&path_cstr, &c_args).expect("execvp failed");
                }
                Ok(ForkResult::Parent { child }) => match waitpid(child, None) {
                    Ok(status) => {
                        if let WaitStatus::Exited(_, code) = status {
                            if code != 0 && stderr_file.is_none() {
                                eprintln!("Program exited with code: {}", code);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error waiting for child: {}", e),
                },
                Err(e) => {
                    eprintln!("fork failed: {}", e);
                }
            }
        }
        None => println!("{}: command not found", cmd),
    }
}

///
/// #### **<ins>Function</ins>** 
/// ```Rust
///     is_builtin(cmd: &str) -> bool
/// ```
/// Checks if a command is a shell builtin.
///
/// ### Arguments
/// - `cmd`: Command name
///
/// ### Returns
/// `true` if `cmd` is `exit`, `echo`, `type`, `pwd` or `cd`
///
pub fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "exit" | "echo" | "type" | "pwd" | "cd")
}

///
/// #### **<ins>Function</ins>** 
/// ```Rust
///     find_executable_in_path(cmd: &str) -> Option<std::path::PathBuf>
/// ```
/// Searches PATH for an executable command.
///
/// ### Arguments
/// - `cmd`: Command name (no path separators)
///
/// ### Returns
/// - `Some(PathBuf)` if found and executable (mode & 0111 != 0)
/// - `None` otherwise
///
/// ### Algorithm
/// - Iterates PATH directories
/// - Joins `cmd` to each path
/// - Checks: file exists + is regular file + has execute permission
///
pub fn find_executable_in_path(cmd: &str) -> Option<std::path::PathBuf> {
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let full_path = path.join(cmd);
            if full_path.is_file() {
                if let Ok(metadata) = fs::metadata(&full_path) {
                    let permissions = metadata.permissions();
                    if permissions.mode() & 0o111 != 0 {
                        return Some(full_path);
                    }
                }
            }
        }
    }
    None
}
