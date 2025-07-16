use std::error::Error;
use std::fs::File;
use std::io::{Write, BufWriter, BufReader, BufRead};

pub const HELP: &str = "help";
pub const ADD: &str = "add";
pub const LIST: &str = "list";
pub const LIST_DONE: &str = "list_done";
pub const LIST_NOT_DONE: &str = "list_not_done";
pub const DONE: &str = "done";
pub const CLEAR: &str = "clear";
pub const CLEAR_DONE: &str = "clear_done";
pub const CLEAR_NOT_DONE: &str = "clear_not_done";
pub const EXPORT: &str = "export";
pub const IMPORT: &str = "import";
pub const EXIT: &str = "exit";

const TASK_DONE_EXPORT: &str = "1";
const TASK_NOT_DONE_EXPORT: &str = "0";

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
    pub fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    pub fn help(&self) {
        println!("Available commands:");
        println!("{} - Show this help message", HELP);
        println!("{} <task> - Add a new task", ADD);
        println!("{} - List all tasks", LIST);
        println!("{} - List completed tasks", LIST_DONE);
        println!("{} - List pending tasks", LIST_NOT_DONE);
        println!("{} <index> - Mark task as done", DONE);
        println!("{} - Clear all tasks", CLEAR);
        println!("{} - Clear completed tasks", CLEAR_DONE);
        println!("{} - Clear pending tasks", CLEAR_NOT_DONE);
        println!("{} - Import tasks from a file", IMPORT);
        println!("{} - Export tasks to a file", EXPORT);
        println!("{} - Exit the application", EXIT);
    }

    pub fn add(&mut self, task: String) {
        self.tasks.push(Task {
            description: task,
            done: false,
        });
    }

    pub fn list(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
        } else {
            for (i, task) in self.tasks.iter().enumerate() {
                print!("{}: ", i + 1);
                task.print();
            }
        }
    }

    pub fn list_done(&self) {
        let done_tasks: Vec<&Task> = self.tasks.iter().filter(|task| task.done).collect();
        if done_tasks.is_empty() {
            println!("No completed tasks available.");
        } else {
            for (i, task) in done_tasks.iter().enumerate() {
                print!("{}: ", i + 1);
                task.print();
            }
        }
    }

    pub fn list_not_done(&self) {
        let not_done_tasks: Vec<&Task> = self.tasks.iter().filter(|task| !task.done).collect();
        if not_done_tasks.is_empty() {
            println!("No pending tasks available.");
        } else {
            for (i, task) in not_done_tasks.iter().enumerate() {
                print!("{}: ", i + 1);
                task.print();
            }
        }
    }

    pub fn done(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done();
        } else {
            println!("Task not found.");
        }
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
        println!("All tasks cleared.");
    }

    pub fn clear_done(&mut self) {
        self.tasks.retain(|task| !task.done);
        println!("All completed tasks cleared.");
    }

    pub fn clear_not_done(&mut self) {
        self.tasks.retain(|task| task.done);
        println!("All pending tasks cleared.");
    }

    pub fn import(&mut self, filename: String) -> Result<(), Box<dyn Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            println!("Line: {}", line);
        }

        Ok(())
    }

    pub fn export(&self, filename: String) -> Result<(), Box<dyn Error>> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file); // optionally buffer output

        for task in &self.tasks {
            let status = if task.done { TASK_DONE_EXPORT } else { TASK_NOT_DONE_EXPORT };
            writeln!(writer, "{};{}", status, task.description)?;
        }

        Ok(())
    }
}
