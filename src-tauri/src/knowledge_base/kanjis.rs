use std::sync::Arc;

use super::entity::*;
use serde::{Deserialize, Serialize};
use tauri::{Wry, async_runtime::RwLock};
use tauri_plugin_store::Store;

#[derive()]
pub struct KanjisState(Arc<RwLock<Kanjis>>);
impl EntitiesState for KanjisState {
    type Entities = Kanjis;

    fn new(kanjis: Kanjis) -> Self {
        Self(Arc::new(RwLock::new(kanjis)))
    }

    async fn get_entities(&self) -> Vec<Kanji> {
        let kanjis = self.0.read().await;
        kanjis.get_entities()
    }

    async fn add_entity(&self, payload: KanjiCreatePayload) -> Result<Kanji, String> {
        let mut kanjis = self.0.write().await;
        kanjis.add_entity(payload)
    }

    async fn has_entity(&self, identifier: &String) -> bool {
        let kanjis = self.0.read().await;
        kanjis.has_entity(identifier)
    }
}

pub struct Kanjis {
    store: Arc<Store<Wry>>,
    kanjis: Vec<Kanji>,
}
impl Entities for Kanjis {
    type Entity = Kanji;
    type EntityCreatePayload = KanjiCreatePayload;

    fn new(store: Arc<Store<Wry>>) -> Self {
        Self {
            store,
            kanjis: Vec::new(),
        }
    }
    fn get_store(&self) -> &Arc<Store<Wry>> {
        &self.store
    }
    fn borrow_entities(&self) -> &Vec<Self::Entity> {
        &self.kanjis
    }
    fn borrow_entities_mut(&mut self) -> &mut Vec<Self::Entity> {
        &mut self.kanjis
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, specta::Type)]
pub struct Kanji {
    pub id: String,
    pub kanji: String,
    pub readings: Vec<String>,
    pub tags: Vec<String>,
    #[serde(default)]
    pub writing_mnemonic: Option<String>,
    #[serde(default)]
    pub reading_mnemonic: Option<String>,
}
impl Entity for Kanji {
    fn identifier(&self) -> String {
        self.kanji.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, specta::Type)]
pub struct KanjiCreatePayload {
    pub kanji: String,
    pub meaning: String,
    pub readings: Vec<String>,
}
impl EntityCreatePayload<Kanji> for KanjiCreatePayload {
    fn to_entity(&self) -> Kanji {
        Kanji {
            id: uuid::Uuid::new_v4().to_string(),
            kanji: self.kanji.clone(),
            readings: self.readings.clone(),
            tags: vec![],
            writing_mnemonic: None,
            reading_mnemonic: None,
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_kanjis(state: tauri::State<'_, KanjisState>) -> Result<Vec<Kanji>, String> {
    Ok(state.get_entities().await)
}

#[tauri::command]
#[specta::specta]
pub async fn add_kanji(
    state: tauri::State<'_, KanjisState>,
    payload: KanjiCreatePayload,
) -> Result<Kanji, String> {
    state.add_entity(payload).await
}

#[tauri::command]
#[specta::specta]
pub async fn has_kanji(
    state: tauri::State<'_, KanjisState>,
    kanji: String,
) -> Result<bool, String> {
    Ok(state.has_entity(&kanji).await)
}
