// Kiosk JavaScript injected into EVERY frame of the webview.
//
// This is passed to `WebviewWindowBuilder::initialization_script`, which on
// Windows/WebView2 maps to `AddScriptToExecuteOnDocumentCreated` — so the script
// runs on the top document AND every child frame, including the cross-origin exam
// iframe. That closes the gap where document-level JS in index.html could not
// reach the exam content. OS-level keys are still handled by the keyboard hook
// (windows.rs); this layer covers in-page actions (copy/paste/right-click/devtools).

pub const KIOSK_SCRIPT: &str = r#"
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
    var key = (e.key || '').toLowerCase();

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
