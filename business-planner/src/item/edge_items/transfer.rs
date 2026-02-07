use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    data::TransferData,
    graphs::ProductionLine,
    item::{
        DataSource, ItemAssociatedTypes, ItemObject, Registry,
        edge_items::{EdgeItem, EdgeItemInternals},
    },
    resolver::TransferResolver,
};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransferItem {
    name: Option<String>,
    timestamps: Option<DataSource>,
    // stock_levels: Option<DataSource>,
}

impl TransferItem {}

impl EdgeItem for TransferItem {
    fn new() -> Self {
        Self { name: None, timestamps: None }
    }
}

impl ItemAssociatedTypes for TransferItem {
    type Resolver = TransferResolver;
    type Data = TransferData;
}

impl ItemObject for TransferItem {
    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Self> {
        &registry.transfers
    }

    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Self> {
        &mut registry.transfers
    }
}

impl EdgeItemInternals for TransferItem {
    type Graph = ProductionLine;
}

impl Display for TransferItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
