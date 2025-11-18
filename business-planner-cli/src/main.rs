use std::path::PathBuf;

use business_planner::{api::session::{LoadSessionError, Session, load_session}, registry::{AppRegistry, RegistryItem, structs::Material}};
use strum::IntoEnumIterator;
use clap::{Parser, Subcommand};

pub mod shells;
pub mod subcommands;
pub mod error;

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

pub fn init_shell(session: &Session, is_interactive: &bool) {
    let mut user_requested_exit = false;

    while !user_requested_exit {
        let result = match is_interactive {
            true => shells::interactive::prompt_user(
                || {    
                    let commands = subcommands::top_level::Command::iter();
                    let commands = commands.clone().map(|command| { format!("{}", command) }).collect();
                    
                    Ok(commands) 
                },
                subcommands::top_level::parse_interactive_subcommand,
                session,
                &mut user_requested_exit,
            ),
            _ => shells::non_interactive::prompt_user(
                subcommands::top_level::parse_non_interactive_subcommand,
                session,
                &mut user_requested_exit,
            ),
        };

        if let Err(error) = result {
            println!("{:#?}", error);
        }
    }
}

pub fn main () {
    let args = Cli::parse();

    let session = match args.command {
        Commands::Create => {
            println!("Creating a new planner file");
            Session::new()
        }
        Commands::Load { path } => {
            println!("Loading {}", path.display());
            match load_session(path) {
                Ok(session) => session,
                Err(LoadSessionError::ReadFileError(error)) => panic!("{:#?}", error),
                Err(LoadSessionError::XmlDeserializationError(error)) => panic!("{:#?}", error),
            }
        }
    };

    init_shell(&session, &args.interactive);
}