use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod config;
mod output;
mod palette;
mod target;
mod template;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List(list) => commands::list::execute(list.format.as_deref()),
        Commands::Apply { palette } => commands::apply::execute(&palette),
    }
}
