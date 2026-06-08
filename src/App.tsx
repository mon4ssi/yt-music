import { useEffect } from 'react'
import './App.css'

const YT_MUSIC_URL = 'https://music.youtube.com'

function App() {
  useEffect(() => {
    const isTauri = '__TAURI_INTERNALS__' in window
    if (isTauri) {
      window.location.href = YT_MUSIC_URL
    }
  }, [])

  return (
    <div className="app-splash">
      <p className="app-splash-text">Loading YouTube Music...</p>
    </div>
  )
}

export default App
