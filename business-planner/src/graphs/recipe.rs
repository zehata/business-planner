use std::collections::HashMap;

use petgraph::prelude::DiGraphMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{graphs::{Graph, Graphs}, item::{IngredientItem, RequirementItem}};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Recipe {
    graph: DiGraphMap<Uuid, Uuid>,
}

impl Graph for Recipe {
    type Node = IngredientItem;
    type Edge = RequirementItem;

    fn get_graphs(graphs: &Graphs) -> &HashMap<Uuid, Self> {
        &graphs.recipes
    }

    fn get_graphs_mut(graphs: &mut Graphs) -> &mut HashMap<Uuid, Self> {
        &mut graphs.recipes
    }

    fn get_graph_map(&self) -> &DiGraphMap<Uuid, Uuid> {
        &self.graph
    }

    fn get_graph_map_mut(&mut self) -> &mut DiGraphMap<Uuid, Uuid> {
        &mut self.graph
    }
}