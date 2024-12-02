use crate::commands::Commands;
use clap::{Args, ColorChoice, Parser};

#[derive(Parser)]
#[command(
    version = concat!("v", env!("CARGO_PKG_VERSION"), " (build ", env!("CARGO_PKG_NAME"), ")"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = env!("CARGO_PKG_DESCRIPTION"),
    color = ColorChoice::Always,
    arg_required_else_help = true
)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Option<Commands>,
}

#[derive(Args,Clone, Copy)]
pub struct CommonOpts {
	/// Enable verbose output
	#[arg(short = 'v', long = "verbose", default_value_t = false, display_order = 1000, help = "Enable verbose output [default: false]")]
	pub verbose: bool,

	/// Enable dry-run mode
	#[arg(short = 'd', long = "dry-run", default_value_t = false, display_order = 1001, help = "Enable dry-run mode [default: false]")]
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
