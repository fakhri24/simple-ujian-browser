use std::ptr::null_mut;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx,
    HHOOK, HOOKPROC, KBDLLHOOKSTRUCT, KBDLLHOOKSTRUCT_FLAGS, MSG,
    WH_KEYBOARD_LL,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VK_DELETE, VK_ESCAPE, VK_F4, VK_F5, VK_LWIN, VK_RWIN, VK_TAB,
    VK_SNAPSHOT,
};

const LLKHF_ALTDOWN: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(0x20);

static mut HOOK_HANDLE: Option<HHOOK> = None;

unsafe extern "system" fn keyboard_hook_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code >= 0 {
        let kb_struct = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
        let vk_code = kb_struct.vkCode;
        let alt_down = (kb_struct.flags & LLKHF_ALTDOWN) != KBDLLHOOKSTRUCT_FLAGS(0);

        let block = match vk_code {
            // Alt+Tab (task switcher)
            _ if vk_code == VK_TAB.0 as u32 && alt_down => true,
            // Win key (both left and right) — Start menu
            _ if vk_code == VK_LWIN.0 as u32 || vk_code == VK_RWIN.0 as u32 => true,
            // Alt+F4 (close window)
            _ if vk_code == VK_F4.0 as u32 && alt_down => true,
            // Ctrl+Esc (Start menu alternative)
            _ if vk_code == VK_ESCAPE.0 as u32 && is_ctrl_pressed() => true,
            // Alt+Esc (window cycling)
            _ if vk_code == VK_ESCAPE.0 as u32 && alt_down => true,
            // Print Screen (screenshot)
            _ if vk_code == VK_SNAPSHOT.0 as u32 => true,
            // Alt+Print Screen (active window screenshot)
            // Already covered by VK_SNAPSHOT + alt_down
            // F5 (refresh in browser)
            _ if vk_code == VK_F5.0 as u32 && !alt_down => true,
            // Ctrl+Alt+Delete — can't fully block at hook level,
            // but block Ctrl+Alt combos
            _ if vk_code == VK_DELETE.0 as u32 && is_ctrl_pressed() && alt_down => true,
            _ => false,
        };

        if block {
            return LRESULT(1);
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

fn is_ctrl_pressed() -> bool {
    const VK_CONTROL: i32 = 0x11;
    unsafe { (GetAsyncKeyState(VK_CONTROL) & 0x8000u16 as i16) != 0 }
}

pub fn enable_keyboard_hook() {
    std::thread::spawn(|| {
        unsafe {
            let h_module = GetModuleHandleW(None).unwrap();
            let hook_proc: HOOKPROC = Some(keyboard_hook_proc);

            let hook = SetWindowsHookExW(WH_KEYBOARD_LL, hook_proc, h_module, 0);

            match hook {
                Ok(h) => {
                    HOOK_HANDLE = Some(h);
                    println!("[Kiosk] Keyboard hook installed successfully");
                    println!("[Kiosk] Blocked: Win key, Alt+Tab, Alt+F4, Ctrl+Esc, Alt+Esc, PrintScreen, F5");

                    // Message loop required to keep the low-level keyboard hook alive.
                    // Runs on a dedicated thread so the main thread can drive
                    // the Tauri event loop without blocking.
                    let mut msg = MSG::default();
                    while GetMessageW(&mut msg, None, 0, 0).into() {
                        // Keep processing messages to keep hook alive
                    }
                }
                Err(e) => {
                    eprintln!("[Kiosk] Failed to install keyboard hook: {}", e);
                }
            }
        }
    });
}

#[allow(dead_code)]
pub fn disable_keyboard_hook() {
    unsafe {
        if let Some(hook) = HOOK_HANDLE.take() {
            let _ = UnhookWindowsHookEx(hook);
            println!("[Kiosk] Keyboard hook removed");
        }
    }
}
