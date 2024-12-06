use crate::commands::commons::CommonOpts;
use crate::file_operations::{read_dir_recursive, validate_path_exists};
use std::fs;
use std::path::{Path, PathBuf};

use super::command::Command;
use super::commons::{generate_new_name_with_timestamp, println_line_path_info};

pub struct RenameCommand {
	recursive: bool,
	paths: Option<Vec<PathBuf>>,
	common: CommonOpts,
}

impl RenameCommand {
	pub fn new(recursive: bool, paths: Option<Vec<PathBuf>>, common: CommonOpts) -> Self {
		Self { recursive, paths, common }
	}
}

impl Command for RenameCommand {
	fn execute(&self) {
		if let Some(paths) = &self.paths {
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
}

impl RenameCommand {
	fn process_path(&self, path: &Path) {
		match read_dir_recursive(path, self.recursive) {
			Ok(files) => {
				for file in files {
					if let Some(new_path) = generate_new_name_with_timestamp(&file) {
						if !self.common.dry_run {
							if let Err(e) = fs::rename(&file, &new_path) {
								eprintln!("Erro ao renomear o arquivo {}: {}", file.display(), e);
							}
						}
						println_line_path_info(&file, &new_path, self.common);
					}
				}
			}
			Err(e) => {
				eprintln!("Erro ao ler o diretório {}: {}", path.display(), e);
			}
		}
	}
}
