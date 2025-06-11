use std::io::{self, Write};
use shell_words;

mod rush;
use rush::RushShell;

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

