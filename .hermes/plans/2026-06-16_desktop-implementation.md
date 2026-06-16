# Simple Ujian Browser — Desktop Implementation Plan

> **For Hermes:** Use subagent-driven-development skill to implement this plan task-by-task.

**Goal:** Build a Tauri 2 desktop exam browser for Windows that loads simple-ujian.web.app in a locked-down fullscreen kiosk, blocking task switching and unauthorized navigation.

**Architecture:** Tauri 2 app with Rust backend (lockdown logic, config, keyboard hooks) and Vanilla JS frontend (URL filter, UI restrictions, exit dialog). Config-driven via local JSON file. Custom user agent for SEB compatibility.

**Tech Stack:** Rust (Tauri 2), Vanilla JS (frontend), JSON (config)

**Platform Strategy:**
- **Development:** macOS (`cargo tauri dev` for UI/logic testing)
- **Production build:** Windows .msi + .exe via GitHub Actions CI/CD
- **macOS/iOS users:** Use official Safe Exam Browser (SEB) — not this app
- **This app is ONLY for Windows (and later Android)**
- Platform-specific code uses `#[cfg(target_os = "windows")]` conditional compilation

---

## Project Context

- **Website:** https://simple-ujian.web.app/
- **SEB Detection:** `navigator.userAgent.includes("SEB")` or `window.SafeExamBrowser`
- **Hybrid Strategy:** macOS/iOS → SEB official. Windows/Android → This custom app.
- **Project folder:** `~/project/ujian/simple-ujian-browser/`
- **Existing files:** `docs/ARCHITECTURE.md`, `README.md`
- **User:** Rust beginner, knows JS, learning full-stack dev
- **Repo:** Will be on GitHub (for CI/CD)

---

## Task 1: Install Rust Toolchain

**Objective:** Install Rust and verify the installation works.

**Files:** None (system install)

**Step 1: Install Rust via rustup**

Run:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Choose option `1` (default installation) when prompted.

**Step 2: Source the environment**

Run:
```bash
source "$HOME/.cargo/env"
```

**Step 3: Verify installation**

Run:
```bash
rustc --version
cargo --version
```

Expected output (version numbers may vary):
```
rustc 1.XX.0 (XXXX 2026-XX-XX)
cargo 1.XX.0 (XXXX 2026-XX-XX)
```

**Step 4: Verify**

Run `rustc --version` — should print a version number.

---

## Task 2: Install Tauri CLI

**Objective:** Install the Tauri command-line tool for project scaffolding and building.

**Files:** None (system install)

**Step 1: Install Tauri CLI**

Run:
```bash
cargo install tauri-cli
```

This takes 2-5 minutes (compiles from source).

**Step 2: Verify**

Run:
```bash
cargo tauri --version
```

Expected: `tauri-cli 2.x.x`

---

## Task 3: Scaffold Tauri 2 Project

**Objective:** Create the Tauri project inside the simple-ujian-browser folder.

**Files:**
- Create: `~/project/ujian/simple-ujian-browser/src-tauri/` (Tauri backend)
- Create: `~/project/ujian/simple-ujian-browser/src/` (frontend)

**Step 1: Initialize Tauri project**

Run from `~/project/ujian/simple-ujian-browser/`:
```bash
cd ~/project/ujian/simple-ujian-browser
cargo tauri init --window-title "Simple Ujian Browser" --dev-url "https://simple-ujian.web.app" --before-dev-command "" --before-build-command ""
```

When prompted:
- Identifier: `id.simple-ujian-browser`
- Frontend dist directory: `../src`
- Frontend dev URL: `https://simple-ujian.web.app`

**Step 2: Create minimal frontend**

Create `src/index.html`:
```html
<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Simple Ujian Browser</title>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body {
      font-family: -apple-system, BlinkMacSystemFont, sans-serif;
      background: #1a1a2e;
      color: #fff;
      display: flex;
      justify-content: center;
      align-items: center;
      height: 100vh;
      user-select: none;
    }
    .loading { text-align: center; }
    .loading h1 { font-size: 2rem; margin-bottom: 1rem; }
    .spinner {
      width: 40px; height: 40px;
      border: 4px solid rgba(255,255,255,0.3);
      border-top: 4px solid #fff;
      border-radius: 50%;
      animation: spin 1s linear infinite;
      margin: 0 auto;
    }
    @keyframes spin { to { transform: rotate(360deg); } }
  </style>
</head>
<body>
  <div class="loading">
    <h1>Simple Ujian Browser</h1>
    <p>Memuat ujian...</p>
    <div class="spinner"></div>
  </div>
  <script>
    // Inject SEB detection for simple-ujian compatibility
    window.SafeExamBrowser = { version: "1.0.0", custom: true };
  </script>
</body>
</html>
```

**Step 3: Verify scaffold builds**

Run:
```bash
cd ~/project/ujian/simple-ujian-browser
cargo tauri dev --no-watch
```

Expected: A window opens showing "Simple Ujian Browser" loading screen.
Press Ctrl+C to stop.

**Step 4: Commit**

```bash
cd ~/project/ujian/simple-ujian-browser
git init
echo "target/" > .gitignore
echo "node_modules/" >> .gitignore
echo ".DS_Store" >> .gitignore
git add -A
git commit -m "feat: scaffold Tauri 2 project with minimal frontend"
```

---

## Task 4: Configure Window Properties (Fullscreen Kiosk)

**Objective:** Make the app open in fullscreen with no window decorations.

**Files:**
- Modify: `src-tauri/tauri.conf.json`

**Step 1: Edit tauri.conf.json**

Update the `app.windows` section in `src-tauri/tauri.conf.json`:

```json
{
  "app": {
    "windows": [
      {
        "title": "Simple Ujian Browser",
        "fullscreen": true,
        "decorations": false,
        "resizable": false,
        "alwaysOnTop": true,
        "url": "index.html"
      }
    ],
    "security": {
      "csp": "default-src 'self' https://simple-ujian.web.app https://*.googleapis.com https://*.firebaseapp.com https://www.gstatic.com; script-src 'self' 'unsafe-inline' https://*.googleapis.com https://*.firebaseapp.com https://www.gstatic.com; style-src 'self' 'unsafe-inline'; img-src 'self' https: data:; connect-src 'self' https://*.googleapis.com https://*.firebaseapp.com wss://*.firebaseio.com"
    }
  },
  "build": {
    "frontendDist": "../src"
  }
}
```

Key settings:
- `fullscreen: true` — covers entire screen
- `decorations: false` — no title bar, no close/minimize buttons
- `alwaysOnTop: true` — stays above other windows
- `resizable: false` — cannot be resized

**Step 2: Verify**

Run:
```bash
cargo tauri dev --no-watch
```

Expected: App opens fullscreen, no title bar, covers entire screen.

**Step 3: Commit**

```bash
git add -A
git commit -m "feat: configure fullscreen kiosk window"
```

---

## Task 5: Set Custom User Agent (SEB Detection)

**Objective:** Make simple-ujian recognize this browser as an exam browser.

**Files:**
- Modify: `src-tauri/tauri.conf.json`

**Step 1: Add user agent to window config**

In `src-tauri/tauri.conf.json`, add to the window object:

```json
{
  "app": {
    "windows": [
      {
        "title": "Simple Ujian Browser",
        "fullscreen": true,
        "decorations": false,
        "resizable": false,
        "alwaysOnTop": true,
        "url": "index.html",
        "userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 SimpleExamBrowser/1.0 SEB"
      }
    ]
  }
}
```

Note: User agent includes "SEB" which simple-ujian checks for.

**Step 2: Also keep the JS fallback**

Ensure `src/index.html` still has:
```javascript
window.SafeExamBrowser = { version: "1.0.0", custom: true };
```

This is a safety net in case user agent alone isn't enough.

**Step 3: Verify**

Run `cargo tauri dev --no-watch`. The app should load simple-ujian.web.app and NOT show the "Safe Exam Browser Diperlukan" warning.

**Step 4: Commit**

```bash
git add -A
git commit -m "feat: set custom user agent for SEB compatibility"
```

---

## Task 6: Load simple-ujian.web.app in WebView

**Objective:** Replace the loading screen with the actual exam website.

**Files:**
- Modify: `src/index.html`

**Step 1: Update index.html to load the exam**

Replace `src/index.html`:
```html
<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <title>Simple Ujian Browser</title>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { overflow: hidden; }
    #exam-frame {
      width: 100vw;
      height: 100vh;
      border: none;
    }
    #blocked-overlay {
      display: none;
      position: fixed;
      top: 0; left: 0; right: 0; bottom: 0;
      background: rgba(220, 53, 69, 0.95);
      color: white;
      justify-content: center;
      align-items: center;
      z-index: 9999;
      font-family: -apple-system, sans-serif;
      text-align: center;
    }
    #blocked-overlay h2 { font-size: 2rem; margin-bottom: 1rem; }
    #blocked-overlay p { font-size: 1.2rem; opacity: 0.9; }
  </style>
</head>
<body>
  <iframe id="exam-frame" src="https://simple-ujian.web.app"></iframe>
  <div id="blocked-overlay">
    <div>
      <h2>🚫 URL Diblokir</h2>
      <p>Anda hanya dapat mengakses halaman ujian.</p>
    </div>
  </div>
  <script>
    // Inject SEB detection
    window.SafeExamBrowser = { version: "1.0.0", custom: true };

    // Disable right-click
    document.addEventListener('contextmenu', e => e.preventDefault());

    // Disable copy/paste/cut
    ['copy', 'paste', 'cut'].forEach(event => {
      document.addEventListener(event, e => e.preventDefault());
    });

    // Disable text selection
    document.body.style.userSelect = 'none';

    // Disable keyboard shortcuts (frontend level — supplements Rust hooks)
    document.addEventListener('keydown', (e) => {
      if (e.ctrlKey && ['c','v','x','a','p'].includes(e.key.toLowerCase())) {
        e.preventDefault();
      }
      if (e.key === 'F12') e.preventDefault();
      if (e.ctrlKey && e.shiftKey && e.key === 'I') e.preventDefault();
    });
  </script>
</body>
</html>
```

**Step 2: Verify**

Run `cargo tauri dev --no-watch`. Should show simple-ujian.web.app inside the app fullscreen. Right-click should be disabled.

**Step 3: Commit**

```bash
git add -A
git commit -m "feat: load simple-ujian in iframe with frontend restrictions"
```

---

## Task 7: Add Config File Reader (Rust)

**Objective:** Read exam configuration from a local JSON file.

**Files:**
- Create: `config/exam-config.json`
- Modify: `src-tauri/Cargo.toml` (add serde + regex dependencies)
- Create: `src-tauri/src/config.rs`

**Step 1: Create default config file**

Create `config/exam-config.json`:
```json
{
  "exam_url": "https://simple-ujian.web.app",
  "exam_name": "Ujian Sekolah",
  "whitelist": [
    "https://simple-ujian.web.app",
    "https://accounts.google.com",
    "https://*.googleapis.com",
    "https://*.firebaseapp.com",
    "https://www.gstatic.com"
  ],
  "fullscreen": true,
  "block_shortcuts": true,
  "block_task_switching": true,
  "disable_right_click": true,
  "disable_copy_paste": true,
  "admin_password": "guru2026"
}
```

**Step 2: Add dependencies to Cargo.toml**

Add to `[dependencies]` in `src-tauri/Cargo.toml`:
```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
regex = "1"
```

**Step 3: Create config module**

Create `src-tauri/src/config.rs`:
```rust
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct ExamConfig {
    pub exam_url: String,
    pub exam_name: String,
    pub whitelist: Vec<String>,
    pub fullscreen: bool,
    pub block_shortcuts: bool,
    pub block_task_switching: bool,
    pub disable_right_click: bool,
    pub disable_copy_paste: bool,
    pub admin_password: String,
}

impl ExamConfig {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .expect("Failed to read config file");
            serde_json::from_str(&content)
                .expect("Failed to parse config JSON")
        } else {
            Self::default_config()
        }
    }

    fn config_path() -> PathBuf {
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));
        exe_dir.join("config.json")
    }

    fn default_config() -> Self {
        Self {
            exam_url: "https://simple-ujian.web.app".to_string(),
            exam_name: "Ujian Sekolah".to_string(),
            whitelist: vec![
                "https://simple-ujian.web.app".to_string(),
                "https://accounts.google.com".to_string(),
                "https://*.googleapis.com".to_string(),
                "https://*.firebaseapp.com".to_string(),
                "https://www.gstatic.com".to_string(),
            ],
            fullscreen: true,
            block_shortcuts: true,
            block_task_switching: true,
            disable_right_click: true,
            disable_copy_paste: true,
            admin_password: "guru2026".to_string(),
        }
    }

    pub fn is_url_allowed(&self, url: &str) -> bool {
        self.whitelist.iter().any(|pattern| {
            if pattern.contains('*') {
                let regex_pattern = pattern
                    .replace('.', "\\.")
                    .replace('*', ".*");
                regex::Regex::new(&format!("^{}$", regex_pattern))
                    .map(|re| re.is_match(url))
                    .unwrap_or(false)
            } else {
                url.starts_with(pattern)
            }
        })
    }
}
```

**Step 4: Wire config into lib.rs**

Update `src-tauri/src/lib.rs`:
```rust
mod config;

use config::ExamConfig;

#[tauri::command]
fn get_config() -> ExamConfig {
    ExamConfig::load()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 5: Verify**

Run `cargo tauri dev --no-watch`. Should compile and run without errors.

**Step 6: Commit**

```bash
git add -A
git commit -m "feat: add JSON config reader with URL whitelist"
```

---

## Task 8: Pass Config to Frontend & Sync Restrictions

**Objective:** Make exam config available to frontend JS, apply restrictions from config.

**Files:**
- Modify: `src/index.html`

**Step 1: Replace index.html with config-aware version**

```html
<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <title>Simple Ujian Browser</title>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { overflow: hidden; }
    #exam-frame { width: 100vw; height: calc(100vh - 32px); margin-top: 32px; border: none; }
    #info-bar {
      position: fixed; top: 0; left: 0; right: 0; height: 32px;
      background: rgba(0,0,0,0.7); color: #fff;
      display: flex; align-items: center; justify-content: space-between;
      padding: 0 12px; font-size: 0.8rem; z-index: 9998;
      font-family: -apple-system, sans-serif;
    }
    #blocked-overlay {
      display: none; position: fixed; top: 0; left: 0; right: 0; bottom: 0;
      background: rgba(220, 53, 69, 0.95); color: white;
      justify-content: center; align-items: center;
      z-index: 9999; font-family: -apple-system, sans-serif; text-align: center;
    }
    #blocked-overlay h2 { font-size: 2rem; margin-bottom: 1rem; }
    #blocked-overlay p { font-size: 1.2rem; opacity: 0.9; }
    #exit-overlay {
      display: none; position: fixed; top: 0; left: 0; right: 0; bottom: 0;
      background: rgba(0,0,0,0.85); z-index: 10000;
      justify-content: center; align-items: center;
      font-family: -apple-system, sans-serif;
    }
    .exit-box {
      background: #1e1e2e; padding: 2rem; border-radius: 12px;
      width: 360px; text-align: center; color: #fff;
    }
    .exit-box h3 { margin-bottom: 1rem; }
    .exit-box p { margin-bottom: 1.5rem; opacity: 0.8; font-size: 0.9rem; }
    .exit-box input {
      width: 100%; padding: 0.75rem; border: 1px solid #444;
      border-radius: 8px; background: #2a2a3e; color: #fff;
      font-size: 1rem; margin-bottom: 1rem; outline: none;
    }
    .exit-box .error { color: #ff6b6b; font-size: 0.85rem; margin-bottom: 1rem; display: none; }
    .exit-box .btn-row { display: flex; gap: 0.5rem; }
    .exit-box button {
      flex: 1; padding: 0.75rem; border: none; border-radius: 8px;
      color: #fff; cursor: pointer; font-size: 1rem;
    }
    .btn-cancel { background: #444; }
    .btn-exit { background: #dc3545; }
  </style>
</head>
<body>
  <!-- Info bar -->
  <div id="info-bar">
    <span id="exam-name">Simple Ujian Browser</span>
    <span style="opacity:0.6;">Ctrl+Shift+Q untuk keluar</span>
  </div>

  <!-- Exam iframe -->
  <iframe id="exam-frame" src="https://simple-ujian.web.app"></iframe>

  <!-- Blocked URL overlay -->
  <div id="blocked-overlay">
    <div>
      <h2>🚫 URL Diblokir</h2>
      <p>Anda hanya dapat mengakses halaman ujian.</p>
    </div>
  </div>

  <!-- Admin exit dialog -->
  <div id="exit-overlay">
    <div class="exit-box">
      <h3>🔑 Keluar Ujian</h3>
      <p>Masukkan password admin untuk keluar</p>
      <input type="password" id="exit-password" placeholder="Password admin">
      <div id="exit-error" class="error">Password salah!</div>
      <div class="btn-row">
        <button class="btn-cancel" onclick="closeExitDialog()">Batal</button>
        <button class="btn-exit" onclick="confirmExit()">Keluar</button>
      </div>
    </div>
  </div>

  <script>
    // === SEB Detection ===
    window.SafeExamBrowser = { version: "1.0.0", custom: true };

    // === Tauri invoke helper ===
    async function invoke(cmd, args) {
      return window.__TAURI__.core.invoke(cmd, args);
    }

    // === Config-driven initialization ===
    let examConfig = null;

    async function initExamBrowser() {
      try {
        examConfig = await invoke('get_config');

        // Update UI from config
        document.getElementById('exam-name').textContent = examConfig.exam_name;
        document.getElementById('exam-frame').src = examConfig.exam_url;

        // Apply frontend restrictions based on config
        if (examConfig.disable_right_click) {
          document.addEventListener('contextmenu', e => e.preventDefault());
        }
        if (examConfig.disable_copy_paste) {
          ['copy', 'paste', 'cut'].forEach(event => {
            document.addEventListener(event, e => e.preventDefault());
          });
        }

        console.log('[SimpleUjianBrowser] Loaded:', examConfig.exam_name);
      } catch (e) {
        console.error('[SimpleUjianBrowser] Config error:', e);
      }
    }

    // === Keyboard shortcuts (frontend level) ===
    document.addEventListener('keydown', (e) => {
      // Block Ctrl+C, Ctrl+V, Ctrl+X, Ctrl+A, Ctrl+P
      if (e.ctrlKey && ['c','v','x','a','p'].includes(e.key.toLowerCase())) {
        e.preventDefault();
      }
      // Block F12 (DevTools)
      if (e.key === 'F12') e.preventDefault();
      // Block Ctrl+Shift+I (DevTools)
      if (e.ctrlKey && e.shiftKey && e.key === 'I') e.preventDefault();

      // Admin exit: Ctrl+Shift+Q
      if (e.ctrlKey && e.shiftKey && e.key === 'Q') {
        e.preventDefault();
        showExitDialog();
      }
      // Escape to close exit dialog
      if (e.key === 'Escape') closeExitDialog();
      // Enter to confirm exit
      if (e.key === 'Enter' && document.getElementById('exit-overlay').style.display === 'flex') {
        confirmExit();
      }
    });

    // === Admin exit dialog ===
    function showExitDialog() {
      document.getElementById('exit-overlay').style.display = 'flex';
      document.getElementById('exit-password').value = '';
      document.getElementById('exit-error').style.display = 'none';
      document.getElementById('exit-password').focus();
    }

    function closeExitDialog() {
      document.getElementById('exit-overlay').style.display = 'none';
    }

    async function confirmExit() {
      const password = document.getElementById('exit-password').value;
      if (examConfig && password === examConfig.admin_password) {
        await invoke('exit_app');
      } else {
        document.getElementById('exit-error').style.display = 'block';
      }
    }

    // === Disable text selection ===
    document.body.style.userSelect = 'none';

    // === Initialize ===
    initExamBrowser();
  </script>
</body>
</html>
```

**Step 2: Add exit_app command to Rust**

In `src-tauri/src/lib.rs`, add:
```rust
#[tauri::command]
fn exit_app() {
    std::process::exit(0);
}
```

And register it:
```rust
.invoke_handler(tauri::generate_handler![get_config, exit_app])
```

**Step 3: Verify**

Run `cargo tauri dev --no-watch`. Should show:
- Info bar with exam name at top
- simple-ujian.web.app loading in iframe
- Right-click disabled
- Ctrl+Shift+Q → exit dialog appears
- Wrong password → error
- Correct password (guru2026) → app closes

**Step 4: Commit**

```bash
git add -A
git commit -m "feat: config-driven frontend with exit dialog and info bar"
```

---

## Task 9: Windows Keyboard Hook (Full Implementation)

**Objective:** Block Alt+Tab, Win key, Ctrl+Esc on Windows via low-level keyboard hook.

**Files:**
- Create: `src-tauri/src/kiosk/mod.rs`
- Create: `src-tauri/src/kiosk/windows.rs`
- Modify: `src-tauri/Cargo.toml` (add windows crate)
- Modify: `src-tauri/src/lib.rs`

**Step 1: Add Windows dependency to Cargo.toml**

```toml
[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.58", features = [
    "Win32_UI_WindowsHooks",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_System_LibraryLoader",
    "Win32_System_Threading"
] }
```

**Step 2: Create kiosk module structure**

Create `src-tauri/src/kiosk/mod.rs`:
```rust
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
```

Create `src-tauri/src/kiosk/windows.rs`:
```rust
use std::ptr::null_mut;
use windows::Win32::Foundation::{LRESULT, WPARAM, LPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsHooks::{SetWindowsHookExW, UnhookWindowsHookEx, HOOKPROC, WH_KEYBOARD_LL};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    KBDLLHOOKSTRUCT, VK_TAB, VK_LWIN, VK_RWIN, VK_ESCAPE, VK_F4,
    GetMessageW, MSG,
};

// Blocked key combinations:
// - Alt+Tab: VK_TAB with ALT flag (bit 5 of KBDLLHOOKSTRUCT.flags for ALT, or check msg)
// - Win key: VK_LWIN, VK_RWIN
// - Alt+F4: VK_F4 with ALT
// - Ctrl+Esc: VK_ESCAPE with CTRL

const LLKHF_ALTDOWN: u32 = 0x20;

static mut HOOK_HANDLE: *mut std::ffi::c_void = null_mut();

unsafe extern "system" fn keyboard_hook_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code >= 0 {
        let kb_struct = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
        let vk_code = kb_struct.vkCode;
        let flags = kb_struct.flags;
        let alt_down = (flags & LLKHF_ALTDOWN) != 0;

        let block = match vk_code {
            // Alt+Tab
            _ if vk_code == VK_TAB.0 as u32 && alt_down => true,
            // Win keys
            _ if vk_code == VK_LWIN.0 as u32 || vk_code == VK_RWIN.0 as u32 => true,
            // Alt+F4
            _ if vk_code == VK_F4.0 as u32 && alt_down => true,
            // Ctrl+Esc
            _ if vk_code == VK_ESCAPE.0 as u32 && is_ctrl_pressed() => true,
            _ => false,
        };

        if block {
            return LRESULT(1); // Swallow the keystroke
        }
    }

    // Pass to next hook
    windows::Win32::UI::WindowsHooks::CallNextHookEx(None, code, wparam, lparam)
}

fn is_ctrl_pressed() -> bool {
    use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
    const VK_CONTROL: i32 = 0x11;
    unsafe { (GetAsyncKeyState(VK_CONTROL) & 0x8000u16 as i16) != 0 }
}

pub fn enable_keyboard_hook() {
    unsafe {
        let h_module = GetModuleHandleW(None).unwrap();
        let hook_proc: HOOKPROC = Some(keyboard_hook_proc);

        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            hook_proc,
            h_module,
            0,
        );

        match hook {
            Ok(h) => {
                HOOK_HANDLE = h.0 as *mut std::ffi::c_void;
                println!("[Kiosk] Keyboard hook installed successfully");

                // Message loop to keep hook alive
                let mut msg = MSG::default();
                while GetMessageW(&mut msg, None, 0, 0).into() {
                    // Keep processing messages
                }
            }
            Err(e) => {
                eprintln!("[Kiosk] Failed to install keyboard hook: {}", e);
            }
        }
    }
}

pub fn disable_keyboard_hook() {
    unsafe {
        if !HOOK_HANDLE.is_null() {
            UnhookWindowsHookEx(windows::Win32::Foundation::HANDLE(HOOK_HANDLE as _));
            HOOK_HANDLE = null_mut();
            println!("[Kiosk] Keyboard hook removed");
        }
    }
}
```

**Step 3: Wire into lib.rs**

```rust
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
```

**Step 4: Verify compilation on macOS**

Run `cargo tauri dev --no-watch` on macOS.
Expected: Compiles fine. Prints: `[Kiosk] Skipping lockdown (not Windows — use SEB on macOS/iOS)`
Windows code is `#[cfg(target_os = "windows")]` gated — won't compile on macOS.

**Step 5: Commit**

```bash
git add -A
git commit -m "feat: Windows keyboard hook — block Alt+Tab, Win key, Ctrl+Esc"
```

---

## Task 10: GitHub Actions CI/CD (Build Windows .msi + .exe)

**Objective:** Automate Windows builds via GitHub Actions so you can develop on macOS and get Windows installers.

**Files:**
- Create: `.github/workflows/build.yml`

**Step 1: Create GitHub repo and push**

```bash
cd ~/project/ujian/simple-ujian-browser
gh repo create simple-ujian-browser --private --source . --push
```

**Step 2: Create CI/CD workflow**

Create `.github/workflows/build.yml`:
```yaml
name: Build Simple Ujian Browser

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Rust cache
        uses: Swatinem/rust-cache@v2
        with:
          workspaces: src-tauri -> target

      - name: Install dependencies
        run: npm install

      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: simple-ujian-browser-windows
          path: |
            src-tauri/target/release/bundle/msi/*.msi
            src-tauri/target/release/bundle/nsis/*.exe

  build-macos:
    runs-on: macos-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Rust cache
        uses: Swatinem/rust-cache@v2
        with:
          workspaces: src-tauri -> target

      - name: Install dependencies
        run: npm install

      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: simple-ujian-browser-macos
          path: src-tauri/target/release/bundle/dmg/*.dmg
```

**Step 3: Test workflow**

Push a tag to trigger:
```bash
git tag v0.1.0
git push origin v0.1.0
```

Go to GitHub → Actions tab → watch the build.

**Step 4: Download artifacts**

After build succeeds:
- Go to Actions → latest run → Artifacts
- Download `simple-ujian-browser-windows` → contains .msi and .exe
- Download `simple-ujian-browser-macos` → contains .dmg

**Step 5: Commit**

```bash
git add -A
git commit -m "ci: add GitHub Actions for Windows + macOS builds"
git push
```

---

## Task 11: Copy Config to Build Output

**Objective:** Ensure exam-config.json is bundled with the app.

**Files:**
- Modify: `src-tauri/tauri.conf.json` (add resources)
- Create: `src-tauri/resources/config.json`

**Step 1: Copy config to Tauri resources**

Copy `config/exam-config.json` to `src-tauri/resources/config.json`.

**Step 2: Add resources to tauri.conf.json**

```json
{
  "bundle": {
    "resources": ["resources/*"]
  }
}
```

**Step 3: Update config.rs to find bundled config**

```rust
fn config_path() -> PathBuf {
    // First check next to executable (for development/manual override)
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    let exe_config = exe_dir.join("config.json");
    if exe_config.exists() {
        return exe_config;
    }

    // Then check in Tauri resource directory
    if let Ok(resource_dir) = std::env::var("TAURI_RESOURCE_DIR") {
        let resource_config = PathBuf::from(resource_dir).join("config.json");
        if resource_config.exists() {
            return resource_config;
        }
    }

    // Fallback
    exe_config
}
```

**Step 4: Verify**

Run `cargo tauri dev --no-watch`. Config should load from `config/` directory.

**Step 5: Commit**

```bash
git add -A
git commit -m "feat: bundle config.json with build output"
```

---

## Task 12: Final Cleanup & Documentation

**Objective:** Polish project, update README, verify everything works.

**Files:**
- Modify: `README.md`
- Modify: `.gitignore`

**Step 1: Update .gitignore**

```
target/
node_modules/
.DS_Store
*.dmg
*.msi
*.exe
src-tauri/resources/config.json
```

**Step 2: Update README.md**

```markdown
# Simple Ujian Browser

Custom exam browser untuk Windows — bagian dari ekosistem ujian digital.

## Strategi Hybrid

| Platform | Solusi |
|----------|--------|
| macOS/iPad | Safe Exam Browser (SEB) — install dari App Store |
| **Windows** | **Simple Ujian Browser — this app** |
| **Android** | **Simple Ujian Browser — planned** |

## Download

Download .msi atau .exe dari [GitHub Releases](../../releases).

## Config

Edit `config.json` yang terinstall bersama app:
- `exam_url` — URL ujian
- `whitelist` — allowed URLs
- `admin_password` — password untuk keluar (Ctrl+Shift+Q)

## Development

```bash
# Prerequisites
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install tauri-cli

# Run dev mode (macOS — for UI testing)
cargo tauri dev

# Build via CI/CD
git tag v0.x.x && git push origin v0.x.x
# → GitHub Actions builds .msi + .exe
```

## Exit

**Ctrl+Shift+Q** → enter admin password → app closes.

## Related Projects

- [simple-ujian](../simple-ujian/) — Website ujian online
- [grader-mtk](../grader-mtk/) — Sistem grading
- [web-ujian-mandiri](../web-ujian-mandiri/) — Ujian mandiri
```

**Step 3: Final commit & push**

```bash
git add -A
git commit -m "docs: final cleanup and README"
git push
```

---

## Summary: Task Order

```
Task 1:  Install Rust              ← START HERE
Task 2:  Install Tauri CLI
Task 3:  Scaffold project
Task 4:  Window config (fullscreen)
Task 5:  Custom user agent (SEB)
Task 6:  Load simple-ujian
Task 7:  Config reader (Rust)
Task 8:  Config + frontend + exit dialog
Task 9:  Windows keyboard hook
Task 10: GitHub Actions CI/CD
Task 11: Bundle config with build
Task 12: Cleanup & docs
```

## Verification Checklist

- [ ] `cargo tauri dev` opens fullscreen window
- [ ] simple-ujian.web.app loads (no SEB warning)
- [ ] Right-click disabled
- [ ] Copy/paste disabled
- [ ] Info bar shows exam name
- [ ] Ctrl+Shift+Q opens exit dialog
- [ ] Wrong password → error
- [ ] Correct password → app closes
- [ ] Config.json is read correctly
- [ ] GitHub Actions produces .msi + .exe artifacts
- [ ] Windows: Alt+Tab, Win key, Ctrl+Esc blocked
