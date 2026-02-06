use uuid::Uuid;

mod ingredient;
mod material;
mod store;

use crate::{
    graphs::Graphs,
    item::{ItemObject, Registry},
    session::PersistentData,
};

pub use {ingredient::IngredientItem, material::MaterialItem, store::StoreItem};

#[allow(private_bounds)]
pub trait Item: ItemInternals {
    fn create(item: Self, registry: &mut Registry) -> Uuid {
        let uuid = Uuid::new_v4();
        Self::get_item_registry_mut(registry).insert(uuid, item);
        uuid
    }

    fn contains(id: &Uuid, registry: &Registry) -> bool {
        Self::get_item_registry(registry).contains_key(id)
    }

    fn read<'a>(id: &Uuid, registry: &'a Registry) -> Option<&'a Self> {
        Self::get_item_registry(registry).get(id)
    }

    fn update(id: &Uuid, registry: &mut Registry, item: Self) {
        Self::get_item_registry_mut(registry).insert(*id, item);
    }

    fn delete(id: &Uuid, persistent_data: &mut PersistentData) -> Option<Self> {
        Self::get_item_registry_mut(persistent_data.get_registry_mut()).remove(id)
    }

    fn list(registry: &Registry) -> impl Iterator<Item = (&Uuid, Option<&str>)>;
}

#[allow(private_bounds)]
pub trait NodeItem: Item + NodeItemInternals {
    fn delete(id: &Uuid, persistent_data: &mut PersistentData) -> Option<Self> {
        Self::remove_from_graphs(id, persistent_data.get_graphs());
        Self::get_item_registry_mut(persistent_data.get_registry_mut()).remove(id)
    }
}

pub(crate) trait ItemInternals: ItemObject {}

pub(crate) trait NodeItemInternals: ItemObject {
    fn remove_from_graphs(id: &Uuid, graphs: &mut Graphs);
}
