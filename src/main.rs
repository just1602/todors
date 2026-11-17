use std::{path::PathBuf, process::ExitCode};

use clap::Parser;
use todors::{
    cli::{Cli, Commands},
    config::Config,
    handlers::*,
    storage::TaskStorage,
    tasks::error::TaskError,
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

fn run() -> Result<(), TaskError> {
    let Cli {
        config_path,
        command,
    } = Cli::parse();

    let config_file_path = if let Some(path) = config_path {
        path
    } else if let Ok(path) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(path).join("todors/config.toml")
    } else if let Ok(home_path) = std::env::var("HOME") {
        PathBuf::from(home_path).join(".config/todors/config.toml")
    } else {
        std::process::exit(1);
    };

    let config = Config::from_path(config_file_path);
    let storage = TaskStorage::new(config.todo_file());

    match command {
        Commands::Add(params) => handle_add(params, storage),
        Commands::Done(params) => handle_done(params, storage),
        Commands::List(params) => handle_list(params, storage),
        Commands::Remove(params) => handle_remove(params, storage),
        Commands::Edit(params) => handle_edit(params, config),
        Commands::Due(params) => handle_due(params, storage),
        Commands::Undone(params) => handle_undone(params, storage),
        Commands::Clean(params) => handle_clean(params, storage),
        Commands::Modify(params) => handle_modify(params, storage),
        Commands::Next(params) => handle_next(params, storage),
    }
}
