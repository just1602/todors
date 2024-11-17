use std::{fs::OpenOptions, io::Write, path::PathBuf};

use crate::tasks::{
    error::TaskError,
    list::{TaskList, TaskListItem},
    task::Task,
};

pub trait TaskStorage {
    fn get_all(&self) -> Result<TaskList, TaskError>;
    fn perist(&self, tasks: TaskList) -> Result<(), TaskError>;
}

pub struct FileTaskStorage {
    todo_file: PathBuf,
}

impl FileTaskStorage {
    pub fn new(todo_file: PathBuf) -> Self {
        FileTaskStorage { todo_file }
    }
}

impl TaskStorage for FileTaskStorage {
    fn get_all(&self) -> Result<TaskList, TaskError> {
        let Ok(content) = std::fs::read_to_string(&self.todo_file) else {
            return Err(TaskError::FailedToOpenTodoFile);
        };

        let mut tasks = TaskList::new();
        for (idx, line) in content.lines().enumerate() {
            let Ok(task) = line.parse::<Task>() else {
                return Err(TaskError::FailedToParse);
            };

            tasks.push(TaskListItem { idx: idx + 1, task })
        }

        Ok(tasks)
    }

    fn perist(&self, tasks: TaskList) -> Result<(), TaskError> {
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

        for item in tasks {
            match file.write_fmt(format_args!("{}\n", item.task)) {
                Ok(_) => {}
                Err(_) => return Err(TaskError::FailedToSave),
            }
        }

        Ok(())
    }
}
