use std::collections::hash_map::Entry;

use uuid::Uuid;

use crate::{data::StoreData, io::{error::ReadError, excel::{create_vec_from_cells, read_once}}, item::{DataSource, StoreItem}, resolver::{ItemResolver, Resolver}};

#[derive(Debug, Default)]
pub struct StoreResolver {
    data: StoreData,
}

impl ItemResolver for StoreResolver {
    type ItemObject = StoreItem;
    type Data = StoreData;

    fn resolve<'a>(resolver: &'a mut Resolver, id: &Uuid, registry_item: &Self::ItemObject) -> Result<&'a Self::Data, ReadError> {
        let resolver: Result<&mut StoreResolver, ReadError> = match resolver.stores.entry(*id) {
            Entry::Occupied(entry) => {
                Ok(entry.into_mut())
            },
            Entry::Vacant(entry) => {
                let timestamps = match &registry_item.get_timestamps_range() {
                    Some(DataSource::Excel(excel_data_source)) => {
                        let path = excel_data_source.get_file_path();
                        let sheet = excel_data_source.get_sheet();
                        let range = excel_data_source.get_range();

                        match (path, sheet, range) {
                            (Some(path), Some(sheet), Some(range)) => {
                                let range = read_once(
                                    path,
                                    sheet,
                                    range,
                                )?;
                                
                                create_vec_from_cells(range.cells())?
                            },
                            _ => {
                                vec![]
                            }
                        }
                    },
                    Some(_) => todo!(),
                    None => vec![],
                };
                let data = StoreData::new(timestamps);
                Ok(entry.insert(StoreResolver{
                    data
                }))
            },
        };
        Ok(&resolver?.data)
    }
}