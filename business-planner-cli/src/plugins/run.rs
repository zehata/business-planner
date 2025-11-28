use business_planner::api::{plugins::{PluginProcess, PluginResponse, get_plugins, run_plugin}, registry::RegistryItemType, session::Session};
use inquire::Select;

use crate::{Error, NonError, registry::{get_registry_item_types, retrying_prompt_uuid}};

pub fn get_run_plugins_interactive_subcommand () -> Result<Vec<String>, Error> {
    Ok(get_plugins()?.keys().cloned().collect())
}

fn retrying_send_response (plugin_process: &mut PluginProcess, item_type: String, session: &mut Session) -> Result<(), Error> {
    let mut registry_item_valid = false;
    while !registry_item_valid {
        let id = retrying_prompt_uuid()?;
        match &item_type[..] {
            "material" => {
                if let Some(material) = session.read(RegistryItemType::Material, id) {
                    plugin_process.send_response(material);
                    registry_item_valid = true;
                };
            },
            "store" => {
                if let Some(store) = session.read(RegistryItemType::Store, id) {
                    plugin_process.send_response(store);
                    registry_item_valid = true;
                };
            },
            _ => return Err(Error::InvalidInput),
        };
    }
    Ok(())
}

pub async fn parse_interactive_run_plugins_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    let mut plugin_process = run_plugin(command)?;

    while let Some(response) = plugin_process.responses.try_next() {
        match response {
            Ok(PluginResponse::DataRequest(message)) => {
                println!("{message}");
                let item_type = Select::new("Item type", get_registry_item_types()).prompt()?;
                let _ = retrying_send_response(&mut plugin_process, item_type, session);
            },
            Ok(PluginResponse::Message(message )) => {
                println!("{message}")
            },
            Ok(PluginResponse::Report(report )) => {
                println!("{report}")
            },
            Ok(PluginResponse::ProcessEnded) => {
                println!()
            },
            Err(error) => return Err(Error::BusinessPlannerError(error))
        }
    };

    Ok(NonError::Continue)
}