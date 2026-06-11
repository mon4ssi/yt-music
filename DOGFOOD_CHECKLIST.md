# RC Dogfood Checklist

Run through each item on a release build. Check off when verified.

## Installation

- [ ] App launches successfully after install
- [ ] Version matches expected tag (check Help > About or footer)
- [ ] No crash on first launch

## Window Management

- [ ] Main window opens at 1280x800
- [ ] Window position is restored on relaunch (within same session)
- [ ] Window size is restored on relaunch
- [ ] Single-instance: second launch focuses existing window instead of creating a new one

## Navigation

- [ ] Main window loads music.youtube.com
- [ ] Google sign-in works (redirect to accounts.google.com allowed)
- [ ] Navigation outside allowed domains (music.youtube.com, accounts.google.com, accounts.youtube.com) is blocked
- [ ] Navigation Guard: typing a blocked URL does not navigate
- [ ] Auth session persists across app restarts

## Tray Menu

- [ ] Tray icon is visible in menu bar
- [ ] Play/Pause works from tray menu
- [ ] Next Track works from tray menu
- [ ] Previous Track works from tray menu
- [ ] Toggle Mini-Player works from tray menu
- [ ] Quit exits the app cleanly

## Media Keys

- [ ] Media Play/Pause key controls playback
- [ ] Media Next Track key skips forward
- [ ] Media Previous Track key skips backward
- [ ] Media keys work when app is in background

## Command Palette (Cmd+K / Ctrl+K)

- [ ] Opens with Cmd+K on macOS
- [ ] Opens with Ctrl+K on non-macOS
- [ ] Closes with Escape
- [ ] Search filters actions by label
- [ ] Arrow keys navigate the list
- [ ] Enter executes the selected action
- [ ] Play/Pause action works
- [ ] Next Track action works
- [ ] Previous Track action works
- [ ] Toggle Mini-Player action works
- [ ] Toggle Theme action works
- [ ] Toggle Telemetry action works
- [ ] Export Diagnostics copies JSON to clipboard
- [ ] Go to Home / Explore / Library navigation works
- [ ] Check for Updates action runs without error (may report no updates)
- [ ] Switch to Beta/Stable Channel persists across palette reopen

## Mini-Player

- [ ] Opens from tray or command palette
- [ ] Window is always-on-top
- [ ] Window is frameless
- [ ] Play/Pause button works
- [ ] Next Track button works
- [ ] Previous Track button works
- [ ] Mini-player closes properly
- [ ] Mini-player does not appear in taskbar/dock
- [ ] Focus main window from mini-player works (click or close)

## Theme

- [ ] Toggle Theme switches between light and dark
- [ ] Theme preference persists across restart
- [ ] All overlay windows (mini-player, command palette) respect the theme

## Bridge Health

- [ ] Command palette footer shows health indicator dot
- [ ] Health indicator is green (healthy) when playback is active
- [ ] Indicator is yellow during startup
- [ ] Watchdog re-injects content script if bridge becomes stale
- [ ] get_bridge_health command returns valid report

## Notifications

- [ ] Track change notification appears
- [ ] Notification is dismissible

## Autostart

- [ ] Toggle autostart in settings persists
- [ ] App launches on login when autostart is enabled

## Telemetry & Diagnostics

- [ ] Telemetry is disabled by default
- [ ] Toggle Telemetry persists the setting
- [ ] Export Diagnostics produces valid JSON
- [ ] Diagnostics JSON includes: app_version, platform, telemetry_enabled, uptime_secs, bridge_health, events

## Updates

- [ ] get_update_channel returns "stable" by default
- [ ] set_update_channel accepts "stable" and "beta"
- [ ] set_update_channel rejects invalid channel names
- [ ] check_for_updates runs without error (may return no update available)

## Performance & Stability

- [ ] App runs for 30+ minutes without crashing
- [ ] Memory usage stays stable over time
- [ ] CPU usage is idle when music is paused
- [ ] Switching tracks does not cause visual glitches

## Build & Packaging

- [ ] CI workflow completes successfully
- [ ] DMG mounts and drag-to-Applications install works
- [ ] App is signed (verify with `codesign -dv /Applications/YT\ Music.app`)
- [ ] App is notarized (verify with `spctl --assess --verbose /Applications/YT\ Music.app`)
