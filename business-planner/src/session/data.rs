use uuid::Uuid;

use crate::{
    error::Error,
    io::error::{IoError, ReadError},
    item::NodeItem,
    session::Session,
};

impl Session {
    pub fn resolve<I: NodeItem>(&mut self, id: &Uuid) -> Result<&I::Data, Error> {
        let item = self
            .persistent_data
            .registry
            .read::<I>(id)
            .ok_or(IoError::ReadError(ReadError::NoDataSource))?;

        match self.resolver.resolve(id, item) {
            Ok(data) => Ok(data),
            Err(error) => Err(IoError::ReadError(error))?,
        }
    }
}
