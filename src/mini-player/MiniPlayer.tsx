import { useCallback, useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import type { PlaybackState } from '../bridge/types'

function MiniPlayer() {
  const [state, setState] = useState<PlaybackState | null>(null)

  useEffect(() => {
    const unlisten = listen<PlaybackState>('playback-state-changed', (e) => {
      setState(e.payload)
    })
    return () => {
      unlisten.then((fn: () => void) => fn())
    }
  }, [])

  const handleExpand = useCallback(() => {
    invoke('focus_main_window')
  }, [])

  return (
    <div className="mini-player">
      {state?.thumbnail ? (
        <img className="mini-thumb" src={state.thumbnail} alt="" />
      ) : (
        <div className="mini-thumb mini-thumb--empty" />
      )}
      <div className="mini-info">
        <p className="mini-title">{state?.title || 'No track'}</p>
        <p className="mini-artist">{state?.artist || ''}</p>
      </div>
      <div className="mini-controls">
        <button className="mini-btn" onClick={() => invoke('previous_track')} aria-label="Previous">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><polygon points="19,20 9,12 19,4"/><rect x="5" y="4" width="2" height="16"/></svg>
        </button>
        <button className="mini-btn mini-btn--play" onClick={() => invoke('toggle_playback')} aria-label={state?.isPlaying ? 'Pause' : 'Play'}>
          {state?.isPlaying ? (
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
          ) : (
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><polygon points="8,5 19,12 8,19"/></svg>
          )}
        </button>
        <button className="mini-btn" onClick={() => invoke('next_track')} aria-label="Next">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><polygon points="5,4 15,12 5,20"/><rect x="17" y="4" width="2" height="16"/></svg>
        </button>
      </div>
      <button className="mini-expand" onClick={handleExpand} aria-label="Expand">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="15 3 21 3 21 9"/><polyline points="9 21 3 21 3 15"/><line x1="21" y1="3" x2="14" y2="10"/><line x1="3" y1="21" x2="10" y2="14"/></svg>
      </button>
    </div>
  )
}

export default MiniPlayer
