use tauri::Manager;

#[derive(Debug)]
pub enum PlaybackCommand {
  PlayPause,
  Next,
  Previous,
  #[allow(dead_code)]
  Seek(f64),
}

fn js_for(cmd: &PlaybackCommand) -> String {
  match cmd {
    PlaybackCommand::PlayPause => {
      r#"(function(){var e=document.querySelector('.ytmusic-player-bar .play-pause-button, #play-pause-button, ytmusic-play-pause-button');if(e){e.click()}else{console.warn('yt-music: play/pause not found')}})()"#.into()
    }
    PlaybackCommand::Next => {
      r#"(function(){var e=document.querySelector('.ytmusic-player-bar .next-button, #next-button, ytmusic-next-button');if(e){e.click()}else{console.warn('yt-music: next not found')}})()"#.into()
    }
    PlaybackCommand::Previous => {
      r#"(function(){var e=document.querySelector('.ytmusic-player-bar .previous-button, #previous-button, ytmusic-previous-button');if(e){e.click()}else{console.warn('yt-music: previous not found')}})()"#.into()
    }
    PlaybackCommand::Seek(t) => {
      format!(
        r#"(function(){{var v=document.querySelector('video');if(v){{v.currentTime={t}}}else{{console.warn('yt-music: seek video not found')}}}})()"#,
        t = t
      )
    }
  }
}

pub fn execute(app: &tauri::AppHandle, cmd: PlaybackCommand) {
  let js = js_for(&cmd);
  if let Some(window) = app.get_webview_window("main") {
    if let Err(e) = window.eval(&js) {
      log::warn!("yt-music: command eval failed: {e:?}");
      crate::diagnostics::record(app, "error", &format!("command eval failed for {:?}: {e:?}", cmd), "commands::execute");
    }
  } else {
    log::warn!("yt-music: main window not found");
    crate::diagnostics::record(app, "warn", "main window not found for command execution", "commands::execute");
  }
}

pub fn navigate_to(app: &tauri::AppHandle, page: &str) {
  let js = match page {
    "home" => r#"(function(){var e=document.querySelector('a[href="/"], a[title="Home"], tp-yt-paper-item[aria-label="Home"]');if(e){e.click()}else{console.warn('yt-music: home nav not found')}})()"#,
    "explore" => r#"(function(){var e=document.querySelector('a[href="/explore"], a[title="Explore"], tp-yt-paper-item[aria-label="Explore"]');if(e){e.click()}else{console.warn('yt-music: explore nav not found')}})()"#,
    "library" => r#"(function(){var e=document.querySelector('a[href="/library"], a[title="Library"], tp-yt-paper-item[aria-label="Library"]');if(e){e.click()}else{console.warn('yt-music: library nav not found')}})()"#,
    _ => return,
  };
  if let Some(window) = app.get_webview_window("main") {
    if let Err(e) = window.eval(js) {
      log::warn!("yt-music: navigate_to eval failed: {e:?}");
      crate::diagnostics::record(app, "error", &format!("navigate_to eval failed: {e:?}"), "commands::navigate_to");
    }
  }
}
