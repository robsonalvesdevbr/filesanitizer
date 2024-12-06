use crate::commands;
use clap::{ColorChoice, Parser};
use commands::subcommands::Commands;

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
