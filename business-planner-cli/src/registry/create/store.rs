use std::{path::PathBuf, str::FromStr};

use business_planner::api::{item::{DataSource, ExcelDataSource, PostgresqlDataSource, StoreItem}, session::Session};
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
    let mut store = StoreItem::new();

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
    let mut store = StoreItem::new();
    
    if let Some(name) = arg_matches.get_one::<String>("name") {
        store.set_name(name);
    }

    if let Some(timestamp_data_source) = arg_matches.get_one::<String>("timestamps_source") {
        match &timestamp_data_source[..] {
            "excel" => {
                let timestamp_data_file = arg_matches.get_one::<String>("timestamps_file");
                let file_path = timestamp_data_file.map(|file_path| PathBuf::from_str(file_path).unwrap());
                
                let timestamp_data_sheet = arg_matches.get_one::<String>("timestamps_sheet");
                let sheet = timestamp_data_sheet.map(String::as_str);

                let timestamp_data_range = arg_matches.get_one::<String>("timestamps_range");
                let range = timestamp_data_range.map(String::as_str);

                let excel_data_source = ExcelDataSource::new(file_path.as_ref(), sheet, range);
                store.set_timestamps_range(Some(DataSource::Excel(excel_data_source)));
            },
            "csv" => {
                todo!();
                // if 
                //     let Some(timestamp_data_file) = arg_matches.get_one::<String>("timestamps_range") &&
                //     let Some(timestamp_data_query) = arg_matches.get_one::<String>("timestamps_range")
                // {
                    
                // }
            },
            "psql" => {
                if let Some(timestamp_data_query) = arg_matches.get_one::<String>("timestamps_query") {
                    let postgres_data_source = PostgresqlDataSource::new(timestamp_data_query);
                    store.set_timestamps_range(Some(DataSource::Postgres(postgres_data_source)));
                }
            },
            _ => return Err(Error::InvalidInput)
        }
    }

    session.create(store);
    Ok(NonError::Continue)
}