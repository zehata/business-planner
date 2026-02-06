use std::path::PathBuf;

use crate::{
    graphs::Graphs,
    item::{Item, Registry},
    resolver::Resolver,
};
use serde::{Deserialize, Serialize};

mod data;
mod error;
mod graphs;
mod registry;
mod save;

pub use error::{LoadSessionError, SaveSessionError};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PersistentData {
    schema_version: i32,
    registry: Registry,
    graphs: Graphs,
}

impl PersistentData {
    pub(crate) fn get_registry_mut(&mut self) -> &mut Registry {
        &mut self.registry
    }

    pub(crate) fn get_graphs(&mut self) -> &mut Graphs {
        &mut self.graphs
    }

    pub(crate) fn delete<T: Item>(&mut self, id: &Uuid) -> Option<T> {
        T::delete(id, self)
    }
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
