use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExamConfig {
    pub exam_url: String,
    pub exam_name: String,
    pub whitelist: Vec<String>,
    pub fullscreen: bool,
    pub block_shortcuts: bool,
    pub block_task_switching: bool,
    pub disable_right_click: bool,
    pub disable_copy_paste: bool,
    // SHA-256 hex of the admin password. Read from config.json (deserialize) but
    // NEVER serialized back to the frontend, so the secret never leaves Rust.
    // Generate with: printf '%s' 'your-password' | sha256sum
    #[serde(default, skip_serializing)]
    pub admin_password_hash: String,
}

impl ExamConfig {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        match config_path {
            Some(path) => {
                println!("[Config] Loading config from: {}", path.display());
                let content = match fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!(
                            "[Config] Failed to read {}: {} — using defaults",
                            path.display(),
                            e
                        );
                        return Self::default_config();
                    }
                };
                match serde_json::from_str(&content) {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        eprintln!("[Config] Failed to parse config JSON: {} — using defaults", e);
                        Self::default_config()
                    }
                }
            }
            None => {
                println!("[Config] No config.json found, using defaults");
                Self::default_config()
            }
        }
    }

    /// Verify a plaintext password against the stored SHA-256 hash.
    /// Comparison is constant-time to avoid leaking the hash via timing.
    pub fn verify_password(&self, input: &str) -> bool {
        if self.admin_password_hash.is_empty() {
            return false;
        }
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let digest = hasher.finalize();
        let mut hex = String::with_capacity(digest.len() * 2);
        for byte in digest {
            hex.push_str(&format!("{:02x}", byte));
        }
        constant_time_eq(
            hex.as_bytes(),
            self.admin_password_hash.to_lowercase().as_bytes(),
        )
    }

    fn config_path() -> Option<PathBuf> {
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));

        // Search order: exe_dir first, then resources/ subdirectory
        let candidates = [
            exe_dir.join("config.json"),
            exe_dir.join("resources").join("config.json"),
        ];

        for path in &candidates {
            if path.exists() {
                println!("[Config] Found config at: {}", path.display());
                return Some(path.clone());
            }
            println!("[Config] Not found: {}", path.display());
        }

        None
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
            // SHA-256 of the default password "guru2026". Override in config.json.
            admin_password_hash:
                "8ae731714ca5770a7b2f2c88f6e9e444e116aa61caa9e3874e04f4406c9d62ef".to_string(),
        }
    }

    pub fn is_url_allowed(&self, url: &str) -> bool {
        self.whitelist.iter().any(|pattern| {
            // Each whitelist entry is an origin ("scheme://host"). Build a regex that
            // matches that exact origin optionally followed by a path. `*` matches a
            // host segment only ([^/]*, never crossing a slash) so that entries like
            // "https://*.googleapis.com" cannot be bypassed by
            // "https://evil.com/?x=.googleapis.com" or "https://host.googleapis.com.evil.com".
            let escaped = pattern.replace('.', "\\.").replace('*', "[^/]*");
            regex::Regex::new(&format!("^{}(/.*)?$", escaped))
                .map(|re| re.is_match(url))
                .unwrap_or(false)
        })
    }
}

/// Length-checked, constant-time byte comparison (no early exit on mismatch).
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_password_verifies() {
        let cfg = ExamConfig::default_config();
        assert!(cfg.verify_password("guru2026"));
        assert!(!cfg.verify_password("wrong"));
        assert!(!cfg.verify_password(""));
    }

    #[test]
    fn empty_hash_rejects_everything() {
        let mut cfg = ExamConfig::default_config();
        cfg.admin_password_hash = String::new();
        assert!(!cfg.verify_password("guru2026"));
    }

    #[test]
    fn whitelist_wildcard_matches() {
        let cfg = ExamConfig::default_config();
        // exact-prefix entries
        assert!(cfg.is_url_allowed("https://simple-ujian.web.app/exam/1"));
        assert!(cfg.is_url_allowed("https://accounts.google.com/o/oauth2"));
        // wildcard entries (regex escaping fix)
        assert!(cfg.is_url_allowed("https://storage.googleapis.com/x"));
        assert!(cfg.is_url_allowed("https://myproj.firebaseapp.com/"));
        // origin with no path is allowed
        assert!(cfg.is_url_allowed("https://www.gstatic.com"));
        // not whitelisted
        assert!(!cfg.is_url_allowed("https://evil.com/"));
        // suffix-style bypasses must be rejected
        assert!(!cfg.is_url_allowed("https://googleapis.com.evil.com/"));
        assert!(!cfg.is_url_allowed("https://simple-ujian.web.app.evil.com/"));
        assert!(!cfg.is_url_allowed("https://evil.com/?x=.googleapis.com"));
    }
}
