pub mod words;

pub fn setup(app: &mut tauri::App) {
    words::setup(app);
}
