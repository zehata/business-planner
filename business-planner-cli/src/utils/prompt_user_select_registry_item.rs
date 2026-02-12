use business_planner::api::{item::Item, session::Session};
use inquire::{InquireError, Select};
use uuid::Uuid;

pub async fn prompt_user_select_registry_item<'a, T>(
    session: &'a Session,
    message: &str,
) -> Result<&'a Uuid, InquireError>
where
    T: 'a + Item,
{
    let materials = session.list::<T>().collect::<Vec<_>>();
    let material_names = materials
        .iter()
        .map(|(uuid, _)| uuid.to_string())
        .collect::<Vec<_>>();
    let selection = Select::new(message, material_names).raw_prompt()?;
    let (id, _) = materials.get(selection.index).unwrap();
    Ok(id)
}
