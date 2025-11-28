use std::{fs, path::{self, Path, PathBuf}};

use crate::{registry::{Registry, RegistryItem, RegistryItemType}, session::error::{LoadSessionError, SaveSessionError}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod error;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SessionData {
    pub schema_version: i32,
    pub registry: Registry,
}

impl Default for SessionData {
    fn default() -> Self {
        SessionData {
            schema_version: 1,
            registry: Registry::default(),
        }
    }
}

#[derive(Default)]
pub struct Session {
    pub last_save_location: Option<PathBuf>,
    pub(crate) data: SessionData,
}

impl Session {
    pub fn create(&mut self, item_type: RegistryItemType) {
        self.data.registry.create(item_type);
    }

    pub fn read(&mut self, item_type: RegistryItemType, id: Uuid) -> Option<RegistryItem<'_>> {
        self.data.registry.read(item_type, id)
    }

    pub fn delete(&mut self, item_type: RegistryItemType, id: Uuid) {
        self.data.registry.delete(item_type, id);
    }
}

pub fn create_session() -> Session {
    Session::default()
}

pub fn load_session(path: &PathBuf) -> Result<Session, LoadSessionError> {
    let serialized_session_data = fs::read_to_string(path)?;
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