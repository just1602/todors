pub mod add;
pub mod done;
pub mod due;
pub mod edit;
mod handlers;
pub mod list;
pub mod modify;
pub mod remove;
pub mod undone;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use handlers::handle_clean;

use crate::cli::add::Add;
use crate::cli::done::Done;
use crate::cli::due::Due;
use crate::cli::edit::Edit;
use crate::cli::list::List;
use crate::cli::modify::Modify;
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
            Commands::Due(due) => due.execute(storage),
            Commands::Undone(undone) => undone.execute(storage),
            Commands::Clean => handle_clean(storage),
            Commands::Modify(modify) => modify.execute(storage),
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
    Due(Due),
    Undone(Undone),
    Clean,
    Modify(Modify),
}
