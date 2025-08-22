use lindera::dictionary::{DictionaryKind, load_embedded_dictionary};
use lindera::mode::Mode;
use lindera::segmenter::Segmenter;
use lindera::token::Token;
use lindera::tokenizer::Tokenizer;

use crate::translation::{MyEntry, MyEntryDisplay};

pub struct Word {
    word: String,
    tauri_app_handle: tauri::AppHandle,
}

impl Word {
    pub fn new(word: String, tauri_app_handle: &tauri::AppHandle) -> Self {
        Word {
            word,
            tauri_app_handle: tauri_app_handle.clone(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.word
    }

    fn katakana_to_hiragana(input: &str) -> String {
        input
            .chars()
            .map(|c| {
                if ('ァ'..='ン').contains(&c) {
                    // Shift Unicode value from Katakana to Hiragana block
                    std::char::from_u32(c as u32 - 0x60).unwrap()
                } else {
                    c
                }
            })
            .collect()
    }

    fn get_lemma(&self) -> Result<String, String> {
        let tokens = self.tokenize()?;
        if let Some(token) = tokens.first() {
            if let Some(lemma) = token.lemma() {
                return Ok(lemma.to_string());
            }
        }
        Err("No lemma found".to_string())
    }

    fn tokenize(&self) -> Result<Vec<MyToken>, String> {
        let text = &self.word;
        let dictionary = load_embedded_dictionary(DictionaryKind::IPADIC)
            .map_err(|_| format!("Could not load dictionary"))?;
        let segmenter = Segmenter::new(Mode::Normal, dictionary, None);
        let tokenizer = Tokenizer::new(segmenter);

        let mut tokens = tokenizer
            .tokenize(text)
            .map_err(|_| format!("Could not tokenize work {}", self.word))?;

        let mut token_vec = Vec::new();
        for token in tokens.iter_mut() {
            token_vec.push(MyToken::from_lindera_token(token));
        }

        Ok(token_vec)
    }

    pub fn to_hiragana(&self) -> Result<String, String> {
        let tokens = self.tokenize()?;

        let mut hiragana_output = String::new();
        for token in tokens.iter() {
            hiragana_output.push_str(token.reading().unwrap_or(token.text()));
        }

        Ok(hiragana_output)
    }

    pub async fn find_in_dictionary(&self) -> Result<Option<MyEntry>, String> {
        println!("Searching for word in dictionary: {}", self.word);
        let lemma = self.get_lemma()?;
        println!("Searching for lemma: {}", lemma);
        let lemma_dict_entry =
            crate::translation::translate_word(&lemma, &self.tauri_app_handle).await;
        if let Some(entry) = lemma_dict_entry {
            println!("Found entry: {:?}", entry.entry_display());
            return Ok(Some(entry));
        } else {
            println!("No entry found for lemma: {}", lemma);
            let word_dict_entry =
                crate::translation::translate_word(&self.word, &self.tauri_app_handle).await;
            if let Some(entry) = word_dict_entry {
                println!("Found entry for word: {:?}", entry.entry_display());
                return Ok(Some(entry));
            } else {
                println!("No entry found for word: {}", self.word);
                return Ok(None);
            }
        }
    }

    fn unique_entries(entries: Vec<MyEntry>) -> Vec<MyEntry> {
        let mut seen = std::collections::HashSet::new();
        entries
            .into_iter()
            .filter(|entry| seen.insert(entry.word_string().clone()))
            .collect()
    }

    pub async fn find_all_candidates(
        &self,
        app_handle: &tauri::AppHandle,
    ) -> Result<Vec<MyEntry>, String> {
        let mut candidates = Vec::new();

        let lemma = self.get_lemma()?;
        let lemma_dict_entries = crate::translation::translate_word_all(&lemma, app_handle).await;
        candidates.extend(lemma_dict_entries);

        let word_dict_entries =
            crate::translation::translate_word_all(&self.word, app_handle).await;
        candidates.extend(word_dict_entries);

        Ok(Self::unique_entries(candidates))
    }
}

pub struct MyToken {
    details: Vec<String>,
}
impl MyToken {
    pub fn from_lindera_token(token: &mut Token) -> Self {
        MyToken {
            details: token.details().iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn text(&self) -> &str {
        &self.details[0] // Assuming the first detail is the surface form
    }

    pub fn reading(&self) -> Option<&str> {
        if self.details.len() > 7 {
            Some(&self.details[7]) // Assuming the reading is at index 7
        } else {
            None
        }
    }
    pub fn lemma(&self) -> Option<&str> {
        if self.details.len() > 6 {
            Some(&self.details[6]) // Assuming the lemma is at index 6
        } else {
            None
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_word_dict_entry(
    word: String,
    app_handle: tauri::AppHandle,
) -> Result<Option<MyEntryDisplay>, String> {
    let word_instance = Word::new(word, &app_handle);
    match word_instance.find_in_dictionary().await {
        Ok(Some(entry)) => Ok(Some(entry.entry_display())),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_word_candidates(
    word: String,
    app_handle: tauri::AppHandle,
) -> Result<Vec<MyEntryDisplay>, String> {
    let word_instance = Word::new(word, &app_handle);
    match word_instance.find_all_candidates(&app_handle).await {
        Ok(entries) => Ok(entries
            .into_iter()
            .map(|entry| entry.entry_display())
            .collect()),
        Err(e) => Err(e),
    }
}
