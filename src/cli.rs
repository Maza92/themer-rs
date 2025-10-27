use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub struct List {
    #[arg(long)]
    pub format: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    List(List),
    Apply { palette: String },
}
