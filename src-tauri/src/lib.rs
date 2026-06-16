mod config;
mod kiosk;

use config::ExamConfig;
use tauri::{Emitter, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
fn get_config(state: tauri::State<'_, ExamConfig>) -> ExamConfig {
    // admin_password_hash is #[serde(skip_serializing)], so it is never sent here.
    state.inner().clone()
}

#[tauri::command]
fn verify_admin_password(password: String, state: tauri::State<'_, ExamConfig>) -> bool {
    state.verify_password(&password)
}

#[tauri::command]
fn exit_app() {
    std::process::exit(0);
}

fn is_internal_url(url: &tauri::Url) -> bool {
    // The app's own pages load over the tauri:// (or http://tauri.localhost) origin.
    url.scheme() == "tauri"
        || matches!(url.host_str(), Some("tauri.localhost") | Some("localhost"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = ExamConfig::load();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(config.clone())
        .invoke_handler(tauri::generate_handler![
            get_config,
            exit_app,
            verify_admin_password
        ])
        .setup(move |app| {
            // OS-level kiosk (keyboard hook on Windows, no-op elsewhere)
            kiosk::enable_kiosk_mode();

            let handle = app.handle().clone();
            let nav_config = config.clone();

            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("Simple Ujian Browser")
                .fullscreen(config.fullscreen)
                .decorations(false)
                .resizable(false)
                .always_on_top(true)
                .user_agent(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                     SimpleExamBrowser/1.0 SEB",
                )
                // Inject kiosk JS into every frame (incl. the cross-origin exam iframe).
                // for_all_frames is required: plain initialization_script is main-frame only.
                .initialization_script_for_all_frames(kiosk::webview_setup::KIOSK_SCRIPT)
                // Restrict navigation to whitelisted domains; notify UI when blocked.
                .on_navigation(move |url| {
                    let allowed = is_internal_url(url) || nav_config.is_url_allowed(url.as_str());
                    if !allowed {
                        let _ = handle.emit("blocked-url", url.to_string());
                    }
                    allowed
                })
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
