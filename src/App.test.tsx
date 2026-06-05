import { render, screen } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import App from './App'

describe('App', () => {
  it('renders loading state initially', () => {
    render(<App />)
    expect(screen.getByText('Loading YouTube Music...')).toBeInTheDocument()
  })
})
