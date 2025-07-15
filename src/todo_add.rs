
use crate::base::TASKS;

pub fn add_task(task: String) {
    let mut tasks = TASKS.lock().unwrap(); // Lock mutex for mutable access
    tasks.push(task);
    println!("Task added!");
}
