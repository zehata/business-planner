use std::path::PathBuf;

use crate::{item::{Registry}, resolver::Resolver};
use serde::{Deserialize, Serialize};

pub mod error;
mod registry;
mod data;
mod save;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct PersistentData {
    pub schema_version: i32,
    pub registry: Registry,
}

impl Default for PersistentData {
    fn default() -> Self {
        PersistentData {
            schema_version: 1,
            registry: Registry::default(),
        }
    }
}

#[derive(Default)]
pub struct Session {
    pub last_save_location: Option<PathBuf>,
    pub(crate) resolver: Resolver,
    pub(crate) persistent_data: PersistentData,
}