use crate::base::TASKS;

pub fn mark_task_done(index: usize) {
    let mut tasks = TASKS.lock().unwrap();
    if index == 0 || index > tasks.len() {
        println!("Invalid task id");
        return;
    }
    tasks[index - 1].done = true;
    println!("Task {} marked as done.", index);
}
