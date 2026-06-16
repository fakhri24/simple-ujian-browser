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
        .setup(|app| {
            // 1. Enable OS-level kiosk (keyboard hook on Windows)
            kiosk::enable_kiosk_mode();

            // 2. Apply WebView2-level kiosk (context menu + iframe injection)
            #[cfg(target_os = "windows")]
            {
                // In Tauri 2, App has handle() which provides webview access
                let handle = app.handle();
                if let Some(wv) = handle.default_webview() {
                    kiosk::webview_setup::apply_webview_kiosk(&wv);
                } else {
                    eprintln!("[Kiosk] No default webview found during setup");
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
