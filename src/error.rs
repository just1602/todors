mod app;
mod task;

use rootcause::{ReportConversion, markers};

pub use app::{AppError, ConfigError, EditorError, EnvironmentError, OutputError};
pub use rootcause::Report;
pub use rootcause::bail;
pub use rootcause::option_ext::OptionExt;
pub use rootcause::prelude::ResultExt;
pub use task::TaskError;

pub type TaskResult<T> = std::result::Result<T, Report<TaskError>>;
pub type AppResult<T> = std::result::Result<T, Report<AppError>>;

impl<T> ReportConversion<TaskError, markers::Mutable, T> for AppError
where
    AppError: markers::ObjectMarkerFor<T>,
{
    fn convert_report(
        report: Report<TaskError, markers::Mutable, T>,
    ) -> Report<Self, markers::Mutable, T> {
        report.context_transform(AppError::Task)
    }
}
