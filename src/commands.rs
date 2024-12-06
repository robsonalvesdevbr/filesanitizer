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
		Some(Commands::Rename { recursive, paths, common }) => {
			let processor = RenameProcessor::new(recursive, common);
			let paths = paths.or_else(|| env::current_dir().ok().map(|path| vec![path]));
			processor.process(paths);
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

	Ok(paths)
}

fn println_line_path_info(path: &Path, new_path: &Path, common: CommonOpts) {
	if !common.verbose {
		return;
	}

	if path.is_dir() {
		return;
	}

	let dry_run = if common.dry_run { "Dry-run mode enabled." } else { "" };

	let name = path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("Invalid UTF-8")).to_string_lossy();
	let new_name = new_path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("Invalid UTF-8")).to_string_lossy();
	let name_group = format!("{} -> {:<130}", name, new_name).chars().take(130).collect::<String>();

	//println!("{:<10}: {:<130} {:<10}", "Diretório", name.bold().blue(), dry_run.yellow());
	println!("{:<10}: {} {:<10}", "File", name_group.blue(), dry_run.yellow());
}

fn generate_new_name_with_timestamp(file: &Path) -> Option<PathBuf> {
	let normalized_path = normalize_path(file);

	if normalized_path.is_dir() {
		return Some(file.to_path_buf());
	}

	let re = Regex::new(r"(^\d{8}_\d{6})").unwrap();
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

pub fn validate_path_exists(path: &Path) -> bool {
	if !path.exists() {
		println!("File not found: {:?}", path);
		false
	} else {
		true
	}
}

// Helper to normalize file paths
fn normalize_path(file: &Path) -> PathBuf {
	let original_path = PathBuf::from(file);
	let normalized: String = original_path.to_string_lossy().nfkc().collect();
	PathBuf::from(normalized)
}

struct RenameProcessor {
	recursive: bool,
	common: CommonOpts,
}

impl RenameProcessor {
	fn new(recursive: bool, common: CommonOpts) -> Self {
		Self { recursive, common }
	}

	fn process(&self, paths: Option<Vec<PathBuf>>) {
		self.print_command_info();

		if let Some(paths) = paths {
			for path_argument in paths {
				let path = Path::new(&path_argument);
				if validate_path_exists(path) {
					self.process_path(path);
				}
			}
		} else {
			println!("No files provided.");
		}
	}

	fn print_command_info(&self) {
		let dry_run = if self.common.dry_run { "Dry-run mode enabled." } else { "" };
		let recursive_msg = if self.recursive { "Recursive mode enabled." } else { "" };
		let verbose = if self.common.verbose { "Verbose mode enabled." } else { "" };

		println!("{}", "-".repeat(100).yellow());

		if !dry_run.is_empty() {
			println!("{}", dry_run.yellow());
		}
		if !recursive_msg.is_empty() {
			println!("{}", recursive_msg.yellow());
		}
		if !verbose.is_empty() {
			println!("{}", verbose.yellow());
		}

		println!("{}", "-".repeat(100).yellow());
	}

	fn process_path(&self, path: &Path) {
		//println_line_path_info(path, path, self.common);

		let valor_recursivo = match read_dir_recursive(path, self.recursive) {
			Ok(mut valor_recursivo) => {
				valor_recursivo.sort_by(|a, b| {
					let a_is_dir = a.is_dir();
					let b_is_dir = b.is_dir();

					match (a_is_dir, b_is_dir) {
						(true, false) => std::cmp::Ordering::Less,    // Diretórios antes dos arquivos
						(false, true) => std::cmp::Ordering::Greater, // Arquivos depois dos diretórios
						_ => {
							// Ordena alfabeticamente considerando normalização Unicode
							let a_normalized = a.to_string_lossy().nfkc().collect::<String>();
							let b_normalized = b.to_string_lossy().nfkc().collect::<String>();
							a_normalized.cmp(&b_normalized)
						}
					}
				});
				valor_recursivo
			}
			Err(e) => {
				println!("Error: {}", e);
				return;
			}
		};

		let mut current_dir: Option<PathBuf> = None;

		for file in valor_recursivo {
			let parent_dir = file.parent();

			if current_dir.as_deref() != parent_dir {
				if let Some(dir) = parent_dir {
					println!("{:<10}: {:<130} {:<10}", "Diretório", dir.to_string_lossy().bold().blue(), "");
					current_dir = Some(dir.to_path_buf());
				}
			}

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
		let args = ["test", "rename", "--recursive", "--verbose", "--dry-run", "list.txt"];
		let cli = Cli::parse_from(args);
		if let Some(Commands::Rename { recursive, paths, common }) = cli.command {
			assert!(recursive, "Expected recursive to be true: {}", recursive);
			assert_eq!(paths, Some(vec![PathBuf::from("list.txt")]), "Expected paths to be Some([PathBuf::from(\"list.txt\")]): {:?}", paths);
			assert!(common.verbose, "Expected verbose to be true: {}", common.verbose);
			assert!(common.dry_run, "Expected dry_run to be true: {}", common.dry_run);
		} else {
			panic!("Expected Rename command");
		}
	}

	#[test]
	fn test_handle_subcommand_rename_args_with_verbose_false() {
		let args = ["test", "rename", "--recursive", "--dry-run", "list.txt"];
		let cli = Cli::parse_from(args);
		if let Some(Commands::Rename { recursive, paths, common }) = cli.command {
			assert!(recursive, "Expected recursive to be true: {}", recursive);
			assert_eq!(paths, Some(vec![PathBuf::from("list.txt")]), "Expected paths to be Some([PathBuf::from(\"list.txt\")]): {:?}", paths);
			assert!(!common.verbose, "Expected verbose to be false: {}", common.verbose);
			assert!(common.dry_run, "Expected dry_run to be true: {}", common.dry_run);
		} else {
			panic!("Expected Rename command");
		}
	}
}
