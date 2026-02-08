use std::{
    fs,
    path::{Component, Path, PathBuf},
};
use tauri::{AppHandle, Manager};

#[allow(dead_code)]
pub struct PluginApi {
    app: AppHandle,
}

impl PluginApi {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    #[allow(dead_code)]
    pub fn app_data_dir(&self) -> Result<PathBuf, String> {
        self.app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to resolve app data dir: {e}"))
    }

    pub fn temp_dir(&self) -> PathBuf {
        std::env::temp_dir()
    }

    #[allow(dead_code)]
    pub fn create_dir_all(&self, path: &Path) -> Result<(), String> {
        fs::create_dir_all(path).map_err(|e| format!("Create dir failed: {e}"))
    }

    pub fn remove_file(&self, path: &Path) -> Result<(), String> {
        fs::remove_file(path).map_err(|e| format!("Remove file failed: {e}"))
    }

    pub fn remove_dir_all(&self, path: &Path) -> Result<(), String> {
        fs::remove_dir_all(path).map_err(|e| format!("Remove dir failed: {e}"))
    }

    #[cfg(windows)]
    pub fn get_registry_string(&self, key_path: &str, name: &str) -> Result<String, String> {
        use winreg::RegKey;
        use winreg::enums::{HKEY_CURRENT_USER, KEY_QUERY_VALUE};

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let key = hkcu
            .open_subkey_with_flags(key_path, KEY_QUERY_VALUE)
            .map_err(|e| format!("Registry open failed: {e}"))?;
        let value: String = key
            .get_value(name)
            .map_err(|e| format!("Registry read failed: {e}"))?;
        Ok(value)
    }

    #[cfg(not(windows))]
    pub fn get_registry_string(&self, _key_path: &str, _name: &str) -> Result<String, String> {
        Err("Registry доступен только на Windows.".to_string())
    }

    #[cfg(windows)]
    pub fn restart_explorer(&self) -> Result<(), String> {
        use std::process::Command;
        
        let status = Command::new("taskkill")
            .args(["/F", "/IM", "explorer.exe"])
            .status()
            .map_err(|e| format!("Failed to stop Explorer: {e}"))?;

        if !status.success() {
            return Err(format!("Failed to stop Explorer (code: {status})"));
        }

        Command::new("explorer.exe")
            .spawn()
            .map_err(|e| format!("Failed to start Explorer: {e}"))?;

        Ok(())
    }

    #[cfg(not(windows))]
    pub fn restart_explorer(&self) -> Result<(), String> {
        Err("Перезапуск Explorer доступен только на Windows.".to_string())
    }

    #[cfg(windows)]
    pub fn set_registry_string(
        &self,
        key_path: &str,
        name: &str,
        value: &str,
    ) -> Result<(), String> {
        use winreg::RegKey;
        use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let key = hkcu
            .open_subkey_with_flags(key_path, KEY_SET_VALUE)
            .map_err(|e| format!("Registry open failed: {e}"))?;
        key.set_value(name, &value)
            .map_err(|e| format!("Registry write failed: {e}"))?;
        Ok(())
    }

    #[cfg(not(windows))]
    pub fn set_registry_string(
        &self,
        _key_path: &str,
        _name: &str,
        _value: &str,
    ) -> Result<(), String> {
        Err("Registry доступен только на Windows.".to_string())
    }
}

pub fn short_path(path: &Path, max_segments: usize) -> String {
    let mut parts: Vec<String> = Vec::new();
    for component in path.components() {
        if let Component::Normal(part) = component {
            parts.push(part.to_string_lossy().to_string());
        }
    }

    if parts.is_empty() {
        return path.display().to_string();
    }

    if parts.len() <= max_segments {
        return parts.join("\\");
    }

    let start = parts.len().saturating_sub(max_segments);
    format!("…\\{}", parts[start..].join("\\"))
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let bytes_f = bytes as f64;
    if bytes_f >= TB {
        format!("{:.2} TB", bytes_f / TB)
    } else if bytes_f >= GB {
        format!("{:.2} GB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.2} MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.2} KB", bytes_f / KB)
    } else {
        format!("{bytes} B")
    }
}
