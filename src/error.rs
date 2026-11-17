mod app;
mod task;

pub use app::{AppError, ConfigError, EditorError, EnvironmentError, OutputError};
pub use rootcause::bail;
pub use rootcause::option_ext::OptionExt;
pub use rootcause::prelude::ResultExt;
pub use task::TaskError;

pub type Result<T> = rootcause::Result<T>;
