use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Rename {
            pattern,
            recursive,
            clean_style_font,
            paths,
            common,
        }) => {
            if let Some(pattern) = pattern {
                println!("Pattern to use: {}", pattern);
            } else {
                println!("No pattern provided.");
            }

            if recursive {
                println!("Recursive mode enabled.");
            } else {
                println!("Recursive mode disabled.");
            }

            if clean_style_font {
                println!("Remove style font enabled.");
            } else {
                println!("Remove style font disabled.");
            }

            if let Some(paths) = paths {
                for file in paths {
                    println!("File to rename: {:?}", file);
                }
            } else {
                println!("No files provided.");
            }

            if common.dry_run {
                println!("Dry-run mode enabled.");
            } else {
                println!("Dry-run mode disabled.");
            }

            if common.verbose {
                println!("Verbose mode enabled.");
            } else {
                println!("Verbose mode disabled.");
            }
        }
        Some(Commands::Test { list, common }) => {
            if list {
                println!("Listing test values...");
            }

            if common.dry_run {
                println!("Dry-run mode enabled.");
            } else {
                println!("Dry-run mode disabled.");
            }

            if common.verbose {
                println!("Verbose mode enabled.");
            } else {
                println!("Verbose mode disabled.");
            }
        }
        None => {}
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // Turn dry-run on
    //#[arg(short, long, value_name = "DRY_RUN", global = true, default_value_t = false, display_order = 1001)]
    //dry_run: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Args)]
struct CommonOpts {
    /// Enable verbose output
    #[arg(
        short = 'v',
        long = "verbose",
        default_value_t = false,
        display_order = 1000
    )]
    verbose: bool,

    /// Enable dry-run mode
    #[arg(
        short = 'd',
        long = "dry-run",
        default_value_t = false,
        display_order = 1001
    )]
    dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Rename files
    Rename {
        /// Enable recursive mode
        #[arg(short, long, value_name = "RECURSIVE", display_order = 0)]
        recursive: bool,

        /// The pattern to use for renaming
        #[arg(short, long, value_name = "PATTERN", display_order = 1)]
        pattern: Option<String>,

        /// Remove style font
        #[arg(
            short,
            long,
            value_name = "CLEAN_STYLE_FONT",
            default_value_t = true,
            display_order = 2
        )]
        clean_style_font: bool,

        /// The files to rename
        paths: Option<Vec<PathBuf>>,

        /// Common options for all subcommands
        #[command(flatten)]
        common: CommonOpts,
    },
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long, display_order = 0)]
        list: bool,

        /// Common options for all subcommands
        #[command(flatten)]
        common: CommonOpts,
    },
}