import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import MiniPlayer from './MiniPlayer'
import './MiniPlayer.css'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <MiniPlayer />
  </StrictMode>,
)
