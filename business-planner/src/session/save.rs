use std::{fs::File, path::{self, Path, PathBuf}};

use crate::{resolver::Resolver, session::error::{LoadSessionError, SaveSessionError}};
use ciborium::{cbor, from_reader, into_writer};

use super::Session;

impl Session {
    pub fn create_session() -> Self {
        Self::default()
    }

    pub fn load_session(path: &PathBuf) -> Result<Self, LoadSessionError> {
        let file = File::open(path)?;
        let persistent_data = from_reader(file)?; 
        let path = path::absolute(path).ok();
        Ok(Session {
            last_save_location: path,
            persistent_data,
            resolver: Resolver::default(),
        })
    }

    pub fn save_to_last_save_location(&self, overwrite: bool) -> Result<(), SaveSessionError> {
        match &self.last_save_location {
            Some(path) => self.save_to_location(path, overwrite),
            None => Err(SaveSessionError::UndefinedSavePath),
        }
    }

    pub fn save_to_location(&self, path: &Path, overwrite: bool) -> Result<(), SaveSessionError> {
        if path.exists() && !overwrite {
            return Err(SaveSessionError::FileExists)
        }

        let cbor_value = cbor!(self.persistent_data)?;
        let file = File::create(path)?;
        into_writer(&cbor_value, file)?;
        
        Ok(())
    }
}