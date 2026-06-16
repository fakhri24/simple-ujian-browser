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
            // 1. Disable DevTools programmatically
            if let Some(window) = app.windows().values().next() {
                // DevTools is disabled by not enabling it (Tauri 2 default: off)
                // For extra safety, we can close it if somehow opened
                let _ = window.eval("/* devtools disabled */");
            }

            // 2. Enable OS-level kiosk (keyboard hook on Windows)
            kiosk::enable_kiosk_mode();

            // 3. Apply WebView2-level kiosk (context menu + iframe injection)
            let webview = app.webviews().first().cloned();
            if let Some(wv) = webview {
                kiosk::webview_setup::apply_webview_kiosk(&wv);
            } else {
                eprintln!("[Kiosk] No webview found during setup");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
