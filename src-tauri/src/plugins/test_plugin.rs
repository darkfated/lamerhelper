use crate::core::{
    Logger, Plugin, PluginApi, PluginMeta, SettingField, SettingKind, SettingOption, SettingUi,
};
use serde::Deserialize;
use serde_json::json;

pub fn plugin() -> Box<dyn Plugin> {
    Box::new(TestPlugin)
}

struct TestPlugin;

#[derive(Deserialize, Default)]
struct TestSettings {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    username: String,
    #[serde(default)]
    notes: String,
    #[serde(default)]
    refresh_rate: i32,
    #[serde(default)]
    accent: String,
    #[serde(default)]
    mode: String,
}

impl Plugin for TestPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            id: "test_plugin".to_string(),
            name: "Тестовый плагин".to_string(),
            description: "Проверяет все типы настроек UI.".to_string(),
            category: "Другое".to_string(),
            settings: vec![
                SettingField {
                    key: "enabled".to_string(),
                    label: "Включить".to_string(),
                    kind: SettingKind::Boolean,
                    description: Some("Проверка переключателя.".to_string()),
                    required: false,
                    default_value: json!(true),
                    options: None,
                    ui: None,
                },
                SettingField {
                    key: "username".to_string(),
                    label: "Имя пользователя".to_string(),
                    kind: SettingKind::Text,
                    description: Some("Обычное текстовое поле.".to_string()),
                    required: true,
                    default_value: json!("darkfated"),
                    options: None,
                    ui: Some(SettingUi {
                        placeholder: Some("Введите имя".to_string()),
                        ..Default::default()
                    }),
                },
                SettingField {
                    key: "notes".to_string(),
                    label: "Заметки".to_string(),
                    kind: SettingKind::Text,
                    description: Some("Многострочный текст.".to_string()),
                    required: false,
                    default_value: json!("Например: список задач, пояснения..."),
                    options: None,
                    ui: Some(SettingUi {
                        rows: Some(4),
                        placeholder: Some("Введите заметки".to_string()),
                        ..Default::default()
                    }),
                },
                SettingField {
                    key: "refresh_rate".to_string(),
                    label: "Частота обновления".to_string(),
                    kind: SettingKind::Number,
                    description: Some("Числовое поле с min/max/step.".to_string()),
                    required: true,
                    default_value: json!(60),
                    options: None,
                    ui: Some(SettingUi {
                        min: Some(1.0),
                        max: Some(240.0),
                        step: Some(1.0),
                        unit: Some("Hz".to_string()),
                        ..Default::default()
                    }),
                },
                SettingField {
                    key: "accent".to_string(),
                    label: "Цвет акцента".to_string(),
                    kind: SettingKind::Color,
                    description: Some("HEX цвет (#RRGGBB).".to_string()),
                    required: true,
                    default_value: json!("#7ae2ff"),
                    options: None,
                    ui: Some(SettingUi {
                        placeholder: Some("#7ae2ff".to_string()),
                        ..Default::default()
                    }),
                },
                SettingField {
                    key: "mode".to_string(),
                    label: "Режим".to_string(),
                    kind: SettingKind::Select,
                    description: Some("Выпадающий список вариантов.".to_string()),
                    required: true,
                    default_value: json!("balanced"),
                    options: Some(vec![
                        SettingOption {
                            label: "Экономичный".to_string(),
                            value: json!("eco"),
                        },
                        SettingOption {
                            label: "Сбалансированный".to_string(),
                            value: json!("balanced"),
                        },
                        SettingOption {
                            label: "Производительность".to_string(),
                            value: json!("performance"),
                        },
                    ]),
                    ui: None,
                },
            ],
        }
    }

    fn run(
        &self,
        _api: &PluginApi,
        settings: &serde_json::Value,
        logger: &mut Logger,
    ) -> Result<(), String> {
        let settings: TestSettings = serde_json::from_value(settings.clone()).unwrap_or_default();
        logger.info(format!("enabled: {}", settings.enabled));
        logger.info(format!("username: {}", settings.username));
        logger.info(format!("notes: {}", settings.notes));
        logger.info(format!("refresh_rate: {}", settings.refresh_rate));
        logger.info(format!("accent: {}", settings.accent));
        logger.info(format!("mode: {}", settings.mode));
        logger.success("Тест завершён.".to_string());
        Ok(())
    }
}
