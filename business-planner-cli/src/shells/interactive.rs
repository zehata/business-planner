use business_planner::api::session::Session;
use inquire::Select;
use crate::error;

pub fn prompt_user<T, U>(
    commands: T,
    parser: U,
    session: &Session,
    user_requested_exit: &mut bool
) -> Result<(), error::Error>
where
T: FnOnce() -> Result<Vec<String>, error::Error>,
U: FnOnce(&str, &Session, &mut bool) -> Result<(), error::Error>
{
    let ans = Select::new("Select", commands()?).prompt()?;

    parser(&ans, session, user_requested_exit)
}