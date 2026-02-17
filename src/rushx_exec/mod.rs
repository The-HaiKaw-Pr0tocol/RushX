use std::env;
use std::ffi::CString;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::{ForkResult, execvp, fork};

pub fn run_external(cmd: &str, args: &[&str]) {
    match find_executable_in_path(cmd) {
        Some(path) => {
            let path_cstr = CString::new(path.to_str().unwrap()).unwrap();
            let mut c_args: Vec<CString> = Vec::with_capacity(args.len());

            // argv[0] must be the command name (POSIX requirement).
            c_args.push(CString::new(cmd).unwrap());

            for &arg in &args[1..] {
                c_args.push(CString::new(arg).unwrap());
            }

            match unsafe { fork() } {
                Ok(ForkResult::Child) => {
                    execvp(&path_cstr, &c_args).expect("execvp failed");
                }
                Ok(ForkResult::Parent { child }) => match waitpid(child, None) {
                    Ok(status) => {
                        if let WaitStatus::Exited(_, code) = status {
                            if code != 0 {
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

pub fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "exit" | "echo" | "type")
}

pub fn type_command(cmd: &str) {
    if is_builtin(cmd) {
        println!("{} is a shell builtin", cmd);
        return;
    }

    match find_executable_in_path(cmd) {
        Some(path) => println!("{} is {}", cmd, path.display()),
        None => println!("{}: not found", cmd),
    }
}

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
