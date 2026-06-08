use serde::{Deserialize, Serialize};
use tauri::Emitter;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackState {
  pub title: String,
  pub artist: String,
  pub album: String,
  pub thumbnail: String,
  pub is_playing: bool,
  pub duration: f64,
  pub current_time: f64,
  pub volume: f64,
}

#[tauri::command]
pub fn update_playback_state(
  app: tauri::AppHandle,
  state: PlaybackState,
) {
  log::info!(
    "playback: {} — {} (playing: {}, {:.0}s / {:.0}s)",
    state.title,
    state.artist,
    state.is_playing,
    state.current_time,
    state.duration,
  );

  let _ = app.emit("playback-state-changed", &state);
}

pub const CONTENT_SCRIPT: &str = r#"(function() {
  var ready = false;

  function extractState() {
    try {
      var video = document.querySelector('video');
      var titleEl = document.querySelector('.ytmusic-player-bar .title');
      var artistEl = document.querySelector('.ytmusic-player-bar .byline');
      var thumbnailEl = document.querySelector('.ytmusic-player-bar .thumbnail img, .ytmusic-player-bar img');

      if (!titleEl && !video) return null;

      return {
        title: titleEl ? titleEl.textContent.trim() : '',
        artist: artistEl ? artistEl.textContent.trim() : '',
        album: '',
        thumbnail: thumbnailEl ? thumbnailEl.src : '',
        isPlaying: video ? !video.paused : false,
        duration: video ? video.duration : 0,
        currentTime: video ? video.currentTime : 0,
        volume: video ? video.volume : 1,
      };
    } catch(e) {
      return null;
    }
  }

  function send() {
    var state = extractState();
    if (state) {
      try {
        window.__TAURI_INTERNALS__.invoke('update_playback_state', { state: state });
      } catch(e) {
        /* IPC not ready yet */
      }
    }
  }

  function boot() {
    if (ready) return;
    ready = true;
    setTimeout(send, 2000);
    setInterval(send, 1000);
    new MutationObserver(send).observe(document.body, { childList: true, subtree: true });
  }

  if (document.body) {
    boot();
  } else {
    document.addEventListener('DOMContentLoaded', boot);
  }
})();"#;
