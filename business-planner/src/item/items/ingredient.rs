use std::{collections::HashMap, fmt::{self, Display}};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data::IngredientData, graphs::{Graph, Graphs, Recipe}, item::{Item, ItemAssociatedTypes, ItemObject, NodeItem, Registry, items::{ItemInternals, NodeItemInternals}}, resolver::IngredientResolver, session::PersistentData};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IngredientItem {
    name: Option<String>,
    material: Option<Uuid>,
}

impl IngredientItem {
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

    pub fn set_material(&mut self, id: &Uuid) {
        self.material = Some(*id);
    }
}

impl Item for IngredientItem {
    fn delete(id: &Uuid, persistent_data: &mut PersistentData) -> Option<Self> {
        <Self as NodeItem>::delete(id, persistent_data)
    }

    fn list_names(registry: &Registry) -> Vec<(&Uuid, Option<&str>)> {
        registry.ingredients.iter().map(|(uuid, ingredient)| {
            (uuid, ingredient.get_name())
        }).collect()
    }
}

impl ItemInternals for IngredientItem {}

impl NodeItem for IngredientItem {}

impl ItemAssociatedTypes for IngredientItem {
    type Resolver = IngredientResolver;
    type Data = IngredientData;
}

impl ItemObject for IngredientItem {
    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Self> {
        &registry.ingredients
    }
    
    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Self> {
        &mut registry.ingredients
    }
}

impl NodeItemInternals for IngredientItem {
    fn remove_from_graphs(id: &Uuid, graphs: &mut Graphs) {
        let recipes = Recipe::get_graphs_mut(graphs);
        recipes.iter_mut().for_each(|(_, recipe)| {
            let _ = recipe.remove_node(id);
        });
    }
}

impl Display for IngredientItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}