use clap::{arg, Args, ValueEnum};
use colored::Colorize;
use core::fmt;
use regex::Regex;
use std::{
	fs,
	path::{Path, PathBuf},
};
use unicode_normalization::UnicodeNormalization;

#[derive(Args, Clone, Copy)]
pub struct CommonOpts {
	/// Enable verbose output
	#[arg(short = 'v', long = "verbose", default_value_t = false, display_order = 1000, help = "Enable verbose output [default: false]")]
	pub verbose: bool,

	/// Enable dry-run mode
	#[arg(short = 'd', long = "dry-run", default_value_t = false, display_order = 1001, help = "Enable dry-run mode [default: false]")]
	pub dry_run: bool,
}

#[derive(Args, Clone, Copy)]
pub struct ProjectOpts {
	/// This is useful for cleaning up project directories, example for .NET projects with `bin` and `obj` folders.
	#[arg(short, long, default_value_t = false, display_order = 2, help = "Delete .NET project directories [default: false]")]
	csharp: bool,

	/// This is useful for cleaning up project directories, example for Angular projects with `node_modules` and `dist` folders.
	#[arg(short, long, default_value_t = false, display_order = 3, help = "Delete Angular project directories [default: false]")]
	angular: bool,
}

/// Project type
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum ProjectType {
	#[value(name = "csharp", alias = "c#", help = "C# project")]
	CSharp,
	#[value(name = "rust", alias = "rs", help = "Rust project")]
	Rust,
	#[value(name = "angular", help = "Angular project")]
	Angular,
	#[value(name = "node", help = "Node project")]
	Node,
}

impl fmt::Display for ProjectType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ProjectType::CSharp => write!(f, "csharp"),
			ProjectType::Rust => write!(f, "rust"),
			ProjectType::Angular => write!(f, "angular"),
			ProjectType::Node => write!(f, "node"),
		}
	}
}

impl CommonOpts {
	pub fn handle_common_opts(&self) {
		if self.dry_run {
			println!("Dry-run mode enabled.");
		}
		if self.verbose {
			println!("Verbose mode enabled.");
		}
	}
}

pub fn println_line_path_info(path: &Path, new_path: &Path, common: CommonOpts) {
	if !common.verbose {
		return;
	}

	if path.is_dir() {
		return;
	}

	let dry_run = if common.dry_run { "Dry-run mode enabled." } else { "" };

	let name = path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("Invalid UTF-8")).to_string_lossy();
	let new_name = new_path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("Invalid UTF-8")).to_string_lossy();
	let name_group = format!("{} -> {:<130}", name.blue(), new_name.green()).chars().take(130).collect::<String>();

	//println!("{:<10}: {:<130} {:<10}", "Diretório", name.bold().blue(), dry_run.yellow());
	println!("{:<10}: {} {:<10}", "File", name_group, dry_run.yellow());
}

pub fn generate_new_name_with_timestamp(file: &Path) -> Option<PathBuf> {
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

pub fn normalize_path(file: &Path) -> PathBuf {
	let original_path = PathBuf::from(file);
	let normalized: String = original_path.to_string_lossy().nfkc().collect();
	PathBuf::from(normalized)
}
