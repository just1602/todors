use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("todo file I/O error")]
    TodoFileIo(#[from] io::Error),

    #[error("failed to parse task on line {line}: {detail}")]
    ParseTask { line: usize, detail: String },

    #[error("failed to parse query: {detail}")]
    ParseQuery { detail: String },

    #[error("invalid priority: {0}")]
    InvalidPriority(char),

    #[error("empty task subject")]
    EmptySubject,

    #[error("failed to parse {count} task lines")]
    Multiple { count: usize },
}
