
mod commands;
mod common;
use clap::Parser;
use commands::handle_command;
use common::Cli;

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();
    handle_command(cli.command);
    Ok(())
}
