use std::path::PathBuf;

use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Rename { pattern, recursive, paths }) => {

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

            if let Some(paths) = paths {
                for file in paths {
                    println!("File to rename: {:?}", file);
                }
            } else {
                println!("No files provided.");
            }

            // dry-run is a global option
            if cli.dry_run {
                println!("Dry-run mode enabled.");
            }
        }
        Some(Commands::Test { list }) => {
            if list {
                println!("Listing test values...");
            }
        },
        None => {}
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn dry-run on
    #[arg(short, long, value_name = "DRY_RUN", global = true, default_value_t = false)]
    dry_run: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// rename files
    Rename {
        /// The pattern to use for renaming
        #[arg(short, long, value_name = "PATTERN")]
        pattern: Option<String>,

        /// Enable recursive mode
        #[arg(short, long, value_name = "RECURSIVE")]
        recursive: bool,

        /// The files to rename
        paths: Option<Vec<PathBuf>>,
    },
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}


// use std::path::PathBuf;

// use clap::{Parser, Subcommand};

// #[derive(Parser)]
// #[command(version, about, long_about = None)]
// struct Cli {
//     /// Optional name to operate on
//     name: Option<String>,

//     /// Sets a custom config file
//     #[arg(short, long, value_name = "FILE")]
//     config: Option<PathBuf>,

//     /// Turn debugging information on
//     #[arg(short, long, action = clap::ArgAction::Count)]
//     debug: u8,

//     #[command(subcommand)]
//     command: Option<Commands>,
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// does testing things
//     Test {
//         /// lists test values
//         #[arg(short, long)]
//         list: bool,
//     },
// }

// fn main() {
//     let cli = Cli::parse();

//     // You can check the value provided by positional arguments, or option arguments
//     if let Some(name) = cli.name.as_deref() {
//         println!("Value for name: {name}");
//     }

//     if let Some(config_path) = cli.config.as_deref() {
//         println!("Value for config: {}", config_path.display());
//     }

//     // You can see how many times a particular flag or argument occurred
//     // Note, only flags can have multiple occurrences
//     match cli.debug {
//         0 => println!("Debug mode is off"),
//         1 => println!("Debug mode is kind of on"),
//         2 => println!("Debug mode is on"),
//         _ => println!("Don't be crazy"),
//     }

//     // You can check for the existence of subcommands, and if found use their
//     // matches just as you would the top level cmd
//     match &cli.command {
//         Some(Commands::Test { list }) => {
//             if *list {
//                 println!("Printing testing lists...");
//             } else {
//                 println!("Not printing testing lists...");
//             }
//         }
//         None => {}
//     }

//     // Continued program logic goes here...
// }