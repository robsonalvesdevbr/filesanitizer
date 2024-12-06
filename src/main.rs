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

#[cfg(test)]
mod main_tests {
	use std::path::PathBuf;

	use commands::Commands;
	use common::CommonOpts;

	use super::*;

	#[test]
	fn test_handle_command_rename_all_args() {
		let args = ["test", "rename", "--recursive", "--verbose", "--dry-run", "list.txt"];
		let cli = Cli::parse_from(args);
		handle_command(cli.command);
	}

	#[test]
	fn test_handle_command_rename() {
		let command = Commands::Rename { recursive: true, paths: Some(vec![PathBuf::from("file.txt")]), common: CommonOpts { verbose: true, dry_run: false } };

		handle_command(Some(command));
	}

	#[test]
	fn test_handle_command_test() {
		let command = Commands::Test { list: true, common: CommonOpts { verbose: true, dry_run: false } };

		handle_command(Some(command));
	}

	#[test]
	fn test_handle_command_none() {
		handle_command(None);
	}
}
