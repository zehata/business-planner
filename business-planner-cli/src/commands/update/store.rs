use std::{env, path::PathBuf, str::FromStr};

use business_planner::api::{
    item::{DataSource, ExcelDataSource, PostgresqlDataSource, StoreItem},
    session::Session,
};
use clap::{Arg, ArgMatches, Command};
use inquire::{InquireError, Select, Text};
use uuid::Uuid;

use crate::{
    Error, NonError,
    utils::{Menu, NoSubcommands, PathFilter, PlannerResult, prompt_select_path},
};

pub struct UpdateStoreMenu {}

impl Menu for UpdateStoreMenu {
    type SubcommandsEnum = NoSubcommands;

    fn get_command() -> Command {
        get_update_store_subcommand()
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        update_store_interactive(session).await
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        update_store_non_interactive(arg_matches, session).await
    }
}

pub fn get_update_store_subcommand() -> Command {
    Command::new("store")
        .arg(Arg::new("by_id").long("by_id").required(true))
        .arg(Arg::new("name").long("name"))
        .arg(Arg::new("timestamps_source").long("timestamps_source"))
        .arg(
            Arg::new("timestamps_file")
                .long("timestamps_file")
                .required_if_eq_any([("timestamps_source", "excel"), ("timestamps_source", "csv")])
                .requires("timestamps_sheet")
                .requires("timestamps_range")
                .conflicts_with("timestamps_query"),
        )
        .arg(
            Arg::new("timestamps_sheet")
                .long("timestamps_sheet")
                .required_if_eq_any([("timestamps_source", "excel"), ("timestamps_source", "csv")])
                .requires("timestamps_file")
                .requires("timestamps_range")
                .conflicts_with("timestamps_query"),
        )
        .arg(
            Arg::new("timestamps_range")
                .long("timestamps_range")
                .required_if_eq_any([("timestamps_source", "excel"), ("timestamps_source", "csv")])
                .requires("timestamps_file")
                .requires("timestamps_sheet")
                .conflicts_with("timestamps_query"),
        )
        .arg(
            Arg::new("timestamps_query")
                .long("timestamps_query")
                .required_if_eq("timestamps_source", "psql")
                .conflicts_with_all(["timestamps_file", "timestamps_sheet", "timestamps_range"]),
        )
}

pub fn get_store_by_uuid<'a>(session: &'a mut Session, uuid: &Uuid) -> Option<&'a StoreItem> {
    session.read::<StoreItem>(uuid)
}

pub async fn prompt_populate_excel_data_source(
    excel_data_source: &mut ExcelDataSource,
) -> Result<(), Error> {
    let mut user_ok = false;
    while !user_ok {
        match Select::new("", vec!["File path", "Sheet", "Range", "Ok"]).prompt()? {
            "File path" => {
                let current_dir = env::current_dir()?;
                match prompt_select_path(&current_dir, PathFilter::File(Some("xlsx"))).await {
                    Ok(path) => excel_data_source.set_file_path(Some(&path)),
                    Err(Error::InquireError(InquireError::OperationCanceled)) => continue,
                    Err(error) => return Err(error),
                }
            }
            "Sheet" => {
                excel_data_source.set_sheet(Text::new("Sheet").prompt_skippable()?.as_deref());
            }
            "Range" => {
                excel_data_source.set_range(Text::new("Range").prompt_skippable()?.as_deref());
            }
            "Ok" => user_ok = true,
            _ => return Err(Error::InvalidInput),
        }
    }
    Ok(())
}

pub async fn update_store_interactive(session: &mut Session) -> Result<NonError, Error> {
    let (store_id, mut store) = {
        let stores = session.list::<StoreItem>().collect::<Vec<_>>();
        let store_names = stores
            .iter()
            .map(|(_, store)| store)
            .collect();
        let selected_option = Select::new("Select", store_names)
            .raw_prompt_skippable()?
            .ok_or(Error::UserCancelled)?;
        let (store_id, _) = *stores
            .get(selected_option.index)
            .expect("User cannot select if subcommands is empty");

        (
            store_id.to_owned(),
            session
                .read::<StoreItem>(store_id)
                .ok_or(Error::InvalidInput)?
                .to_owned(),
        )
    };

    let unchanged_name_hint = store.get_name();
    let name = Text::new("name")
        .with_help_message(&format!(
            "Store name. Leave empty to keep unchanged {}",
            unchanged_name_hint
        ))
        .prompt_skippable()?;
    if let Some(name) = name {
        store.set_name(&name);
    }

    match Select::new(
        "Timestamps data source",
        vec!["(Unchanged)", "Excel", "CSV", "PostgreSQL"],
    )
    .prompt_skippable()?
    {
        Some("Excel") => match store.get_timestamps_range_mut() {
            Some(DataSource::Excel(excel_data_source)) => {
                prompt_populate_excel_data_source(excel_data_source).await?;
            }
            _ => {
                let mut excel_data_source = ExcelDataSource::new(None, None, None);
                prompt_populate_excel_data_source(&mut excel_data_source).await?;
                store.set_timestamps_range(Some(DataSource::Excel(excel_data_source)));
            }
        },
        Some("CSV") => {
            todo!()
        }
        Some("PostgreSQL") => {
            todo!()
        }
        _ => return Err(Error::InvalidInput),
    }

    session.update(&store_id, store);

    Ok(NonError::Continue)
}

pub async fn update_store_non_interactive(
    arg_matches: &ArgMatches,
    session: &mut Session,
) -> Result<NonError, Error> {
    let string = arg_matches
        .get_one::<String>("by_id")
        .ok_or(Error::InvalidInput)?;
    let id = Uuid::parse_str(string)?;
    let mut store = get_store_by_uuid(session, &id)
        .ok_or(Error::InvalidInput)?
        .to_owned();

    if let Some(name) = arg_matches.get_one::<String>("store_name") {
        store.set_name(name);
    };

    if let Some(timestamp_data_source) = arg_matches.get_one::<String>("timestamps_source") {
        match &timestamp_data_source[..] {
            "excel" => {
                let data_source = store.get_timestamps_range_mut();

                let timestamp_data_file = arg_matches.get_one::<String>("timestamps_file");
                let file_path =
                    timestamp_data_file.map(|file_path| PathBuf::from_str(file_path).unwrap());

                let timestamp_data_sheet = arg_matches.get_one::<String>("timestamps_sheet");
                let sheet = timestamp_data_sheet.map(String::as_str);

                let timestamp_data_range = arg_matches.get_one::<String>("timestamps_range");
                let range = timestamp_data_range.map(String::as_str);

                match data_source {
                    Some(DataSource::Excel(excel_data_source)) => {
                        if let Some(file_path) = file_path {
                            excel_data_source.set_file_path(Some(&file_path));
                        }
                        if let Some(sheet) = sheet {
                            excel_data_source.set_sheet(Some(sheet));
                        }
                        if let Some(range) = range {
                            excel_data_source.set_range(Some(range));
                        }
                    }
                    _ => {
                        let excel_data_source =
                            ExcelDataSource::new(file_path.as_ref(), sheet, range);
                        store.set_timestamps_range(Some(DataSource::Excel(excel_data_source)));
                    }
                }
            }
            "csv" => {
                todo!();
                // if
                //     let Some(timestamp_data_file) = arg_matches.get_one::<String>("timestamps_range") &&
                //     let Some(timestamp_data_query) = arg_matches.get_one::<String>("timestamps_range")
                // {

                // }
            }
            "psql" => {
                if let Some(timestamp_data_query) =
                    arg_matches.get_one::<String>("timestamps_query")
                {
                    let postgres_data_source = PostgresqlDataSource::new(timestamp_data_query);
                    store.set_timestamps_range(Some(DataSource::Postgres(postgres_data_source)));
                }
            }
            _ => return Err(Error::InvalidInput),
        }
    }

    session.update(&id, store);

    Ok(NonError::Continue)
}

#[cfg(test)]
mod test {
    use business_planner::api::session::create_session;

    use crate::utils;

    use super::*;

    fn get_command_matches(test_buffer: &str) -> Result<ArgMatches, Error> {
        utils::get_command_matches(test_buffer, &UpdateStoreMenu::get_command())
    }

    #[test]
    fn test_missing_range_data() {
        let buffer = "--by_id TEST_ID --timestamps_source excel";
        get_command_matches(buffer).expect_err("No timestamps file name and range data provided");
    }

    #[test]
    fn test_missing_file_name() {
        let buffer =
            "--by_id TEST_ID --timestamps_source excel --timestamps_file samples/excel.xlsx";
        get_command_matches(buffer).expect_err("No timestamps range provided");
    }

    #[test]
    fn test_excel_source() {
        let buffer = "--by_id TEST_ID --timestamps_source excel --timestamps_file samples/excel.xlsx --timestamps_range Sheet1!A1";
        let arg_matches =
            get_command_matches(buffer).expect("Getting command matches should succeed");

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
        let arg_matches = get_command_matches(test_buffer)?;
        update_store_non_interactive(&arg_matches, session).await
    }

    #[tokio::test]
    async fn test_update_store() {
        let mut session = create_session();
        let uuid = session.create(StoreItem::new());

        let buffer = format!("--by_id {} --name \"test name\"", uuid);
        let result = parse(&buffer, &mut session).await;
        result.expect("update command should run successfully");
        let material = session.read::<StoreItem>(&uuid);
        assert_eq!(
            material
                .expect("store should exist")
                .get_name(),
            "test name"
        )
    }
}
