use std::io::{self, Write};

mod todo_list;

use todo_list::*;

fn main() {
    let mut task: Task = Task::new(String::from("Example Task"));
    task.print();
    task.done();
    task.print();
    println!("Goodbye!");
}
