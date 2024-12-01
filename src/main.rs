mod commands;
mod common;

use clap::Parser;
use commands::handle_command;
use common::Cli;

fn main() {
    let cli = Cli::parse();
    handle_command(cli.command);
}
