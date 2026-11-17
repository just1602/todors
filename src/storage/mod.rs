use std::{fs::OpenOptions, io::Write, path::PathBuf};

use crate::{
    error::{ResultExt, TaskError, TaskResult},
    tasks::{list::TaskList, task::Task},
};

pub struct TaskStorage {
    todo_file: PathBuf,
}

impl TaskStorage {
    pub fn new(todo_file: PathBuf) -> Self {
        Self { todo_file }
    }
}

impl TaskStorage {
    pub fn get_all(&self) -> TaskResult<TaskList> {
        let content = std::fs::read_to_string(&self.todo_file)
            .context_transform(TaskError::TodoFileIo)
            .attach_with(|| format!("todo file: {}", self.todo_file.display()))?;

        let mut tasks = TaskList::new();
        for (idx, line) in content.lines().enumerate() {
            let task = Task::from_str(idx + 1, line)?;

            tasks.push(task)
        }

        Ok(tasks)
    }

    pub fn persist(&self, tasks: TaskList) -> TaskResult<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&self.todo_file)
            .context_transform(TaskError::TodoFileIo)
            .attach_with(|| format!("todo file: {}", self.todo_file.display()))?;

        for task in tasks {
            writeln!(file, "{task}").context_transform(TaskError::TodoFileIo)?;
        }

        Ok(())
    }
}
