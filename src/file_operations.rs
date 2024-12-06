use std::{
	fs,
	path::{Path, PathBuf},
};

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

pub fn validate_path_exists(path: &Path) -> bool {
	if !path.exists() {
		println!("File not found: {:?}", path);
		false
	} else {
		true
	}
}
