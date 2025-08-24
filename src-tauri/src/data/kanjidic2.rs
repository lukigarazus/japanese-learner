use std::fs::File;
use tauri::Manager;
use tauri::async_runtime::RwLock;
use tauri::path::BaseDirectory;

// const KANJIDIC2_CSV: &str = include_str!("../../data/kanjidic2.csv");

pub fn parse_kanjidic2(app: &tauri::App) -> Result<Kanjidic2State, String> {
    let kanjidic2_path = app
        .path()
        .resolve("data/kanjidic2.csv", BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;
    let file = File::open(&kanjidic2_path).map_err(|e| e.to_string())?;
    Ok(Kanjidic2State(RwLock::new(Kanjidic2::new(file))))
}

pub struct Kanjidic2State(pub RwLock<Kanjidic2>);
pub struct Kanjidic2 {
    entries: Vec<Kanjidic2Entry>,
}

impl Kanjidic2 {
    pub fn new(file: File) -> Self {
        let mut rdr = csv::Reader::from_reader(file);
        let mut entries = Vec::new();
        for result in rdr.deserialize() {
            match result {
                Ok(entry) => entries.push(entry),
                Err(e) => {
                    eprintln!("Error reading CSV: {}", e);
                    continue;
                }
            }
        }
        Kanjidic2 { entries }
    }

    pub fn find_by_kanji(&self, kanji: &String) -> Option<Kanjidic2Entry> {
        for entry in self.entries.iter() {
            if &entry.literal == kanji {
                return Some(entry.clone());
            }
        }
        None
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, specta::Type)]
pub struct Kanjidic2Entry {
    pub literal: String,
    pub ja_on: String,
    pub ja_kun: String,
    pub heisig: String,
    pub heisig6: String,
}
