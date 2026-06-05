import { useCallback, useEffect, useRef, useState } from 'react'
import './App.css'

const YT_MUSIC_URL = 'https://music.youtube.com'
const LOAD_TIMEOUT_MS = 30_000

function App() {
  const iframeRef = useRef<HTMLIFrameElement>(null)
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | undefined>(undefined)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(false)

  const clearTimer = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current)
      timeoutRef.current = undefined
    }
  }, [])

  const startTimer = useCallback(() => {
    clearTimer()
    timeoutRef.current = setTimeout(() => {
      setLoading(false)
      setError(true)
    }, LOAD_TIMEOUT_MS)
  }, [clearTimer])

  const handleLoad = useCallback(() => {
    clearTimer()
    setLoading(false)
    setError(false)
  }, [clearTimer])

  const handleRetry = useCallback(() => {
    setError(false)
    setLoading(true)
    startTimer()
    if (iframeRef.current) {
      iframeRef.current.src = YT_MUSIC_URL
    }
  }, [startTimer])

  useEffect(() => {
    startTimer()
    return clearTimer
  }, [startTimer, clearTimer])

  return (
    <div className="app-container">
      {loading && (
        <div className="app-overlay">
          <p className="app-status">Loading YouTube Music...</p>
        </div>
      )}
      {error && (
        <div className="app-overlay app-overlay--error">
          <h2 className="app-error-heading">Unable to load YouTube Music</h2>
          <p className="app-error-desc">
            Check your internet connection and try again.
          </p>
          <button className="app-retry-btn" onClick={handleRetry} type="button">
            Retry
          </button>
        </div>
      )}
      <iframe
        ref={iframeRef}
        className="app-iframe"
        src={YT_MUSIC_URL}
        title="YouTube Music"
        onLoad={handleLoad}
        allow="autoplay *; clipboard-write *; encrypted-media *; fullscreen *"
      />
    </div>
  )
}

export default App
