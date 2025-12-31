use std::{collections::HashMap, fmt::{self, Display}};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data::StoreData, graphs::{Graph, Graphs, ProductionLine}, item::{DataSource, Item, ItemAssociatedTypes, ItemObject, NodeItem, Registry, items::{ItemInternals, NodeItemInternals}}, resolver::StoreResolver};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct StoreItem {
    name: Option<String>,
    material: Option<Uuid>,
    timestamps: Option<DataSource>,
    // stock_levels: Option<DataSource>,
}

impl StoreItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    pub fn get_material(&self) -> Option<&Uuid> {
        self.material.as_ref()
    }

    pub fn set_material(&mut self, id: Option<&Uuid>) {
        self.material = id.cloned();
    }

    pub fn get_timestamps_range(&self) -> Option<&DataSource> {
        self.timestamps.as_ref()
    }

    pub fn get_timestamps_range_mut(&mut self) -> Option<&mut DataSource> {
        self.timestamps.as_mut()
    }

    pub fn set_timestamps_range(&mut self, data_source: Option<DataSource>) {
        self.timestamps = data_source;
    }
}

impl ItemInternals for StoreItem {
    
}

impl Item for StoreItem {
    fn list_names(registry: &Registry) -> Vec<(&Uuid, Option<&str>)> {
        registry.stores.iter().map(|(uuid, store)| {
            (uuid, store.get_name())
        }).collect()
    }
}

impl NodeItem for StoreItem {

}

impl ItemAssociatedTypes for StoreItem {
    type Resolver = StoreResolver;
    type Data = StoreData;
}

impl ItemObject for StoreItem {
    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Self> {
        &registry.stores
    }
    
    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Self> {
        &mut registry.stores
    }    
}

impl NodeItemInternals for StoreItem {
    fn remove_from_graphs(id: &Uuid, graphs: &mut Graphs) {
        let production_lines = ProductionLine::get_graphs_mut(graphs);
        production_lines.iter_mut().for_each(|(_, production_line)| {
            let _ = production_line.remove_node(id);
        });
    }
}

impl Display for StoreItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: &str = self.get_name().unwrap_or("");
        let timestamps_range = match self.get_timestamps_range() {
            Some(data_source) => {
                &format!("{}", data_source)
            },
            None => "",
        };
        write!(f, "Name: {}\nTimestamps:\n{}", name, timestamps_range)
    }
}