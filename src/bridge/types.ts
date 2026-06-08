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
