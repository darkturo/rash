#[allow(unused_imports)]

use std::env;
use std::str;
use std::io::{self, Write};
use std::collections::HashMap;
use std::process::Command;
use walkdir::WalkDir;

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
        _ => {
            let (found, p) = search_in_path(&command);
            if found {
                println!("{} is {}", command, p);
            } else {
                println!("{}: not found", command);
            }
        }
    }
}

fn search(program: &String, path: &Vec<String>) -> (bool, String) {
    for p in path {
        for entry in WalkDir::new(p).into_iter().filter_map(|e| e.ok()) {
            let bin = entry.path().display().to_string();
            if bin.ends_with(&format!("/{}", program)) {
                return (true, bin);
             }                 
        }
    }
    return (false, String::new())
}

fn search_in_path(program : &String) -> (bool, String) {
    let path : Vec<String> = env::var("PATH").unwrap().split(":").map(str::to_string).collect();
    return search(&program, &path);
}

fn execute(program : &String, arguments : &Vec<String>) {
    let cmd = program.rsplit("/").next().unwrap();
    let output = Command::new(cmd).args(arguments).output().expect("failed to execute process");

    print!("{}", String::from_utf8_lossy(&output.stdout));
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
            Some(cmd) => {
                let (found, file_path) = search_in_path(&cmd.to_string());
                if found {
                    execute(&file_path, &args);
                } else {
                    println!("{}: command not found", input.trim());
                }
            },
            None => continue,
        }
    }
}

