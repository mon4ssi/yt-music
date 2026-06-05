use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let app = tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::new().build())
    .plugin(
      tauri_plugin_single_instance::init(|app, _args, _cwd| {
        let _ = app.get_webview_window("main").map(|w| w.set_focus());
      }),
    )
    .plugin(
      tauri_plugin_global_shortcut::Builder::new()
        .with_handler(move |_app, shortcut, event| {
          if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
            let key = shortcut;
            if key.matches(Modifiers::empty(), Code::MediaPlay)
              || key.matches(Modifiers::empty(), Code::MediaPause)
            {
              log::info!("media play/pause");
            } else if key.matches(Modifiers::empty(), Code::MediaTrackNext) {
              log::info!("media next");
            } else if key.matches(Modifiers::empty(), Code::MediaTrackPrevious) {
              log::info!("media previous");
            }
          }
        })
        .build(),
    )
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      let play_pause =
        MenuItem::with_id(app, "play_pause", "Play/Pause", true, None::<&str>)?;
      let next = MenuItem::with_id(app, "next", "Next Track", true, None::<&str>)?;
      let previous =
        MenuItem::with_id(app, "previous", "Previous Track", true, None::<&str>)?;
      let separator = PredefinedMenuItem::separator(app)?;
      let quit =
        MenuItem::with_id(app, "quit", "Quit YT Music", true, None::<&str>)?;

      let menu = Menu::with_items(app, &[&play_pause, &next, &previous, &separator, &quit])?;

      TrayIconBuilder::new().menu(&menu).build(app)?;

      let media_shortcuts = [
        Shortcut::new(Some(Modifiers::empty()), Code::MediaPlay),
        Shortcut::new(Some(Modifiers::empty()), Code::MediaTrackNext),
        Shortcut::new(Some(Modifiers::empty()), Code::MediaTrackPrevious),
      ];

      for shortcut in &media_shortcuts {
        if let Err(e) = app.global_shortcut().register(shortcut.clone()) {
          log::warn!("failed to register shortcut {shortcut}: {e}");
        }
      }

      Ok(())
    })
    .on_menu_event(|app, event| {
      match event.id().as_ref() {
        "play_pause" => log::info!("tray play/pause"),
        "next" => log::info!("tray next"),
        "previous" => log::info!("tray previous"),
        "quit" => app.exit(0),
        _ => {}
      }
    })
    .build(tauri::generate_context!())
    .expect("error while building tauri application");

  app.run(|_handle, _event| {});
}
