pub mod csv_command;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "csv", about = "show csv or convert to other formats")]
    Csv(csv_command::Command),
}
