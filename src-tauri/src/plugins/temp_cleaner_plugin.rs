use crate::core::{
    Logger, Plugin, PluginApi, PluginMeta, PluginPreview, SettingField, SettingKind,
};
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

pub fn plugin() -> Box<dyn Plugin> {
    Box::new(TempCleanerPlugin)
}

struct TempCleanerPlugin;

#[derive(Deserialize)]
struct TempCleanerSettings {
    #[serde(default = "default_true")]
    user_temp: bool,
    #[serde(default)]
    system_temp: bool,
    #[serde(default)]
    update_cache: bool,
    #[serde(default)]
    minidumps: bool,
    #[serde(default)]
    dry_run: bool,
}

impl Default for TempCleanerSettings {
    fn default() -> Self {
        Self {
            user_temp: true,
            system_temp: false,
            update_cache: false,
            minidumps: false,
            dry_run: false,
        }
    }
}

fn default_true() -> bool {
    true
}

impl Plugin for TempCleanerPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            id: "temp_cleaner".to_string(),
            name: "Очистка системы".to_string(),
            description: "Очистка неудаляемых временных файлов.".to_string(),
            settings: vec![
                SettingField {
                    key: "user_temp".to_string(),
                    label: "Временные файлы пользователя".to_string(),
                    kind: SettingKind::Boolean,
                    description: Some(
                        "Удаляет кэш и временные файлы, которые создают приложения и система. Но Windows не очищает их, поэтому со временем они могут занимать много места.".to_string(),
                    ),
                    required: false,
                    default_value: json!(true),
                    options: None,
                    ui: None,
                },
                SettingField {
                    key: "system_temp".to_string(),
                    label: "Временные файлы системы".to_string(),
                    kind: SettingKind::Boolean,
                    description: Some(
                        "Очищает системные временные файлы. Может потребоваться запуск с правами администратора."
                            .to_string(),
                    ),
                    required: false,
                    default_value: json!(false),
                    options: None,
                    ui: None,
                },
                SettingField {
                    key: "update_cache".to_string(),
                    label: "Кэш обновлений Windows".to_string(),
                    kind: SettingKind::Boolean,
                    description: Some(
                        "Удаляет загруженные файлы обновлений. Иногда помогает освободить много места."
                            .to_string(),
                    ),
                    required: false,
                    default_value: json!(false),
                    options: None,
                    ui: None,
                },
                SettingField {
                    key: "minidumps".to_string(),
                    label: "Отчёты об ошибках".to_string(),
                    kind: SettingKind::Boolean,
                    description: Some(
                        "Удаляет диагностические дампы и отчёты о сбоях. Полезно, если они уже не нужны."
                            .to_string(),
                    ),
                    required: false,
                    default_value: json!(false),
                    options: None,
                    ui: None,
                },
                SettingField {
                    key: "dry_run".to_string(),
                    label: "Режим проверки (без удаления)".to_string(),
                    kind: SettingKind::Boolean,
                    description: Some("Показывает, что будет удалено, без изменений.".to_string()),
                    required: false,
                    default_value: json!(false),
                    options: None,
                    ui: None,
                },
            ],
        }
    }

    fn preview(&self, api: &PluginApi) -> Result<Option<PluginPreview>, String> {
        let settings = TempCleanerSettings::default();
        let targets = build_targets(api, &settings);
        let mut size_errors = 0u64;
        let mut total_bytes = 0u64;
        let mut details = Vec::new();

        for target in targets {
            let size = dir_size(&target.path, None, &mut size_errors);
            total_bytes = total_bytes.saturating_add(size);
            details.push(format!("{}: {}", target.label, format_bytes(size)));
        }

        let note = if details.is_empty() {
            None
        } else {
            Some(details.join(" • "))
        };

        Ok(Some(PluginPreview {
            title: "Можно очистить".to_string(),
            value: format_bytes(total_bytes),
            note,
        }))
    }

    fn run(
        &self,
        api: &PluginApi,
        settings: &serde_json::Value,
        logger: &mut Logger,
    ) -> Result<(), String> {
        let settings: TempCleanerSettings =
            serde_json::from_value(settings.clone()).unwrap_or_default();

        let targets = build_targets(api, &settings);
        let mut items: Vec<CleanItem> = Vec::new();
        let mut size_errors: u64 = 0;

        for target in targets {
            if !target.enabled {
                continue;
            }
            logger.info(format!("Раздел: {}", target.label));
            if !target.path.exists() {
                logger.warn(format!("Путь не найден: {}", target.path.display()));
                continue;
            }
            let entries = match fs::read_dir(&target.path) {
                Ok(entries) => entries,
                Err(err) => {
                    size_errors += 1;
                    logger.warn(format!(
                        "Не удалось открыть {}: {err}",
                        target.path.display()
                    ));
                    continue;
                }
            };

            for entry in entries {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(err) => {
                        size_errors += 1;
                        logger.warn(format!("Ошибка чтения элемента: {err}"));
                        continue;
                    }
                };
                let path = entry.path();
                let metadata = match entry.metadata() {
                    Ok(meta) => meta,
                    Err(err) => {
                        size_errors += 1;
                        logger.warn(format!(
                            "Не удалось получить метаданные {}: {err}",
                            path.display()
                        ));
                        continue;
                    }
                };
                let is_dir = metadata.is_dir();
                let size = if is_dir {
                    dir_size(&path, Some(logger), &mut size_errors)
                } else {
                    metadata.len()
                };
                items.push(CleanItem {
                    path,
                    is_dir,
                    size,
                    tag: target.tag.clone(),
                });
            }
        }

        if size_errors > 0 {
            logger.warn(format!(
                "При подсчёте размера возникло ошибок: {size_errors}. Итоги могут быть неполными."
            ));
        }

        let mut files = 0u64;
        let mut dirs = 0u64;
        let mut errors = 0u64;
        let mut freed_bytes = 0u64;

        for item in items {
            let path = item.path;
            let is_dir = item.is_dir;
            let size = item.size;
            let tag = item.tag;
            if is_dir {
                if settings.dry_run {
                    dirs += 1;
                    logger.info(format!(
                        "[{tag}] [dry] папка: {} ({})",
                        path.display(),
                        format_bytes(size)
                    ));
                } else {
                    match api.remove_dir_all(&path) {
                        Ok(()) => {
                            dirs += 1;
                            freed_bytes = freed_bytes.saturating_add(size);
                            logger.info(format!(
                                "[{tag}] Удалено: {} ({})",
                                path.display(),
                                format_bytes(size)
                            ));
                        }
                        Err(err) => {
                            errors += 1;
                            logger.warn(format!(
                                "Не удалось удалить {}: {err}",
                                path.display()
                            ));
                        }
                    }
                }
            } else if settings.dry_run {
                files += 1;
                logger.info(format!(
                    "[{tag}] [dry] файл: {} ({})",
                    path.display(),
                    format_bytes(size)
                ));
            } else {
                match api.remove_file(&path) {
                    Ok(()) => {
                        files += 1;
                        freed_bytes = freed_bytes.saturating_add(size);
                        logger.info(format!(
                            "[{tag}] Удалено: {} ({})",
                            path.display(),
                            format_bytes(size)
                        ));
                    }
                    Err(err) => {
                        errors += 1;
                        logger.warn(format!(
                            "Не удалось удалить {}: {err}",
                            path.display()
                        ));
                    }
                }
            }
        }

        logger.info(format!(
            "Итог: файлов {files}, папок {dirs}, ошибок {errors}."
        ));
        if settings.dry_run {
            logger.info("Освобождено: 0 B (режим проверки).".to_string());
        } else {
            logger.info(format!("Освобождено: {}", format_bytes(freed_bytes)));
        }

        if errors > 0 {
            Err(format!("Часть элементов не удалось обработать: {errors}"))
        } else {
            Ok(())
        }
    }
}

struct CleanTarget {
    tag: String,
    label: String,
    path: PathBuf,
    enabled: bool,
}

struct CleanItem {
    path: PathBuf,
    is_dir: bool,
    size: u64,
    tag: String,
}

fn build_targets(api: &PluginApi, settings: &TempCleanerSettings) -> Vec<CleanTarget> {
    let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
    let system_root = PathBuf::from(system_root);
    let mut targets = Vec::new();

    targets.push(CleanTarget {
        tag: "TEMP".to_string(),
        label: "%TEMP% пользователя".to_string(),
        path: api.temp_dir(),
        enabled: settings.user_temp,
    });

    targets.push(CleanTarget {
        tag: "SYS".to_string(),
        label: "Системный TEMP".to_string(),
        path: system_root.join("Temp"),
        enabled: settings.system_temp,
    });

    targets.push(CleanTarget {
        tag: "UPD".to_string(),
        label: "Кэш обновлений Windows".to_string(),
        path: system_root.join("SoftwareDistribution").join("Download"),
        enabled: settings.update_cache,
    });

    targets.push(CleanTarget {
        tag: "DMP".to_string(),
        label: "Минидампы ошибок".to_string(),
        path: system_root.join("Minidump"),
        enabled: settings.minidumps,
    });

    targets
}

fn dir_size(path: &Path, mut logger: Option<&mut Logger>, errors: &mut u64) -> u64 {
    let mut size = 0u64;
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(err) => {
            *errors += 1;
            if let Some(logger) = logger.as_deref_mut() {
                logger.warn(format!(
                    "Не удалось прочитать папку {}: {err}",
                    path.display()
                ));
            }
            return size;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                *errors += 1;
                if let Some(logger) = logger.as_deref_mut() {
                    logger.warn(format!("Ошибка чтения элемента: {err}"));
                }
                continue;
            }
        };
        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(meta) => meta,
            Err(err) => {
                *errors += 1;
                if let Some(logger) = logger.as_deref_mut() {
                    logger.warn(format!(
                        "Не удалось получить метаданные {}: {err}",
                        path.display()
                    ));
                }
                continue;
            }
        };
        if metadata.is_dir() {
            size = size.saturating_add(dir_size(&path, logger.as_deref_mut(), errors));
        } else {
            size = size.saturating_add(metadata.len());
        }
    }

    size
}

fn format_bytes(bytes: u64) -> String {
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
