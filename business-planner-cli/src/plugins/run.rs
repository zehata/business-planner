use business_planner::api::{plugins::{DataRequest, PluginResponse, run_plugin}, item::{MaterialItem, StoreItem}, session::Session};
use inquire::Select;

use crate::{Error, NonError, registry::{get_registry_item_types, read::prompt_user_select_registry_item}};

pub async fn parse_interactive_run_plugins_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    let mut plugin_process = run_plugin(command)?;

    while let Some(response) = plugin_process.responses.try_next() {
        match response {
            Ok(response) => {
                match response {
                    PluginResponse::AnyDataRequest(mut data_request) => {
                        let item_type = Select::new("Item type", get_registry_item_types()).prompt()?;
                        match &item_type[..] {
                            "material" => {
                                let id = *prompt_user_select_registry_item::<MaterialItem>(session, "Material id").await?;
                                let data = session.resolve::<MaterialItem>(&id)?;
                                data_request.send_response(data.clone())?;
                            },
                            "store" => {
                                let id = *prompt_user_select_registry_item::<StoreItem>(session, "Store id").await?;
                                let data = session.resolve::<StoreItem>(&id)?;
                                data_request.send_response(data.clone())?;
                            },
                            _ => return Err(Error::InvalidInput),
                        };
                    },
                    PluginResponse::MaterialDataRequest(mut data_request) => {
                        let id = *prompt_user_select_registry_item::<MaterialItem>(session, "Material id").await?;
                                let data = session.resolve::<MaterialItem>(&id)?;
                        data_request.send_response(data.clone())?;
                    },
                    PluginResponse::Message(message) => {
                        println!("{message}");
                    },
                    PluginResponse::Report(message) => {
                        println!("{message}");
                    },
                    PluginResponse::ProcessEnded => {
                        return Ok(NonError::Continue)
                    },
                }
            },
            Err(error) => return Err(Error::BusinessPlannerError(error))
        }
    };

    plugin_process.await_exit()?;
    Ok(NonError::Continue)
}