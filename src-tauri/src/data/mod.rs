use tauri::Manager;

pub mod heisig_kanji;
pub mod kanjidic2;

pub fn setup(tauri_app: &tauri::App) {
    tauri_app.manage(heisig_kanji::get_heisig_kanjis());
    let res = kanjidic2::parse_kanjidic2(tauri_app);
    if let Ok(kanjidic2_reader) = res {
        println!("Successfully parsed kanjidic2.csv");
        tauri_app.manage(kanjidic2_reader);
    } else if let Err(e) = res {
        eprintln!("Failed to parse kanjidic2.csv: {}", e);
    }
}

pub fn get_heisig_kanjis(
    tauri_app_handle: &tauri::AppHandle,
) -> tauri::State<heisig_kanji::HeisigKanjis> {
    tauri_app_handle.state::<heisig_kanji::HeisigKanjis>()
}

pub fn get_kanjidic2_entries_reader(
    tauri_app_handle: &tauri::AppHandle,
) -> tauri::State<kanjidic2::Kanjidic2State> {
    tauri_app_handle.state::<kanjidic2::Kanjidic2State>()
}
