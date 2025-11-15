use std::{collections::HashMap, fs, path::{self, Path, PathBuf}};

use crate::{session::error::{LoadSessionError, SaveSessionError}, structs::{Material, Stock}};
use serde::{Deserialize, Serialize};

pub mod error;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SessionData {
    schema_version: i32,
    materials: HashMap<String, Material>,
    stocks: HashMap<String, Stock>,
}

impl Default for SessionData {
    fn default() -> Self {
        SessionData {
            schema_version: 1,
            materials: HashMap::new(),
            stocks: HashMap::new(),
        }
    }
}

#[derive(Default)]
pub struct Session {
    pub last_save_location: Option<PathBuf>,
    pub data: SessionData,
}

impl Session {
    pub fn new() -> Session {
        Session::default()
    }
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

pub fn save_to_last_save_location(session: &Session, overwrite: bool) -> Result<(), SaveSessionError> {
    match &session.last_save_location {
        Some(path) => save_to_location(session, path, overwrite),
        None => Err(SaveSessionError::UndefinedSavePath),
    }
}

pub fn save_to_location(session: &Session, path: &Path, overwrite: bool) -> Result<(), SaveSessionError> {
    let serialized: String = serde_xml_rs::to_string(&session.data)?;
    if path.exists() && !overwrite {
        return Err(SaveSessionError::FileExists)
    }

    fs::write(path, serialized)?;
    
    Ok(())
}