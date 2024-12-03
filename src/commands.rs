use crate::common::CommonOpts;
use clap::Subcommand;
use colored::Colorize;
use std::{
	fs,
	path::{Path, PathBuf},
};

#[derive(Subcommand)]
pub enum Commands {
	/// Rename files
	Rename {
		/// Enable recursive mode
		#[arg(short, long, default_value_t = false, display_order = 0, help = "Enable recursive mode [default: false]")]
		recursive: bool,

		/// Clean style font
		#[arg(short, long, default_value_t = true, display_order = 2, help = "Remove style font from the file name [default: true]")]
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
		Some(Commands::Rename { recursive, clean_style_font, paths, common }) => {
			handle_rename_command(recursive, clean_style_font, paths, common);
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

pub fn read_dir_recursive(dir: &Path) -> Vec<PathBuf> {
	let mut paths = Vec::new();

	if dir.is_dir() {
		if let Ok(entries) = fs::read_dir(dir) {
			for entry in entries.flatten() {
				let path = entry.path();
				paths.push(path.clone());
				if path.is_dir() {
					paths.extend(read_dir_recursive(&path));
				}
			}
		}
	}

	paths
}

fn println_line_path_info(path: &Path, new_path: &Path, common: CommonOpts) {
	if !common.verbose {
		return;
	}

	let dry_run = if common.dry_run { "Dry-run mode enabled." } else { "" };

	let name = path.to_str().unwrap_or("Invalid UTF-8").chars().take(50).collect::<String>();
	let new_name = new_path.to_str().unwrap_or("Invalid UTF-8").chars().take(50).collect::<String>();

	if path.is_dir() {
		println!("{:<10}: {:<130} {:<10}", "Diretório", name.bold().blue(), dry_run.yellow());
	} else {
		println!("{:<10}: {:<130} {:<10}", "File", format!("{} -> {}", name.bold().blue(), new_name.bold().blue()), dry_run.yellow());
	}
}

fn generate_new_name_with_timestamp(file: &Path) -> Option<PathBuf> {
	if !file.is_file() {
		return Some(file.to_path_buf());
	}
	let metadata = fs::metadata(file).unwrap();
	let created = metadata.created().unwrap();
	let created: chrono::DateTime<chrono::Local> = created.into();
	let new_name_with_timestamp = format!("{}{}", created.format("%Y%m%d_%H%M%S_"), file.file_name().unwrap().to_str().unwrap());
	let new_path = file.with_file_name(new_name_with_timestamp);
	Some(new_path)
}

// fn rename_file(file: &Path, new_file: &Path) -> bool {
// 	fs::rename(file, new_file).is_ok()
// }

fn handle_rename_command(recursive: bool, clean_style_font: bool, paths: Option<Vec<PathBuf>>, common: CommonOpts) {
	let dry_run = if common.dry_run { "Dry-run mode enabled." } else { "" };
	let recursive_msg = if recursive { "Recursive mode enabled." } else { "" };
	let clean_style_font_msg = if clean_style_font { "Clean style font enabled." } else { "" };
	let verbose = if common.verbose { "Verbose mode enabled." } else { "" };

	println!("{}", "-".repeat(100).yellow());

	if !dry_run.is_empty() {
		println!("{}", dry_run.yellow());
	}
	if !recursive_msg.is_empty() {
		println!("{}", recursive_msg.yellow());
	}
	if !clean_style_font_msg.is_empty() {
		println!("{}", clean_style_font_msg.yellow());
	}
	if !verbose.is_empty() {
		println!("{}", verbose.yellow());
	}

	println!("{}", "-".repeat(100).yellow());

	if let Some(paths) = paths {
		for path_argument in paths {
			let path = Path::new(&path_argument);

			if path.exists() {
				println_line_path_info(path, path, common);
				for file in read_dir_recursive(path) {
					match generate_new_name_with_timestamp(&file) {
						Some(new_path) => {
							if !common.dry_run {
								fs::rename(file.clone(), new_path.clone()).unwrap();
							}
							println_line_path_info(&file.clone(), &new_path.clone(), common);
						}
						None => {
							println!("File not found: {:?}", path_argument);
						}
					}
				}
			} else {
				println!("File not found: {:?}", path_argument);
			}
		}
	} else {
		println!("No files provided.");
	}
}

#[cfg(test)]
mod commands_tests {
	use clap::Parser;
	use std::path::PathBuf;

	use crate::commands::Commands;
	use crate::common::Cli;

	#[test]
	fn test_handle_subcommand_rename_args() {
		let args = ["test", "rename", "--recursive", "--clean-style-font", "--verbose", "--dry-run", "list.txt"];
		let cli = Cli::parse_from(args);
		if let Some(Commands::Rename { recursive, clean_style_font, paths, common }) = cli.command {
			assert!(recursive, "Expected recursive to be true: {}", recursive);
			assert!(clean_style_font, "Expected clean_style_font to be true: {}", clean_style_font);
			assert_eq!(paths, Some(vec![PathBuf::from("list.txt")]), "Expected paths to be Some([PathBuf::from(\"list.txt\")]): {:?}", paths);
			assert!(common.verbose, "Expected verbose to be true: {}", common.verbose);
			assert!(common.dry_run, "Expected dry_run to be true: {}", common.dry_run);
		} else {
			panic!("Expected Rename command");
		}
	}

	#[test]
	fn test_handle_subcommand_rename_args_with_verbose_false() {
		let args = ["test", "rename", "--recursive", "--clean-style-font", "--dry-run", "list.txt"];
		let cli = Cli::parse_from(args);
		if let Some(Commands::Rename { recursive, clean_style_font, paths, common }) = cli.command {
			assert!(recursive, "Expected recursive to be true: {}", recursive);
			assert!(clean_style_font, "Expected clean_style_font to be true: {}", clean_style_font);
			assert_eq!(paths, Some(vec![PathBuf::from("list.txt")]), "Expected paths to be Some([PathBuf::from(\"list.txt\")]): {:?}", paths);
			assert!(!common.verbose, "Expected verbose to be false: {}", common.verbose);
			assert!(common.dry_run, "Expected dry_run to be true: {}", common.dry_run);
		} else {
			panic!("Expected Rename command");
		}
	}
}
