use std::sync::Mutex;
use std::time::Instant;
use serde::Serialize;
use tauri::{AppHandle, Manager};

const STALE_THRESHOLD_SECS: u64 = 10;
const RECOVERY_COOLDOWN_SECS: u64 = 30;

#[derive(Debug, Clone, Serialize)]
pub struct BridgeHealthReport {
    pub status: String,
    pub last_heartbeat_ms_ago: u64,
    pub total_heartbeats: u64,
    pub recovery_attempts: u64,
}

pub struct BridgeHealthState {
    startup: Instant,
    last_heartbeat: Option<Instant>,
    total_heartbeats: u64,
    recovery_attempts: u64,
    last_recovery: Option<Instant>,
}

impl BridgeHealthState {
    pub fn new() -> Self {
        Self {
            startup: Instant::now(),
            last_heartbeat: None,
            total_heartbeats: 0,
            recovery_attempts: 0,
            last_recovery: None,
        }
    }

    pub fn record_heartbeat(&mut self) {
        self.last_heartbeat = Some(Instant::now());
        self.total_heartbeats += 1;
    }

    pub fn report(&self) -> BridgeHealthReport {
        let elapsed = self
            .last_heartbeat
            .map(|hb| hb.elapsed().as_millis() as u64)
            .unwrap_or_else(|| self.startup.elapsed().as_millis() as u64);

        let status = match self.last_heartbeat {
            Some(hb) if hb.elapsed().as_secs() < STALE_THRESHOLD_SECS => "healthy",
            Some(_) => "degraded",
            None if self.startup.elapsed().as_secs() < STALE_THRESHOLD_SECS + 5 => "starting",
            None => "dead",
        };

        BridgeHealthReport {
            status: status.to_string(),
            last_heartbeat_ms_ago: elapsed,
            total_heartbeats: self.total_heartbeats,
            recovery_attempts: self.recovery_attempts,
        }
    }

    fn should_recover(&self) -> bool {
        let bridge_dead = match self.last_heartbeat {
            Some(hb) => hb.elapsed().as_secs() > STALE_THRESHOLD_SECS,
            None => self.startup.elapsed().as_secs() > STALE_THRESHOLD_SECS,
        };
        let cooldown_ok = match self.last_recovery {
            Some(rec) => rec.elapsed().as_secs() > RECOVERY_COOLDOWN_SECS,
            None => true,
        };
        bridge_dead && cooldown_ok
    }

    fn record_recovery(&mut self) {
        self.last_recovery = Some(Instant::now());
        self.recovery_attempts += 1;
    }
}

#[tauri::command]
pub fn heartbeat(state: tauri::State<'_, Mutex<BridgeHealthState>>) {
    state.lock().unwrap().record_heartbeat();
}

#[tauri::command]
pub fn get_bridge_health(
    state: tauri::State<'_, Mutex<BridgeHealthState>>,
) -> BridgeHealthReport {
    state.lock().unwrap().report()
}

pub fn start_watchdog(app: AppHandle, content_script: &'static str) {
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(12));

        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let should_recover = {
                let state = app.state::<Mutex<BridgeHealthState>>();
                let guard = state.lock().unwrap();
                guard.should_recover()
            };

            if should_recover {
                log::warn!("bridge watchdog: heartbeat stale, attempting recovery");

                {
                    let state = app.state::<Mutex<BridgeHealthState>>();
                    let mut guard = state.lock().unwrap();
                    guard.record_recovery();
                }

                if let Some(window) = app.get_webview_window("main") {
                    if let Err(e) = window.eval(content_script) {
                        log::warn!("bridge recovery: injection failed: {e}");
                    } else {
                        log::info!("bridge recovery: content script re-injected");
                    }
                }
            }
        }
    });
}
