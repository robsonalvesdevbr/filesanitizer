use crate::common::CommonOpts;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
	/// Rename files
	Rename {
		/// Enable recursive mode
		#[arg(short, long, value_name = "RECURSIVE", default_value = "false", default_value_t = false, display_order = 0, help = "Enable recursive mode [default: false]")]
		recursive: bool,

		/// The pattern to use for renaming
		#[arg(short, long, value_name = "PATTERN", display_order = 1, help = "The pattern to use for renaming [default: {artist} - {title}]")]
		pattern: Option<String>,

		/// Clean style font
		#[arg(short, long, value_name = "CLEAN_STYLE_FONT", default_value = "true", default_value_t = true, display_order = 2, help = "Remove style font from the file name [default: true]")]
		clean_style_font: bool,

		/// The files to rename
		paths: Option<Vec<PathBuf>>,

		/// Common options for all subcommands
		#[command(flatten)]
		common: CommonOpts,
	},
	/// Does testing things
	Test {
		/// Lists test values
		#[arg(short, long, display_order = 0)]
		list: bool,

		/// Common options for all subcommands
		#[command(flatten)]
		common: CommonOpts,
	},
}

pub fn handle_command(command: Option<Commands>) {
	match command {
		Some(Commands::Rename { pattern, recursive, clean_style_font, paths, common }) => {
			handle_rename_command(pattern, recursive, clean_style_font, paths);
			common.handle_common_opts();
		}
		Some(Commands::Test { list, common }) => {
			if list {
				println!("Listing test values...");
			}
			common.handle_common_opts();
		}
		None => {
			println!("Nenhum comando fornecido. Utilize --help para mais informações.");
		}
	}
}

fn handle_rename_command(pattern: Option<String>, recursive: bool, clean_style_font: bool, paths: Option<Vec<PathBuf>>) {
	println!("Pattern to use: {}", pattern.unwrap_or("No pattern provided".to_string()));
	println!("Recursive mode: {}", if recursive { "Enabled" } else { "Disabled" });
	println!("Remove style font: {}", if clean_style_font { "Enabled" } else { "Disabled" });

	if let Some(paths) = paths {
		for file in paths {
			println!("File to rename: {:?}", file);
		}
	} else {
		println!("No files provided.");
	}
}

#[cfg(test)]
mod tests {
	use clap::Parser;
	use std::path::PathBuf;

	use crate::commands::Commands;
	use crate::common::Cli;

	#[test]
	fn test_handle_subcommand_rename_args() {
		let args = [
			"test",
			"rename",
			"--pattern",
			"*.txt",
			"--recursive",
			"--clean-style-font",
			"--verbose",
			"--dry-run",
			"list.txt",
		];
		let cli = Cli::parse_from(args);
		if let Some(Commands::Rename { recursive, pattern, clean_style_font, paths, common }) = cli.command {
			assert_eq!(recursive, true);
			assert_eq!(pattern, Some("*.txt".to_string()));
			assert_eq!(clean_style_font, true);
			assert_eq!(paths, Some(vec![PathBuf::from("list.txt")]));
			assert_eq!(common.verbose, true);
			assert_eq!(common.dry_run, true);
		} else {
			panic!("Expected Rename command");
		}
	}
}
