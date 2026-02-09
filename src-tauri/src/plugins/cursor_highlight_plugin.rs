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
                    label: "HotTrackingColor — панель выделения и ссылки".to_string(),
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
        let hilight = read_registry_color_hex(api, "Hilight")
            .unwrap_or_else(default_hilight_color);

        let hot_tracking = read_registry_color_hex(api, "HotTrackingColor")
            .unwrap_or_else(default_hot_tracking_color);

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

        let desired_hilight = normalize_to_hex(&settings.hilight_color)?;
        let desired_hot_tracking = normalize_to_hex(&settings.hot_tracking_color)?;

        apply_color(
            api,
            logger,
            "Hilight",
            &desired_hilight,
        )?;

        apply_color(
            api,
            logger,
            "HotTrackingColor",
            &desired_hot_tracking,
        )?;

        logger.success("Готово! Для применения перезагрузите компьютер.".to_string());
        Ok(())
    }
}

fn apply_color(
    api: &PluginApi,
    logger: &mut Logger,
    name: &str,
    desired_hex: &str,
) -> Result<(), String> {
    let current_hex = read_registry_color_hex(api, name);

    if current_hex.as_deref().map(|c| c.eq_ignore_ascii_case(desired_hex)) == Some(true) {
        logger.info(format!("{name} уже установлен: {desired_hex}"));
        return Ok(());
    }

    let (r, g, b) = parse_color(desired_hex)?;
    let value = format!("{r} {g} {b}");

    api.set_registry_string("Control Panel\\Colors", name, &value)?;
    logger.info(format!("{name} установлен: {value}"));
    Ok(())
}

fn parse_color(value: &str) -> Result<(u8, u8, u8), String> {
    if let Some(rgb) = parse_rgb_string(value) {
        return Ok(rgb);
    }

    let hex = value.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return Err("Цвет должен быть HEX (#RRGGBB) или RGB (R G B).".to_string());
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Некорректный R".to_string())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Некорректный G".to_string())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Некорректный B".to_string())?;
    Ok((r, g, b))
}

fn parse_rgb_string(value: &str) -> Option<(u8, u8, u8)> {
    let parts: Vec<&str> = value.split(|c| c == ' ' || c == ',').filter(|p| !p.is_empty()).collect();
    if parts.len() != 3 {
        return None;
    }
    Some((
        parts[0].parse().ok()?,
        parts[1].parse().ok()?,
        parts[2].parse().ok()?,
    ))
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

fn normalize_to_hex(input: &str) -> Result<String, String> {
    let (r, g, b) = parse_color(input)?;
    Ok(rgb_to_hex(r, g, b))
}

fn read_registry_color_hex(api: &PluginApi, name: &str) -> Option<String> {
    api.get_registry_string("Control Panel\\Colors", name)
        .ok()
        .and_then(|v| parse_rgb_string(&v))
        .map(|(r, g, b)| rgb_to_hex(r, g, b))
}
