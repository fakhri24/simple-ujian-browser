use serde::{Deserialize, Serialize};
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
    pub admin_password: String,
}

impl ExamConfig {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        match config_path {
            Some(path) => {
                println!("[Config] Loading config from: {}", path.display());
                let content = fs::read_to_string(&path)
                    .expect("Failed to read config file");
                serde_json::from_str(&content)
                    .expect("Failed to parse config JSON")
            }
            None => {
                println!("[Config] No config.json found, using defaults");
                Self::default_config()
            }
        }
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
            admin_password: "guru2026".to_string(),
        }
    }

    pub fn is_url_allowed(&self, url: &str) -> bool {
        self.whitelist.iter().any(|pattern| {
            if pattern.contains('*') {
                let regex_pattern = pattern
                    .replace('.', "\\\\.")
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
