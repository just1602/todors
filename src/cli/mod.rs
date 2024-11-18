pub mod add;
pub mod done;
pub mod edit;
mod handlers;
pub mod list;
pub mod remove;
pub mod undone;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use handlers::{handle_clean, handle_due, handle_modify};

use crate::cli::add::Add;
use crate::cli::done::Done;
use crate::cli::edit::Edit;
use crate::cli::list::List;
use crate::cli::remove::Remove;
use crate::cli::undone::Undone;
use crate::config::Config;
use crate::storage::TaskStorage;

#[derive(Parser)]
#[command(
    version,
    about = "todors - CLI todo app using the todo.txt format.",
    long_about = None)
]
pub struct Cli {
    #[arg(long = "config", short = 'c', help = "Path to the config file.")]
    pub config_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run(self, config: Config, storage: TaskStorage) {
        let result = match self.command {
            Commands::Add(add) => add.execute(storage),
            Commands::Done(done) => done.execute(storage),
            Commands::List(list) => list.execute(storage),
            Commands::Remove(remove) => remove.execute(storage),
            Commands::Edit(edit) => edit.execute(config),
            Commands::Due => handle_due(storage),
            Commands::Undone(undone) => undone.execute(storage),
            Commands::Clean => handle_clean(storage),
            Commands::Modify(params) => handle_modify(storage, params),
        };

        if let Err(e) = result {
            eprintln!("An error occured: {}", e);
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    Add(Add),
    Done(Done),
    List(List),
    Remove(Remove),
    Edit(Edit),
    Due,
    Undone(Undone),
    Clean,
    Modify(ModifyParams),
}

#[derive(Parser)]
#[command(
    name = "modify",
    visible_alias = "mod",
    about = "Modify selected tasks as desired"
)]
struct ModifyParams {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    query: Vec<String>,

    #[arg(long, short, visible_alias = "pri")]
    priority: Option<char>,
}
