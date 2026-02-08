use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SettingKind {
    Boolean,
    Number,
    Text,
    Color,
    Select,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SettingOption {
    pub label: String,
    pub value: Value,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SettingUi {
    pub placeholder: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub rows: Option<u32>,
    pub unit: Option<String>,
    pub danger: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SettingField {
    pub key: String,
    pub label: String,
    pub kind: SettingKind,
    pub description: Option<String>,
    pub required: bool,
    #[serde(rename = "default")]
    pub default_value: Value,
    pub options: Option<Vec<SettingOption>>,
    pub ui: Option<SettingUi>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PluginMeta {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub settings: Vec<SettingField>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub settings: Vec<SettingField>,
    pub defaults: Value,
}

impl PluginInfo {
    pub fn from_meta(meta: PluginMeta) -> Self {
        let defaults = defaults_from_fields(&meta.settings);
        Self {
            id: meta.id,
            name: meta.name,
            description: meta.description,
            category: meta.category,
            settings: meta.settings,
            defaults,
        }
    }
}

pub fn defaults_from_fields(fields: &[SettingField]) -> Value {
    let mut map = Map::new();
    for field in fields {
        map.insert(field.key.clone(), field.default_value.clone());
    }
    Value::Object(map)
}

pub fn merge_settings(settings: &Value, defaults: &Value) -> Value {
    match (settings, defaults) {
        (Value::Object(values), Value::Object(defaults_map)) => {
            let mut merged = defaults_map.clone();
            for (key, value) in values {
                merged.insert(key.clone(), value.clone());
            }
            Value::Object(merged)
        }
        _ => defaults.clone(),
    }
}

pub fn validate_settings(settings: &Value, fields: &[SettingField]) -> Vec<String> {
    let mut errors = Vec::new();
    let values = match settings.as_object() {
        Some(map) => map,
        None => {
            errors.push("Настройки должны быть объектом.".to_string());
            return errors;
        }
    };

    for field in fields {
        if !field.required {
            continue;
        }
        match values.get(&field.key) {
            Some(Value::Null) | None => errors.push(format!("Поле '{}' обязательно.", field.label)),
            _ => {}
        }
    }

    errors
}
