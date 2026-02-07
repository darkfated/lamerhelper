use std::{
    fs,
    path::{Path, PathBuf},
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
        use winreg::enums::{HKEY_CURRENT_USER, KEY_QUERY_VALUE};
        use winreg::RegKey;

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
        use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
        use winreg::RegKey;

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
