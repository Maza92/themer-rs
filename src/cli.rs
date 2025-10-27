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

#[derive(Parser, Debug)]
pub struct ListTargets {
    #[arg(long)]
    pub format: Option<String>,
}

#[derive(Parser)]
pub struct Validate {
    pub target: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    List(List),
    ListTargets(ListTargets),
    Apply { palette: String },
    Validate(Validate),
}
