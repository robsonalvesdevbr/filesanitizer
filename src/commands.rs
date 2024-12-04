use crate::common::CommonOpts;
use clap::Subcommand;
use colored::Colorize;
use regex::Regex;
use std::{
	env, fs,
	path::{Path, PathBuf},
};
use unicode_normalization::UnicodeNormalization;

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
			let processor = RenameProcessor::new(recursive, clean_style_font, common);
			let direct = match &paths {
				Some(paths) => Some(paths.clone()),
				None => match env::current_dir() {
					Ok(path) => Some(vec![path]),
					Err(e) => {
						println!("Error: {}", e);
						None
					}
				},
			};

			processor.process(direct);
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

pub fn read_dir_recursive(dir: &Path, recursive: bool) -> Result<Vec<PathBuf>, std::io::Error> {
	let mut paths = Vec::new();

	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();
			paths.push(path.clone());
			if path.is_dir() && recursive {
				paths.extend(read_dir_recursive(&path, recursive)?);
			}
		}
	}

	// Pré-processa os nomes para evitar normalizações repetitivas
	paths.sort_by(|a, b| {
		let a_name = a.to_string_lossy().nfkc().collect::<String>();
		let b_name = b.to_string_lossy().nfkc().collect::<String>();
		a_name.cmp(&b_name)
	});

	Ok(paths)
}

fn println_line_path_info(path: &Path, new_path: &Path, common: CommonOpts) {
	if !common.verbose {
		return;
	}

	let dry_run = if common.dry_run { "Dry-run mode enabled." } else { "" };

	let name = path.to_str().unwrap_or("Invalid UTF-8");
	let new_name = new_path.to_str().unwrap_or("Invalid UTF-8");
	let name_group = format!("{} -> {:<130}", name, new_name).chars().take(130).collect::<String>();

	if path.is_dir() {
		println!("{:<10}: {:<130} {:<10}", "Diretório", name.bold().blue(), dry_run.yellow());
	} else {
		//println!("{:<10}: {:<130} {:<10}", "File", format!("{} -> {}", name.bold().blue(), new_name.bold().blue()).chars().take(130).collect::<String>(), dry_run.yellow());
		println!("{:<10}: {} {:<10}", "File", name_group.blue(), dry_run.yellow());
	}
}

fn generate_new_name_with_timestamp(file: &Path) -> Option<PathBuf> {
	let original_path = PathBuf::from(file);
	let original_string = original_path.to_string_lossy();
	let normalized_string: String = original_string.nfkc().collect();
	let binding = PathBuf::from(normalized_string);
	let normalized_path: &Path = binding.as_path();

	if normalized_path.is_dir() {
		return Some(file.to_path_buf());
	}

	let re = Regex::new(r"(\d{8}_\d{6})").unwrap();
	if re.is_match(normalized_path.file_name().unwrap().to_str().unwrap()) {
		return None;
	}

	let metadata = fs::metadata(file).unwrap();
	let created = metadata.created().unwrap();
	let created: chrono::DateTime<chrono::Local> = created.into();
	let new_name_with_timestamp = format!("{}{}", created.format("%Y%m%d_%H%M%S_"), normalized_path.file_name().unwrap().to_str().unwrap());
	let new_path = normalized_path.with_file_name(new_name_with_timestamp);
	Some(new_path)
}

struct RenameProcessor {
	recursive: bool,
	clean_style_font: bool,
	common: CommonOpts,
}

impl RenameProcessor {
	fn new(recursive: bool, clean_style_font: bool, common: CommonOpts) -> Self {
		Self { recursive, clean_style_font, common }
	}

	fn process(&self, paths: Option<Vec<PathBuf>>) {
		self.print_command_info();

		if let Some(paths) = paths {
			for path_argument in paths {
				let path = Path::new(&path_argument);

				if path.exists() {
					self.process_path(path);
				} else {
					println!("File not found: {:?}", path_argument);
				}
			}
		} else {
			println!("No files provided.");
		}
	}

	fn print_command_info(&self) {
		let dry_run = if self.common.dry_run { "Dry-run mode enabled." } else { "" };
		let recursive_msg = if self.recursive { "Recursive mode enabled." } else { "" };
		let clean_style_font_msg = if self.clean_style_font { "Clean style font enabled." } else { "" };
		let verbose = if self.common.verbose { "Verbose mode enabled." } else { "" };

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
	}

	fn process_path(&self, path: &Path) {
		println_line_path_info(path, path, self.common);

		let valor_recursivo = match read_dir_recursive(path, self.recursive) {
			Ok(valor_recursivo) => valor_recursivo,
			Err(e) => {
				println!("Error: {}", e);
				return;
			}
		};

		for file in valor_recursivo {
			match generate_new_name_with_timestamp(&file) {
				Some(new_path) => {
					if !self.common.dry_run {
						if file.exists() {
							fs::rename(&file, &new_path).ok();
						} else {
							println!("File not found: {:?}", file);
							continue;
						}
					}
					println_line_path_info(&file, &new_path, self.common);
				}
				None => {
					continue;
				}
			}
		}
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
