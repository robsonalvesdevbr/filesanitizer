// src/main.rs
mod cli;
mod commands;
mod file_operations;
use clap::Parser;
use cli::Cli;

fn main() -> Result<(), std::io::Error> {
	let cli = Cli::parse();
	cli.execute_command();
	Ok(())
}
