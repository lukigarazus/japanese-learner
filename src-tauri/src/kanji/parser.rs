use std::collections::HashMap;

use tauri::AppHandle;
use tokio::sync::RwLockReadGuard;
use wana_kana;

use crate::data::kanjidic2::Kanjidic2;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, specta::Type)]
pub enum Furigana {
    Kanji { character: char, reading: String },
    Other(char),
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct FuriganaString(Vec<Furigana>);

impl FuriganaString {
    pub fn to_html(&self) -> String {
        self.0
            .iter()
            .map(|f| match f {
                Furigana::Kanji { character, reading } => {
                    format!("<ruby><rb>{}</rb><rt>{}</rt></ruby>", character, reading)
                }
                Furigana::Other(s) => s.to_string(),
            })
            .collect()
    }

    pub fn from_furigana_parser(input: furigana_parser::FuriganaString) -> Self {
        FuriganaString(
            input
                .to_vec()
                .into_iter()
                .map(|f| match f {
                    furigana_parser::Furigana::Kanji { character, reading } => {
                        Furigana::Kanji { character, reading }
                    }
                    furigana_parser::Furigana::Other(s) => Furigana::Other(s),
                })
                .collect(),
        )
    }
}

pub async fn parse_word(
    app: &AppHandle,
    word: &String,
    reading: Option<String>,
) -> Result<FuriganaString, String> {
    let kanjidic2_state = crate::data::get_kanjidic2_entries_reader(app);
    let kanjidic2 = kanjidic2_state.0.read().await;

    if word.chars().count() == 0 {
        return Err("Empty word".to_string());
    }

    let reading = reading
        .ok_or("Reading not provided".to_string())
        .or(crate::conversion::convert(word))?;

    let mut kanji_readings = HashMap::new();
    let mut prev = None;
    for char in word.chars() {
        let is_kanji = wana_kana::utils::is_char_kanji(char);
        if is_kanji {
            if let Some(readings) = get_readings_for_kanji(char, &kanjidic2) {
                kanji_readings.insert(char, readings);
                prev = Some(char);
            } else {
                println!("No readings found for kanji: {}", char);
            }
        } else if char == '々' {
            if let Some(prev_char) = prev {
                if let Some(readings) = kanji_readings.get(&prev_char) {
                    let mut new_readings = vec![];
                    for reading in readings {
                        new_readings.push(transform_first_to_dakuten(reading));
                        new_readings.push(reading.clone());
                    }
                    kanji_readings.insert(char, new_readings);
                }
            }
        }

        if !is_kanji {
            prev = None;
        }
    }
    let result = furigana_parser::parse_furigana(&word, &reading, &kanji_readings);

    if let Err(err) = &result {
        println!(
            "Failed to parse word: {}, reading: {}: readings: {:?}",
            word, reading, kanji_readings
        );
    }

    result
        .map(|furigana| FuriganaString::from_furigana_parser(furigana))
        .map_err(|_| "Failed to parse furigana".to_string())
}

fn get_readings_for_kanji(
    char: char,
    kanjidic2: &RwLockReadGuard<Kanjidic2>,
) -> Option<Vec<String>> {
    let kanji_entry = kanjidic2.find_by_kanji(&char.to_string());
    if let Some(kanji_entry) = kanji_entry {
        let kun_readings = kanji_entry
            .ja_kun
            .clone()
            .split(';')
            .map(|s| s.split('.').map(|s| s.to_string()).collect::<Vec<_>>())
            .flatten()
            .map(|s| s.replace("-", ""))
            .collect::<Vec<_>>();
        let on_readings = kanji_entry
            .ja_on
            .clone()
            .split(';')
            .map(|s| wana_kana::ConvertJapanese::to_hiragana(s))
            .collect::<Vec<_>>();
        let mut mappings = kun_readings;
        mappings.extend(on_readings);

        for s in mappings.clone() {
            let last_char = s.chars().last().unwrap_or_default();

            if last_char == 'つ' {
                let mut s2 = s.clone();
                s2.pop();
                s2.push('っ');
                mappings.push(s2);
            }
        }

        Some(mappings)
    } else {
        None
    }
}

fn transform_first_to_dakuten(s: &String) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { transform_to_dakuten(&c) } else { c })
        .collect()
}

fn transform_to_dakuten(c: &char) -> char {
    match c {
        'か' => 'が',
        'き' => 'ぎ',
        'く' => 'ぐ',
        'け' => 'げ',
        'こ' => 'ご',
        'さ' => 'ざ',
        'し' => 'じ',
        'す' => 'ず',
        'せ' => 'ぜ',
        'そ' => 'ぞ',
        'た' => 'だ',
        'ち' => 'ぢ',
        'つ' => 'づ',
        'て' => 'で',
        'と' => 'ど',
        'は' => 'ば',
        'ひ' => 'び',
        'ふ' => 'ぶ',
        'へ' => 'べ',
        'ほ' => 'ぼ',
        _ => *c,
    }
}
