use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;
use tauri_plugin_updater::UpdaterExt;

const SETTINGS_FILE: &str = "settings.json";

fn channel_endpoints(channel: &str) -> Vec<url::Url> {
    let base = format!(
        "https://releases.mon4ssi.com/yt-music/{channel}/{{target}}-{{arch}}/{{current_version}}",
    );
    vec![url::Url::parse(&base).expect("invalid updater endpoint URL")]
}

#[derive(Serialize)]
pub struct UpdateInfo {
    pub available: bool,
    pub version: Option<String>,
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
