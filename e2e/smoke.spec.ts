import { test, expect } from '@playwright/test'

test('renders scaffold page', async ({ page }) => {
  await page.goto('/')
  await expect(page.getByRole('heading', { name: 'Scaffold Ready' })).toBeVisible()
  await expect(page.getByText('Tauri v2 and React 19 are wired.')).toBeVisible()
})
