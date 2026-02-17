use std::io::{self, Write};

use crate::rushx_exec;
use crate::rushx_term;

pub fn run() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--rushx-shell") {
        run_shell();
    } else {
        rushx_term::run();
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
                    rushx_exec::type_command(args[1]);
                }
            }
            _ => rushx_exec::run_external(args[0], &args),
        }
    }
}
