#[allow(unused_imports)]

use std::str;
use std::io::{self, Write};

fn echo(msg: &[String]) {
    println!("{}", msg.join(" "));
}

fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    
        let mut parts = input.split_whitespace();
        let command = parts.next();
        let args : Vec<String> = parts.map(str::to_string).collect();
        match command {
            Some("exit") => {
                if args.is_empty() || args == ["0"] {
                    break;
                } else {
                    println!("Wrong exit");
                }
            },
            Some("echo") => echo(&args),
            Some(_) => println!("{}: command not found", input.trim()),
            None => continue,
        }
    }
}

