use crate::commands;
use clap::{arg, Args, ColorChoice, Parser, ValueEnum};
use commands::subcommands::Commands;
use core::fmt;

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
