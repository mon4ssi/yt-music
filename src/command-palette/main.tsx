import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import CommandPalette from './CommandPalette'
import '../styles/theme.css'
import './CommandPalette.css'
import { initTheme } from '../bridge/theme.ts'

initTheme()

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <CommandPalette />
  </StrictMode>,
)
