import { test, expect } from '@playwright/test'

test('renders YouTube Music iframe', async ({ page }) => {
  await page.goto('/')
  const iframe = page.locator('iframe[title="YouTube Music"]')
  await expect(iframe).toBeAttached()
  await expect(iframe).toHaveAttribute('src', 'https://music.youtube.com')
})
