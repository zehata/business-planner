use std::path::{Path, PathBuf};

use crate::session::{self};

use crate::api::error::BusinessPlannerError;

pub use session::Session;

pub fn create_session () -> Session {
    Session::create_session()
}

pub fn load_session (path: &PathBuf) -> Result<Session, BusinessPlannerError> {
    Ok(Session::load_session(path)?)
}

pub fn save_to_location (session: &session::Session, path: &Path, overwrite: bool) -> Result<(), BusinessPlannerError> {
    Ok(session.save_to_location(path, overwrite)?)
}

pub fn save_to_last_save_location (session: &Session, overwrite: bool) -> Result<(), BusinessPlannerError> {
    Ok(session.save_to_last_save_location(overwrite)?)
}