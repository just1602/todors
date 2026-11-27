use std::io::{self, Write};

use clap::CommandFactory;
use clap_complete::generate;

use crate::{
    cli::{Cli, Completion},
    error::{AppError, AppResult, OutputError, ResultExt},
};

pub fn handle_completion(params: Completion) -> AppResult<()> {
    let mut cmd = Cli::command();
    let mut script = Vec::new();
    generate(params.shell, &mut cmd, "todors", &mut script);

    let mut stdout = io::stdout().lock();
    stdout
        .write_all(&script)
        .context_transform(|err| AppError::from(OutputError::Io(err)))?;
    stdout
        .flush()
        .context_transform(|err| AppError::from(OutputError::Io(err)))
}
