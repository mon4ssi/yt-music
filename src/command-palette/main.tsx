import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import CommandPalette from './CommandPalette'
import './CommandPalette.css'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <CommandPalette />
  </StrictMode>,
)
