use std::collections::hash_map::Entry;

use uuid::Uuid;

use crate::{data::MaterialData, io::error::ReadError, item::MaterialItem, resolver::{ItemResolver, Resolver}};

#[derive(Debug, Default)]
pub struct MaterialResolver {
    data: MaterialData,
}

impl ItemResolver for MaterialResolver {
    type Item = MaterialItem;
    type Data = MaterialData;

    fn resolve<'a>(resolver: &'a mut Resolver, id: &Uuid, _registry_item: &MaterialItem) -> Result<&'a MaterialData, ReadError> {
        let resolver: Result<&mut MaterialResolver, ReadError> = match resolver.materials.entry(*id) {
            Entry::Occupied(entry) => {
                Ok(entry.into_mut())
            },
            Entry::Vacant(entry) => {
                let data = MaterialData {  };
                let resolver = MaterialResolver {
                    data,
                };
                Ok(entry.insert(resolver))
            },
        };
        Ok(&resolver?.data)
    }
}