use uuid::Uuid;

mod ingredient;
mod store;
mod material;

use crate::{graphs::Graphs, item::{ItemObject, Registry}};

pub use {ingredient::IngredientItem, store::StoreItem, material::MaterialItem};

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

    fn get<'a>(id: &Uuid, registry: &'a mut Registry) -> Option<&'a mut Self> {
        Self::get_item_registry_mut(registry).get_mut(id)
    }

    fn update(id: &Uuid, registry: &mut Registry, item: Self) {
        Self::get_item_registry_mut(registry).insert(*id, item);
    }

    fn delete(id: &Uuid, registry: &mut Registry) -> Option<Self> {
        Self::get_item_registry_mut(registry).remove(id)
    }

    fn list(registry: &Registry) -> Vec<String> {
        Self::get_item_registry(registry).keys().map(|key| {
            key.to_string()
        }).collect()
    }

    fn list_names(registry: &Registry) -> Vec<(&Uuid, Option<&str>)>;
}

#[allow(private_bounds)]
pub trait NodeItem: Item + NodeItemInternals {

}

pub(crate) trait ItemInternals: ItemObject {
}

pub(crate) trait NodeItemInternals: ItemObject {
    fn remove_from_graphs(id: &Uuid, graphs: &mut Graphs);
}