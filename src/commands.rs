use std::path::PathBuf;
use clap::Subcommand;
use crate::common::CommonOpts;

#[derive(Subcommand)]
pub enum Commands {
    /// Rename files
    Rename {
        /// Enable recursive mode
        #[arg(short, long, value_name = "RECURSIVE", display_order = 0)]
        recursive: bool,

        /// The pattern to use for renaming
        #[arg(short, long, value_name = "PATTERN", display_order = 1)]
        pattern: Option<String>,

        /// Clean style font
        #[arg(short, long, value_name = "CLEAN_STYLE_FONT", default_value_t = true, display_order = 2)]
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
        Some(Commands::Rename {
            pattern,
            recursive,
            clean_style_font,
            paths,
            common,
        }) => {
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

fn handle_rename_command(
    pattern: Option<String>,
    recursive: bool,
    clean_style_font: bool,
    paths: Option<Vec<PathBuf>>,
) {
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
