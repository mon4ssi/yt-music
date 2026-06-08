import { test, expect } from '@playwright/test'

test('renders splash before navigation', async ({ page }) => {
  await page.goto('/')
  await expect(page.getByText('Loading YouTube Music...')).toBeVisible()
})
