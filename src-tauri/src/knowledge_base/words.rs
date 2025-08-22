use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::{Manager, Wry, async_runtime::RwLock};
use tauri_plugin_store::{Store, StoreExt};

const WORDS_STORE_FILE: &str = "words.json";

pub fn setup(app: &mut tauri::App) {
    let store = app.store(WORDS_STORE_FILE);

    if let Ok(store) = store {
        let mut words = Words {
            store,
            words: Vec::new(),
        };
        words.load_words();
        app.manage(WordsState::new(words));
    }
}

pub struct WordsState(pub Arc<RwLock<Words>>);
impl WordsState {
    pub fn new(words: Words) -> Self {
        Self(Arc::new(RwLock::new(words)))
    }

    pub async fn get_words(&self) -> Vec<Word> {
        let words = self.0.read().await;
        words.get_words()
    }

    pub async fn add_word(&self, payload: WordCreatePayload) -> Result<Word, String> {
        let mut words = self.0.write().await;
        words.add_word(payload)
    }

    pub async fn has_word(&self, word: &String) -> bool {
        let words = self.0.read().await;
        words.has_word(word)
    }
}

pub struct Words {
    store: Arc<Store<Wry>>,
    words: Vec<Word>,
}

impl Words {
    fn load_words(&mut self) -> Option<Vec<Word>> {
        let loaded_words = self.store.get("words").and_then(|data| {
            serde_json::from_value::<Vec<Word>>(data)
                .map_err(|e| e.to_string())
                .ok()
        });

        if let Some(words) = loaded_words {
            self.words = words.clone();
            return Some(words);
        }
        None
    }

    pub fn get_words(&self) -> Vec<Word> {
        self.words.clone()
    }

    pub fn has_word(&self, word: &String) -> bool {
        self.words.iter().any(|w| &w.word == word)
    }

    pub fn add_word(&mut self, word: WordCreatePayload) -> Result<Word, String> {
        let word = word.to_word();
        if self.has_word(&word.word) {
            return Err("Word already exists".to_string());
        }
        self.words.push(word.clone());
        let words_value = serde_json::to_value(&self.words).map_err(|e| e.to_string())?;
        self.store.set("words", words_value);
        Ok(word)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, specta::Type)]
pub struct Word {
    pub id: String,
    pub word: String,
    pub meaning: String,
    pub kanji_readings: Vec<KanjiReading>,
}

#[derive(Serialize, Deserialize, Debug, Clone, specta::Type)]
pub struct KanjiReading {
    pub reading: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, specta::Type)]
pub struct WordCreatePayload {
    pub word: String,
    pub meaning: String,
    pub kanji_readings: Vec<KanjiReading>,
}
impl WordCreatePayload {
    pub fn to_word(&self) -> Word {
        Word {
            id: uuid::Uuid::new_v4().to_string(),
            word: self.word.clone(),
            meaning: self.meaning.clone(),
            kanji_readings: self.kanji_readings.clone(),
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_words(state: tauri::State<'_, WordsState>) -> Result<Vec<Word>, String> {
    Ok(state.get_words().await)
}

#[tauri::command]
#[specta::specta]
pub async fn add_word(
    state: tauri::State<'_, WordsState>,
    payload: WordCreatePayload,
) -> Result<Word, String> {
    state.add_word(payload).await
}

#[tauri::command]
#[specta::specta]
pub async fn has_word(state: tauri::State<'_, WordsState>, word: String) -> Result<bool, String> {
    Ok(state.has_word(&word).await)
}
