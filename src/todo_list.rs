use crate::base::{TASKS, Task};

pub fn list_tasks() {
    let tasks = TASKS.lock().unwrap();
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }
    println!("All tasks:");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}: [{}] {}", i + 1, if task.done { "x" } else { " " }, task.description);
    }
}

pub fn list_done_tasks() {
    let tasks = TASKS.lock().unwrap();
    let done_tasks: Vec<_> = tasks.iter().filter(|t| t.done).collect();
    if done_tasks.is_empty() {
        println!("No done tasks found.");
        return;
    }
    println!("Done tasks:");
    for (i, task) in done_tasks.iter().enumerate() {
        println!("{}: {}", i + 1, task.description);
    }
}
