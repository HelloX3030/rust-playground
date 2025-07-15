use std::io::{self, Write};

mod base;
mod todo_add;
mod todo_clear;
mod todo_done;
mod todo_help;
mod todo_list_done;
mod todo_list;
mod todo_remove;

use base::*;
use todo_add::*;
use todo_clear::*;
use todo_done::*;
use todo_help::*;
use todo_list::*;
use todo_list_done::*;
use todo_list::*;
use todo_remove::*;

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
                    add_task(task);
                }
            }
            Some(EXIT) => break,
            Some(LIST) => list_tasks(),
            Some(LIST_DONE) => list_done_tasks(),
            Some(DONE) => {
                if args.is_empty() {
                    println!("Usage: {DONE} <task_id>");
                } else {
                    match args[0].parse::<usize>() {
                        Ok(task_id) => {
                            if task_id == 0 {
                                println!("Task ID must be greater than 0.");
                            } else {
                                mark_task_done(task_id - 1);
                            }
                        }
                        Err(_) => println!("Invalid task ID: {}", args[0]),
                    }
                }
            },
            Some(CLEAR) => clear_tasks(),
            Some(CLEAR_DONE) => clear_done_tasks(),
            Some(cmd) => println!("Unknown command: {}", cmd),
            None => continue,
        }
    }
    println!("Goodbye!");
}
