import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

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
  { id: 'home', label: 'Go to Home', icon: '🏠', invokeCmd: 'navigate_to', invokeArgs: { page: 'home' } },
  { id: 'explore', label: 'Go to Explore', icon: '🔍', invokeCmd: 'navigate_to', invokeArgs: { page: 'explore' } },
  { id: 'library', label: 'Go to Library', icon: '📚', invokeCmd: 'navigate_to', invokeArgs: { page: 'library' } },
]

function CommandPalette() {
  const inputRef = useRef<HTMLInputElement>(null)
  const [query, setQuery] = useState('')
  const [selectedIdx, setSelectedIdx] = useState(0)

  const filtered = useMemo(
    () => ACTIONS.filter((a) => a.label.toLowerCase().includes(query.toLowerCase())),
    [query],
  )

  const close = useCallback(() => {
    getCurrentWindow().close()
  }, [])

  const run = useCallback(
    (action: Action) => {
      invoke(action.invokeCmd, action.invokeArgs ?? {})
      close()
    },
    [close],
  )

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === 'Escape') {
        close()
      } else if (e.key === 'ArrowDown') {
        e.preventDefault()
        setSelectedIdx((i) => Math.min(i + 1, filtered.length - 1))
      } else if (e.key === 'ArrowUp') {
        e.preventDefault()
        setSelectedIdx((i) => Math.max(i - 1, 0))
      } else if (e.key === 'Enter') {
        e.preventDefault()
        if (filtered[selectedIdx]) run(filtered[selectedIdx])
      }
    },
    [close, filtered, selectedIdx, run],
  )

  useEffect(() => {
    inputRef.current?.focus()
  }, [])

  return (
    <div className="palette-overlay" onKeyDown={handleKeyDown}>
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
    </div>
  )
}

export default CommandPalette
