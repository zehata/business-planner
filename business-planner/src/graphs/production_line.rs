use std::collections::HashMap;

use petgraph::prelude::DiGraphMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    graphs::{Graph, GraphData, Graphs},
    item::{StoreItem, TransferItem},
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductionLine {
    data: GraphData,
    graph: DiGraphMap<Uuid, Uuid>,
}

impl Graph for ProductionLine {
    type Node = StoreItem;
    type Edge = TransferItem;

    fn get_name(&self) -> &str {
        &self.data.name
    }

    fn get_data(&self) -> &GraphData {
        &self.data
    }

    fn set_data(&mut self, data: GraphData) {
        self.data = data
    }

    fn get_graphs(graphs: &Graphs) -> &HashMap<Uuid, Self> {
        &graphs.production_lines
    }

    fn get_graphs_mut(graphs: &mut Graphs) -> &mut HashMap<Uuid, Self> {
        &mut graphs.production_lines
    }

    fn get_graph_map(&self) -> &DiGraphMap<Uuid, Uuid> {
        &self.graph
    }

    fn get_graph_map_mut(&mut self) -> &mut DiGraphMap<Uuid, Uuid> {
        &mut self.graph
    }
}
