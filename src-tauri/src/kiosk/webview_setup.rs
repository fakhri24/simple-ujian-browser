// WebView2-specific kiosk enhancements
// PLACEHOLDER: WebView2 COM API integration needs Tauri 2 raw handle access
// Currently blocked by Tauri 2 API limitations (no public with_webview/COM access)
//
// TODO: When Tauri 2 exposes raw WebView2 handle, implement:
// 1. SetAreDefaultContextMenusEnabled(false) — disable native right-click
// 2. AddScriptToExecuteOnDocumentCreated — inject JS into ALL frames (including cross-origin iframes)
//
// For now, the JS restrictions in index.html apply to the outer document only.
// The keyboard hook (windows.rs) provides OS-level blocking.

/// JavaScript that SHOULD be injected into all frames via WebView2's
/// AddScriptToExecuteOnDocumentCreated. Kept here as reference for future implementation.
#[allow(dead_code)]
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

/// WebView2 COM API code reference for future implementation:
///
/// ```rust,ignore
/// use windows::Win32::Web::WebView2::*;
///
/// webview.with_webview(|w| {
///     unsafe {
///         let core = w.controller().CoreWebView2().unwrap();
///         let settings = core.Settings().unwrap();
///         settings.SetAreDefaultContextMenusEnabled(false).unwrap();
///         core.AddScriptToExecuteOnDocumentCreated(IFRAME_KIOSK_JS, None).unwrap();
///     }
/// });
/// ```
///
/// Required Cargo.toml features:
/// - tauri = { features = ["webview2-com"] }
/// - windows = { features = ["Win32_Web_WebView2"] }
pub fn _placeholder() {
    // This function exists to keep the module available for future use
}
