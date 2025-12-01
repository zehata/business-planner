use business_planner::api::session::Session;
use inquire::Select;
use crate::{Error, NonError};

pub async fn shell<T>(
    options: Vec<String>,
    parser: T,
    session: &mut Session,
) -> Result<NonError, Error>
where
T: AsyncFnOnce(&str, &mut Session) -> Result<NonError, Error>
{
    let selected_option = Select::new("Select", options).prompt()?;
    parser(&selected_option, session).await
}