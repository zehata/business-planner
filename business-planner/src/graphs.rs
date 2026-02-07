use std::{collections::{HashMap, hash_map::Entry}, fmt::Display};

use petgraph::prelude::DiGraphMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::item::{EdgeItem, NodeItem};

mod error;
mod production_line;
mod recipe;

pub use {
    error::{GraphObjectMissing, GraphsError},
    production_line::ProductionLine,
    recipe::Recipe,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GraphData {
    name: String,
}

impl GraphData {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }
}

impl Display for GraphData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)
    }
}

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

    pub fn read<'a, T: Graph + 'a>(&'a self, id: &Uuid) -> Option<&'a GraphData> {
        Some(T::get_graphs(self).get(id)?.get_data())
    }

    pub fn update<T: Graph>(&mut self, id: &Uuid, data: GraphData) {
        let entry = match T::get_graphs_mut(self).entry(*id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(T::new()),
        };

        entry.set_data(data);
    }

    pub fn get_mut<T: Graph>(&mut self, id: &Uuid) -> Option<&mut T> {
        T::get_graphs_mut(self).get_mut(id)
    }

    pub fn delete<T: Graph>(&mut self, id: &Uuid) -> Option<T> {
        T::get_graphs_mut(self).remove(id)
    }

    pub fn list<'a, T: Graph + 'a>(&'a self) -> impl Iterator<Item = (&'a Uuid, &'a T)> {
        T::get_graphs(self).iter()
    }
}

pub trait Graph: Sized + Default {
    type Node: NodeItem;
    type Edge: EdgeItem;

    fn new() -> Self {
        Self::default()
    }

    fn get_name(&self) -> &str {
        self.get_data().get_name()
    }

    fn get_data(&self) -> &GraphData;

    fn set_data(&mut self, data: GraphData);

    fn get_graphs(graphs: &Graphs) -> &HashMap<Uuid, Self>;

    fn get_graphs_mut(graphs: &mut Graphs) -> &mut HashMap<Uuid, Self>;

    fn get_graph_map(&self) -> &DiGraphMap<Uuid, Uuid>;

    fn get_graph_map_mut(&mut self) -> &mut DiGraphMap<Uuid, Uuid>;

    fn add_node(&mut self, node_id: &Uuid) {
        self.get_graph_map_mut().add_node(*node_id);
    }

    fn remove_node(&mut self, node_id: &Uuid) -> Result<(), GraphsError> {
        let node_exists = self.get_graph_map_mut().remove_node(*node_id);
        if !node_exists {
            return Err(GraphObjectMissing::Node)?;
        }
        Ok(())
    }

    fn add_edge(
        &mut self,
        edge_id: &Uuid,
        from_node_id: &Uuid,
        to_node_id: &Uuid,
    ) -> Result<(), GraphsError> {
        self.get_graph_map_mut()
            .add_edge(*from_node_id, *to_node_id, *edge_id);
        Ok(())
    }

    fn remove_edge(&mut self, from_node_id: &Uuid, to_node_id: &Uuid) -> Result<(), GraphsError> {
        self.get_graph_map_mut()
            .remove_edge(*from_node_id, *to_node_id)
            .ok_or(GraphObjectMissing::Edge)?;
        Ok(())
    }
}
