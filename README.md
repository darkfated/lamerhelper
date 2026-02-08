# LamerHelper

Простой интерфейс, мощные плагины. Укрась и ускорь свой ПК за пару кликов.

## Стек

- Frontend: Vue 3, Vite
- Backend: Tauri 2, Rust

## Документация

- `yarn install`
- `yarn tauri dev`
- `yarn tauri build`

## Архитектура

1. UI при старте вызывает `list_plugins` и получает список `PluginInfo` с настройками.
2. Пользователь выбирает плагин, а интерфейс берёт дефолтные значения и готовит форму настроек.
3. По нажатию "Применить" вызывается `run_plugin` с `id` и объектом `settings`.
4. Backend формирует task и запускает плагин.
5. Процесс выполнения передаётся на фронт для логирования.

## Система плагинов

### Описание

- Контракт плагина описан в `src-tauri/src/core/plugin.rs` через трейт `Plugin`.
- Регистрация плагинов происходит автоматически в `src-tauri/build.rs`.
- Файлы в `src-tauri/src/plugins/` с суффиксом `_plugin.rs` подхватываются сборкой.
- На этапе сборки генерируется `plugins.generated.rs`, который подключается в `src-tauri/src/core/registry.rs`.

### Как добавить новый плагин

1. Создайте файл `src-tauri/src/plugins/*_plugin.rs`.
2. Опишите `PluginMeta` и настройки.
3. Реализуйте функциональность в `run`.

Шаблон:

```rust
use crate::core::{Logger, Plugin, PluginApi, PluginMeta, SettingField, SettingKind};
use serde::Deserialize;
use serde_json::json;

pub fn plugin() -> Box<dyn Plugin> {
    Box::new(MyPlugin)
}

struct MyPlugin;

#[derive(Deserialize, Default)]
struct MySettings {
    #[serde(default)]
    enabled: bool,
}

impl Plugin for MyPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            id: "my_plugin".to_string(),
            name: "Мой плагин".to_string(),
            description: "Делает полезную вещь".to_string(),
            category: "Другое".to_string(),
            settings: vec![SettingField {
                key: "enabled".to_string(),
                label: "Включить".to_string(),
                kind: SettingKind::Boolean,
                description: None,
                required: false,
                default_value: json!(false),
                options: None,
                ui: None,
            }],
        }
    }

    fn run(
        &self,
        _api: &PluginApi,
        settings: &serde_json::Value,
        logger: &mut Logger,
    ) -> Result<(), String> {
        let settings: MySettings = serde_json::from_value(settings.clone()).unwrap_or_default();
        if settings.enabled {
            logger.success("Сработало".to_string());
            Ok(())
        } else {
            Err("Плагин выключен".to_string())
        }
    }
}
```
