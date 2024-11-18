use clap::Parser;

use crate::{config::Config, tasks::error::TaskError};

#[derive(Parser)]
#[command(name = "edit", about = "Edit the todo file with a text editor")]
pub struct Edit {
    item: Option<u32>,
}

impl Edit {
    pub fn execute(&self, config: Config) -> Result<(), TaskError> {
        let editor = match std::env::var("EDITOR") {
            Ok(value) => value,
            // TODO: check if nvim -> vim -> nano is in the path, else bailout
            // TODO: add (better) loggin / log that if you want to chose the editor, set the EDITOR env
            // var
            Err(_) => "nvim".to_string(),
        };
        let mut cmd = std::process::Command::new(editor);

        if let Some(item) = self.item {
            cmd.arg(format!("+{item}"));
        }

        if let Err(e) = cmd.arg(config.todo_file()).status() {
            // TODO: use a logging library instead of `eprintln!`
            eprintln!("Failed to edit the todo file: {}", e);
            return Err(TaskError::FailedToOpenTodoFile);
        }

        Ok(())
    }
}
