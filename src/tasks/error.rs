use std::fmt::Display;

#[derive(Debug)]
pub enum TaskError {
    TaskNotFound,
    FailedToParse,
    FailedToParseQuery,
    FailedToSave,
    FailedToOpenTodoFile,
    FailedToWriteToStdout,
}

impl Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskError::TaskNotFound => f.write_str("Task not found"),
            TaskError::FailedToParse => f.write_str("Failed to parse a task"),
            TaskError::FailedToParseQuery => f.write_str("Failed to parse the query"),
            TaskError::FailedToSave => f.write_str("Failed to save a task"),
            TaskError::FailedToOpenTodoFile => f.write_str("Failed to open todo.txt file"),
            TaskError::FailedToWriteToStdout => f.write_str("Failed to write to stdout"),
        }
    }
}

impl std::error::Error for TaskError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}
