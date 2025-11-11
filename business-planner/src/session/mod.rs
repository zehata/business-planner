use serde::{Deserialize, Serialize};
use std::{fs, path::{self, Path, PathBuf}};

use crate::errors::session::{LoadSessionError, SaveSessionError};

#[derive(Default)]
pub struct Session {
    pub last_save_location: Option<PathBuf>,
    pub data: SessionData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionData {
    schema_version: i32,
}

impl Default for SessionData {
    fn default() -> Self {
        SessionData {
            schema_version: 1,
        }
    }
}

impl Session {
    pub fn save_to_last_save_location(&self, overwrite: bool) -> Result<(), SaveSessionError> {
        match &self.last_save_location {
            Some(path) => self.save_to_location(path, overwrite),
            None => Err(SaveSessionError::UndefinedSavePath),
        }
    }

    pub fn save_to_location(&self, path: &Path, overwrite: bool) -> Result<(), SaveSessionError> {
        let serialized: String = serde_xml_rs::to_string(&self.data)?;
        if path.exists() && !overwrite {
            return Err(SaveSessionError::FileExists)
        }

        fs::write(path, serialized)?;
        
        Ok(())
    }
}

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