mod config;
mod kiosk;

use config::ExamConfig;

#[tauri::command]
fn get_config() -> ExamConfig {
    ExamConfig::load()
}

#[tauri::command]
fn exit_app() {
    std::process::exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_config, exit_app])
        .setup(|_app| {
            kiosk::enable_kiosk_mode();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
