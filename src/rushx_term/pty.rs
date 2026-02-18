//!
//! ## `rushx_term::pty` <ins>module</ins>: PTY Management & Shell Spawning
//!
//! Pseudoterminal allocation and shell process lifecycle. Allocates a
//! master/slave PTY pair via `openpty(2)`, forks a child process,
//! establishes a new session with `setsid(2)`, sets the slave as the
//! controlling terminal via `ioctl(TIOCSCTTY)`, wires stdin/stdout/stderr
//! to the slave fd, and overlays the shell binary via `execvp(3)`.
//!
//! The parent retains the master fd for bidirectional I/O with the
//! child shell process: bytes written to the master appear on the shell's
//! stdin, and bytes the shell writes to stdout/stderr are readable from
//! the master.
//!
//! ## Metadata
//!
//! - **File**: src/rushx_term/pty.rs
//! - **Module**: rushx_term::pty
//! - **Last Update**: 02/18/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol
//!

/*=============================================================================*/

use std::ffi::CString;
use std::os::fd::RawFd;

use nix::pty::openpty;
use nix::unistd::{ForkResult, Pid, close, dup2, execvp, fork, setsid};

use super::config;

///
/// #### **<ins>Struct</ins>**
/// ```Rust
///     PtyPair { pub master: RawFd, pub slave: RawFd }
/// ```
/// Holds the file descriptors from an `openpty(2)` allocation.
///
/// ### Fields
/// - `master`: PTY master fd — retained by the parent (terminal emulator) for I/O
/// - `slave`: PTY slave fd — becomes the child's (shell) stdin/stdout/stderr
///
pub struct PtyPair {
    pub master: RawFd,
    pub slave: RawFd,
}

///
/// #### **<ins>Struct</ins>**
/// ```Rust
///     SpawnedShell { pub master_fd: RawFd, pub child_pid: Pid }
/// ```
/// Result of a successful `spawn_shell()`. The parent (terminal emulator)
/// uses `master_fd` for all I/O with the child shell, and `child_pid`
/// for lifecycle management (signals, `waitpid`).
///
/// ### Fields
/// - `master_fd`: PTY master fd for bidirectional shell I/O
/// - `child_pid`: PID of the forked shell child process
///
pub struct SpawnedShell {
    pub master_fd: RawFd,
    pub child_pid: Pid,
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     open_pty_pair() -> Result<PtyPair, nix::Error>
/// ```
/// Allocates a new pseudoterminal master/slave pair via `openpty(2)`.
///
/// ### Returns
/// - `Ok(PtyPair)` with owned master and slave file descriptors
/// - `Err(nix::Error)` if `openpty(2)` fails (e.g., out of PTY devices)
///
/// ### Syscalls
/// - `openpty(2)`: Allocates and configures a PTY pair. Internally
///   performs `posix_openpt` + `grantpt` + `unlockpt` + `ptsname`.
///
pub fn open_pty_pair() -> Result<PtyPair, nix::Error> {
    let result = openpty(None, None)?;

    Ok(PtyPair {
        master: result.master,
        slave: result.slave,
    })
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     spawn_shell(pty: PtyPair) -> Result<SpawnedShell, nix::Error>
/// ```
/// Forks a child process and execs the RushX shell inside the PTY slave.
///
/// ### Arguments
/// - `pty`: The allocated PTY pair (consumed, slave fd is closed in parent)
///
/// ### Child Process Setup (POSIX Session & Controlling Terminal)
/// 1. Closes the master fd (only the parent needs it)
/// 2. Calls `setsid(2)` → creates a new session, child becomes session leader
/// 3. Sets slave as controlling terminal via `ioctl(TIOCSCTTY)`
/// 4. Duplicates slave fd onto fds 0, 1, 2 (stdin/stdout/stderr) via `dup2(2)`
/// 5. Closes the original slave fd (now duplicated, no longer needed)
/// 6. Calls `execvp(3)` to overlay with `rushx --rushx-shell`
///
/// ### Parent Process Behavior
/// 1. Closes the slave fd (only the child uses it)
/// 2. Returns the master fd and child PID
///
/// ### Returns
/// - `Ok(SpawnedShell)` on successful fork (parent path only)
/// - `Err(nix::Error)` if `fork(2)` fails
///
/// ### Safety
/// - `fork(2)` is `unsafe`, the child must only call async-signal-safe
///   functions before `execvp`. All heap allocations (`CString`) are done
///   **before** the fork. Post-fork child code uses only `setsid`, `ioctl`,
///   `dup2`, `close`, and `execvp`, all async-signal-safe.
/// - `ioctl(TIOCSCTTY)` is called via raw `libc::ioctl` (no nix wrapper).
/// - On `execvp` failure, the child calls `libc::_exit(1)` to avoid
///   running Rust destructors or `atexit` handlers in the forked child.
///
pub fn spawn_shell(pty: PtyPair) -> Result<SpawnedShell, nix::Error> {
    let master_fd = pty.master;
    let slave_fd = pty.slave;

    let shell_path_cstr = CString::new(config::SHELL_PATH).expect("invalid SHELL_PATH");
    let shell_name_cstr = CString::new("rushx").expect("invalid argv[0]");
    let shell_flag_cstr = CString::new(config::SHELL_FLAG).expect("invalid SHELL_FLAG");

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let _ = close(master_fd);

            setsid().expect("setsid(2) failed");
            //-- Arg 0 = "don't steal from another session". --//
            unsafe {
                if libc::ioctl(slave_fd, libc::TIOCSCTTY, 0) < 0 {
                    libc::_exit(1);
                }
            }

            dup2(slave_fd, libc::STDIN_FILENO).expect("dup2(2) stdin");
            dup2(slave_fd, libc::STDOUT_FILENO).expect("dup2(2) stdout");
            dup2(slave_fd, libc::STDERR_FILENO).expect("dup2(2) stderr");

            if slave_fd > libc::STDERR_FILENO {
                let _ = close(slave_fd);
            }

            let args = [shell_name_cstr, shell_flag_cstr];
            let _ = execvp(&shell_path_cstr, &args);

            unsafe { libc::_exit(1) }
        }

        Ok(ForkResult::Parent { child }) => {
            let _ = close(slave_fd);

            Ok(SpawnedShell {
                master_fd,
                child_pid: child,
            })
        }

        Err(e) => Err(e),
    }
}
