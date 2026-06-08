import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { BridgeHealth } from '../bridge/types'

interface Action {
  id: string
  label: string
  icon: string
  invokeCmd: string
  invokeArgs?: Record<string, string>
}

const ACTIONS: Action[] = [
  { id: 'play_pause', label: 'Play / Pause', icon: '▶', invokeCmd: 'toggle_playback' },
  { id: 'next', label: 'Next Track', icon: '⏭', invokeCmd: 'next_track' },
  { id: 'previous', label: 'Previous Track', icon: '⏮', invokeCmd: 'previous_track' },
  { id: 'mini_player', label: 'Toggle Mini-Player', icon: '🎛', invokeCmd: 'toggle_mini_player' },
  { id: 'toggle_theme', label: 'Toggle Theme', icon: '🌗', invokeCmd: 'toggle_theme' },
  { id: 'home', label: 'Go to Home', icon: '🏠', invokeCmd: 'navigate_to', invokeArgs: { page: 'home' } },
  { id: 'explore', label: 'Go to Explore', icon: '🔍', invokeCmd: 'navigate_to', invokeArgs: { page: 'explore' } },
  { id: 'library', label: 'Go to Library', icon: '📚', invokeCmd: 'navigate_to', invokeArgs: { page: 'library' } },
]

function CommandPalette() {
  const inputRef = useRef<HTMLInputElement>(null)
  const [query, setQuery] = useState('')
  const [selectedIdx, setSelectedIdx] = useState(0)
  const [health, setHealth] = useState<BridgeHealth | null>(null)

  useEffect(() => {
    invoke<BridgeHealth>('get_bridge_health').then(setHealth).catch(console.error)
  }, [])

  const healthColor = health
    ? health.status === 'healthy' ? 'var(--health-ok)' : health.status === 'starting' ? 'var(--health-warn)' : 'var(--health-err)'
    : 'var(--health-warn)'

  const filtered = useMemo(
    () => ACTIONS.filter((a) => a.label.toLowerCase().includes(query.toLowerCase())),
    [query],
  )

  const close = useCallback(() => {
    invoke('close_palette').catch(console.error)
  }, [])

  const run = useCallback(
    (action: Action) => {
      invoke(action.invokeCmd, action.invokeArgs ?? {}).catch(console.error)
      close()
    },
    [close],
  )

  const closeRef = useRef(close)
  const runRef = useRef(run)
  const filteredRef = useRef(filtered)
  const selectedIdxRef = useRef(selectedIdx)
  useEffect(() => {
    closeRef.current = close
    runRef.current = run
    filteredRef.current = filtered
    selectedIdxRef.current = selectedIdx
  })

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        closeRef.current()
      } else if (e.key === 'ArrowDown') {
        e.preventDefault()
        setSelectedIdx((i) => Math.min(i + 1, filteredRef.current.length - 1))
      } else if (e.key === 'ArrowUp') {
        e.preventDefault()
        setSelectedIdx((i) => Math.max(i - 1, 0))
      } else if (e.key === 'Enter') {
        e.preventDefault()
        const idx = selectedIdxRef.current
        const items = filteredRef.current
        if (items[idx]) runRef.current(items[idx])
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [])

  useEffect(() => {
    inputRef.current?.focus()
  }, [])

  return (
    <div className="palette-overlay">
      <input
        ref={inputRef}
        className="palette-input"
        type="text"
        placeholder="Type a command..."
        value={query}
        onChange={(e) => {
          setQuery(e.target.value)
          setSelectedIdx(0)
        }}
      />
      <ul className="palette-list" role="listbox">
        {filtered.map((action, i) => (
          <li
            key={action.id}
            className={`palette-item ${i === selectedIdx ? 'palette-item--active' : ''}`}
            role="option"
            aria-selected={i === selectedIdx}
            onClick={() => run(action)}
            onMouseEnter={() => setSelectedIdx(i)}
          >
            <span className="palette-icon">{action.icon}</span>
            <span className="palette-label">{action.label}</span>
          </li>
        ))}
        {filtered.length === 0 && (
          <li className="palette-empty">No matching commands</li>
        )}
      </ul>
      <div className="palette-footer">
        <span className="palette-health" style={{ backgroundColor: healthColor }} title={health ? `Bridge: ${health.status} (${health.lastHeartbeatMsAgo}ms ago)` : 'Loading...'} />
        <span className="palette-version">v0.1.0</span>
      </div>
    </div>
  )
}

export default CommandPalette
