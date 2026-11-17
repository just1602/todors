use std::{io, process::ExitStatus};

use thiserror::Error;

use super::TaskError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Task(#[from] TaskError),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Environment(#[from] EnvironmentError),

    #[error(transparent)]
    Editor(#[from] EditorError),

    #[error(transparent)]
    Output(#[from] OutputError),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config file I/O error")]
    Io(#[from] io::Error),

    #[error("invalid UTF-8 in config file")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("invalid TOML config")]
    Toml(#[from] toml::de::Error),
}

#[derive(Debug, Error)]
pub enum EnvironmentError {
    #[error("missing environment variable: {0}")]
    Missing(&'static str),

    #[error("environment variable is not valid Unicode: {0}")]
    NotUnicode(&'static str),
}

#[derive(Debug, Error)]
pub enum EditorError {
    #[error("failed to launch editor")]
    Io(#[from] io::Error),

    #[error("editor exited with status {0}")]
    NonZeroStatus(ExitStatus),
}

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("failed to write output")]
    Io(#[from] io::Error),
}
