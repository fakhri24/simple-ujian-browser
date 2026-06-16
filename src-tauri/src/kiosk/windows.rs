use std::ptr::null_mut;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx,
    HHOOK, HOOKPROC, KBDLLHOOKSTRUCT, KBDLLHOOKSTRUCT_FLAGS, MSG,
    WH_KEYBOARD_LL,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VK_ESCAPE, VK_F4, VK_LWIN, VK_RWIN, VK_TAB,
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
            _ if vk_code == VK_TAB.0 as u32 && alt_down => true,
            _ if vk_code == VK_LWIN.0 as u32 || vk_code == VK_RWIN.0 as u32 => true,
            _ if vk_code == VK_F4.0 as u32 && alt_down => true,
            _ if vk_code == VK_ESCAPE.0 as u32 && is_ctrl_pressed() => true,
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
    unsafe {
        let h_module = GetModuleHandleW(None).unwrap();
        let hook_proc: HOOKPROC = Some(keyboard_hook_proc);

        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, hook_proc, h_module, 0);

        match hook {
            Ok(h) => {
                HOOK_HANDLE = Some(h);
                println!("[Kiosk] Keyboard hook installed successfully");

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
