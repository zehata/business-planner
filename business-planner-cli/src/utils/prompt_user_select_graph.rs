use business_planner::api::{graphs::Graph, session::Session};
use inquire::{InquireError, Select};
use uuid::Uuid;

pub async fn prompt_user_select_graph<'a, T>(
    session: &'a Session,
    message: &str,
) -> Result<&'a Uuid, InquireError>
where
    T: 'a + Graph,
{
    let graphs = session.list_graphs::<T>().collect::<Vec<_>>();
    let graph_names = graphs
        .iter()
        .map(|(uuid, _)| uuid.to_string())
        .collect::<Vec<_>>();
    let selection = Select::new(message, graph_names).raw_prompt()?;
    let (id, _) = graphs.get(selection.index).unwrap();
    Ok(id)
}
