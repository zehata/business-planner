use std::path::Path;
use business_planner::session::Session;
use inquire::Text;

use crate::errors::Error;

pub fn save(session: &Session) -> Result<(), Error> {
    let ans = Text::new("Save to: ").prompt()?;
    let path = Path::new(&ans);
    session.save_to_location(path)?;
    Ok(())
}