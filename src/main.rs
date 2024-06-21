use clap::Parser;
use std::path::PathBuf;
use todors::{cli::Cli, config::Config};

fn main() {
    let cli = Cli::parse();

    let config_file_path = if let Some(path) = &cli.config_path {
        // FIXME: is there a way to not clone here
        path.clone()
    } else if let Ok(path) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(format!("{}/todors/config.toml", path))
    } else if let Ok(home_path) = std::env::var("HOME") {
        PathBuf::from(format!("{}/.config/todors/config.toml", home_path))
    } else {
        std::process::exit(1);
    };

    let config = Config::from_path(config_file_path);

    cli.run(config)
}
