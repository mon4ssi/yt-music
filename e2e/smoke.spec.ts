import { test, expect } from '@playwright/test'

test('app shell loads without error', async ({ page }) => {
  await page.goto('/')
  await expect(page.locator('#root')).toBeAttached()
})
