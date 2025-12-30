use uuid::Uuid;
use crate::{error::Error, graphs::{Graph, Graphs, GraphsError}, item::{EdgeItem, NodeItem}, session::Session};

impl Session {
    pub fn create_graph<T: Graph>(&mut self) -> Uuid {
        self.persistent_data.graphs.create::<T>()
    }

    pub fn delete_graph<T: Graph>(&mut self, id: &Uuid) -> Result<(), GraphsError> {
        self.persistent_data.graphs.delete::<T>(id)
    }

    pub fn add_node<I: NodeItem, G: Graph>(&mut self, node_id: &Uuid, graph_id: &Uuid) -> Result<(), Error> {
        if !self.contains::<I>(node_id) {
            return Err(GraphsError::NoEdgeWithId)?
        }
        let graph = Graphs::get_mut::<G>(&mut self.persistent_data.graphs, graph_id).ok_or(GraphsError::NoGraphWithId)?;
        graph.get_graph_map_mut().add_node(*node_id);
        Ok(())
    }

    pub fn remove_node<I: NodeItem, G: Graph>(&mut self, node_id: &Uuid, graph_id: &Uuid) -> Result<(), Error> {
        let graph = Graphs::get_mut::<G>(&mut self.persistent_data.graphs, graph_id).ok_or(GraphsError::NoGraphWithId)?;
        Ok(graph.remove_node(node_id)?)
    }

    pub fn add_edge<E: EdgeItem, G: Graph>(&mut self, from_node_id: &Uuid, to_node_id: &Uuid, graph_id: &Uuid, item: E) -> Result<(), Error> {
        let edge_id = E::create(item, &mut self.persistent_data.registry);
        let graph = Graphs::get_mut::<G>(&mut self.persistent_data.graphs, graph_id).ok_or(GraphsError::NoGraphWithId)?;
        Ok(graph.add_edge(&edge_id, from_node_id, to_node_id)?)
    }

    pub fn remove_edge<E: EdgeItem, G: Graph>(&mut self, from_node_id: &Uuid, to_node_id: &Uuid, graph_id: &Uuid) -> Result<(), Error> {
        let graph = Graphs::get_mut::<G>(&mut self.persistent_data.graphs, graph_id).ok_or(GraphsError::NoGraphWithId)?;
        let edge_id = graph.get_graph_map_mut().remove_edge(*from_node_id, *to_node_id).ok_or(GraphsError::NoEdgeWithId)?;
        E::delete(&edge_id, &mut self.persistent_data.registry).ok_or(GraphsError::NoEdgeWithId)?;
        Ok(graph.remove_edge(from_node_id, to_node_id)?)
    }
}