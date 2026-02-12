#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::os::unix::process::CommandExt;

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
            _ => {
                /*-- Not a builtin -> execute as external program --*/
                execute_external_command(&args);
            }
        }
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
        None => eprintln!("{}: not found", cmd),
    }
}

/**
 * Searches for an executable command in the system PATH:
 *      Looping through each directory in PATH 
 *      Construct full path by joining directory + command name. 
 *      If file exists AND has execute permissions, return the path,
 *      Else return None
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

/**
 * Executes an external program with its arguments:
 *      Find the executable in PATH
 *      If found, run it with all arguments
 *      Else print "command not found"
 */
fn execute_external_command(args: &[&str]) {
    let cmd = args[0];
    
    match find_executable_in_path(cmd) {
        Some(path) => {
            let cmd_args = &args[1..];
            
            match Command::new(path)
                .arg0(cmd)
                .args(cmd_args)
                .status() {
                    Ok(status) => {
                        /*-- Program executed - we don't need to do anything --*/
                        /*-- Check if program failed --*/
                        if let Some(code) = status.code() {
                            /*-- Program ran but exited with error code --*/
                            if code != 0 {
                                eprintln!("Program exited with code: {}", code);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error executing command: {}", e),
                }
        }
        None => eprintln!("{}: command not found", cmd),
    }
}