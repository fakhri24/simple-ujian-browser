# Simple Exam Browser — Architecture & Implementation Guide
## Hybrid Approach: SEB (Apple) + Custom Tauri (Windows/Android)

> **Project:** ~/project/ujian/simple-ujian-browser/
> **Parent:** ~/project/ujian/ (alongside simple-ujian, grader-mtk, web-ujian-mandiri)
> **Website:** https://simple-ujian.web.app/
> **SEB Reference:** [github.com/SafeExamBrowser/seb-mac](https://github.com/SafeExamBrowser/seb-mac) (MPL license)

---

## 1. STRATEGI: HYBRID APPROACH

```
┌─────────────────────────────────────────────────────────┐
│                    simple-ujian.web.app                  │
│              (satu website, semua platform)              │
└────────────┬──────────────────┬─────────────────────────┘
             │                  │
    ┌────────▼────────┐  ┌─────▼──────────────┐
    │   Safe Exam     │  │  SimpleExamBrowser  │
    │   Browser (SEB) │  │  (Tauri + Rust)     │
    │                 │  │                     │
    │  • macOS ✅     │  │  • Windows ✅       │
    │  • iOS/iPad ✅  │  │  • Android ✅       │
    │                 │  │                     │
    │  (proven, 10+   │  │  (custom, belajar   │
    │   years, ETH    │  │   Rust + Tauri)     │
    │   Zurich)       │  │                     │
    └─────────────────┘  └─────────────────────┘
```

### Kenapa Hybrid?

| Platform | Solusi | Alasan |
|----------|--------|--------|
| macOS | SEB | Lockdown Apple sulit (CGEventTap, Accessibility). SEB sudah solve 10+ tahun. |
| iOS/iPad | SEB | Butuh ASAM + entitlement Apple. Tidak bisa dari app pihak ketiga tanpa MDM. |
| Windows | Custom Tauri | SEB Windows pakai Gecko (100MB+). Tauri ringan (5-10MB). Full control. Belajar Rust. |
| Android | Custom Tauri 2 | SEB tidak ada di Android. Tauri 2 support Android. Kiosk mode via Screen Pinning. |

### Sweet Spot yang Didapat:
- ✅ macOS/iOS lockdown → SEB (proven, zero effort)
- ✅ Windows lockdown → Custom Tauri (belajar Rust, full control)
- ✅ Android lockdown → Custom Tauri 2 (belajar mobile)
- ✅ simple-ujian → satu website, support semua browser
- ✅ Detection → user agent check, kompatibel dengan semua approach

---

## 2. DETECTION: simple-ujian Compatibility

simple-ujian mendeteksi browser yang diizinkan dengan:

```javascript
isSEB = !!(window.SafeExamBrowser
         || navigator.userAgent.includes("SEB")
         || navigator.userAgent.includes("SafeExamBrowser"))
```

**Artinya:**
- SEB official → otomatis terdeteksi (user agent mengandung "SEB")
- Custom Tauri app → tinggal set custom user agent:
  ```
  "Mozilla/5.0 SimpleExamBrowser/1.0 SEB"
  ```
- Atau inject `window.SafeExamBrowser = { version: "1.0" }` dari frontend JS

**URL yang perlu di-whitelist:**
```
https://simple-ujian.web.app/*
https://*.googleapis.com/*     (Firebase auth)
https://*.firebaseapp.com/*    (Firebase hosting)
https://www.gstatic.com/*      (Firebase SDK)
```

---

## 3. CONFIG FORMAT (config.json)

Untuk custom Tauri app (Windows + Android):

```json
{
  "exam_url": "https://simple-ujian.web.app",
  "whitelist": [
    "https://simple-ujian.web.app",
    "https://*.googleapis.com",
    "https://*.firebaseapp.com",
    "https://www.gstatic.com"
  ],
  "block_shortcuts": true,
  "block_task_switching": true,
  "block_notifications": true,
  "fullscreen": true,
  "disable_right_click": true,
  "disable_copy_paste": true,
  "admin_password": "guru2026",
  "exam_name": "UAS Semester Genap 2026"
}
```

Untuk SEB (macOS/iiOS) → pakai file .seb yang sama dari simple-ujian.web.app.

---

## 4. TRANSLATION: SEB Features → Tauri/Rust (Windows Focus)

### 4.1 Fullscreen Kiosk

```rust
// tauri.conf.json
{
  "app": {
    "windows": [{
      "fullscreen": true,
      "decorations": false,
      "alwaysOnTop": true,
      "resizable": false,
      "userAgent": "Mozilla/5.0 SimpleExamBrowser/1.0 SEB"
    }]
  }
}
```

### 4.2 Block Keyboard Shortcuts (Windows)

```rust
// src/keyboard/windows.rs
use windows::Win32::UI::WindowsHooks::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

// Blocked shortcuts:
// - Alt+Tab (VK_TAB + ALT)
// - Win key (VK_LWIN, VK_RWIN)
// - Alt+F4 (VK_F4 + ALT)
// - Ctrl+Esc (VK_ESCAPE + CTRL)
// - Ctrl+Shift+Esc (Task Manager)

unsafe extern "system" fn keyboard_hook_proc(
    code: i32, wparam: WPARAM, lparam: LPARAM,
) -> LRESULT {
    if code >= 0 {
        let kb = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
        if is_blocked_shortcut(kb.vkCode, wparam) {
            return LRESULT(1); // Block
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}
```

### 4.3 Block Keyboard Shortcuts (Android)

Android kiosk berbeda — tidak ada keyboard hook.
Pendekatan: Android Screen Pinning + disable back/home button.

```kotlin
// Via Tauri Android native code
// Activity.kt
class MainActivity : TauriActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        startLockTask() // Screen Pinning
    }

    override fun onBackPressed() {
        // Disabled during exam
    }
}
```

### 4.4 URL Navigation Filter

```javascript
// Frontend JS (Vanilla)
document.addEventListener('click', (e) => {
    const link = e.target.closest('a');
    if (link && !isAllowedURL(link.href)) {
        e.preventDefault();
        showBlockedMessage();
    }
});

function isAllowedURL(url) {
    const allowed = [
        'https://simple-ujian.web.app',
        'https://*.googleapis.com',
        'https://*.firebaseapp.com',
        'https://www.gstatic.com'
    ];
    return allowed.some(pattern => {
        const regex = new RegExp(pattern.replace(/\./g, '\\.').replace(/\*/g, '.*'));
        return regex.test(url);
    });
}
```

### 4.5 Disable Right-Click & Copy-Paste

```javascript
// Disable right-click
document.addEventListener('contextmenu', e => e.preventDefault());

// Disable copy/paste/cut
['copy', 'paste', 'cut'].forEach(event => {
    document.addEventListener(event, e => e.preventDefault());
});

// Disable text selection
document.body.style.userSelect = 'none';
```

---

## 5. PROJECT STRUCTURE

```
~/project/ujian/
├── simple-ujian/              ← Website ujian (Firebase, already exists)
├── grader-mtk/                ← Grading system (already exists)
├── web-ujian-mandiri/         ← Ujian mandiri (already exists)
└── simple-ujian-browser/      ← NEW: Custom exam browser
    ├── docs/
    │   └── ARCHITECTURE.md    ← This file (was SEB-TO-TAURI-GUIDE.md)
    ├── src-tauri/             ← (future) Tauri Rust backend
    │   ├── Cargo.toml
    │   ├── tauri.conf.json
    │   └── src/
    │       ├── main.rs
    │       ├── lib.rs
    │       ├── kiosk/
    │       │   ├── mod.rs
    │       │   ├── windows.rs     ← Windows-specific lockdown
    │       │   └── android.rs     ← Android-specific lockdown
    │       ├── config.rs          ← JSON config reader
    │       ├── url_filter.rs      ← Whitelist navigation filter
    │       └── keyboard.rs        ← Keyboard shortcut blocker
    ├── src/                 ← (future) Frontend (Vanilla JS)
    │   ├── index.html
    │   ├── main.js
    │   └── style.css
    ├── config/
    │   └── config.json      ← Default exam config
    ├── README.md
    └── ROADMAP.md           ← Development roadmap
```

---

## 6. RUST CRATES NEEDED

| Crate | Fungsi | Platform |
|---|---|---|
| `tauri` | App framework | All |
| `serde` + `serde_json` | Parse config JSON | All |
| `windows` | Win32 API (keyboard hook, kiosk) | Windows |
| `url` | URL parsing & matching | All |

---

## 7. DEVELOPMENT ROADMAP

### Phase 1: Windows App (Minggu 1-6)
- [ ] Install Rust + Tauri CLI
- [ ] Tauri project setup
- [ ] Load simple-ujian.web.app with custom user agent
- [ ] Fullscreen + no decorations + always on top
- [ ] Config JSON reader (Rust serde)
- [ ] URL whitelist filter
- [ ] Disable right-click, copy-paste (JS)
- [ ] Keyboard hook: block Alt+Tab, Win key, Ctrl+Esc
- [ ] Admin password exit
- [ ] Build .msi installer

### Phase 2: Android App (Minggu 7-10)
- [ ] Tauri 2 Android setup
- [ ] Screen Pinning / kiosk mode
- [ ] Disable back button, home button
- [ ] URL whitelist (same logic as Windows)
- [ ] Build .apk

### Phase 3: Polish & Deploy (Minggu 11-12)
- [ ] Testing di berbagai PC Windows + Android tablet
- [ ] Dokumentasi setup guru (per platform)
- [ ] Distribusi ke sekolah

---

## 8. GUIDE UNTUK GURU (Nanti)

### macOS / iPad:
1. Download Safe Exam Browser dari App Store
2. Buka https://simple-ujian.web.app
3. Klik "Unduh Konfigurasi Ujian (.seb)"
4. Buka file .seb → SEB otomatis terbuka dan load ujian

### Windows:
1. Download SimpleExamBrowser.msi dari [lokasi distribusi]
2. Install seperti aplikasi biasa
3. Buka SimpleExamBrowser → otomatis load ujian

### Android:
1. Download SimpleExamBrowser.apk dari [lokasi distribusi]
2. Install (izinkan "Unknown Sources" jika diminta)
3. Buka SimpleExamBrowser → otomatis load ujian
