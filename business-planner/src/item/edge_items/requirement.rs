use std::{collections::HashMap, fmt::{self, Display}};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data::RequirementData, graphs::ProductionLine, item::{ItemAssociatedTypes, ItemObject, Registry, edge_items::{EdgeItem, EdgeItemInternals}}, resolver::RequirementResolver};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RequirementItem {
}

impl EdgeItem for RequirementItem {
}

impl ItemAssociatedTypes for RequirementItem {
    type Resolver = RequirementResolver;
    type Data = RequirementData;
}

impl ItemObject for RequirementItem {
    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Self> {
        &registry.requirements
    }
    
    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Self> {
        &mut registry.requirements
    }
}

impl EdgeItemInternals for RequirementItem {
    type Graph = ProductionLine;
}

impl Display for RequirementItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}