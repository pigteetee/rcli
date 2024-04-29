use anyhow::Result;
use clap::Parser;
use rcli::{
    commands::{csv_command, Commands},
    Program,
};

fn main() -> Result<()> {
    let program = Program::parse();

    match program.commands {
        Commands::Csv(options) => csv_command::action(&options.input, &options.output, &options)?,
    }

    return Ok(());
}
