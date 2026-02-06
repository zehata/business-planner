#[derive(Debug)]
pub enum GraphsError {
    GraphObjectMissing(GraphObjectMissing),
}

impl From<GraphObjectMissing> for GraphsError {
    fn from(value: GraphObjectMissing) -> Self {
        GraphsError::GraphObjectMissing(value)
    }
}

#[derive(Debug)]
pub enum GraphObjectMissing {
    Graph,
    Node,
    Edge,
}
