# Current Development: 自動書籍章節產生功能

## 目標

當 Book Project 頁面「+」按鈕被點擊時，彈出對話框詢問：
- 書名（可編輯，預設帶入現有書名）
- 章節數（必填）
- 其他補充（可選填）

用戶填寫後，組合 prompt，帶入 project metadata，呼叫 Gemini API 產生章節標題與建議內容。用戶可編輯草稿，按「接受」後，將章節存入 DB。

---

## 1. UI/元件設計

### 1.1 ChapterDraftDialog.vue
- Props:
  - `bookTitle: string`
  - `defaultChapterCount: number`
  - `visible: boolean`
- Emits:
  - `confirm({ bookTitle, chapterCount, extraInfo })`
  - `cancel`
- Slots:
  - 自訂標題/說明
- 功能：
  - 表單輸入（書名、章節數、補充說明）
  - 「產生草稿」按鈕
  - Loading 狀態
  - 顯示 AI 產生的章節標題與內容（可編輯）
  - 「接受」/「取消」按鈕

#### 元件範本
```vue
<template>
  <Modal v-if="visible" @close="$emit('cancel')">
    <h2>自動產生章節草稿</h2>
    <form @submit.prevent="onGenerate">
      <label>書名</label>
      <input v-model="localBookTitle" />
      <label>章節數</label>
      <input type="number" v-model.number="chapterCount" min="1" />
      <label>其他補充（可選）</label>
      <input v-model="extraInfo" />
      <button type="submit" :disabled="loading">產生草稿</button>
    </form>
    <div v-if="loading">AI 產生中...</div>
    <div v-if="chapters.length">
      <div v-for="(ch, i) in chapters" :key="i">
        <input v-model="ch.title" />
        <textarea v-model="ch.content" />
      </div>
      <button @click="onAccept">接受</button>
    </div>
    <button @click="$emit('cancel')">取消</button>
  </Modal>
</template>
<script setup lang="ts">
import { ref } from 'vue'
const props = defineProps<{ bookTitle: string, defaultChapterCount: number, visible: boolean }>()
const emit = defineEmits(['confirm', 'cancel'])
const localBookTitle = ref(props.bookTitle)
const chapterCount = ref(props.defaultChapterCount)
const extraInfo = ref('')
const loading = ref(false)
const chapters = ref<{ title: string, content: string }[]>([])
const onGenerate = async () => {
  loading.value = true
  // 呼叫 API 產生章節
  // ...
  loading.value = false
}
const onAccept = () => {
  emit('confirm', { bookTitle: localBookTitle.value, chapterCount: chapterCount.value, extraInfo: extraInfo.value, chapters: chapters.value })
}
</script>
```

---

## 2. Prompt 組合範例

```text
請根據以下資訊，幫我產生本書的章節標題與每章建議內容：
書名：「{{bookTitle}}」
章節數：{{chapterCount}}
其他補充：{{extraInfo}}
專案資訊：{{projectInfo}}
請用如下格式：
1. 章節標題一\n- 建議內容...
2. 章節標題二\n- 建議內容...
...
```

---

## 3. API 實作細節

### 3.1 前端
- 呼叫 `/api/ai/generate-chapters`，POST，body:
```json
{
  "bookTitle": "...",
  "chapterCount": 10,
  "extraInfo": "...",
  "projectInfo": { ... }
}
```
- 回傳：
```json
{
  "chapters": [
    { "title": "章節一", "content": "..." },
    ...
  ]
}
```

### 3.2 後端（Rust/Tauri）
- 接收請求，組合 prompt，呼叫 Gemini API。
- 解析 AI 回應，轉為章節陣列。
- 回傳前端。
- 用戶按「接受」時，前端呼叫 `/api/chapters/batch-create`，將章節資料存入 DB。

---

## 4. 資料結構

- 章節：
```ts
interface Chapter {
  id?: number
  project_id: number
  title: string
  content: string
  chapter_number: number
  created_at?: string
  updated_at?: string
}
```

---

## 5. 開發步驟
1. 建立 ChapterDraftDialog.vue 元件
2. 實作 prompt 組合與 API 呼叫
3. 後端串接 Gemini API，解析回應
4. 前端顯示/編輯章節草稿
5. 按「接受」時批次寫入章節
6. UI/UX 微調與測試

---

## 6. 章節內容編輯頁（Chapter Content Editor）
- 新增章節後，點 sidebar 章節可進入 chapter content 編輯頁。
- 編輯頁設計：
  1. 介面類似 NewBlogArticle.vue，可編輯章節標題與內文。
  2. 具備 AI 協作（右側 AI assistant），可共用 DraftView 與 AI 對話區。
  3. prompt 輸入區可針對章節自訂。
- 章節內容頁與 blog/article 編輯頁元件高度共用，支援 DraftView、AI 協作、即時儲存、Undo/Redo。
- 需求同步於本文件，作為開發依據。

---

> 本文件為本階段開發唯一依據，所有規格、資料結構、API 需同步於 .cursor/ 相關文件。 