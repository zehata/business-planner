use std::collections::hash_map::Entry;

use uuid::Uuid;

use crate::{data::TransferData, io::error::ReadError, item::TransferItem, resolver::ItemResolver};

#[derive(Debug, Default)]
pub struct TransferResolver {
    data: TransferData,
}

impl ItemResolver for TransferResolver {
    type ItemObject = TransferItem;
    type Data = TransferData;

    fn resolve<'a>(resolver: &'a mut super::Resolver, id: &Uuid, _registry_item: &Self::ItemObject) -> Result<&'a Self::Data, ReadError> {
        let resolver: Result<&mut TransferResolver, ReadError> = match resolver.transfers.entry(*id) {
            Entry::Occupied(entry) => {
                Ok(entry.into_mut())
            },
            Entry::Vacant(entry) => {
                let data = TransferData::new();
                Ok(entry.insert(TransferResolver{
                    data
                }))
            },
        };
        Ok(&resolver?.data)
    }
}