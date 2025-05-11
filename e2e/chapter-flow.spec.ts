import { test, expect } from '@playwright/test'

test.describe('書籍章節主流程', () => {
  test('用戶可產生章節草稿並審查/接受', async ({ page }) => {
    await page.goto('http://localhost:4000/')
    // 假設首頁有「Book Projects」按鈕
    await page.click('text=Book Projects')
    // 點擊某個書籍專案
    await page.click('text=AI 寫作入門')
    // 點擊「+」產生章節
    await page.click('button:has-text("+")')
    // 填寫章節產生表單
    await page.fill('input[placeholder="書名"]', 'AI 寫作入門')
    await page.fill('input[type="number"]', '3')
    await page.fill('textarea', '希望每章有小結')
    await page.click('button:has-text("產生草稿")')
    // 等待 AI 產生
    await page.waitForSelector('input[placeholder="章節標題"]')
    // 編輯第一章標題
    await page.fill('input[placeholder="章節標題"]', 'AI 是什麼？')
    // 接受草稿
    await page.click('button:has-text("接受")')
    // 跳轉到審查頁
    await expect(page).toHaveURL(/review-chapters/)
    // 檢查章節出現
    await expect(page.locator('text=AI 是什麼？')).toBeVisible()
  })
}) 