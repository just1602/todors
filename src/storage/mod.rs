use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use tempfile::NamedTempFile;

use crate::{
    error::{IteratorExt, ResultExt, TaskError, TaskResult},
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

        content
            .lines()
            .enumerate()
            .map(|(idx, line)| Task::from_str(idx + 1, line).attach(format!("line {}", idx + 1)))
            .collect_reports_vec()
            .map_err(|reports| {
                let count = reports.len();
                reports.context(TaskError::Multiple { count })
            })
    }

    pub fn persist(&self, tasks: TaskList) -> TaskResult<()> {
        let parent = self
            .todo_file
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));

        let temp_file = NamedTempFile::new_in(parent)
            .context_transform(TaskError::TodoFileIo)
            .attach_with(|| format!("todo file: {}", self.todo_file.display()))?;

        let mut file: &File = temp_file.as_file();
        for task in tasks {
            writeln!(file, "{task}").context_transform(TaskError::TodoFileIo)?;
        }

        file.flush().context_transform(TaskError::TodoFileIo)?;
        file.sync_all().context_transform(TaskError::TodoFileIo)?;

        temp_file
            .persist(&self.todo_file)
            .map_err(std::io::Error::from)
            .context_transform(TaskError::TodoFileIo)
            .attach_with(|| format!("todo file: {}", self.todo_file.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::TaskStorage;
    use crate::{error::TaskError, tasks::task::Task};
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn it_reports_all_malformed_task_lines() {
        let todo_file = NamedTempFile::new().unwrap();
        fs::write(todo_file.path(), "\nvalid task\n\n").unwrap();

        let err = TaskStorage::new(todo_file.path().to_path_buf())
            .get_all()
            .unwrap_err();

        assert!(matches!(
            err.current_context(),
            TaskError::Multiple { count: 2 }
        ));

        let message = format!("{err}");
        assert!(message.contains("line 1"), "{message}");
        assert!(message.contains("line 3"), "{message}");
    }

    #[test]
    fn it_persists_tasks_to_the_todo_file() {
        let todo_file = NamedTempFile::new().unwrap();
        let storage = TaskStorage::new(todo_file.path().to_path_buf());
        let tasks = vec![Task::from_str(1, "(A) a task").unwrap()];

        storage.persist(tasks).unwrap();

        let content = fs::read_to_string(todo_file.path()).unwrap();
        assert_eq!(content, "(A) a task\n");
    }
}
