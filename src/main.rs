#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    
        let command = input.trim();
        if command == "exit 0" {
            break;
        }
        print!("{}: command not found\n", command);    
    }
}

