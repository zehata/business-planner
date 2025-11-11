use std::{fs, path::{self, PathBuf}};

use crate::{errors::session::LoadSessionError, structs::Session};

pub fn create() -> Session {
    Session::default()
}

pub fn load(path: PathBuf) -> Result<Session, LoadSessionError> {
    let serialized_session_data = fs::read_to_string(&path)?;
    let session_data = serde_xml_rs::from_str(&serialized_session_data)?;
    let path = path::absolute(path).ok();
    Ok(Session {
        last_save_location: path,
        data: session_data,
    })
}