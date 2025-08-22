use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::{Manager, Wry};
use tauri_plugin_store::{Store, StoreExt};

pub trait EntitiesState: Sized + Send + Sync + 'static {
    type Entities: Entities;

    fn setup(app: &mut tauri::App, store_file: &str) {
        let store = app.store(store_file);

        if let Ok(store) = store {
            let mut entities = Self::Entities::new(store);
            entities.load_entities();
            app.manage(Self::new(entities));
        }
    }
    fn new(entities: Self::Entities) -> Self;
    async fn get_entities(&self) -> Vec<<Self::Entities as Entities>::Entity>;
    async fn add_entity(
        &self,
        payload: <Self::Entities as Entities>::EntityCreatePayload,
    ) -> Result<<Self::Entities as Entities>::Entity, String>;
    async fn has_entity(&self, identifier: &String) -> bool;
}

pub trait Entities {
    type Entity: Entity + Clone;
    type EntityCreatePayload: EntityCreatePayload<Self::Entity>;

    fn new(store: Arc<Store<Wry>>) -> Self;
    fn get_store(&self) -> &Arc<Store<Wry>>;
    fn borrow_entities(&self) -> &Vec<Self::Entity>;
    fn borrow_entities_mut(&mut self) -> &mut Vec<Self::Entity>;

    fn load_entities(&mut self) -> Option<Vec<Self::Entity>> {
        let loaded_entities = self.get_store().get("entities").and_then(|data| {
            serde_json::from_value::<Vec<Self::Entity>>(data)
                .map_err(|e| e.to_string())
                .ok()
        });

        if let Some(entities) = loaded_entities {
            *self.borrow_entities_mut() = entities.clone();
            return Some(entities);
        }
        None
    }
    fn get_entities(&self) -> Vec<Self::Entity> {
        self.borrow_entities().clone()
    }
    fn store_entities(&self) -> Result<(), String> {
        let entities_json =
            serde_json::to_value(self.borrow_entities()).map_err(|e| e.to_string())?;
        self.get_store().set("entities", entities_json);
        Ok(())
    }
    fn has_entity(&self, identifier: &String) -> bool {
        self.borrow_entities()
            .iter()
            .any(|e| &e.identifier() == identifier)
    }
    fn add_entity(&mut self, entity: Self::EntityCreatePayload) -> Result<Self::Entity, String> {
        let entity = entity.to_entity();
        if self.has_entity(&entity.identifier()) {
            return Err("Entity already exists".to_string());
        }
        self.borrow_entities_mut().push(entity.clone());
        Ok(entity)
    }
}

pub trait Entity: Sized + Clone + Send + Sync + Serialize + for<'de> Deserialize<'de> {
    fn identifier(&self) -> String;
}

pub trait EntityCreatePayload<Entity>:
    Sized + Clone + Send + Sync + Serialize + for<'de> Deserialize<'de>
{
    fn to_entity(&self) -> Entity;
}
