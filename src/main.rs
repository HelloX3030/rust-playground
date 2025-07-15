use std::io::{self, Write};

mod todo_help;

fn main() {
    loop {
        print!("> "); // CLI prompt
        io::stdout().flush().unwrap(); // Force flush to show prompt

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim(); // remove newline + spaces

        // Split input into words
        let mut parts = input.split_whitespace();
        let command = parts.next(); // first word
        let args: Vec<&str> = parts.collect(); // remaining words

        match command {
            Some("help") => todo_help::help(),
            Some("add") => {
                if args.is_empty() {
                    println!("Usage: add <task>");
                } else {
                    let task = args.join(" ");
                    println!("Would add task: {}", task);
                    // call your add_task() function here
                }
            }
            Some("exit") => break,
            Some("list") => {
                // call list function
                println!("Would list tasks.");
            }
            Some("done") => {
                // call mark done function
                println!("Would mark task as done.");
            }
            Some(cmd) => println!("Unknown command: {}", cmd),
            None => continue,
        }
    }

    println!("Goodbye!");
}
