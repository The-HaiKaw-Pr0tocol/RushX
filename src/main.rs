#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn main() {
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
            _ => println!("{}: command not found", args[0]),
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