use std::collections::hash_map::Entry;

use uuid::Uuid;

use crate::{data::RequirementData, io::error::ReadError, item::RequirementItem, resolver::ItemResolver};

#[derive(Debug, Default)]
pub struct RequirementResolver {
    data: RequirementData,
}

impl ItemResolver for RequirementResolver {
    type ItemObject = RequirementItem;
    type Data = RequirementData;

    fn resolve<'a>(resolver: &'a mut super::Resolver, id: &Uuid, _registry_item: &Self::ItemObject) -> Result<&'a Self::Data, ReadError> {
        let resolver: Result<&mut RequirementResolver, ReadError> = match resolver.requirements.entry(*id) {
            Entry::Occupied(entry) => {
                Ok(entry.into_mut())
            },
            Entry::Vacant(entry) => {
                let data = RequirementData::new();
                Ok(entry.insert(RequirementResolver{
                    data
                }))
            },
        };
        Ok(&resolver?.data)
    }
}