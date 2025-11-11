use business_planner::session::Session;
use inquire::Select;
use crate::{errors::Error};

pub fn prompt_user<T>(
    commands: Vec<&str>,
    parser: T,
    session: &Session,
    user_requested_exit: &mut bool
) -> Result<(), Error>
where T: FnOnce(&str, &Session, &mut bool) -> Result<(), Error>
{
    let ans = Select::new("Select", commands).prompt()?;

    parser(ans, session, user_requested_exit)
}