# Test info

- Name: 用戶可建立 Blog 專案並完成所有步驟
- Location: /Users/stingtao/project_new/stingtaoCreate_0.1/e2e/blog-project-flow.spec.ts:3:1

# Error details

```
Error: page.click: Test timeout of 30000ms exceeded.
Call log:
  - waiting for locator('button:has-text("下一步")')

    at /Users/stingtao/project_new/stingtaoCreate_0.1/e2e/blog-project-flow.spec.ts:10:14
```

# Page snapshot

```yaml
- img
- text: stingtaoCreate v0.2
- button "Collapse Sidebar":
  - img
- navigation:
  - list:
    - listitem:
      - link "Home":
        - /url: /
    - list
    - listitem:
      - link "Settings":
        - /url: /settings
    - listitem:
      - link "Help":
        - /url: /help
- heading "Create New Project" [level=1]
- text: 1 Goals 2 Project Details 3 Time Commitment 4 Publishing Platform 5 Project Plan
- heading "Choose Project Type" [level=2]
- paragraph: Select the type of project you want to create
- text: 📝
- heading "Blog" [level=3]
- paragraph: Create blog posts, articles, or short-form content
- text: 📚
- heading "Book" [level=3]
- paragraph: Write books, novels, or long-form content
- button "Next"
```

# Test source

```ts
   1 | import { test, expect } from '@playwright/test'
   2 |
   3 | test('用戶可建立 Blog 專案並完成所有步驟', async ({ page }) => {
   4 |   // 1. 首頁
   5 |   await page.goto('http://localhost:4000/')
   6 |   await page.click('.new-project-btn')
   7 |
   8 |   // 2. 選擇 Blog 專案
   9 |   await page.click('.project-type-card:has-text("Blog")')
> 10 |   await page.click('button:has-text("下一步")')
     |              ^ Error: page.click: Test timeout of 30000ms exceeded.
  11 |
  12 |   // 3. 填寫目標
  13 |   await page.fill('textarea#project-goal', '成為 AI 寫作達人')
  14 |   await page.click('button:has-text("下一步")')
  15 |
  16 |   // 4. 填寫詳細資料
  17 |   await page.fill('input#project-title', 'AI 寫作部落格')
  18 |   await page.selectOption('select#blog-category', 'technology')
  19 |   await page.fill('textarea#project-description', '這是一個關於 AI 寫作的部落格')
  20 |   await page.fill('textarea#target-audience', '對 AI 寫作有興趣的初學者')
  21 |   await page.click('button:has-text("下一步")')
  22 |
  23 |   // 5. 時間承諾
  24 |   await page.selectOption('select#time-commitment', 'casual')
  25 |   await page.fill('input#deadline', '2099-12-31')
  26 |   await page.click('button:has-text("下一步")')
  27 |
  28 |   // 6. 發布平台（如有，若無可略過）
  29 |   if (await page.locator('button:has-text("下一步")').isVisible()) {
  30 |     await page.click('button:has-text("下一步")')
  31 |   }
  32 |
  33 |   // 7. 計畫
  34 |   await page.click('button:has-text("建立")')
  35 |
  36 |   // 8. 驗證導向專案頁或顯示成功訊息
  37 |   await expect(page).toHaveURL(/projects/)
  38 |   await expect(page.locator('text=AI 寫作部落格')).toBeVisible()
  39 | }) 
```