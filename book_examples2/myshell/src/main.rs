use std::io::{stdin, stdout, Error, ErrorKind, Write};
use std::process::Command;

fn main() {
    println!("Welcome to Myshell");
    loop {
        print!("$ ");
        stdout().flush().unwrap();

        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Unable to read user input");

        let command_to_execute = user_input.trim();
        let command_args: Vec<&str> = command_to_execute.split_whitespace().collect();

        if command_args.len() > 0 {
            let child = match command_args[0] {
                "show" if command_args.len() > 1 => match command_args[1] {
                    "files" => Command::new("ls").args(&command_args[2..]).spawn(),
                    "process" => Command::new("ps").args(&command_args[2..]).spawn(),
                    _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid Command")),
                },
                "show" if command_args.len() == 1 => {
                    Err(Error::new(ErrorKind::InvalidInput, "Invalid Command"))
                }
                "quit" => std::process::exit(0),
                _ => Command::new(command_args[0])
                    .args(&command_args[1..])
                    .spawn(),
            };
            match child {
                Ok(mut child) => {
                    if !child.wait().unwrap().success() {
                        println!("\n Child process failed")
                    }
                }
                Err(e) => match e.kind() {
                    ErrorKind::InvalidInput => {
                        eprintln!("Show command supports following opions: files, process ")
                    }

                    _ => eprintln!("Enter a valid command"),
                },
            }
        }
    }
}
