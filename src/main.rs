use std::io::{self, Write};

mod todo_help;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        // Split input into words
        let mut parts = input.split_whitespace();
        let command = parts.next();
        let args: Vec<&str> = parts.collect();

        match command {
            Some("help") => todo_help::help(),
            Some("add") => {
                if args.is_empty() {
                    println!("Usage: add <task>");
                } else {
                    let task = args.join(" ");
                    println!("Would add task: {}", task);
                }
            }
            Some("exit") => break,
            Some("list") => {
                println!("Would list tasks.");
            }
            Some("done") => {
                println!("Would mark task as done.");
            }
            Some(cmd) => println!("Unknown command: {}", cmd),
            None => continue,
        }
    }
    println!("Goodbye!");
}
