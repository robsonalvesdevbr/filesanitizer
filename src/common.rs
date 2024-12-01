use clap::{Args, Parser};
use crate::commands::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Args)]
pub struct CommonOpts {
    /// Enable verbose output
    #[arg(short = 'v', long = "verbose", default_value_t = false, display_order = 1000)]
    pub verbose: bool,

    /// Enable dry-run mode
    #[arg(short = 'd', long = "dry-run", default_value_t = false, display_order = 1001)]
    pub dry_run: bool,
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
