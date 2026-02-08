use crate::core::{Logger, Plugin, PluginApi, PluginMeta, SettingField, SettingKind, SettingUi};
use serde::Deserialize;
use serde_json::json;

pub fn plugin() -> Box<dyn Plugin> {
    Box::new(CursorHighlightPlugin)
}

struct CursorHighlightPlugin;

#[derive(Deserialize)]
struct CursorHighlightSettings {
    #[serde(default = "default_hilight_color")]
    hilight_color: String,
    #[serde(default = "default_hot_tracking_color")]
    hot_tracking_color: String,
}

fn default_hilight_color() -> String {
    "#3aa3ff".to_string()
}

fn default_hot_tracking_color() -> String {
    "#2bd3a7".to_string()
}

impl Plugin for CursorHighlightPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            id: "cursor_highlight".to_string(),
            name: "Цвет выделения".to_string(),
            description: "Настройка цветов выделения и подсветки в Windows.".to_string(),
            category: "Визуал".to_string(),
            settings: vec![
                SettingField {
                    key: "hilight_color".to_string(),
                    label: "Hilight — выделение текста".to_string(),
                    kind: SettingKind::Color,
                    description: Some("HKCU\\Control Panel\\Colors\\Hilight".to_string()),
                    required: true,
                    default_value: json!(default_hilight_color()),
                    options: None,
                    ui: Some(SettingUi {
                        placeholder: Some("#3aa3ff".to_string()),
                        ..Default::default()
                    }),
                },
                SettingField {
                    key: "hot_tracking_color".to_string(),
                    label: "HotTrackingColor — панель/ссылки".to_string(),
                    kind: SettingKind::Color,
                    description: Some("HKCU\\Control Panel\\Colors\\HotTrackingColor".to_string()),
                    required: true,
                    default_value: json!(default_hot_tracking_color()),
                    options: None,
                    ui: Some(SettingUi {
                        placeholder: Some("#2bd3a7".to_string()),
                        ..Default::default()
                    }),
                },
            ],
        }
    }

    fn defaults(&self, api: &PluginApi) -> Option<serde_json::Value> {
        let hilight = read_registry_color(api, "Hilight", &default_hilight_color());
        let hot_tracking =
            read_registry_color(api, "HotTrackingColor", &default_hot_tracking_color());
        Some(json!({
            "hilight_color": hilight,
            "hot_tracking_color": hot_tracking
        }))
    }

    fn run(
        &self,
        api: &PluginApi,
        settings: &serde_json::Value,
        logger: &mut Logger,
    ) -> Result<(), String> {
        let settings: CursorHighlightSettings =
            serde_json::from_value(settings.clone()).unwrap_or(CursorHighlightSettings {
                hilight_color: default_hilight_color(),
                hot_tracking_color: default_hot_tracking_color(),
            });
        let (r, g, b) = parse_color(&settings.hilight_color)?;
        let hilight_value = format!("{r} {g} {b}");
        api.set_registry_string("Control Panel\\Colors", "Hilight", &hilight_value)?;
        logger.info(format!("Hilight установлен в {hilight_value}"));

        let (r, g, b) = parse_color(&settings.hot_tracking_color)?;
        let hot_tracking_value = format!("{r} {g} {b}");
        api.set_registry_string(
            "Control Panel\\Colors",
            "HotTrackingColor",
            &hot_tracking_value,
        )?;
        logger.info(format!("HotTrackingColor установлен в {hot_tracking_value}"));
        logger.info("Применяю изменения через перезапуск Explorer...".to_string());
        match api.restart_explorer() {
            Ok(()) => logger.success("Explorer перезапущен.".to_string()),
            Err(err) => logger.warn(format!("Не удалось перезапустить Explorer: {err}")),
        }
        Ok(())
    }
}

fn parse_color(value: &str) -> Result<(u8, u8, u8), String> {
    if let Some(rgb) = parse_rgb_string(value) {
        return Ok(rgb);
    }

    let trimmed = value.trim();
    let hex = trimmed.strip_prefix('#').unwrap_or(trimmed);
    if hex.len() != 6 {
        return Err("Цвет должен быть HEX (#RRGGBB) или RGB (R G B).".to_string());
    }
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Некорректный R".to_string())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Некорректный G".to_string())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Некорректный B".to_string())?;
    Ok((r, g, b))
}

fn parse_rgb_string(value: &str) -> Option<(u8, u8, u8)> {
    let parts: Vec<&str> = value
        .split(|c| c == ' ' || c == ',')
        .filter(|part| !part.trim().is_empty())
        .collect();
    if parts.len() != 3 {
        return None;
    }
    let r = parts[0].trim().parse::<u8>().ok()?;
    let g = parts[1].trim().parse::<u8>().ok()?;
    let b = parts[2].trim().parse::<u8>().ok()?;
    Some((r, g, b))
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

fn read_registry_color(api: &PluginApi, name: &str, fallback_hex: &str) -> String {
    match api.get_registry_string("Control Panel\\Colors", name) {
        Ok(value) => {
            if let Some((r, g, b)) = parse_rgb_string(&value) {
                rgb_to_hex(r, g, b)
            } else {
                fallback_hex.to_string()
            }
        }
        Err(_) => fallback_hex.to_string(),
    }
}
