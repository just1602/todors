use std::path::PathBuf;

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
        self.todo_dir.join("todo.txt")
    }
}
