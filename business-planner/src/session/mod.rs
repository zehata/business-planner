use std::{fs::File, path::{self, Path, PathBuf}};

use crate::{registry::{Registry, RegistryItem}, session::error::{LoadSessionError, SaveSessionError}};
use ciborium::{cbor, from_reader, into_writer};
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
    pub fn create<T>(&mut self, item: T) -> Uuid where T: RegistryItem<RegistryItem = T> {
        self.data.registry.create::<T>(item)
    }

    pub fn read<T>(&self, id: &Uuid) -> Option<&T> where T: RegistryItem<RegistryItem = T> {
        self.data.registry.read::<T>(id)
    }

    pub fn get<T>(&mut self, id: &Uuid) -> Option<&mut T> where T: RegistryItem<RegistryItem = T> {
        self.data.registry.get::<T>(id)
    }

    pub fn update<T>(&mut self, id: &Uuid, item: T)
    where T: RegistryItem<RegistryItem = T> {
        self.data.registry.update::<T>(id, item)
    }

    pub fn delete<T>(&mut self, id: &Uuid) where T: RegistryItem<RegistryItem = T> {
        self.data.registry.delete::<T>(id);
    }

    pub fn list<T>(&self) -> Vec<String> where T: RegistryItem<RegistryItem = T> {
        self.data.registry.list::<T>()
    }

    pub fn list_names<T>(&self) -> Vec<(&Uuid, Option<&str>)> where T: RegistryItem<RegistryItem = T> {
        self.data.registry.list_names::<T>()
    }
}

pub fn create_session() -> Session {
    Session::default()
}

pub fn load_session(path: &PathBuf) -> Result<Session, LoadSessionError> {
    let file = File::open(path)?;
    let session_data = from_reader(file)?; 
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
    if path.exists() && !overwrite {
        return Err(SaveSessionError::FileExists)
    }

    let cbor_value = cbor!(session.data)?;
    let file = File::create(path)?;
    into_writer(&cbor_value, file)?;
    
    Ok(())
}