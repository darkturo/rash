#[allow(unused_imports)]

use std::str;
use std::io::{self, Write};
use std::collections::HashMap;

fn echo_cmd(msg: &[String]) {
    println!("{}", msg.join(" "));
}

fn type_cmd(args: &[String]) {
    let mut builtins = HashMap::new();
    builtins.insert("echo", "echo <message>");
    builtins.insert("exit", "exit <exit_status>");
    builtins.insert("type", "type <command>");

    if args.is_empty() || args.len() > 1 {
        match builtins.get(&"type") {
            Some(&help) => { 
                println!("{}", help);
                return;
            },
            _ => println!("Error: invalid argments"),
        }
    }

    let command = args[0].clone();
    match builtins.get(command.as_str()) {
        Some(_) => println!("{} is a shell builtin", command),
        _ => println!("{}: not found", command),
    }
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
            Some("echo") => echo_cmd(&args),
            Some("type") => type_cmd(&args),
            Some(_) => println!("{}: command not found", input.trim()),
            None => continue,
        }
    }
}

