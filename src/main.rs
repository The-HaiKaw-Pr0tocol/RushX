#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::ffi::CString;
use nix::unistd::{fork, execvp, ForkResult};
use nix::sys::wait::waitpid;

mod terminal;

fn main() {
    // If launched with --rushx-shell, run the interactive shell REPL.
    // Otherwise, launch the terminal emulator window.
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a: &String| -> bool {
        a == "--rushx-shell"
    }) {
        run_shell();
    } else {
        terminal::run();
    }
}

fn run_shell() {
    println!("Hi from RushX !!");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input_buffer = String::new();
        io::stdin().read_line(&mut input_buffer).unwrap();
        
        let args: Vec<&str> = input_buffer.split_whitespace().collect();
        
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "exit" => break,
            "echo" => {
                if args.len() > 1 {
                    println!("{}", args[1..].join(" "));
                } else {
                    println!();
                }
            }
            "type" => {
                if args.len() < 2 {
                    println!("type: missing operand");
                } else { 
                    type_command(args[1]);
                }
            }
            _ => run_external(args[0], &args),
        }
    }
}

/**
 * Runs an external program using fork + execvp.
 * The parent process waits for the child to finish.
 *      -> execvp expects null terminated string so we 
 *         convert the rust strings into CString 
 */
fn run_external(cmd: &str, args: &[&str]) {
    match find_executable_in_path(cmd) {
        Some(path) => {
            let path_cstr = CString::new(path.to_str().unwrap()).unwrap();
            
            let mut c_args: Vec<CString> = Vec::with_capacity(args.len());
            
            /*-- argv[0] = command name - POSIX requirement --*/
            c_args.push(CString::new(cmd).unwrap());
            
            for &arg in &args[1..] {
                c_args.push(CString::new(arg).unwrap());
            }
                 
            match unsafe { fork() } {
                Ok(ForkResult::Child) => {
                    execvp(&path_cstr, &c_args).expect("execvp failed");
                }
                Ok(ForkResult::Parent { child }) => {
                    match waitpid(child, None) {
                        Ok(status) => {
                            use nix::sys::wait::WaitStatus;
                            match status {
                                WaitStatus::Exited(_, code) => {
                                    if code != 0 {
                                        eprintln!("Program exited with code: {}", code);
                                    }
                                }
                                _ => {
                                    /* Process was stopped, continued, or killed by signal */
                                }
                            }
                        }
                        Err(e) => eprintln!("Error waiting for child: {}", e),
                    }
                }
                Err(e) => {
                    eprintln!("fork failed: {}", e);
                }
            }
        }
        None => println!("{}: command not found", cmd),
    }
}

fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "exit" | "echo" | "type")
}

fn type_command(cmd: &str) {
    if is_builtin(cmd) {
        println!("{} is a shell builtin", cmd);
        return;
    }
    
    match find_executable_in_path(cmd) {
        Some(path) => println!("{} is {}", cmd, path.display()),
        None => println!("{}: not found", cmd),
    }
}

/**
 * Searches for an executable command in the system PATH. Looping through 
 * each directory in PATH, then constructing full path by joining directory + 
 * command name. If file exists AND has execute permissions, return the path,
 * else return None
 */
fn find_executable_in_path(cmd: &str) -> Option<std::path::PathBuf> {
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let full_path = path.join(cmd);
            if full_path.exists() {
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
    }
    None
}