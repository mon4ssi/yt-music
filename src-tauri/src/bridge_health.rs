use std::sync::Mutex;
use std::time::Instant;
use serde::Serialize;
use tauri::{AppHandle, Manager};

const STALE_THRESHOLD_SECS: u64 = 10;
const RECOVERY_COOLDOWN_SECS: u64 = 30;
const MAX_RECOVERY_ATTEMPTS: u64 = 5;

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
        if self.recovery_attempts >= MAX_RECOVERY_ATTEMPTS {
            return false;
        }
        let bridge_dead = match self.last_heartbeat {
            Some(hb) => hb.elapsed().as_secs() > STALE_THRESHOLD_SECS,
            None => self.startup.elapsed().as_secs() > STALE_THRESHOLD_SECS,
        };
        let backoff = RECOVERY_COOLDOWN_SECS * 2u64.pow(self.recovery_attempts as u32);
        let cooldown_ok = match self.last_recovery {
            Some(rec) => rec.elapsed().as_secs() > backoff,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn fresh_state_is_starting() {
        let state = BridgeHealthState::new();
        let report = state.report();
        assert_eq!(report.status, "starting");
        assert_eq!(report.total_heartbeats, 0);
        assert_eq!(report.recovery_attempts, 0);
    }

    #[test]
    fn heartbeat_makes_state_healthy() {
        let mut state = BridgeHealthState::new();
        state.record_heartbeat();
        let report = state.report();
        assert_eq!(report.status, "healthy");
        assert_eq!(report.total_heartbeats, 1);
    }

    #[test]
    fn state_becomes_degraded_after_stale() {
        let mut state = BridgeHealthState::new();
        state.record_heartbeat();
        // Simulate time passing beyond stale threshold
        state.last_heartbeat = Some(Instant::now() - Duration::from_secs(STALE_THRESHOLD_SECS + 2));
        let report = state.report();
        assert_eq!(report.status, "degraded");
    }

    #[test]
    fn state_is_dead_if_no_heartbeat_long_enough() {
        let state = BridgeHealthState {
            startup: Instant::now() - Duration::from_secs(STALE_THRESHOLD_SECS + 10),
            last_heartbeat: None,
            total_heartbeats: 0,
            recovery_attempts: 0,
            last_recovery: None,
        };
        let report = state.report();
        assert_eq!(report.status, "dead");
    }

    #[test]
    fn should_recover_false_when_bridge_healthy() {
        let mut state = BridgeHealthState::new();
        state.record_heartbeat();
        assert!(!state.should_recover());
    }

    #[test]
    fn should_recover_true_when_bridge_dead_and_cooldown_ok() {
        let mut state = BridgeHealthState::new();
        // Simulate startup far in the past (no heartbeat ever received)
        state.startup = Instant::now() - Duration::from_secs(STALE_THRESHOLD_SECS + 10);
        assert!(state.should_recover());
    }

    #[test]
    fn should_recover_false_at_max_attempts() {
        let mut state = BridgeHealthState::new();
        state.recovery_attempts = MAX_RECOVERY_ATTEMPTS;
        state.startup = Instant::now() - Duration::from_secs(STALE_THRESHOLD_SECS + 10);
        assert!(!state.should_recover());
    }

    #[test]
    fn record_recovery_increments_attempts() {
        let mut state = BridgeHealthState::new();
        assert_eq!(state.recovery_attempts, 0);
        state.record_recovery();
        assert_eq!(state.recovery_attempts, 1);
        assert!(state.last_recovery.is_some());
    }

    #[test]
    fn multiple_heartbeats_counted() {
        let mut state = BridgeHealthState::new();
        for _ in 0..5 {
            state.record_heartbeat();
        }
        assert_eq!(state.total_heartbeats, 5);
        assert!(state.last_heartbeat.is_some());
    }

    #[test]
    fn exponential_backoff_delays_recovery() {
        let mut state = BridgeHealthState::new();
        state.startup = Instant::now() - Duration::from_secs(STALE_THRESHOLD_SECS + 10);
        state.record_recovery(); // attempt 1, cooldown = 30s
        // Immediately checking should return false (cooldown not elapsed)
        assert!(!state.should_recover());
    }
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
                crate::diagnostics::record(&app, "warn", "heartbeat stale, attempting recovery", "bridge_health::watchdog");

                {
                    let state = app.state::<Mutex<BridgeHealthState>>();
                    let mut guard = state.lock().unwrap();
                    guard.record_recovery();
                }

                if let Some(window) = app.get_webview_window("main") {
                    if let Err(e) = window.eval(content_script) {
                        log::warn!("bridge recovery: injection failed: {e}");
                        crate::diagnostics::record(&app, "error", &format!("bridge recovery injection failed: {e}"), "bridge_health::watchdog");
                    } else {
                        log::info!("bridge recovery: content script re-injected");
                        crate::diagnostics::record(&app, "info", "content script re-injected", "bridge_health::watchdog");
                    }
                }
            }
        }
    });
}
