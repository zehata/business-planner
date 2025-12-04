use business_planner::api::{plugins::{DataRequest, PluginResponse, get_plugins, run_plugin}, registry::{Material, Store}, session::Session};
use inquire::Select;

use crate::{Error, NonError, registry::{get_registry_item_types, read::prompt_user_select_registry_item}};

pub fn get_run_plugins_interactive_subcommand () -> Result<Vec<String>, Error> {
    Ok(get_plugins()?.keys().cloned().collect())
}

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
                                let material = prompt_user_select_registry_item::<Material>(session, "Material id").await?;
                                data_request.send_response(material);
                            },
                            "store" => {
                                let store = prompt_user_select_registry_item::<Store>(session, "Store id").await?;
                                data_request.send_response(store);
                            },
                            _ => return Err(Error::InvalidInput),
                        };
                    },
                    PluginResponse::MaterialDataRequest(mut data_request) => {
                        let material = prompt_user_select_registry_item::<Material>(session, "Material id").await?;
                        data_request.send_response(material);
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