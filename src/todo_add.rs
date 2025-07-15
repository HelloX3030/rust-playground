use crate::base::{TASKS, Task};

pub fn add_task(description: String) {
    let mut tasks = TASKS.lock().unwrap();
    tasks.push(Task {
        description,
        done: false,
    });
    println!("Task added!");
}
