mod bridge_health;
mod commands;
mod mini_player;
mod palette;
mod playback;

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

const SETTINGS_FILE: &str = "settings.json";

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
        .with_handler(move |app, shortcut, event| {
          if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
            let key = shortcut;
            if key.matches(Modifiers::empty(), Code::MediaPlay)
              || key.matches(Modifiers::empty(), Code::MediaPause)
            {
              commands::execute(app, commands::PlaybackCommand::PlayPause);
            } else if key.matches(Modifiers::empty(), Code::MediaTrackNext) {
              commands::execute(app, commands::PlaybackCommand::Next);
            } else if key.matches(Modifiers::empty(), Code::MediaTrackPrevious) {
              commands::execute(app, commands::PlaybackCommand::Previous);
            } else if key.matches(Modifiers::META, Code::KeyK)
              || key.matches(Modifiers::CONTROL, Code::KeyK)
            {
              palette::open(app);
            }
          }
        })
        .build(),
    )
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_autostart::init(
      tauri_plugin_autostart::MacosLauncher::LaunchAgent,
      None,
    ))
    .plugin(tauri_plugin_store::Builder::default().build())
    .invoke_handler(tauri::generate_handler![
      playback::update_playback_state,
      bridge_health::heartbeat,
      bridge_health::get_bridge_health,
      toggle_playback,
      next_track,
      previous_track,
      focus_main_window,
      toggle_mini_player,
      navigate_to,
      close_palette,
      get_theme,
      toggle_theme
    ])
    .setup(|app| {
      app.manage(std::sync::Mutex::new(bridge_health::BridgeHealthState::new()));

      let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36";
      let main_url = "https://music.youtube.com".parse::<url::Url>().expect("invalid URL");
      if let Err(e) = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(main_url))
        .title("YouTube Music")
        .inner_size(1280.0, 800.0)
        .min_inner_size(800.0, 500.0)
        .user_agent(user_agent)
        .on_navigation(|url| {
          let allowed = ["music.youtube.com", "accounts.google.com", "accounts.youtube.com"];
          url.host_str().is_some_and(|host| allowed.contains(&host))
        })
        .build()
      {
        log::warn!("failed to create main window: {e}");
      }

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
      let toggle_mini = MenuItem::with_id(
        app,
        "mini_player",
        "Toggle Mini-Player",
        true,
        None::<&str>,
      )?;
      let separator2 = PredefinedMenuItem::separator(app)?;
      let quit =
        MenuItem::with_id(app, "quit", "Quit YT Music", true, None::<&str>)?;

      let menu = Menu::with_items(
        app,
        &[&play_pause, &next, &previous, &separator, &toggle_mini, &separator2, &quit],
      )?;

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

      let palette_shortcuts = [
        Shortcut::new(Some(Modifiers::META), Code::KeyK),
        Shortcut::new(Some(Modifiers::CONTROL), Code::KeyK),
      ];

      for shortcut in &palette_shortcuts {
        if let Err(e) = app.global_shortcut().register(shortcut.clone()) {
          log::warn!("failed to register command palette shortcut: {e}");
        }
      }

      let handle = app.handle().clone();
      std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(5));
        if let Some(window) = handle.get_webview_window("main") {
          if let Err(e) = window.eval(playback::CONTENT_SCRIPT) {
            log::warn!("failed to inject playback script: {e}");
          } else {
            log::info!("playback content script injected");
          }
        }
      });

      bridge_health::start_watchdog(app.handle().clone(), playback::CONTENT_SCRIPT);

      Ok(())
    })
    .on_menu_event(|app, event| {
      match event.id().as_ref() {
        "play_pause" => commands::execute(app, commands::PlaybackCommand::PlayPause),
        "next" => commands::execute(app, commands::PlaybackCommand::Next),
        "previous" => commands::execute(app, commands::PlaybackCommand::Previous),
        "mini_player" => mini_player::create_or_toggle(app),
        "quit" => app.exit(0),
        _ => {}
      }
    })
    .build(tauri::generate_context!())
    .expect("error while building tauri application");

  app.run(|_handle, _event| {});
}

#[tauri::command]
fn toggle_playback(app: tauri::AppHandle) {
  commands::execute(&app, commands::PlaybackCommand::PlayPause);
}

#[tauri::command]
fn next_track(app: tauri::AppHandle) {
  commands::execute(&app, commands::PlaybackCommand::Next);
}

#[tauri::command]
fn previous_track(app: tauri::AppHandle) {
  commands::execute(&app, commands::PlaybackCommand::Previous);
}

#[tauri::command]
fn focus_main_window(app: tauri::AppHandle) {
  if let Some(window) = app.get_webview_window("main") {
    let _ = window.show();
    let _ = window.set_focus();
  }
  mini_player::close(&app);
}

#[tauri::command]
fn toggle_mini_player(app: tauri::AppHandle) {
  mini_player::create_or_toggle(&app);
}

#[tauri::command]
fn navigate_to(app: tauri::AppHandle, page: String) {
  commands::navigate_to(&app, &page);
}

#[tauri::command]
fn close_palette(app: tauri::AppHandle) {
  if let Some(window) = app.get_webview_window("command-palette") {
    let _ = window.close();
  }
}

#[tauri::command]
fn get_theme(app: tauri::AppHandle) -> Result<String, String> {
  use tauri_plugin_store::StoreExt;
  let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
  Ok(store
    .get("theme")
    .and_then(|v| v.as_str().map(ToString::to_string))
    .unwrap_or_default())
}

#[tauri::command]
fn toggle_theme(app: tauri::AppHandle) -> Result<(), String> {
  use tauri_plugin_store::StoreExt;
  let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
  let current = store
    .get("theme")
    .and_then(|v| v.as_str().map(ToString::to_string))
    .unwrap_or_else(|| "light".to_string());
  let next = if current == "light" { "dark" } else { "light" };
  store.set("theme", next);
  store.save().map_err(|e| e.to_string())?;
  app.emit("theme-changed", next).map_err(|e| e.to_string())?;
  Ok(())
}
