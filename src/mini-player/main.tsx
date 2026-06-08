import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import MiniPlayer from './MiniPlayer'
import '../styles/theme.css'
import './MiniPlayer.css'
import { initTheme } from '../bridge/theme.ts'

initTheme()

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <MiniPlayer />
  </StrictMode>,
)
