use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{AppError, AppResult, ConfigError, EnvironmentError, OptionExt, ResultExt};

#[derive(Deserialize)]
pub struct Config {
    pub todo_dir: PathBuf,
}

impl Config {
    pub fn from_path(config_file_path: PathBuf) -> AppResult<Self> {
        let config_file_content = std::fs::read(&config_file_path)
            .context_transform(|err| AppError::from(ConfigError::Io(err)))
            .attach_with(|| format!("config file: {}", config_file_path.display()))?;
        let config_file_content = std::str::from_utf8(&config_file_content)
            .context_transform(|err| AppError::from(ConfigError::Utf8(err)))
            .attach_with(|| format!("config file: {}", config_file_path.display()))?;

        toml::from_str(config_file_content)
            .context_transform(|err| AppError::from(ConfigError::Toml(err)))
            .attach_with(|| format!("config file: {}", config_file_path.display()))
    }

    pub fn todo_file(&self) -> AppResult<PathBuf> {
        let base_path = if self.todo_dir.starts_with("~") {
            let home_path = std::env::var("HOME")
                .ok()
                .context(AppError::from(EnvironmentError::Missing("HOME")))?;

            let folder = self.todo_dir.to_string_lossy();
            let folder = folder.trim_start_matches("~/");

            Path::new(&home_path).join(folder)
        } else {
            self.todo_dir.clone()
        };

        Ok(base_path.join("todo.txt"))
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::error::{AppError, ConfigError};
    use std::path::PathBuf;

    #[test]
    fn it_returns_the_todo_file_name() {
        let config = Config {
            todo_dir: PathBuf::from("/home/test/.todo"),
        };

        assert!(
            config
                .todo_file()
                .unwrap()
                .ends_with("/home/test/.todo/todo.txt")
        );
    }

    #[test]
    fn it_support_tild_as_home_dir() {
        let config = Config {
            todo_dir: PathBuf::from("~/.todo"),
        };

        let todo_file = config.todo_file().unwrap();

        assert!(!todo_file.starts_with("~"));
        assert!(todo_file.ends_with(".todo/todo.txt"));
    }

    #[test]
    fn it_reports_invalid_toml() {
        let config_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(config_file.path(), "todo_dir = ").unwrap();

        let err = match Config::from_path(config_file.path().to_path_buf()) {
            Ok(_) => panic!("invalid TOML should be rejected"),
            Err(err) => err,
        };

        assert!(matches!(
            err.current_context(),
            AppError::Config(ConfigError::Toml(_))
        ));

        let message = format!("{err}");

        assert!(message.contains("invalid TOML"), "{message}");
        assert!(
            message.contains(&format!("config file: {}", config_file.path().display())),
            "{message}"
        );
    }
}
