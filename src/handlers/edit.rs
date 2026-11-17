use crate::{
    cli::Edit,
    config::Config,
    error::{Result, ResultExt, TaskError, bail},
};

pub fn handle_edit(params: Edit, config: Config) -> Result<()> {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());
    let mut cmd = std::process::Command::new(editor);

    if let Some(item) = params.item {
        cmd.arg(format!("+{item}"));
    }

    let status = cmd
        .arg(config.todo_file()?)
        .status()
        .context_transform(TaskError::Io)
        .attach("opening the todo file in the editor")?;

    if !status.success() {
        bail!(TaskError::Other(format!(
            "editor exited with status {status}"
        )));
    }

    Ok(())
}
