use crate::{
    error::Error,
    graphs::{Graph, GraphData, GraphObjectMissing, Graphs, GraphsError},
    item::{EdgeItem},
    session::Session,
};
use uuid::Uuid;

impl Session {
    pub fn create_graph<T: Graph>(&mut self) -> Uuid {
        self.persistent_data.graphs.create::<T>()
    }

    pub fn read_graph<'a, T: Graph + 'a>(&'a self, id: &Uuid) -> Option<&'a GraphData> {
        self.persistent_data.graphs.read::<T>(id)
    }

    pub fn update_graph<T: Graph>(&mut self, id: &Uuid, data: GraphData) {
        self.persistent_data.graphs.update::<T>(id, data);
    }

    pub fn delete_graph<T: Graph>(&mut self, id: &Uuid) -> Option<T> {
        self.persistent_data.graphs.delete::<T>(id)
    }

    pub fn list_graphs<'a, T: Graph + 'a>(&'a self) -> impl Iterator<Item = (&'a Uuid, &'a T)> {
        self.persistent_data.graphs.list::<T>()
    }

    pub fn add_node<G: Graph>(
        &mut self,
        node_id: &Uuid,
        graph_id: &Uuid,
    ) -> Result<(), Error> {
        if !self.contains::<G::Node>(node_id) {
            return Err(GraphsError::GraphObjectMissing(GraphObjectMissing::Edge))?;
        }
        let graph = Graphs::get_mut::<G>(&mut self.persistent_data.graphs, graph_id)
            .ok_or(GraphsError::GraphObjectMissing(GraphObjectMissing::Graph))?;
        graph.get_graph_map_mut().add_node(*node_id);
        Ok(())
    }

    pub fn remove_node<G: Graph>(
        &mut self,
        node_id: &Uuid,
        graph_id: &Uuid,
    ) -> Result<(), Error> {
        let graph = Graphs::get_mut::<G>(&mut self.persistent_data.graphs, graph_id)
            .ok_or(GraphsError::GraphObjectMissing(GraphObjectMissing::Graph))?;
        Ok(graph.remove_node(node_id)?)
    }

    pub fn add_edge<G: Graph>(
        &mut self,
        from_node_id: &Uuid,
        to_node_id: &Uuid,
        graph_id: &Uuid,
        item: G::Edge,
    ) -> Result<(), Error> {
        let edge_id = G::Edge::create(item, &mut self.persistent_data.registry);
        let graph = Graphs::get_mut::<G>(&mut self.persistent_data.graphs, graph_id)
            .ok_or(GraphsError::GraphObjectMissing(GraphObjectMissing::Graph))?;
        Ok(graph.add_edge(&edge_id, from_node_id, to_node_id)?)
    }

    pub fn remove_edge<G: Graph>(
        &mut self,
        from_node_id: &Uuid,
        to_node_id: &Uuid,
        graph_id: &Uuid,
    ) -> Result<(), Error> {
        let graph = Graphs::get_mut::<G>(&mut self.persistent_data.graphs, graph_id)
            .ok_or(GraphsError::GraphObjectMissing(GraphObjectMissing::Graph))?;
        let edge_id = graph
            .get_graph_map_mut()
            .remove_edge(*from_node_id, *to_node_id)
            .ok_or(GraphsError::GraphObjectMissing(GraphObjectMissing::Graph))?;
        G::Edge::delete(&edge_id, &mut self.persistent_data.registry)
            .ok_or(GraphsError::GraphObjectMissing(GraphObjectMissing::Graph))?;
        Ok(graph.remove_edge(from_node_id, to_node_id)?)
    }
}
