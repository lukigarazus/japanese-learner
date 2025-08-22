use specta::Type;
use std::sync::Arc;

#[derive(Debug, serde::Serialize, Type)]
pub struct KanjiPayload {
    id: String,
    kanji: String,
    pronunciation: String,
    primitives: Vec<String>,
    words: Vec<String>,
}
impl KanjiPayload {
    pub fn from_heisig_kanji(heisig_kanji: Arc<crate::data::heisig_kanji::HeisigKanji>) -> Self {
        let pronunciation = heisig_kanji.kunYomi.clone() + &heisig_kanji.onYomi;
        KanjiPayload {
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

#[tauri::command]
#[specta::specta]
pub async fn search_kanji(
    char: String,
    app_handle: tauri::AppHandle,
) -> Result<Option<KanjiPayload>, String> {
    let kanjis = crate::data::get_heisig_kanjis(&app_handle);

    let kanji = kanjis.get_by_kanji(&char);

    if let None = kanji {
        Ok(None)
    } else {
        let kanji = kanji.unwrap();
        Ok(Some(KanjiPayload::from_heisig_kanji(kanji)))
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_heisig_kanjis(app_handle: tauri::AppHandle) -> Vec<KanjiPayload> {
    crate::data::get_heisig_kanjis(&app_handle)
        .get_all()
        .iter()
        .map(|k| KanjiPayload::from_heisig_kanji(k.clone()))
        .collect()
}

#[tauri::command]
#[specta::specta]
pub async fn search_kanjis(
    chars: Vec<String>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<KanjiPayload>, String> {
    let kanjis = crate::data::get_heisig_kanjis(&app_handle);

    let mut result = vec![];

    for ch in chars {
        let kanji = kanjis.get_by_kanji(&ch);
        if let Some(kanji) = kanji {
            result.push(KanjiPayload::from_heisig_kanji(kanji));
        }
    }

    Ok(result)
}
