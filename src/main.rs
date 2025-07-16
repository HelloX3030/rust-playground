use std::io::{self, Write};

mod todo_list;

use todo_list::*;

fn main() {
    let mut todo_list = TodoList::new();
    loop {
        let mut user_input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input = user_input.trim();
        if user_input.is_empty() {
            continue;
        }
        let mut parts = user_input.splitn(2, ' ');
        let command = parts.next().unwrap();
        let argument = parts.next().unwrap_or("");
        match command {
            HELP => {
                todo_list.help();
            }
            ADD => {
                if argument.is_empty() {
                    println!("Usage: add <task>");
                } else {
                    todo_list.add(argument.to_string());
                    println!("Task added: {}", argument);
                }
            }
            LIST => {
                todo_list.list();
            }
            LIST_DONE => {
                todo_list.list_done();
            }
            LIST_NOT_DONE => {
                todo_list.list_not_done();
            }
            DONE => {
                if let Ok(index) = argument.parse::<usize>() {
                    todo_list.done(index - 1);
                } else {
                    println!("Usage: done <task_index>");
                }
            }
            CLEAR => {
                todo_list.clear();
            }
            CLEAR_DONE => {
                todo_list.clear_done();
            }
            CLEAR_NOT_DONE => {
                todo_list.clear_not_done();
            }
            IMPORT => {
                if argument.is_empty() {
                    println!("Usage: import <filename>");
                    continue;
                }
                if let Err(e) = todo_list.import(argument.to_string()) {
                    println!("Error importing tasks: {}", e);
                } else {
                    println!("Tasks imported successfully.");
                }
            }
            EXPORT => {
                if argument.is_empty() {
                    println!("Usage: export <filename>");
                    continue;
                }
                if let Err(e) = todo_list.export(argument.to_string()) {
                    println!("Error exporting tasks: {}", e);
                } else {
                    println!("Tasks exported successfully.");
                }
            }
            EXIT => break,
            _ => println!("Unknown command: {}", command),
        }
    }
    println!("Goodbye!");
}
