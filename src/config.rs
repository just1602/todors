use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub todo_dir: PathBuf,
}

impl Config {
    // TODO: Create `config/error.rs` and instead of using expect, catch error and return the right
    // error type
    pub fn from_path(config_file_path: PathBuf) -> Self {
        let config_file_content =
            std::fs::read(config_file_path).expect("Failed to read config file");
        let config_file_content = std::str::from_utf8(&config_file_content)
            .expect("Failed to convert file content to str.");

        toml::from_str(config_file_content).expect("Failed to parse config file.")
    }

    pub fn todo_file(&self) -> PathBuf {
        let base_path = if self.todo_dir.starts_with("~") {
            let home_path = std::env::var("HOME").expect("Failed to retrieve HOME dir path");

            let folder = self.todo_dir.to_string_lossy();
            let folder = folder.trim_start_matches("~/");

            Path::new(&home_path).join(folder)
        } else {
            self.todo_dir.clone()
        };

        base_path.join("todo.txt")
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use std::path::PathBuf;

    #[test]
    fn it_returns_the_todo_file_name() {
        let config = Config {
            todo_dir: PathBuf::from("/home/test/.todo"),
        };

        assert!(config.todo_file().ends_with("/home/test/.todo/todo.txt"));
    }

    #[test]
    fn it_support_tild_as_home_dir() {
        #[cfg(target_os = "linux")]
        let home_path_prefix = "/home/";
        #[cfg(target_os = "macos")]
        let home_path_prefix = "/Users/";

        let config = Config {
            todo_dir: PathBuf::from("~/.todo"),
        };

        assert!(config.todo_file().starts_with(home_path_prefix));
        assert!(config.todo_file().ends_with(".todo/todo.txt"));
    }
}
