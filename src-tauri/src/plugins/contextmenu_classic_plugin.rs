use crate::core::{Logger, Plugin, PluginApi, PluginMeta, SettingField, SettingKind, SettingUi};
use serde::Deserialize;
use serde_json::json;

pub fn plugin() -> Box<dyn Plugin> {
    Box::new(ClassicContextMenuPlugin)
}

struct ClassicContextMenuPlugin;

#[derive(Deserialize)]
struct ClassicContextSettings {
    #[serde(default = "default_use_classic")]
    use_classic: bool,
}

fn default_use_classic() -> bool {
    false
}

impl Plugin for ClassicContextMenuPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            id: "contextmenu_classic".to_string(),
            name: "Классическое контекстное меню".to_string(),
            description: "Переключатель классического контекстного меню с Windows 10 на 10.".to_string(),
            category: "Визуал".to_string(),
            settings: vec![
                SettingField {
                    key: "use_classic".to_string(),
                    label: "Классическое меню Windows 10".to_string(),
                    kind: SettingKind::Boolean,
                    description: Some("Создаёт или удаляет ключ реестра, который включает классическое контекстное меню в Windows 11.".to_string()),
                    required: true,
                    default_value: json!(default_use_classic()),
                    options: None,
                    ui: Some(SettingUi {
                        placeholder: None,
                        ..Default::default()
                    }),
                },
            ],
        }
    }

    fn defaults(&self, api: &PluginApi) -> Option<serde_json::Value> {
        let inproc_path = "Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32";

        match api.registry_key_exists(inproc_path) {
            Ok(true) => Some(json!({"use_classic": true})),
            Ok(false) => Some(json!({"use_classic": false})),
            Err(_) => Some(json!({"use_classic": false})),
        }
    }

    fn run(
        &self,
        api: &PluginApi,
        settings: &serde_json::Value,
        logger: &mut Logger,
    ) -> Result<(), String> {
        let settings: ClassicContextSettings = serde_json::from_value(settings.clone()).unwrap_or(ClassicContextSettings {
            use_classic: default_use_classic(),
        });

        let clsid_base = "Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}";
        let inproc_path = "Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32";

        if settings.use_classic {
            match api.registry_key_exists(inproc_path) {
                Ok(true) => {
                    logger.info("Классическое меню уже включено.".to_string());
                    return Ok(());
                }
                Ok(false) => {
                    api.create_registry_key(inproc_path)?;
                    api.set_registry_string(inproc_path, "", "")?;
                    logger.info("Классическое меню включено (создан реестр).".to_string());
                    match api.restart_explorer() {
                        Ok(_) => logger.success("Explorer перезапущен, изменения применились.".to_string()),
                        Err(e) => logger.info(format!("Не удалось перезапустить Explorer: {e}. Перезапустите его вручную или выйдите из системы.")),
                    }
                    return Ok(());
                }
                Err(e) => return Err(format!("Ошибка проверки реестра: {e}")),
            }
        } else {
            match api.registry_key_exists(clsid_base) {
                Ok(true) => {
                    api.delete_registry_key(clsid_base)?;
                    logger.info("Классическое меню отключено (удален ключ реестра).".to_string());
                    match api.restart_explorer() {
                        Ok(_) => logger.success("Explorer перезапущен, изменения применились.".to_string()),
                        Err(e) => logger.info(format!("Не удалось перезапустить Explorer: {e}. Перезапустите его вручную или выйдите из системы.")),
                    }
                    return Ok(());
                }
                Ok(false) => {
                    logger.info("Классическое меню уже отключено.".to_string());
                    return Ok(());
                }
                Err(e) => return Err(format!("Ошибка проверки реестра: {e}")),
            }
        }
    }
}
