use uuid::Uuid;

mod requirement;
mod transfer;

use crate::{
    graphs::Graph,
    item::{ItemObject, Registry},
};

pub use {requirement::RequirementItem, transfer::TransferItem};

#[allow(private_bounds)]
pub trait EdgeItem: EdgeItemInternals {
    fn create(item: Self, registry: &mut Registry) -> Uuid {
        let uuid = Uuid::new_v4();
        Self::get_item_registry_mut(registry).insert(uuid, item);
        uuid
    }

    fn contains(id: &Uuid, registry: &Registry) -> bool {
        Self::get_item_registry(registry).contains_key(id)
    }

    fn delete(id: &Uuid, registry: &mut Registry) -> Option<Self> {
        Self::get_item_registry_mut(registry).remove(id)
    }
}

pub(crate) trait EdgeItemInternals: ItemObject {
    type Graph: Graph;
}
