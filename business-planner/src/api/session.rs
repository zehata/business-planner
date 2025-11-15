use std::path::{Path, PathBuf};

use crate::session;

pub use session::error::{SaveSessionError, LoadSessionError};

pub use session::Session;

pub fn load_session (path: PathBuf) -> Result<Session, LoadSessionError> {
    session::load(path)
}

pub fn save_to_location (session: &session::Session, path: &Path, overwrite: bool) -> Result<(), SaveSessionError> {
    session::save_to_location(session, path, overwrite)
}

pub fn save_to_last_save_location (session: &Session, overwrite: bool) -> Result<(), SaveSessionError> {
    session::save_to_last_save_location(session, overwrite)
}