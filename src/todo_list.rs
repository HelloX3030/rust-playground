
pub const HELP: &str = "help";
pub const ADD: &str = "add";
pub const LIST: &str = "list";
pub const LIST_DONE: &str = "list_done";
pub const DONE: &str = "done";
pub const CLEAR: &str = "clear";
pub const CLEAR_DONE: &str = "clear_done";
pub const EXIT: &str = "exit";

pub struct Task {
    pub description: String,
    pub done: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Task {
            description,
            done: false,
        }
    }

    pub fn print(&self) {
        println!("{}: {}", if self.done { "[x]" } else { "[ ]" }, self.description);
    }

    pub fn done(&mut self) {
        self.done = true;
    }
}

pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    fn add(&mut self, task: String) {
        self.tasks.push(Task {
            description: task,
            done: false,
        });
    }
}
