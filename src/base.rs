pub const HELP: &str = "help";
pub const ADD: &str = "add";
pub const LIST: &str = "list";
pub const LIST_DONE: &str = "list_done";
pub const DONE: &str = "done";
pub const CLEAR: &str = "clear";
pub const CLEAR_DONE: &str = "clear_done";
pub const EXIT: &str = "exit";

// global todo list
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static TASKS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
