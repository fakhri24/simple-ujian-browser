// WebView2-specific kiosk enhancements
// Uses Tauri's "unsafe-create" feature to access raw WebView2 COM API
// for things that JS alone cannot do (cross-origin iframe injection, native context menu)

use tauri::Webview;

/// JavaScript injected into ALL frames (including cross-origin iframes)
/// via WebView2's AddScriptToExecuteOnDocumentCreated.
/// Blocks: right-click, copy/paste/cut, F12, DevTools shortcuts, view source, etc.
const IFRAME_KIOSK_JS: &str = r#"
(function() {
  if (window.__kioskInjected) return;
  window.__kioskInjected = true;

  // Block right-click context menu
  document.addEventListener('contextmenu', function(e) {
    e.preventDefault();
    return false;
  }, true);

  // Block copy/paste/cut
  ['copy', 'paste', 'cut'].forEach(function(evt) {
    document.addEventListener(evt, function(e) {
      e.preventDefault();
      return false;
    }, true);
  });

  // Block dangerous keyboard shortcuts
  document.addEventListener('keydown', function(e) {
    var key = e.key.toLowerCase();

    // DevTools
    if (e.key === 'F12') { e.preventDefault(); return false; }
    if (e.ctrlKey && e.shiftKey && key === 'i') { e.preventDefault(); return false; }
    if (e.ctrlKey && e.shiftKey && key === 'j') { e.preventDefault(); return false; }
    if (e.ctrlKey && e.shiftKey && key === 'c') { e.preventDefault(); return false; }

    // Copy/Paste/Cut/Select All/Print
    if (e.ctrlKey && ['c','v','x','a','p'].indexOf(key) !== -1) {
      e.preventDefault(); return false;
    }

    // View source, Save, New tab, Close tab, Reopen tab, Find, Address bar
    if (e.ctrlKey && ['u','s','t','w','l','f'].indexOf(key) !== -1) { e.preventDefault(); return false; }
    if (e.ctrlKey && e.shiftKey && key === 't') { e.preventDefault(); return false; }
    if (e.altKey && key === 'd') { e.preventDefault(); return false; }

    // Refresh
    if (e.key === 'F5') { e.preventDefault(); return false; }
    if (e.ctrlKey && key === 'r') { e.preventDefault(); return false; }
  }, true);

  // Disable text selection
  if (document.body) {
    document.body.style.userSelect = 'none';
    document.body.style.webkitUserSelect = 'none';
  }
})();
"#;

/// Apply WebView2 kiosk restrictions on Windows:
/// 1. Disables native right-click context menu (CoreWebView2Settings)
/// 2. Injects JS into ALL frames including cross-origin iframes
///    (AddScriptToExecuteOnDocumentCreated)
#[cfg(target_os = "windows")]
pub fn apply_webview_kiosk(webview: &Webview) {
    use windows::Win32::Web::WebView2::*;

    let result = webview.with_webview(|w| {
        unsafe {
            // Get CoreWebView2 from the controller
            let core_result = w.controller().CoreWebView2();
            match core_result {
                Ok(core) => {
                    // 1. Disable native WebView2 context menu
                    match core.Settings() {
                        Ok(settings) => {
                            match settings.SetAreDefaultContextMenusEnabled(false) {
                                Ok(_) => println!("[Kiosk] WebView2 native context menu DISABLED"),
                                Err(e) => eprintln!("[Kiosk] Failed to disable context menu: {}", e),
                            }
                        }
                        Err(e) => eprintln!("[Kiosk] Failed to get WebView2 settings: {}", e),
                    }

                    // 2. Inject kiosk JS into ALL frames (including cross-origin iframes)
                    // AddScriptToExecuteOnDocumentCreated runs in EVERY new frame
                    match core.AddScriptToExecuteOnDocumentCreated(IFRAME_KIOSK_JS, None) {
                        Ok(_) => println!("[Kiosk] Kiosk JS injected into ALL frames via AddScriptToExecuteOnDocumentCreated"),
                        Err(e) => eprintln!("[Kiosk] Failed to inject kiosk JS: {}", e),
                    }
                }
                Err(e) => eprintln!("[Kiosk] Failed to get CoreWebView2: {}", e),
            }
        }
    });

    match result {
        Ok(_) => println!("[Kiosk] WebView2 kiosk restrictions applied"),
        Err(e) => eprintln!("[Kiosk] Failed to access WebView2: {}", e),
    }
}

/// No-op on non-Windows platforms
#[cfg(not(target_os = "windows"))]
pub fn apply_webview_kiosk(_webview: &Webview) {
    println!("[Kiosk] WebView2 kiosk skipped (not Windows)");
}
