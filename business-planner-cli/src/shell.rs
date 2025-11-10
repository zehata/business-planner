use std::{io::{Write, stdin, stdout}};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "")]
#[command(about, no_binary_name(true))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Saves the current planner file
    #[command()]
    Save,
    /// Saves the current planner file
    #[command()]
    Exit,
    
}

pub fn shell() {
    let mut user_requested_exit = false;
    while !user_requested_exit {
        print!("> ");
        stdout().flush().expect("Failed to print to stdout");
        
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let parse_result = Cli::try_parse_from(buffer.split_whitespace());

        match parse_result {
            Ok(args) => {
                match args.command {
                    Commands::Save => {},
                    Commands::Exit => {
                        user_requested_exit = true;
                    },
                }
            },
            Err(error) => {println!("{error}");}
        };
    }
}