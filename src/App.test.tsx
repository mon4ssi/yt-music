import { render } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import App from './App'

describe('App', () => {
  it('renders nothing (main window navigates directly to YTM)', () => {
    const { container } = render(<App />)
    expect(container.textContent).toBe('')
  })
})
