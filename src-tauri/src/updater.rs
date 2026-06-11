use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;
use tauri_plugin_updater::UpdaterExt;

const SETTINGS_FILE: &str = "settings.json";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_endpoints_stable() {
        let urls = channel_endpoints("stable");
        assert_eq!(urls.len(), 1);
        let url_str = urls[0].as_str();
        assert!(url_str.contains("/stable/update.json"), "expected /stable/update.json in URL, got {url_str}");
    }

    #[test]
    fn channel_endpoints_beta() {
        let urls = channel_endpoints("beta");
        assert_eq!(urls.len(), 1);
        let url_str = urls[0].as_str();
        assert!(url_str.contains("/beta/update.json"), "expected /beta/update.json in URL, got {url_str}");
    }

    #[test]
    fn update_info_serialization() {
        let info = UpdateInfo {
            available: true,
            version: Some("1.0.0".into()),
            body: Some("Release notes".into()),
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"available\":true"));
        assert!(json.contains("\"version\":\"1.0.0\""));
        assert!(json.contains("\"body\":\"Release notes\""));
    }

    #[test]
    fn update_info_no_update() {
        let info = UpdateInfo {
            available: false,
            version: None,
            body: None,
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"available\":false"));
        assert!(!json.contains("\"version\""));
    }
}

fn channel_endpoints(channel: &str) -> Vec<url::Url> {
    let base = format!(
        "https://releases.mon4ssi.com/yt-music/{channel}/update.json",
    );
    vec![url::Url::parse(&base).expect("invalid updater endpoint URL")]
}

#[derive(Serialize)]
pub struct UpdateInfo {
    pub available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

#[tauri::command]
pub fn get_update_channel(app: AppHandle) -> Result<String, String> {
    let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
    Ok(store
        .get("update_channel")
        .and_then(|v| v.as_str().map(ToString::to_string))
        .unwrap_or_else(|| "stable".to_string()))
}

#[tauri::command]
pub fn set_update_channel(app: AppHandle, channel: String) -> Result<(), String> {
    if channel != "stable" && channel != "beta" {
        return Err("channel must be 'stable' or 'beta'".to_string());
    }
    let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
    store.set("update_channel", channel.as_str());
    store.save().map_err(|e| e.to_string())?;
    app.emit("update-channel-changed", &channel)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn check_for_updates(app: AppHandle) -> Result<UpdateInfo, String> {
    let channel = {
        let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
        store
            .get("update_channel")
            .and_then(|v| v.as_str().map(ToString::to_string))
            .unwrap_or_else(|| "stable".to_string())
    };

    let endpoints = channel_endpoints(&channel);
    let updater = app
        .updater_builder()
        .endpoints(endpoints)
        .map_err(|e| format!("failed to configure updater: {e}"))?
        .build()
        .map_err(|e| format!("failed to build updater: {e}"))?;

    match updater.check().await {
        Ok(Some(update)) => Ok(UpdateInfo {
            available: true,
            version: Some(update.version),
            body: update.body,
        }),
        Ok(None) => Ok(UpdateInfo {
            available: false,
            version: None,
            body: None,
        }),
        Err(e) => Err(format!("update check failed: {e}")),
    }
}
