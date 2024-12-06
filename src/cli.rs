use crate::commands::{command::Command, rename_command::RenameCommand, subcommands::Commands, test_command::TestCommand};
use clap::{ColorChoice, Parser};

#[derive(Parser)]
#[command(
    version = concat!("v", env!("CARGO_PKG_VERSION"), " (build ", env!("CARGO_PKG_NAME"), ")"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = env!("CARGO_PKG_DESCRIPTION"),
    color = ColorChoice::Always,
    arg_required_else_help = true
)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Option<Commands>,
}

impl Cli {
	pub fn execute_command(&self) {
		if let Some(cmd) = &self.command {
			match cmd {
				Commands::Rename { recursive, paths, common } => {
					let command = RenameCommand::new(*recursive, paths.clone(), *common);
					command.execute();
				}
				Commands::Test { list, common } => {
					let command = TestCommand::new(*list, *common);
					command.execute();
				}
				Commands::Delete { recursive: _, project: _, paths: _, common: _ } => todo!(),
			}
		} else {
			println!("Nenhum comando fornecido. Utilize --help para mais informações.");
		}
	}
}
