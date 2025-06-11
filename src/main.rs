#[allow(unused_imports)]

use std::env;
use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::str;
use shell_words;
use walkdir::WalkDir;

struct RushShell {
    builtins : HashMap<String, String>,
    path : Vec<String>,
    current_dir: String,
    home: String,
}

impl RushShell {
    pub fn new() -> Self {
        let mut builtins_table = HashMap::new();
        builtins_table.insert("echo".to_string(), "echo <message>".to_string());
        builtins_table.insert("exit".to_string(), "exit <exit_status>".to_string());
        builtins_table.insert("type".to_string(), "type <command>".to_string());
        builtins_table.insert("pwd".to_string(), "pwd".to_string());
        builtins_table.insert("cd".to_string(), "cd <target_directory>".to_string());
    
        return RushShell{
            path: env::var("PATH").unwrap().split(":").map(str::to_string).collect(),
            builtins: builtins_table,
            current_dir: env::current_dir().expect("Couldn't get current dir").display().to_string(),
            home: env::var("HOME").expect("Undefined $HOME")
        }
    }

    pub fn echo(&self, msg: &[String]) {
        println!("{}", msg.join(" "));
    }

    pub fn cmd_type(&self, args: &[String]) {
        if args.is_empty() || args.len() > 1 {
            match self.builtins.get("type") {
                Some(help) => { 
                    println!("{}", help);
                    return;
                },
                _ => eprintln!("Error: invalid argments"),
            }
        }
    
        let command = args[0].clone();
        match self.builtins.get(command.as_str()) {
            Some(_) => println!("{} is a shell builtin", command),
            _ => match self.search(&command) {
                    Some(p) => println!("{} is {}", command, p),
                    None => eprintln!("{}: not found", command),
                },
        }
    }

    pub fn pwd(&self) {
        let pwd = WalkDir::new(&self.current_dir).max_depth(0).into_iter().next();
        match pwd {
            Some(Ok(entry)) => {
                let abs_path = fs::canonicalize(entry.path()).unwrap();
                println!("{}", abs_path.display());
            },
            Some(Err(error)) => eprintln!("failed to get current directory: {}", error),
            None => eprintln!("no current working directory available!")
        }
    }

    
    pub fn chdir(&mut self, args: &[String]) {
        if args.len() > 1{
            eprintln!("cd: too many arguments");
        } else {
            let mut dir;
            if args.len() == 0 {
                dir = self.home.clone();
            } else {
                dir = args[0].clone();
            }

            if dir.starts_with("~") {
                dir = dir.replace("~", &self.home.to_string());
            }
            let mut path = Path::new(&dir);
            if path.exists() {
                let path_buf = fs::canonicalize(&dir).unwrap();
                path = path_buf.as_path();
    
                if env::set_current_dir(&path).is_ok() {
                    self.current_dir = env::current_dir().expect("Couldn't get current dir").display().to_string();
                } else {
                    eprintln!("cd: error changing directory!");
                }
            } else {
                eprintln!("cd: {}: No such file or directory", dir);
            }    
        }
    }

    fn search(&self, program: &String) -> Option<String> {
        for p in &self.path {
            for entry in WalkDir::new(p).into_iter().filter_map(|e| e.ok()) {
                let bin = entry.path().display().to_string();
                if bin.ends_with(&format!("/{}", program)) {
                    return Some(bin);
                 }
            }
        }
        return None
    }

    pub fn execute(&self, program : &String, arguments : &Vec<String>) {
        let cmd = program.rsplit("/").next().unwrap();
        let output = Command::new(cmd).args(arguments).output().expect("failed to execute process");
    
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }    
}

fn main() {
    let mut rush = RushShell::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = shell_words::split(&input).unwrap_or_default();
        let command = parts.get(0).cloned();
        let args :Vec<String> = parts.drain(1..).collect();

        match command.as_deref() {
            Some("exit") => {
                if args.is_empty() || args == ["0"] {
                    break;
                } else {
                    println!("Wrong exit");
                }
            },
            Some("echo") => rush.echo(&args),
            Some("type") => rush.cmd_type(&args),
            Some("pwd") => rush.pwd(),
            Some("cd") => rush.chdir(&args),
            Some("chdir") => rush.chdir(&args),
            Some(cmd) => {
                match rush.search(&cmd.to_string()) {
                    Some(file_path) => rush.execute(&file_path, &args),
                    None => eprintln!("{}: command not found", input.trim()),
                }
            },
            None => continue,
        }
    }
}

