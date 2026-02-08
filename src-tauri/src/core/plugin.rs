use serde::Serialize;
use serde_json::Value;
use tauri::AppHandle;

use crate::core::api::PluginApi;
use crate::core::logger::{Logger, RunResult};
use crate::core::registry::all_plugins;
use crate::core::settings::{
    PluginInfo, PluginMeta, defaults_from_fields, merge_settings, validate_settings,
};

pub trait Plugin: Send + Sync {
    fn meta(&self) -> PluginMeta;
    fn defaults(&self, _api: &PluginApi) -> Option<Value> {
        None
    }
    fn preview(&self, _api: &PluginApi) -> Result<Option<PluginPreview>, String> {
        Ok(None)
    }
    fn run(&self, api: &PluginApi, settings: &Value, logger: &mut Logger) -> Result<(), String>;
}

pub fn list_plugins(app: AppHandle) -> Vec<PluginInfo> {
    let api = PluginApi::new(app);
    let mut plugins: Vec<PluginInfo> = all_plugins()
        .into_iter()
        .map(|plugin| {
            let meta = plugin.meta();
            let mut info = PluginInfo::from_meta(meta);
            if let Some(defaults) = plugin.defaults(&api) {
                info.defaults = merge_settings(&defaults, &info.defaults);
            }
            info
        })
        .collect();
    plugins.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    plugins
}

#[derive(Serialize, Clone)]
pub struct PluginPreview {
    pub title: String,
    pub value: String,
    pub note: Option<String>,
}

pub fn preview_plugin(app: AppHandle, id: &str) -> Result<Option<PluginPreview>, String> {
    let plugin = all_plugins()
        .into_iter()
        .find(|plugin| plugin.meta().id == id)
        .ok_or_else(|| format!("Плагин не найден: {id}"))?;
    let api = PluginApi::new(app);
    plugin.preview(&api)
}

pub fn run_plugin(app: AppHandle, id: &str, settings: Value) -> Result<RunResult, String> {
    let plugin = all_plugins()
        .into_iter()
        .find(|plugin| plugin.meta().id == id)
        .ok_or_else(|| format!("Плагин не найден: {id}"))?;

    let meta = plugin.meta();
    let api = PluginApi::new(app);
    let mut defaults = defaults_from_fields(&meta.settings);
    if let Some(extra_defaults) = plugin.defaults(&api) {
        defaults = merge_settings(&extra_defaults, &defaults);
    }
    let merged_settings = merge_settings(&settings, &defaults);

    let mut logger = Logger::new();
    logger.section(format!("Плагин: {}", meta.name));
    if !meta.description.is_empty() {
        logger.info(meta.description.clone());
    }

    log_settings(&mut logger, &meta, &merged_settings);

    let validation_errors = validate_settings(&merged_settings, &meta.settings);
    if !validation_errors.is_empty() {
        for err in validation_errors {
            logger.error(err);
        }
        return Ok(RunResult {
            ok: false,
            message: "Некорректные настройки.".to_string(),
            logs: logger.into_logs(),
        });
    }

    logger.info("Запуск...".to_string());

    let result = plugin.run(&api, &merged_settings, &mut logger);

    let (ok, message) = match result {
        Ok(()) => {
            logger.success("Завершено успешно.".to_string());
            (true, "Готово.".to_string())
        }
        Err(err) => {
            logger.error(err.clone());
            (false, err)
        }
    };

    Ok(RunResult {
        ok,
        message,
        logs: logger.into_logs(),
    })
}

fn log_settings(logger: &mut Logger, meta: &PluginMeta, settings: &Value) {
    let values = match settings.as_object() {
        Some(values) => values,
        None => {
            logger.warn("Настройки не являются объектом.".to_string());
            return;
        }
    };

    logger.group("Выбранные настройки:", |logger| {
        for field in &meta.settings {
            let value = values.get(&field.key).unwrap_or(&Value::Null);
            logger.kv(field.label.clone(), format_value(value));
        }
    });
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Null => "не задано".to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => value.clone(),
        Value::Array(_) | Value::Object(_) => {
            serde_json::to_string_pretty(value).unwrap_or_else(|_| "[complex]".to_string())
        }
    }
}
