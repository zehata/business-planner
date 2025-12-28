use std::collections::HashMap;

use petgraph::prelude::DiGraphMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::item::{EdgeItem, Item};

mod production_line;
mod recipe;
mod error;

pub use {production_line::ProductionLine, recipe::Recipe, error::GraphsError};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Graphs {
    pub(crate) production_lines: HashMap<Uuid, ProductionLine>,
    pub(crate) recipes: HashMap<Uuid, Recipe>,
}

impl Graphs {
    pub fn create<T: Graph>(&mut self) -> Uuid {
        let uuid = Uuid::new_v4();
        T::get_graphs_mut(self).insert(uuid, T::new());
        uuid
    }

    pub fn get<T: Graph>(&self, id: &Uuid) -> Option<&T> {
        T::get_graphs(self).get(id)
    }

    pub fn get_mut<T: Graph>(&mut self, id: &Uuid) -> Option<&mut T> {
        T::get_graphs_mut(self).get_mut(id)
    }

    pub fn delete<T: Graph>(&mut self, id: &Uuid) -> Result<(), GraphsError> {
        T::get_graphs_mut(self).remove(id).ok_or(GraphsError::NoGraphWithId)?;
        Ok(())
    }
}

pub trait Graph: Sized + Default {
    type Node: Item;
    type Edge: EdgeItem;

    fn new() -> Self {
        Self::default()
    }

    fn get_graphs(graphs: &Graphs) -> &HashMap<Uuid, Self>;

    fn get_graphs_mut(graphs: &mut Graphs) -> &mut HashMap<Uuid, Self>;

    fn get_graph_map(&self) -> &DiGraphMap<Uuid, Uuid>;

    fn get_graph_map_mut(&mut self) -> &mut DiGraphMap<Uuid, Uuid>;

    fn add_node(&mut self, node_id: &Uuid) {
        self.get_graph_map_mut().add_node(*node_id);
    }

    fn remove_node(&mut self, node_id: &Uuid) -> Result<(), GraphsError> {
        let node_exists = self.get_graph_map_mut().remove_node(*node_id);
        if !node_exists{
            return Err(GraphsError::NoNodeWithId)
        }
        Ok(())
    }

    fn add_edge(&mut self, edge_id: &Uuid, from_node_id: &Uuid, to_node_id: &Uuid) -> Result<(), GraphsError> {
        self.get_graph_map_mut().add_edge(*from_node_id, *to_node_id, *edge_id);
        Ok(())
    }
    
    fn remove_edge(&mut self, from_node_id: &Uuid, to_node_id: &Uuid) -> Result<(), GraphsError> {
        self.get_graph_map_mut().remove_edge(*from_node_id, *to_node_id).ok_or(GraphsError::NoEdgeWithId)?;
        Ok(())
    }
}