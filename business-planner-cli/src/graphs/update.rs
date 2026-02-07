use business_planner::api::{graphs::{Graph, ProductionLine, Recipe}, item::EdgeItem, session::Session};
use clap::Command;
use enum_map::Enum;
use inquire::Text;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use uuid::Uuid;

use crate::{Error, NonError, graphs::GraphsType, utils::{Menu, PlannerResult, prompt_select_command, prompt_user_select_graph, prompt_user_select_registry_item}};

#[derive(Debug, Display, Enum, EnumString, EnumIter)]
pub enum UpdateGraphMenu {
    Data,
    Nodes,
    Edges,
}

impl Menu for UpdateGraphMenu {
    fn get_command() -> clap::Command {
        Command::new("update")
    }
    
    async fn interactive(session: &mut Session) -> PlannerResult {
        let commands = GraphsType::iter();
        let selected_command = prompt_select_command(commands)?;

        match selected_command {
            GraphsType::ProductionLine => {
                let graph_id = prompt_user_select_graph::<ProductionLine>(session, "Select graph").await?.to_owned();
                update_graph_interactive::<ProductionLine>(session, &graph_id).await
            },
            GraphsType::Recipe => {
                let graph_id = prompt_user_select_graph::<Recipe>(session, "Select graph").await?.to_owned();
                update_graph_interactive::<Recipe>(session, &graph_id).await
            }
        }
    }
}

async fn update_graph_interactive<T: Graph>(session: &mut Session, graph_id: &Uuid) -> PlannerResult {
    let commands = UpdateGraphMenu::iter();
    let selected_command = prompt_select_command(commands)?;
    match selected_command {
        UpdateGraphMenu::Data => update_graph_data_interactive::<T>(session, graph_id),
        UpdateGraphMenu::Nodes => update_graph_nodes_interactive::<T>(session, graph_id).await,
        UpdateGraphMenu::Edges => update_graph_edges_interactive::<T>(session, graph_id).await,
    }
}

pub fn update_graph_data_interactive<T: Graph>(session: &mut Session, graph_id: &Uuid) -> PlannerResult {
    let mut graph_data = session.read_graph::<T>(graph_id).ok_or(Error::InvalidInput)?.to_owned();
    let unchanged_name_hint = graph_data.get_name();
    let name = Text::new("name")
        .with_help_message(&format!(
            "Store name. Leave empty to keep unchanged {}",
            unchanged_name_hint
        ))
        .prompt_skippable()?;

    if let Some(name) = name {
        graph_data.set_name(&name);
    };
    session.update_graph::<T>(graph_id, graph_data);
    Ok(NonError::Continue)
}

#[derive(Display, EnumIter)]
enum UpdateGraphAddNodeEdgeMenu {
    Add,
    Remove,
}

async fn update_graph_nodes_interactive<T: Graph>(session: &mut Session, graph_id: &Uuid) -> PlannerResult {
    let commands = UpdateGraphAddNodeEdgeMenu::iter();
    let selected_command = prompt_select_command(commands)?;

    let node_id = prompt_user_select_registry_item::<T::Node>(session, "Select").await?.to_owned();

    match selected_command {
        UpdateGraphAddNodeEdgeMenu::Add => session.add_node::<T>(&node_id, graph_id)?,
        UpdateGraphAddNodeEdgeMenu::Remove => session.remove_node::<T>(&node_id, graph_id)?,
    };
    Ok(NonError::Continue)
}

async fn update_graph_edges_interactive<T: Graph>(session: &mut Session, graph_id: &Uuid) -> PlannerResult {
    let commands = UpdateGraphAddNodeEdgeMenu::iter();
    let selected_command = prompt_select_command(commands)?;

    let from_node_id = prompt_user_select_registry_item::<T::Node>(session, "Select").await?.to_owned();
    let to_node_id = prompt_user_select_registry_item::<T::Node>(session, "Select").await?.to_owned();

    let edge = T::Edge::new();

    match selected_command {
        UpdateGraphAddNodeEdgeMenu::Add => session.add_edge::<T>(&from_node_id, &to_node_id, graph_id, edge)?,
        UpdateGraphAddNodeEdgeMenu::Remove => session.remove_edge::<T>(&from_node_id, &to_node_id, graph_id)?,
    };
    Ok(NonError::Continue)
}