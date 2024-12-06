
use crate::common::CommonOpts;
use clap::Subcommand;
use colored::Colorize;
use regex::Regex;
use std::{env, fs, path::{Path, PathBuf}};
use unicode_normalization::UnicodeNormalization;

#[derive(Subcommand)]
pub enum Commands {
    /// Rename files
    Rename {
        /// Enable recursive mode
        #[arg(short, long, default_value_t = false)]
        recursive: bool,

        /// Clean style font
        #[arg(short, long, default_value_t = true)]
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
        #[arg(short, long)]
        list: bool,

        /// Common options for all subcommands
        #[command(flatten)]
        common: CommonOpts,
    },
}

pub fn handle_command(command: Option<Commands>) {
    if let Some(cmd) = command {
        match cmd {
            Commands::Rename { recursive, clean_style_font, paths, common } => {
                let processor = RenameProcessor::new(recursive, clean_style_font, common);
                let paths = paths.or_else(|| {
                    env::current_dir().ok().map(|path| vec![path])
                });
                processor.process(paths);
            }
            Commands::Test { list, common } => {
                if list {
                    println!("Listing test values...");
                }
                common.handle_common_opts();
            }
        }
    } else {
        println!("Nenhum comando fornecido. Utilize --help para mais informações.");
    }
}

// Centralized validation helper
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
    clean_style_font: bool,
    common: CommonOpts,
}

impl RenameProcessor {
    fn new(recursive: bool, clean_style_font: bool, common: CommonOpts) -> Self {
        Self { recursive, clean_style_font, common }
    }

    fn process(&self, paths: Option<Vec<PathBuf>>) {
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

    fn process_path(&self, path: &Path) {
        if let Ok(files) = read_dir_recursive(path, self.recursive) {
            for file in files {
                if let Some(new_path) = generate_new_name_with_timestamp(&file) {
                    if !self.common.dry_run {
                        fs::rename(&file, &new_path).ok();
                    }
                    println_line_path_info(&file, &new_path, &self.common);
                }
            }
        }
    }
}

// Moved functions here to follow SRP (Single Responsibility Principle)
fn read_dir_recursive(dir: &Path, recursive: bool) -> Result<Vec<PathBuf>, std::io::Error> {
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

fn generate_new_name_with_timestamp(file: &Path) -> Option<PathBuf> {
    let normalized_path = normalize_path(file);
    if normalized_path.is_dir() {
        return None;
    }
    let metadata = fs::metadata(file).ok()?;
    let created = metadata.created().ok()?;
    let created: chrono::DateTime<chrono::Local> = created.into();
    let new_name_with_timestamp = format!(
        "{}{}",
        created.format("%Y%m%d_%H%M%S_"),
        normalized_path.file_name()?.to_str()?
    );
    Some(normalized_path.with_file_name(new_name_with_timestamp))
}

fn println_line_path_info(path: &Path, new_path: &Path, common: &CommonOpts) {
    if common.verbose {
        println!(
            "{} -> {}",
            path.to_string_lossy().blue(),
            new_path.to_string_lossy().green()
        );
    }
}
