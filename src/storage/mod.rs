use std::{fs::OpenOptions, io::Write, path::PathBuf};

use crate::tasks::{error::TaskError, list::TaskList, task::TaskBuilder};

pub struct TaskStorage {
    todo_file: PathBuf,
}

impl TaskStorage {
    pub fn new(todo_file: PathBuf) -> Self {
        Self { todo_file }
    }
}

impl TaskStorage {
    pub fn get_all(&self) -> Result<TaskList, TaskError> {
        let Ok(content) = std::fs::read_to_string(&self.todo_file) else {
            return Err(TaskError::FailedToOpenTodoFile);
        };

        let mut tasks = TaskList::new();
        for (idx, line) in content.lines().enumerate() {
            let task = TaskBuilder::new(idx + 1, line.to_string()).build()?;

            tasks.push(task)
        }

        Ok(tasks)
    }

    pub fn perist(&self, tasks: TaskList) -> Result<(), TaskError> {
        let mut file = if let Ok(file) = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&self.todo_file)
        {
            file
        } else {
            return Err(TaskError::FailedToOpenTodoFile);
        };

        for task in tasks {
            match file.write_fmt(format_args!("{}\n", task)) {
                Ok(_) => {}
                Err(_) => return Err(TaskError::FailedToSave),
            }
        }

        Ok(())
    }
}
