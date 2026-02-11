#[allow(unused_imports)]
use std::io::{self, Write};

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
                    match args[1] {
                        "exit" | "echo" | "type" => {
                            println!("{} is a shell builtin", args[1]);
                        }
                        _ => println!("type: {}: not found", args[1]),
                    }
                }
            }
            _ => println!("{}: command not found", args[0]),
        }
    }
}