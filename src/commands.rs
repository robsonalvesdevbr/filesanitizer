use crate::common::CommonOpts;
use clap::Subcommand;
use std::{fs, path::{Path, PathBuf}};

#[derive(Subcommand)]
pub enum Commands {
	/// Rename files
	Rename {
		/// Enable recursive mode
		#[arg(short, long, value_name = "RECURSIVE", default_value = "false", default_value_t = false, display_order = 0, help = "Enable recursive mode [default: false]")]
		recursive: bool,

		/// The pattern to use for renaming
		//#[arg(short, long, value_name = "PATTERN", display_order = 1, help = "The pattern to use for renaming [default: {artist} - {title}]")]
		//pattern: Option<String>,

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
		Some(Commands::Rename { recursive, clean_style_font, paths, common }) => {
			handle_rename_command(recursive, clean_style_font, paths);
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

pub fn read_dir_recursive(dir: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    // Adiciona o caminho encontrado
                    paths.push(path.clone());
                    // Se for um diretório, chama a função recursivamente
                    if path.is_dir() {
                        paths.extend(read_dir_recursive(&path));
                    }
                }
            }
        }
    }

    paths
}

fn handle_rename_command(_recursive: bool, _clean_style_font: bool, paths: Option<Vec<PathBuf>>) {
	if let Some(paths) = paths {
		for file in paths {
			let path = Path::new(&file);

			if path.exists() {
				for file1 in read_dir_recursive(&path) {
					println!("File: {:?}", file1);
				}
			} else {
				println!("File not found: {:?}", file);	
			}
			
		}
	} else {
		println!("No files provided.");
	}

	// println!("Recursive mode: {}", if recursive { "Enabled" } else { "Disabled" });
	// println!("Remove style font: {}", if clean_style_font { "Enabled" } else { "Disabled" });

	// if let Some(paths) = paths {
	// 	for file in paths {
	// 		println!("File to rename: {:?}", file);
	// 	}
	// } else {
	// 	println!("No files provided.");
	// }
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
			assert_eq!(recursive, true, "Expected recursive to be true: {}", recursive);
			assert_eq!(clean_style_font, true, "Expected clean_style_font to be true: {}", clean_style_font);
			assert_eq!(paths, Some(vec![PathBuf::from("list.txt")]), "Expected paths to be Some([PathBuf::from(\"list.txt\")]): {:?}", paths);
			assert_eq!(common.verbose, true, "Expected verbose to be true: {}", common.verbose);
			assert_eq!(common.dry_run, true, "Expected dry_run to be true: {}", common.dry_run);
		} else {
			panic!("Expected Rename command");
		}
	}

	#[test]
	fn test_handle_subcommand_rename_args_with_verbose_false() {
		let args = ["test", "rename", "--recursive", "--clean-style-font", "--dry-run", "list.txt"];
		let cli = Cli::parse_from(args);
		if let Some(Commands::Rename { recursive, clean_style_font, paths, common }) = cli.command {
			assert_eq!(recursive, true, "Expected recursive to be true: {}", recursive);
			assert_eq!(clean_style_font, true, "Expected clean_style_font to be true: {}", clean_style_font);
			assert_eq!(paths, Some(vec![PathBuf::from("list.txt")]), "Expected paths to be Some([PathBuf::from(\"list.txt\")]): {:?}", paths);
			assert_eq!(common.verbose, false, "Expected verbose to be true: {}", common.verbose);
			assert_eq!(common.dry_run, true, "Expected dry_run to be true: {}", common.dry_run);
		} else {
			panic!("Expected Rename command");
		}
	}
	
}
