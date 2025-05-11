import { test, expect } from '@playwright/test'

test('用戶可建立 Blog 專案並完成所有步驟', async ({ page }) => {
  // 1. 首頁
  await page.goto('http://localhost:4000/')
  await page.click('.new-project-btn')

  // 2. 選擇 Blog 專案
  await page.click('.project-type-card:has-text("Blog")')
  await page.click('button:has-text("下一步")')

  // 3. 填寫目標
  await page.fill('textarea#project-goal', '成為 AI 寫作達人')
  await page.click('button:has-text("下一步")')

  // 4. 填寫詳細資料
  await page.fill('input#project-title', 'AI 寫作部落格')
  await page.selectOption('select#blog-category', 'technology')
  await page.fill('textarea#project-description', '這是一個關於 AI 寫作的部落格')
  await page.fill('textarea#target-audience', '對 AI 寫作有興趣的初學者')
  await page.click('button:has-text("下一步")')

  // 5. 時間承諾
  await page.selectOption('select#time-commitment', 'casual')
  await page.fill('input#deadline', '2099-12-31')
  await page.click('button:has-text("下一步")')

  // 6. 發布平台（如有，若無可略過）
  if (await page.locator('button:has-text("下一步")').isVisible()) {
    await page.click('button:has-text("下一步")')
  }

  // 7. 計畫
  await page.click('button:has-text("建立")')

  // 8. 驗證導向專案頁或顯示成功訊息
  await expect(page).toHaveURL(/projects/)
  await expect(page.locator('text=AI 寫作部落格')).toBeVisible()
}) 