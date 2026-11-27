use std::{path::PathBuf, process::ExitCode};

use clap::Parser;
use todors::{
    cli::{Cli, Commands},
    config::Config,
    error::{AppError, AppResult, EnvironmentError, bail},
    handlers::*,
    storage::TaskStorage,
};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> AppResult<()> {
    let Cli {
        config_path,
        command,
    } = Cli::parse();

    match command {
        Commands::Completion(params) => handle_completion(params),
        Commands::Add(params) => with_storage(config_path, |storage| handle_add(params, storage)),
        Commands::Done(params) => with_storage(config_path, |storage| handle_done(params, storage)),
        Commands::List(params) => with_storage(config_path, |storage| handle_list(params, storage)),
        Commands::Remove(params) => {
            with_storage(config_path, |storage| handle_remove(params, storage))
        }
        Commands::Edit(params) => with_config(config_path, |config| handle_edit(params, config)),
        Commands::Due(params) => with_storage(config_path, |storage| handle_due(params, storage)),
        Commands::Undone(params) => {
            with_storage(config_path, |storage| handle_undone(params, storage))
        }
        Commands::Clean(params) => {
            with_storage(config_path, |storage| handle_clean(params, storage))
        }
        Commands::Modify(params) => {
            with_storage(config_path, |storage| handle_modify(params, storage))
        }
        Commands::Next(params) => with_storage(config_path, |storage| handle_next(params, storage)),
    }
}

fn resolve_config_path(config_path: Option<PathBuf>) -> AppResult<PathBuf> {
    if let Some(path) = config_path {
        return Ok(path);
    }

    if let Ok(path) = std::env::var("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(path).join("todors/config.toml"));
    }

    if let Ok(home_path) = std::env::var("HOME") {
        return Ok(PathBuf::from(home_path).join(".config/todors/config.toml"));
    }

    bail!(AppError::from(EnvironmentError::Missing(
        "XDG_CONFIG_HOME or HOME"
    )))
}

fn with_config<T>(
    config_path: Option<PathBuf>,
    action: impl FnOnce(Config) -> AppResult<T>,
) -> AppResult<T> {
    let config_file_path = resolve_config_path(config_path)?;
    let config = Config::from_path(config_file_path)?;

    action(config)
}

fn with_storage<T>(
    config_path: Option<PathBuf>,
    action: impl FnOnce(TaskStorage) -> AppResult<T>,
) -> AppResult<T> {
    with_config(config_path, |config| {
        let storage = TaskStorage::new(config.todo_file()?);
        action(storage)
    })
}
