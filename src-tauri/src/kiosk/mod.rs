#[cfg(target_os = "windows")]
pub mod windows;

pub fn enable_kiosk_mode() {
    #[cfg(target_os = "windows")]
    {
        windows::enable_keyboard_hook();
        println!("[Kiosk] Windows keyboard hook active");
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("[Kiosk] Skipping lockdown (not Windows — use SEB on macOS/iOS)");
    }
}
