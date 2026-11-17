use std::{
    error::Error,
    fmt::{self, Display},
};

pub use rootcause::bail;
pub use rootcause::option_ext::OptionExt;
pub use rootcause::prelude::ResultExt;

pub type Result<T> = rootcause::Result<T>;

#[derive(Debug)]
pub enum TaskError {
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
    FromUtf8(std::string::FromUtf8Error),
    Toml(toml::de::Error),
    MissingEnv(&'static str),
    ParseTask { line: usize, detail: String },
    ParseQuery(String),
    Other(String),
}

impl Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Utf8(err) => write!(f, "invalid UTF-8: {err}"),
            Self::FromUtf8(err) => write!(f, "invalid UTF-8 string: {err}"),
            Self::Toml(err) => write!(f, "invalid TOML: {err}"),
            Self::MissingEnv(name) => write!(f, "missing environment variable: {name}"),
            Self::ParseTask { line, detail } => {
                write!(f, "failed to parse task on line {line}: {detail}")
            }
            Self::ParseQuery(detail) => write!(f, "failed to parse query: {detail}"),
            Self::Other(detail) => write!(f, "{detail}"),
        }
    }
}

impl Error for TaskError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Utf8(err) => Some(err),
            Self::FromUtf8(err) => Some(err),
            Self::Toml(err) => Some(err),
            Self::MissingEnv(_) | Self::ParseTask { .. } | Self::ParseQuery(_) | Self::Other(_) => {
                None
            }
        }
    }
}
