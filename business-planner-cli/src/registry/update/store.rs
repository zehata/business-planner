use std::{path::PathBuf, str::FromStr};

use business_planner::api::{registry::{DataSource, ExcelDataSource, PostgresqlDataSource, Store}, session::Session};
use clap::{Arg, ArgMatches, Command};
use inquire::Text;
use uuid::Uuid;

use crate::{Error, NonError};

pub fn get_update_store_subcommand() -> Command {
    Command::new("store")
        .no_binary_name(true)
        .arg(
            Arg::new("by_id")
                .long("by_id")
                .required(true)
        )
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

pub fn get_store_by_uuid<'a>(session: &'a mut Session, uuid: &Uuid) -> Option<&'a mut Store> {
    session.read::<Store>(uuid)
}

pub fn select_store<'a>(session: &'a mut Session, arg_matches: &ArgMatches) -> Option<&'a mut Store> {
    if let Some(uuid) = arg_matches.get_one::<String>("by_id") && let Ok(uuid) = Uuid::parse_str(uuid) {
        return get_store_by_uuid(session, &uuid)
    }
    None
}

pub fn get_update_store_interactive_subcommand() -> Vec<String> {
    ["by_id", "by_name"].into_iter().map(|by_field| {
        by_field.to_string()
    }).collect()
}

pub async fn parse_update_store_interactive_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    let Some(store) = session.read::<Store>(&Uuid::parse_str(command)?) else {
        return Err(Error::InvalidInput)
    };

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

    Ok(NonError::Continue)
}

pub async fn parse_update_store_non_interactive_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    let Some(store) = select_store(session, arg_matches) else {
        return Err(Error::InvalidInput)
    };

    if let Some(name) = arg_matches.get_one::<String>("name") {
        store.set_name(name);
    };

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

    Ok(NonError::Continue)
}

#[cfg(test)]
mod test {
    use business_planner::api::session::create_session;

    use crate::shells::non_interactive::{self, parse_buffer};

    use super::*;

    fn get_command_matches(test_buffer: &str) -> Result<ArgMatches, Error> {
        non_interactive::get_command_matches(test_buffer, get_update_store_subcommand())
    }

    #[test]
    fn test_missing_range_data() {
        let buffer = "--by_id TEST_ID --timestamps_source excel";
        get_command_matches(buffer).expect_err("No timestamps file name and range data provided");
    }

    #[test]
    fn test_missing_file_name() {
        let buffer = "--by_id TEST_ID --timestamps_source excel --timestamps_file samples/excel.xlsx";
        get_command_matches(buffer).expect_err("No timestamps range provided");
    }

    #[test]
    fn test_excel_source() {
        let buffer = "--by_id TEST_ID --timestamps_source excel --timestamps_file samples/excel.xlsx --timestamps_range Sheet1!A1";
        let arg_matches = get_command_matches(buffer).expect("Getting command matches should succeed");

        let by_id = arg_matches.get_one::<String>("by_id").unwrap();
        assert_eq!(by_id, "TEST_ID");

        let timestamps_source = arg_matches.get_one::<String>("timestamps_source").unwrap();
        assert_eq!(timestamps_source, "excel");

        let timestamps_file = arg_matches.get_one::<String>("timestamps_file").unwrap();
        assert_eq!(timestamps_file, "samples/excel.xlsx");

        let timestamps_range = arg_matches.get_one::<String>("timestamps_range").unwrap();
        assert_eq!(timestamps_range, "Sheet1!A1");
    }

    #[test]
    fn test_conflicting_datasource() {
        let buffer = "--by_id TEST_ID --timestamps_source excel --timestamps_query \"SELECT *\"";
        get_command_matches(buffer).expect_err("Conflicting data source");
    }

    #[test]
    fn test_conflicting_arg() {
        let buffer = "--by_id TEST_ID --timestamps_source psql --timestamps_file samples/excel.xlsx --timestamps_query \"SELECT *\"";
        get_command_matches(buffer).expect_err("Conflicting arguments");
    }

    async fn parse(test_buffer: &str, session: &mut Session) -> Result<NonError, Error> {
        parse_buffer(
            test_buffer,
            get_update_store_subcommand(),
            parse_update_store_non_interactive_subcommand,
            session
        ).await
    }

    #[tokio::test]
    async fn test_update_store() {
        let mut session = create_session();
        let uuid = session.create::<Store>();
        
        let buffer = format!("--by_id {} --name \"test name\"", uuid);
        let result = parse(&buffer, &mut session).await;
        result.expect("update command should run successfully");
        let material = session.read::<Store>(&uuid);
        assert_eq!(material.expect("store should exist").get_name().expect("name should be set"), "test name")
    }
}