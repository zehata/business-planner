use std::path::PathBuf;

use crate::{graphs::Graphs, item::Registry, resolver::Resolver};
use serde::{Deserialize, Serialize};

mod error;
mod registry;
mod data;
mod graphs;
mod save;

pub use error::{LoadSessionError, SaveSessionError};

#[derive(Serialize, Deserialize)]
pub struct PersistentData {
    schema_version: i32,
    registry: Registry,
    graphs: Graphs,
}

impl Default for PersistentData {
    fn default() -> Self {
        PersistentData {
            schema_version: 1,
            registry: Registry::default(),
            graphs: Graphs::default(),
        }
    }
}

#[derive(Default)]
pub struct Session {
    last_save_location: Option<PathBuf>,
    resolver: Resolver,
    persistent_data: PersistentData,
}