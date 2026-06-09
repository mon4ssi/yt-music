use tauri::{Manager, WebviewUrl};

const MINI_PLAYER_LABEL: &str = "mini-player";

pub fn create_or_toggle(app: &tauri::AppHandle) {
  if let Some(window) = app.get_webview_window(MINI_PLAYER_LABEL) {
    if window.is_visible().unwrap_or(false) {
      let _ = window.hide();
    } else {
      let _ = window.show();
      let _ = window.set_focus();
    }
    return;
  }

  let builder = tauri::WebviewWindowBuilder::new(
    app,
    MINI_PLAYER_LABEL,
    WebviewUrl::App("mini-player.html".into()),
  )
  .title("Now Playing")
  .inner_size(380.0, 86.0)
  .resizable(false)
  .decorations(false)
  .always_on_top(true)
  .skip_taskbar(true);

  if let Err(e) = builder.build() {
    log::warn!("failed to create mini-player window: {e:?}");
    crate::diagnostics::record(app, "error", &format!("failed to create mini-player window: {e:?}"), "mini_player::create_or_toggle");
  }
}

pub fn close(app: &tauri::AppHandle) {
  if let Some(window) = app.get_webview_window(MINI_PLAYER_LABEL) {
    let _ = window.close();
  }
}
