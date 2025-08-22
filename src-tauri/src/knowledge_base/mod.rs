pub mod entity;
pub mod kanjis;
pub mod words;

use entity::*;

const KANJIS_STORE_FILE: &str = "kanjis.json";

pub fn setup(app: &mut tauri::App) {
    words::setup(app);
    kanjis::KanjisState::setup(app, KANJIS_STORE_FILE);
}
