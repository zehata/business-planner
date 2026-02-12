use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    data::StoreData,
    graphs::{Graph, Graphs, ProductionLine},
    item::{
        DataSource, IngredientItem, Item, ItemAssociatedTypes, ItemObject, NodeItem, Registry,
        items::{ItemInternals, NodeItemInternals},
    },
    resolver::StoreResolver,
};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct StoreItem {
    name: String,
    ingredient: Option<Uuid>,
    timestamps: Option<DataSource>,
    // stock_levels: Option<DataSource>,
}

impl StoreItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_ingredient(&self) -> Option<&Uuid> {
        self.ingredient.as_ref()
    }

    pub fn set_ingredient(&mut self, id: Option<&Uuid>) {
        self.ingredient = id.cloned();
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

impl ItemInternals for StoreItem {}

impl Item for StoreItem {
    fn get_name(&self) -> &str {
        self.get_name()
    }

    fn update(id: &Uuid, registry: &mut Registry, item: Self) {
        let ingredient = item.get_ingredient().cloned();
        let previous_store = Self::get_item_registry_mut(registry).insert(*id, item);

        #[allow(clippy::collapsible_if)] // because more fields will be added in the future
        if let Some(previous_store) = previous_store {
            if let Some(previous_ingredient) = previous_store.get_ingredient()
                && let Some(ingredient) = &ingredient
                && previous_ingredient != ingredient
            {
                if let Some(previous_ingredient) =
                    IngredientItem::get_item_registry_mut(registry).get_mut(previous_ingredient)
                {
                    previous_ingredient.remove_associated_store(id);
                }

                if let Some(ingredient) =
                    IngredientItem::get_item_registry_mut(registry).get_mut(ingredient)
                {
                    ingredient.add_associated_store(id);
                }
            }
        }
    }

    fn list(registry: &Registry) -> impl Iterator<Item = (&Uuid, &str)> {
        registry
            .stores
            .iter()
            .map(|(uuid, store)| (uuid, store.get_name()))
    }
}

impl NodeItem for StoreItem {}

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
        production_lines
            .iter_mut()
            .for_each(|(_, production_line)| {
                let _ = production_line.remove_node(id);
            });
    }
}

impl Display for StoreItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: &str = self.get_name();
        let timestamps_range = match self.get_timestamps_range() {
            Some(data_source) => &format!("{}", data_source),
            None => "",
        };
        write!(f, "Name: {}\nTimestamps:\n{}", name, timestamps_range)
    }
}
