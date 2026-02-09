# LamerHelper

Программа с универсальными утилитами для упрощённого использования персонального компьютера.

## ⚙ Стек

**Frontend**: Vue 3, Vite

**Backend**: Tauri 2, Rust

### Команды

```bash
yarn install # установка зависимостей
yarn tauri dev # запуск
yarn tauri build # скомпилировать
```

## 📸 Скриншоты

Продукт пополняется плагинами, отображаемое количество функционала не итоговое.

### Главное меню

<img width="1042" height="801" alt="Главное меню" src="https://github.com/user-attachments/assets/ea84b052-8bc5-4c2e-b355-a63f8a87157b" />

### Вкладка плагина

<img width="1046" height="801" alt="Вкладка плагина" src="https://github.com/user-attachments/assets/9bd21a9d-6bea-46c2-a240-0de75b816ee3" />

## 🧩 Система плагинов

### Описание

Плагины являются Rust-модулями, реализующими трейт `Plugin`.

- Контракт плагина описан в `src-tauri/src/core/plugin.rs`
- Все файлы в `src-tauri/src/plugins/` с суффиксом `_plugin.rs` попадают в регистрацию
- Реестр списка плагинов генерируется в `plugins.generated.rs` и подключается в `src-tauri/src/core/registry.rs`

### Как добавить новый плагин

1. Создайте файл `src-tauri/src/plugins/*_plugin.rs`
2. Опишите метаданные плагина в `PluginMeta` (id, название, описание, категория, настройки, и другое)
3. Реализуйте логику в функции `run` - она запускается при нажатии на кнопку в интерфейсе

### Шаблон

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

## 🔒 Лицензия

Этот проект лицензирован под MIT License. [MIT License](https://github.com/darkfated/lamerhelper/blob/master/LICENSE)
