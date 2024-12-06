use crate::commands::commons::{CommonOpts, ProjectType};
use clap::{ColorChoice, Subcommand};
use colored::Colorize;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
	/// Renames files with a timestamp prefix. Supports recursive renaming.
	#[command(
		about = format!("{}\n{}\n{}\n{}\n{}", "Renames files with a timestamp prefix. Supports recursive renaming.".yellow(),
				"- The `rename` command processes the specified files or directories, renaming each file by adding a timestamp prefix to its name.",
				"- The timestamp is based on the files creation time and follows the format `YYYYMMDD_HHMMSS_`",
				"- The command also handles Unicode normalization, ensuring that file paths are normalized using Unicode Normalization Form KC (NFKC).",
				"- This helps in maintaining consistency in file names, especially when dealing with different Unicode representations."),
		display_order = 0,
		color = ColorChoice::Always
	)]
	Rename {
		/// Enable recursive mode
		#[arg(short, long, default_value_t = false, display_order = 0, help = "Enable recursive mode [default: false]")]
		recursive: bool,

		/// The directories to rename files
		paths: Option<Vec<PathBuf>>,

		/// Common options for all subcommands
		#[command(flatten)]
		common: CommonOpts,
	},
	/// Deletes files and directories. Supports recursive deletion.
	#[command(
		about = format!("{}\n{}\n{}\n{}", "Deletes files and directories. Supports recursive deletion.".yellow(),
				"- The `delete` command processes the specified files or directories, deleting each file or directory.",
				"- The command supports recursive deletion, allowing you to delete directories and their contents.",
				"- This is useful for cleaning up project directories, example for .NET projects with `bin` and `obj` folders."),
		display_order = 1,
		color = ColorChoice::Always
	)]
	Delete {
		/// Enable recursive mode
		#[arg(short, long, default_value_t = false, display_order = 0, help = "Enable recursive mode [default: false]")]
		recursive: bool,

		#[arg(value_enum)]
		project: Option<ProjectType>,

		/// The directories to delete files
		paths: Option<Vec<PathBuf>>,

		/// Common options for all subcommands
		#[command(flatten)]
		common: CommonOpts,
	},
	/// Does testing things
	#[command(about = format!("{}", "Performs test operations.".yellow()), display_order = 1)]
	Test {
		/// Lists test values
		#[arg(short, long, display_order = 0)]
		list: bool,

		/// Common options for all subcommands
		#[command(flatten)]
		common: CommonOpts,
	},
}
