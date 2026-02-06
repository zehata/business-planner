use std::collections::hash_map::Entry;

use uuid::Uuid;

use crate::{
    data::IngredientData,
    io::error::ReadError,
    item::IngredientItem,
    resolver::{ItemResolver, Resolver},
};

#[derive(Debug, Default)]
pub struct IngredientResolver {
    data: IngredientData,
}

impl ItemResolver for IngredientResolver {
    type ItemObject = IngredientItem;
    type Data = IngredientData;

    fn resolve<'a>(
        resolver: &'a mut Resolver,
        id: &Uuid,
        _registry_item: &IngredientItem,
    ) -> Result<&'a IngredientData, ReadError> {
        let resolver: Result<&mut IngredientResolver, ReadError> =
            match resolver.ingredients.entry(*id) {
                Entry::Occupied(entry) => Ok(entry.into_mut()),
                Entry::Vacant(entry) => {
                    let data = IngredientData {};
                    let resolver = IngredientResolver { data };
                    Ok(entry.insert(resolver))
                }
            };
        Ok(&resolver?.data)
    }
}
