import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export async function initTheme() {
  const theme = await invoke<string>('get_theme')
  if (theme) {
    document.documentElement.dataset.theme = theme
  }
  await listen<string>('theme-changed', (e) => {
    document.documentElement.dataset.theme = e.payload
  })
}
