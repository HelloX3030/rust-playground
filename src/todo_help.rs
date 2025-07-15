use crate::base::*;

pub fn help() {
    println!("Available commands:");
    println!("  {ADD} <task>      - Add a new task");
    println!("  {LIST}            - List all tasks");
    println!("  {LIST_DONE}       - List all done tasks");
    println!("  {DONE} <task_id>  - Mark task as done");
    println!("  {HELP}            - Show this help message");
    println!("  {CLEAR}           - Clear all tasks");
    println!("  {CLEAR_DONE}      - Clear all done tasks");
    println!("  {EXIT}            - Quit the app");
}
