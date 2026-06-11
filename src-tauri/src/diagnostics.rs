use std::collections::VecDeque;
use std::sync::Mutex;
use serde::Serialize;
use tauri::{AppHandle, Manager};

const MAX_EVENTS: usize = 50;

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticsEntry {
    timestamp: String,
    level: String,
    message: String,
    location: String,
}

pub struct DiagnosticsState {
    events: VecDeque<DiagnosticsEntry>,
    startup: std::time::Instant,
}

impl DiagnosticsState {
    pub fn new() -> Self {
        Self {
            events: VecDeque::with_capacity(MAX_EVENTS),
            startup: std::time::Instant::now(),
        }
    }

    pub fn record_event(&mut self, level: &str, message: &str, location: &str) {
        if self.events.len() >= MAX_EVENTS {
            self.events.pop_front();
        }
        self.events.push_back(DiagnosticsEntry {
            timestamp: chrono_now(),
            level: level.to_string(),
            message: message.to_string(),
            location: location.to_string(),
        });
    }

    pub fn get_events(&self) -> Vec<DiagnosticsEntry> {
        self.events.iter().cloned().collect()
    }

    pub fn uptime_secs(&self) -> u64 {
        self.startup.elapsed().as_secs()
    }
}

fn chrono_now() -> String {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

pub fn record(app: &AppHandle, level: &str, message: &str, location: &str) {
    if let Some(state) = app.try_state::<Mutex<DiagnosticsState>>() {
        if let Ok(mut guard) = state.lock() {
            guard.record_event(level, message, location);
        }
    }
}

#[tauri::command]
pub fn record_event(app: AppHandle, level: String, message: String, location: String) {
    record(&app, &level, &message, &location);
}

#[tauri::command]
pub fn get_telemetry_enabled(app: tauri::AppHandle) -> Result<bool, String> {
    use tauri_plugin_store::StoreExt;
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    Ok(store
        .get("telemetry_enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false))
}

#[tauri::command]
pub fn toggle_telemetry(app: tauri::AppHandle) -> Result<bool, String> {
    use tauri_plugin_store::StoreExt;
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    let current = store
        .get("telemetry_enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let next = !current;
    store.set("telemetry_enabled", next);
    store.save().map_err(|e| e.to_string())?;
    record(&app, "info", &format!("telemetry {}", if next { "enabled" } else { "disabled" }), "diagnostics::toggle_telemetry");
    Ok(next)
}

#[tauri::command]
pub fn set_telemetry_enabled(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    use tauri_plugin_store::StoreExt;
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    store.set("telemetry_enabled", enabled);
    store.save().map_err(|e| e.to_string())?;
    record(&app, "info", &format!("telemetry {}", if enabled { "enabled" } else { "disabled" }), "diagnostics::set_telemetry_enabled");
    Ok(())
}

#[tauri::command]
pub fn export_diagnostics(app: tauri::AppHandle) -> Result<String, String> {
    use tauri_plugin_store::StoreExt;

    let bridge_health = {
        let state = app.state::<Mutex<crate::bridge_health::BridgeHealthState>>();
        let guard = state.lock().map_err(|e| e.to_string())?;
        guard.report()
    };

    let (events, uptime_secs, telemetry_enabled) = {
        let state = app.state::<Mutex<DiagnosticsState>>();
        let guard = state.lock().map_err(|e| e.to_string())?;
        let events = guard.get_events();
        let uptime = guard.uptime_secs();
        drop(guard);

        let store = app.store("settings.json").map_err(|e| e.to_string())?;
        let telemetry = store
            .get("telemetry_enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        (events, uptime, telemetry)
    };

    let report = serde_json::json!({
        "app_version": env!("CARGO_PKG_VERSION"),
        "platform_os": std::env::consts::OS,
        "platform_arch": std::env::consts::ARCH,
        "telemetry_enabled": telemetry_enabled,
        "uptime_secs": uptime_secs,
        "bridge_health": bridge_health,
        "events": events,
    });

    record(
        &app,
        "info",
        "diagnostics exported",
        "diagnostics::export_diagnostics",
    );

    serde_json::to_string_pretty(&report).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_state_has_no_events() {
        let state = DiagnosticsState::new();
        assert!(state.get_events().is_empty());
    }

    #[test]
    fn record_event_adds_entry() {
        let mut state = DiagnosticsState::new();
        state.record_event("info", "test message", "test::location");
        let events = state.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].level, "info");
        assert_eq!(events[0].message, "test message");
        assert_eq!(events[0].location, "test::location");
    }

    #[test]
    fn events_beyond_max_are_dropped() {
        let mut state = DiagnosticsState::new();
        for i in 0..MAX_EVENTS + 10 {
            state.record_event("info", &format!("event {i}"), "test");
        }
        let events = state.get_events();
        assert_eq!(events.len(), MAX_EVENTS);
        // The oldest events should have been dropped
        assert_eq!(events[0].message, "event 10");
        assert_eq!(events[MAX_EVENTS - 1].message, format!("event {}", MAX_EVENTS + 9));
    }

    #[test]
    fn events_are_recorded_in_order() {
        let mut state = DiagnosticsState::new();
        state.record_event("warn", "first", "test");
        state.record_event("error", "second", "test");
        let events = state.get_events();
        assert_eq!(events[0].message, "first");
        assert_eq!(events[1].message, "second");
    }

    #[test]
    fn uptime_secs_increases() {
        let state = DiagnosticsState::new();
        let u1 = state.uptime_secs();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let u2 = state.uptime_secs();
        assert!(u2 >= u1);
    }

    #[test]
    fn state_handles_various_levels() {
        let mut state = DiagnosticsState::new();
        for level in &["info", "warn", "error", "debug"] {
            state.record_event(level, "test", "test");
        }
        let events = state.get_events();
        assert_eq!(events.len(), 4);
        assert_eq!(events[0].level, "info");
        assert_eq!(events[1].level, "warn");
        assert_eq!(events[2].level, "error");
        assert_eq!(events[3].level, "debug");
    }
}

pub fn install_panic_hook() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "unknown panic".to_string()
        };
        let location = panic_info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_default();
        eprintln!("[yt-music] PANIC: {msg} at {location}");
        previous(panic_info);
    }));
}
