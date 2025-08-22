use specta_typescript::Typescript;
use tauri_specta::{Builder, collect_commands};

mod conversion;
mod data;
mod kanji;
mod knowledge_base;
mod translation;
mod word;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![
            kanji::commands::search_kanji,
            kanji::commands::get_heisig_kanjis,
            kanji::commands::search_kanjis,
            word::get_word_dict_entry,
            word::get_word_candidates,
            knowledge_base::words::get_words,
            knowledge_base::words::add_word,
            knowledge_base::words::has_word,
            knowledge_base::kanjis::get_kanjis,
            knowledge_base::kanjis::add_kanji,
            knowledge_base::kanjis::has_kanji,
        ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        // and finally tell Tauri how to invoke them
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            data::setup(app);
            knowledge_base::setup(app);
            translation::setup(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
