use std::path::PathBuf;
use clap::{Parser, Subcommand};
pub mod interactive;
pub mod shell;

/// Interactive CLI for business-planner
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "business-planner")]
#[command(about = "Interactive CLI for business-planner", long_about = None)]
struct Cli {
    #[arg(
        long,
        require_equals = true,
        value_name = "BOOL",
        num_args = 0..=1,
        default_value_t = false,
    )]
    interactive: bool,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Creates a new planner file
    #[command()]
    Create,
    /// Loads an existing planner file
    #[command(arg_required_else_help = true)]
    Load {
        /// Path to planner file
        #[arg(required = true)]
        path: PathBuf,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Create => {
            println!("Creating a new planner file");
        }
        Commands::Load { path } => {
            println!("Loading {}", path.display());
        }
    }

    match args.interactive {
        true => interactive::interactive(),
        _ => shell::shell(),
    }
}
