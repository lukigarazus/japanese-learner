use jmdict;
use tauri::Manager;

pub fn setup(app: &tauri::App) {
    let dictionary = MyDictionary::new();
    app.manage(dictionary);
}

pub struct MyDictionary {
    kanji_to_entry: std::collections::HashMap<String, MyEntry>,
}

impl MyDictionary {
    pub fn get(app: &tauri::AppHandle) -> tauri::State<Self> {
        app.state::<MyDictionary>()
    }

    pub fn new() -> Self {
        let mut kanji_to_entry = std::collections::HashMap::new();
        for entry in jmdict::entries() {
            let my_entry = MyEntry(entry);
            let word = my_entry.word_string();
            if let Some(word) = word {
                // println!(
                //     "Adding word: {} with reading: {}, {}",
                //     my_entry.kanji_elements_string(),
                //     my_entry.reading_elements_string(),
                //     word
                // );
                kanji_to_entry.insert(word.clone(), my_entry);
            } else {
                println!(
                    "No word found for entry: {:?}",
                    my_entry.reading_elements_string()
                );
            }
        }
        MyDictionary { kanji_to_entry }
    }

    fn find(&self, word: &String) -> Option<MyEntry> {
        println!("Finding word: {}", word);
        let entry = self.kanji_to_entry.get(word).cloned();
        if entry.is_none() {
            let kanji_entry = jmdict::entries()
                .find(|e| e.kanji_elements().any(|k| k.text == word))
                .map(MyEntry);
            if let Some(kanji_entry) = kanji_entry {
                return Some(kanji_entry);
            }
            let reading_entry = jmdict::entries()
                .find(|e| e.reading_elements().any(|r| r.text == word))
                .map(MyEntry);
            if let Some(reading_entry) = reading_entry {
                return Some(reading_entry);
            }
            let meaning_entry = jmdict::entries()
                .find(|e| e.senses().any(|s| s.glosses().any(|g| g.text == word)))
                .map(MyEntry);
            if let Some(meaning_entry) = meaning_entry {
                return Some(meaning_entry);
            }
            return None;
        } else {
            entry
        }
    }

    pub fn find_all(&self, word: &String) -> Vec<MyEntry> {
        let mut candidates = Vec::new();

        let entry = self.find(word);
        if let Some(entry) = entry {
            candidates.push(entry);
        }

        let kanji_entries: Vec<MyEntry> = jmdict::entries()
            .filter(|e| e.kanji_elements().any(|k| k.text == word))
            .map(MyEntry)
            .collect();
        candidates.extend(kanji_entries);

        let reading_entries: Vec<MyEntry> = jmdict::entries()
            .filter(|e| e.reading_elements().any(|r| r.text == word))
            .map(MyEntry)
            .collect();
        candidates.extend(reading_entries);

        let meaning_entries: Vec<MyEntry> = jmdict::entries()
            .filter(|e| {
                e.senses().any(|s| {
                    s.glosses()
                        .any(|g| g.text.matches(word.as_str()).next().is_some())
                })
            })
            .map(MyEntry)
            .collect();
        candidates.extend(meaning_entries);

        candidates
    }

    pub fn get_all(&self) -> Vec<MyEntry> {
        self.kanji_to_entry.values().cloned().collect()
    }
}

#[derive(Debug, Clone)]
pub struct MyEntry(jmdict::Entry);
impl MyEntry {
    pub fn kanji_elements_string(&self) -> String {
        self.0
            .kanji_elements()
            .map(|k| format!("{}", k.text,))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn kanji_element_string(&self) -> Option<String> {
        self.0
            .kanji_elements()
            .map(|k| format!("{}", k.text))
            .collect::<Vec<_>>()
            .get(0)
            .cloned()
    }

    pub fn word_string(&self) -> Option<String> {
        let kanji_string = self.kanji_element_string();
        if kanji_string.is_some() {
            return kanji_string;
        }
        self.reading_element_string().map(|r| r.to_string())
    }

    pub fn reading_elements_string(&self) -> String {
        self.0
            .reading_elements()
            .map(|r| r.text)
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn reading_element_string(&self) -> Option<&str> {
        self.0
            .reading_elements()
            .map(|r| r.text)
            .collect::<Vec<_>>()
            .get(0)
            .map(|s| *s)
    }

    pub fn translations_string(&self) -> String {
        self.0
            .senses()
            .map(|s| s.glosses().map(|g| g.text).collect::<Vec<_>>().join(", "))
            .collect::<Vec<_>>()
            .join("; ")
    }

    pub fn entry_display(&self) -> MyEntryDisplay {
        MyEntryDisplay {
            word: self.word_string().unwrap_or("No word found".to_string()),
            reading: self.reading_elements_string(),
            translations: self.translations_string(),
        }
    }
}

#[derive(Debug, Clone, specta::Type, serde::Serialize, serde::Deserialize)]
pub struct MyEntryDisplay {
    word: String,
    reading: String,
    translations: String,
}

pub async fn translate_word(word: &String, app_handle: &tauri::AppHandle) -> Option<MyEntry> {
    let dictionary = MyDictionary::get(&app_handle);
    dictionary.find(word)
}

pub async fn translate_word_all(word: &String, app_handle: &tauri::AppHandle) -> Vec<MyEntry> {
    let dictionary = MyDictionary::get(&app_handle);
    dictionary.find_all(word)
}

pub async fn get_all_entries(app_handle: &tauri::AppHandle) -> Vec<MyEntry> {
    let dictionary = MyDictionary::get(&app_handle);
    dictionary.get_all()
}
