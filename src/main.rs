#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    println!("Hi from RushX !!");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input_buffer : String = String::new();
        io::stdin().read_line(&mut input_buffer).unwrap();

        println!("{}: command not found", input_buffer.trim());
    }
}
