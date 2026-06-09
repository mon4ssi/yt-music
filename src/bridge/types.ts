export interface PlaybackState {
  title: string
  artist: string
  album: string
  thumbnail: string
  isPlaying: boolean
  duration: number
  currentTime: number
  volume: number
}

export interface BridgeHealth {
  status: string
  lastHeartbeatMsAgo: number
  totalHeartbeats: number
  recoveryAttempts: number
}

export interface DiagnosticsEntry {
  timestamp: string
  level: string
  message: string
  location: string
}
