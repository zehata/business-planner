
use crate::item::Item;
use uuid::Uuid;

use super::Session;

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

    pub fn get<T: Item>(&mut self, id: &Uuid) -> Option<&mut T> {
        self.persistent_data.registry.get::<T>(id)
    }

    pub fn update<T: Item>(&mut self, id: &Uuid, item: T) {
        self.persistent_data.registry.update::<T>(id, item)
    }

    pub fn delete<T: Item>(&mut self, id: &Uuid) {
        self.persistent_data.registry.delete::<T>(id);
    }

    pub fn list<T: Item>(&self) -> Vec<String> {
        self.persistent_data.registry.list::<T>()
    }

    pub fn list_names<T: Item>(&self) -> Vec<(&Uuid, Option<&str>)> {
        self.persistent_data.registry.list_names::<T>()
    }
}