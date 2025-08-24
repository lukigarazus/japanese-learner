use crate::data::kanjidic2::*;
use specta::Type;
use std::sync::Arc;

trait EmptyToNone {
    fn empty_to_none(self) -> Option<Self>
    where
        Self: Sized;
}

impl EmptyToNone for String {
    fn empty_to_none(self) -> Option<Self> {
        if self.is_empty() { None } else { Some(self) }
    }
}

#[derive(Debug, serde::Serialize, Type)]
pub struct HeisigKanjiPayload {
    id: String,
    kanji: String,
    pronunciation: String,
    primitives: Vec<String>,
    words: Vec<String>,
    jlpt_level: Option<u8>,
    heisig_mnemonic: Option<String>,
    koohii_mnemonic_1: Option<String>,
    koohii_mnemonic_2: Option<String>,
}
impl HeisigKanjiPayload {
    pub fn from_heisig_kanji(heisig_kanji: Arc<crate::data::heisig_kanji::HeisigKanji>) -> Self {
        let pronunciation =
            vec![heisig_kanji.kunYomi.clone(), heisig_kanji.onYomi.clone()].join(", ");
        HeisigKanjiPayload {
            id: heisig_kanji.id.clone(),
            kanji: heisig_kanji.kanji.clone(),
            pronunciation,
            primitives: heisig_kanji
                .constituent
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            words: heisig_kanji
                .words
                .split("<br>")
                .map(|s| s.trim().to_string())
                .collect(),
            jlpt_level: heisig_kanji.jlpt.parse::<u8>().ok(),
            heisig_mnemonic: heisig_kanji.heisigStory.clone().empty_to_none(),
            koohii_mnemonic_1: heisig_kanji.koohiiStory1.clone().empty_to_none(),
            koohii_mnemonic_2: heisig_kanji.koohiiStory2.clone().empty_to_none(),
        }
    }
}
//impl Into<KanjiPayload> for Arc<crate::data::heisig_kanji::HeisigKanji> {
//    fn into(self) -> KanjiPayload {
//        KanjiPayload {
//            id: self.id.clone(),
//            kanji: self.kanji.clone(),
//            pronunciation: self.kunYomi.clone(),
//            primitives: self
//                .constituent
//                .split(',')
//                .map(|s| s.trim().to_string())
//                .collect(),
//        }
//    }
//}

#[derive(Debug, specta::Type, serde::Deserialize, serde::Serialize)]
pub enum HeisigKanjiQuery {
    Kanji(String),
    Reading(String),
    Keywords(Vec<String>),
}

#[tauri::command]
#[specta::specta]
pub async fn search_heisig_kanji(
    query: HeisigKanjiQuery,
    app_handle: tauri::AppHandle,
) -> Result<Vec<HeisigKanjiPayload>, String> {
    let kanjis = crate::data::get_heisig_kanjis(&app_handle);

    let kanjis = match query {
        HeisigKanjiQuery::Kanji(char) => kanjis.get_by_kanji(&char),
        HeisigKanjiQuery::Reading(reading) => kanjis.get_by_reading(&reading),
        HeisigKanjiQuery::Keywords(keywords) => kanjis.get_by_keywords(&keywords),
    };

    Ok(kanjis
        .into_iter()
        .map(|k| HeisigKanjiPayload::from_heisig_kanji(k))
        .collect())
}

#[tauri::command]
#[specta::specta]
pub async fn get_heisig_kanjis(app_handle: tauri::AppHandle) -> Vec<HeisigKanjiPayload> {
    crate::data::get_heisig_kanjis(&app_handle)
        .get_all()
        .iter()
        .map(|k| HeisigKanjiPayload::from_heisig_kanji(k.clone()))
        .collect()
}

#[tauri::command]
#[specta::specta]
pub async fn search_heisig_kanjis(
    chars: Vec<String>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<HeisigKanjiPayload>, String> {
    let kanjis = crate::data::get_heisig_kanjis(&app_handle);

    let mut result = vec![];

    for ch in chars {
        let kanji = kanjis.get_by_kanji(&ch);
        if let Some(kanji) = kanji.get(0) {
            result.push(HeisigKanjiPayload::from_heisig_kanji(kanji.clone()));
        }
    }

    Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn get_kanjidic2_by_kanji(
    app_handle: tauri::AppHandle,
    kanji: String,
) -> Result<Vec<Kanjidic2Entry>, String> {
    let kanjidic2_reader_state = crate::data::get_kanjidic2_entries_reader(&app_handle);
    let mut kanjidic2_reader = kanjidic2_reader_state.0.write().await;
    let mut entries = Vec::new();

    if let Some(entry) = kanjidic2_reader.find_by_kanji(&kanji) {
        entries.push(entry.clone());
    }

    Ok(entries)
}

#[tauri::command]
#[specta::specta]
pub async fn parse_word(
    app_handle: tauri::AppHandle,
    word: String,
) -> Result<crate::kanji::parser::FuriganaString, String> {
    crate::kanji::parser::parse_word(&app_handle, &word, None).await
}

#[tauri::command]
#[specta::specta]
pub async fn validate_dictionary(app_handle: tauri::AppHandle) -> Result<(), String> {
    let dictionary = crate::translation::get_all_entries(&app_handle).await;

    let mut c = 0;
    for entry in &dictionary {
        let kanji_string = entry.kanji_element_string();
        if let Some(kanji_string) = kanji_string {
            let parsed = crate::kanji::parser::parse_word(
                &app_handle,
                &kanji_string,
                entry.reading_element_string().map(|s| s.to_string()),
            )
            .await;
            match parsed {
                Ok(furigana_string) => {}
                Err(err) => {
                    println!("Failed to parse kanji string {:?}: {}", kanji_string, err);
                    c += 1;
                }
            }
        }
    }

    println!(
        "Validated {} dictionary entries, found {} errors",
        dictionary.len(),
        c
    );

    Ok(())
}
