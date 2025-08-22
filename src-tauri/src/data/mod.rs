use tauri::Manager;

pub mod heisig_kanji;

pub fn setup(tauri_app: &tauri::App) {
    tauri_app.manage(heisig_kanji::get_heisig_kanjis());
}

pub fn get_heisig_kanjis(
    tauri_app_handle: &tauri::AppHandle,
) -> tauri::State<heisig_kanji::HeisigKanjis> {
    tauri_app_handle.state::<heisig_kanji::HeisigKanjis>()
}
