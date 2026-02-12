use crate::{item::Item, session::Session};
use uuid::Uuid;

impl Session {
    pub fn create<T: Item>(&mut self, item: T) -> Uuid {
        self.persistent_data.registry.create::<T>(item)
    }

    pub fn contains<T: Item>(&self, id: &Uuid) -> bool {
        self.persistent_data.registry.contains::<T>(id)
    }

    pub fn read<T: Item>(&self, id: &Uuid) -> Option<&T> {
        self.persistent_data.registry.read::<T>(id)
    }

    pub fn get_by_name<'a, T: Item + 'a>(&'a self, name: &str) -> Vec<&'a Uuid> {
        self.persistent_data.registry.get_by_name::<T>(name)
    }

    pub fn update<T: Item>(&mut self, id: &Uuid, item: T) {
        self.persistent_data.registry.update::<T>(id, item)
    }

    pub fn delete<T: Item>(&mut self, id: &Uuid) -> Option<T> {
        self.persistent_data.delete::<T>(id)
    }

    pub fn list<T: Item>(&self) -> impl Iterator<Item = (&Uuid, &str)> {
        self.persistent_data.registry.list::<T>()
    }
}
