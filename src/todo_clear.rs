use crate::base::TASKS;

pub fn clear_tasks() {
    let mut tasks = TASKS.lock().unwrap();
    tasks.clear();
    println!("All tasks cleared.");
}

pub fn clear_done_tasks() {
    let mut tasks = TASKS.lock().unwrap();
    tasks.retain(|task| !task.done);
    println!("All done tasks cleared.");
}
