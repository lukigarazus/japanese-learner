use std::collections::HashMap;
use std::sync::Arc;

const HEISIG_JSON: &str = include_str!("./heisig_kanji.json");

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct HeisigKanji {
    pub id: String,
    pub frameNoV4: String,
    pub frameNoV6: String,
    pub keyword: String,
    pub kanji: String,
    pub strokeDiagram: String,
    pub hint: String,
    pub constituent: String,
    pub strokeCount: String,
    pub lessonNo: String,
    pub myStory: String,
    pub heisigStory: String,
    pub heisigComment: String,
    pub koohiiStory1: String,
    pub koohiiStory2: String,
    pub jouYou: String,
    pub jlpt: String,
    pub onYomi: String,
    pub kunYomi: String,
    pub words: String,
    pub readingExamples: String,
}

pub struct HeisigKanjis {
    vec: Vec<Arc<HeisigKanji>>,
    id_map: HashMap<String, Arc<HeisigKanji>>,
    kanji_map: HashMap<String, Arc<HeisigKanji>>,
}

impl HeisigKanjis {
    fn new(vec: Vec<Arc<HeisigKanji>>) -> Self {
        let mut id_map = HashMap::new();
        let mut kanji_map = HashMap::new();

        for kanji in vec.iter() {
            id_map.insert(kanji.id.clone(), kanji.clone());
            kanji_map.insert(kanji.kanji.clone(), kanji.clone());
        }
        Self {
            vec,
            id_map,
            kanji_map,
        }
    }

    pub fn get_by_id(&self, id: &String) -> Option<Arc<HeisigKanji>> {
        self.id_map.get(id).cloned()
    }

    pub fn get_by_kanji(&self, kanji: &String) -> Vec<Arc<HeisigKanji>> {
        let res = self.kanji_map.get(kanji).cloned();
        if let Some(k) = res { vec![k] } else { vec![] }
    }

    pub fn get_by_reading(&self, reading: &String) -> Vec<Arc<HeisigKanji>> {
        self.vec
            .iter()
            .filter(|k| k.onYomi.contains(reading) || k.kunYomi.contains(reading))
            .cloned()
            .collect()
    }

    pub fn get_by_keywords(&self, keywords: &Vec<String>) -> Vec<Arc<HeisigKanji>> {
        self.vec
            .iter()
            .filter(|k| keywords.iter().all(|kw| k.keyword.contains(kw)))
            .cloned()
            .collect()
    }

    pub fn get_all(&self) -> &Vec<Arc<HeisigKanji>> {
        &self.vec
    }
}

pub fn get_heisig_kanjis() -> HeisigKanjis {
    let vec = serde_json::from_str::<Vec<Arc<HeisigKanji>>>(HEISIG_JSON).unwrap();
    HeisigKanjis::new(vec)
}
