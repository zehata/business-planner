use std::{path::PathBuf, str::FromStr};

use business_planner::api::{registry::{DataSource, ExcelDataSource, PostgresqlDataSource, Store}, session::Session};
use clap::{Arg, ArgMatches, Command};
use inquire::Text;

use crate::{Error, NonError};

pub fn get_create_store_subcommand() -> Command {
    Command::new("store")
        .no_binary_name(true)
        .arg(
            Arg::new("name")
                .long("name")
        )
        .arg(
            Arg::new("timestamps_source")
                .long("timestamps_source")
        )
        .arg(
            Arg::new("timestamps_file")
                .long("timestamps_file")
                .required_if_eq_any([
                    ("timestamps_source", "excel"),
                    ("timestamps_source", "csv"),
                ])
                .requires("timestamps_range")
                .conflicts_with("timestamps_query")
        )
        .arg(
            Arg::new("timestamps_range")
                .long("timestamps_range")
                .required_if_eq_any([
                    ("timestamps_source", "excel"),
                    ("timestamps_source", "csv"),
                ])
                .requires("timestamps_file")
                .conflicts_with("timestamps_query")
        )
        .arg(    
            Arg::new("timestamps_query")
                .long("timestamps_query")
                .required_if_eq("timestamps_source", "psql")
                .conflicts_with_all(["timestamps_file", "timestamps_range"])
        )
}

pub async fn create_store_interactive_subcommand(session: &mut Session) -> Result<NonError, Error> {
    let mut store = Store::new();

    let unchanged_name_hint = match store.get_name() {
        Some(name) => &format!("({})", name),
        None => "",
    };
    let name = Text::new("name")
        .with_help_message(&format!("Store name. Leave empty to keep unchanged {}", unchanged_name_hint))
        .prompt_skippable()?;
    if let Some(name) = name {
        store.set_name(&name);
    }
    
    session.create(store);
    Ok(NonError::Continue)
}

pub async fn parse_create_store_non_interactive_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    let mut store = Store::new();
    
    if let Some(name) = arg_matches.get_one::<String>("name") {
        store.set_name(name);
    }

    if let Some(timestamp_data_source) = arg_matches.get_one::<String>("timestamps_source") {
        match &timestamp_data_source[..] {
            "excel" => {
                if 
                    let Some(timestamp_data_file) = arg_matches.get_one::<String>("timestamps_file") &&
                    let Some(timestamp_data_range) = arg_matches.get_one::<String>("timestamps_file")
                {
                    let file_path = PathBuf::from_str(timestamp_data_file).unwrap();
                    
                    let excel_data_source = ExcelDataSource::new(file_path, timestamp_data_range);
                    store.set_timestamps_range(DataSource::Excel(excel_data_source));
                }
            },
            "csv" => {
                unimplemented!();
                // if 
                //     let Some(timestamp_data_file) = arg_matches.get_one::<String>("timestamps_range") &&
                //     let Some(timestamp_data_query) = arg_matches.get_one::<String>("timestamps_range")
                // {
                    
                // }
            },
            "psql" => {
                if let Some(timestamp_data_query) = arg_matches.get_one::<String>("timestamps_query") {
                    let postgres_data_source = PostgresqlDataSource::new(timestamp_data_query);
                    store.set_timestamps_range(DataSource::Postgres(postgres_data_source));
                }
            },
            _ => return Err(Error::InvalidInput)
        }
    }

    session.create(store);
    Ok(NonError::Continue)
}