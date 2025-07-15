use std::io::{self, Write};

mod todo_help;
mod base;

use base::*;

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
            Some(HELP) => todo_help::help(),
            Some(ADD) => {
                if args.is_empty() {
                    println!("Usage: {ADD} <task>");
                } else {
                    let task = args.join(" ");
                    println!("Would add task: {}", task);
                }
            }
            Some(EXIT) => break,
            Some(LIST) => {
                println!("Would list tasks.");
            }
            Some(LIST_DONE) => {
                println!("Would list done tasks.");
            }
            Some(DONE) => {
                println!("Would mark task as done.");
            }
            Some(CLEAR) => {
                println!("Would clear all tasks.");
            }
            Some(CLEAR_DONE) => {
                println!("Would clear all done tasks.");
            }
            Some(cmd) => println!("Unknown command: {}", cmd),
            None => continue,
        }
    }
    println!("Goodbye!");
}
