use tauri::{Manager, WebviewUrl};

const PALETTE_LABEL: &str = "command-palette";

pub fn open(app: &tauri::AppHandle) {
  if let Some(window) = app.get_webview_window(PALETTE_LABEL) {
    let _ = window.show();
    let _ = window.set_focus();
    return;
  }

  let builder = tauri::WebviewWindowBuilder::new(
    app,
    PALETTE_LABEL,
    WebviewUrl::App("command-palette.html".into()),
  )
  .title("Command Palette")
  .inner_size(420.0, 360.0)
  .resizable(false)
  .decorations(false)
  .always_on_top(true)
  .center();

  if let Err(e) = builder.build() {
    log::warn!("failed to create command palette window: {e:?}");
    crate::diagnostics::record(app, "error", &format!("failed to create command palette window: {e:?}"), "palette::open");
  }
}
