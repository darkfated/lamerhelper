use crate::core::{Logger, Plugin, PluginApi, PluginMeta, SettingField, SettingKind};
use serde::Deserialize;
use serde_json::json;
use std::process::Command;

pub fn plugin() -> Box<dyn Plugin> {
    Box::new(NetworkToolsPlugin)
}

struct NetworkToolsPlugin;

#[derive(Deserialize, Default)]
struct NetworkSettings {
    #[serde(default)]
    flush_dns: bool,
    #[serde(default)]
    release_ip: bool,
    #[serde(default)]
    renew_ip: bool,
    #[serde(default)]
    reset_winsock: bool,
}

impl Plugin for NetworkToolsPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            id: "network_tools".to_string(),
            name: "Сетевая очистка".to_string(),
            description: "Помогает исправить проблемы с интернетом: если сайты не открываются, соединение нестабильно или есть ошибки подключения. Действия обновляют сетевые параметры и часто быстро возвращают нормальную работу.".to_string(),
            settings: vec![
                SettingField {
                    key: "flush_dns".to_string(),
                    label: "Очистить DNS-кэш".to_string(),
                    kind: SettingKind::Boolean,
                    description: Some("Полезно, если сайт не открывается или открывается старый/не тот адрес. Обновляет список адресов сайтов.".to_string()),
                    required: false,
                    default_value: json!(false),
                    options: None,
                    ui: None,
                },
                SettingField {
                    key: "release_ip".to_string(),
                    label: "Сбросить IP‑адрес".to_string(),
                    kind: SettingKind::Boolean,
                    description: Some("Помогает при ошибках подключения к сети. Сбрасывает текущий адрес, чтобы запросить новый.".to_string()),
                    required: false,
                    default_value: json!(false),
                    options: None,
                    ui: None,
                },
                SettingField {
                    key: "renew_ip".to_string(),
                    label: "Получить новый IP‑адрес".to_string(),
                    kind: SettingKind::Boolean,
                    description: Some("Запрашивает новый адрес после сброса. Часто помогает вернуть доступ к интернету.".to_string()),
                    required: false,
                    default_value: json!(false),
                    options: None,
                    ui: None,
                },
                SettingField {
                    key: "reset_winsock".to_string(),
                    label: "Сброс сетевых настроек".to_string(),
                    kind: SettingKind::Boolean,
                    description: Some("Используйте, если другие пункты не помогли. Восстанавливает сетевые настройки и может устранить сложные сбои.".to_string()),
                    required: false,
                    default_value: json!(false),
                    options: None,
                    ui: Some(crate::core::SettingUi {
                        danger: Some(true),
                        ..Default::default()
                    }),
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
        if !cfg!(windows) {
            return Err("Модуль доступен только на Windows.".to_string());
        }

        let settings: NetworkSettings =
            serde_json::from_value(settings.clone()).unwrap_or_default();
        let mut any = false;

        if settings.flush_dns {
            any = true;
            run_cmd(logger, "ipconfig", &["/flushdns"])?;
        }
        if settings.release_ip {
            any = true;
            run_cmd(logger, "ipconfig", &["/release"])?;
        }
        if settings.renew_ip {
            any = true;
            run_cmd(logger, "ipconfig", &["/renew"])?;
        }
        if settings.reset_winsock {
            any = true;
            run_cmd(logger, "netsh", &["winsock", "reset"])?;
            logger.warn("После сброса Winsock может потребоваться перезапуск системы.".to_string());
        }

        if !any {
            return Err("Выберите хотя бы одну операцию.".to_string());
        }

        Ok(())
    }
}

fn run_cmd(logger: &mut Logger, cmd: &str, args: &[&str]) -> Result<(), String> {
    logger.info(format!("Запуск: {} {}", cmd, args.join(" ")));
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("Не удалось запустить {cmd}: {e}"))?;

    if !output.stdout.is_empty() {
        let text = String::from_utf8_lossy(&output.stdout);
        logger.info(text.trim().to_string());
    }
    if !output.stderr.is_empty() {
        let text = String::from_utf8_lossy(&output.stderr);
        logger.warn(text.trim().to_string());
    }

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Команда {cmd} завершилась с кодом {:?}", output.status.code()))
    }
}
