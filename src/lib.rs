pub mod commands;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author)]
pub struct Program {
    #[command(subcommand)]
    pub commands: commands::Commands,
}
