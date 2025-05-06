# AI Writing Assistant — Codebase 現況對照 PRD（2024/06）

---

## 主要功能

- 書本計劃管理：用戶可建立書本專案，支援多章節結構，每章節可獨立編輯、管理。
- 章節大綱與草稿產生：AI 可根據書本主題、目標、關鍵字，協助產生全書章節大綱、每章標題與初稿（draft）、brainstorming 建議。
- 章節內容 AI 輔助：每章節可獨立呼叫 AI 產生內容草稿、重寫、批改、風格調整等。
- 作者風格學習：AI 會在用戶多次修改、反饋過程中，學習作者的語氣、習慣、寫作精神，並逐步進化後續 draft 與建議的個人化程度。
- 進化式建議產生：每次 AI 產生 draft 或建議時，會根據作者過去的修改紀錄與偏好，持續優化內容與建議品質。
- 透過 MCP 調用作者過去寫作檔案（可來自 Google Cloud 或 local storage），供 LLM 參考與個人化生成。
- 支援輸入 URL 匯入 YouTube 或外部網站資料，自動 summarize 並存檔，作為寫作資料與靈感來源。

## 已完成

- **三欄式編輯器**：支援 Navigation、Editor、AI Assistant，面板可調整寬度、可收合，響應式設計。
- **Markdown 編輯器**：支援 live preview、行號、語法高亮、split view。
- **自動儲存**：10 秒 debounce，可設定，支援本地備份、錯誤處理、儲存狀態指示、路由保護。
- **Undo/Redo 歷史**：內容與標題皆支援多層歷史、快捷鍵、UI 按鈕。
- **AI 助手多 agent 協作**：DraftGenerator、Planning、Research（Critique）、Editor（Style & Length）、Reviewer（Final Review）、InlineEditor，皆有對應後端與前端串接。
- **Chain of Thought reasoning**：每個 agent 產生推理過程，並存本地資料庫。
- **RAG（檢索增強生成）**：已串接專案設定、舊文章、關鍵字等 context，檢索結果存資料庫。
- **AI 對話/建議**：AI Assistant 面板支援多 agent、對話歷史、引用選取文字、inline AI 編輯、diff view、接受/拒絕。
- **本地資料管理**：AI 建議、推理、RAG 結果、文章版本皆有本地儲存，支援匯出/匯入。
- **狀態指示/保護**：儲存/已儲存狀態、未儲存變更保護、跨路由自動儲存與 guard。
- **UI/UX**：大部分 PRD 需求已覆蓋，細節如多主題、進階 diff、協作、行動裝置、進階 SEO、外部發佈串接等為未來增強方向。

---

## 進行中/未完成

- **多版本草稿選擇**：目前僅單一草稿，未支援多版本選擇與比較。
- **A/B 風格比較**：尚未有多種風格即時比較與切換。
- **可視化大綱/拖曳**：Planning agent 已有大綱建議，尚未有可視化與拖曳重組 UI。
- **出版檢查清單/分數**：Reviewer agent 已有最終建議，出版分數、檢查清單、one-click apply UI 尚未完全。
- **進階 diff view**：目前僅支援 inline diff，尚未有完整段落/多版本 diff。
- **協作編輯/多人同步**：尚未實作。
- **行動裝置優化**：UI 響應式已初步支援，尚未針對行動裝置深度優化。
- **外部平台串接/進階 SEO**：尚未實作。
- **主題切換/多主題**：僅有淺色/深色主題，尚未支援多主題與自訂。
- **離線/同步**：尚未支援離線編輯與跨裝置同步。

### 即將開發

- 向量資料庫整合：導入語意向量資料庫，強化 AI 檢索與內容推薦能力。
- MCP（Meta-Chain Prompt）進化：每次文章修改自動萃取進化型 prompt，持續優化 AI 寫作輔助。
- 跨文章內容讀取：AI agent 可讀取、分析所有過去文章，提升內容一致性與知識積累。
- Gemma 小模型微調：根據每次文章 diff，自動收集資料微調本地 Gemma 小模型，打造個人化 AI 寫作助手。
- Gemma 小模型 prompt 產生：使用 Gemma 小模型自動產生 prompt，提升 AI 回應多樣性與上下文適應性。

---

## 技術基礎

- **架構**：Tauri + Vue3 + Rust，資料庫本地儲存（SQLite），Gemini API，RAG context。
- **資料管理**：AI agent 推理、建議、RAG 檢索結果、文章版本皆存本地資料庫，支援匯出/匯入。
- **AI 串接**：多 agent 協作、Chain of Thought、RAG、inline AI 編輯、diff view、AI 對話歷史。
- **UI/UX**：以 Markdown 寫作為核心，支援 split view、行號、語法高亮、狀態指示、快捷鍵、浮動工具列、inline AI 編輯。

---

## PRD 文件維護規則（Cursor Rule）

- 本文件僅作為 codebase 實作現況的唯一權威說明。
- 每次 codebase 有重大結構或功能變動時，必須以『codebase 現況對照 PRD』格式直接覆蓋本文件。
- 內容必須聚焦於：
  1. 已完成
  2. 進行中/未完成
  3. 技術基礎
- 嚴禁僅做片段增補、補丁式修改，必須全檔覆蓋式更新。
- 任何人員查閱本文件時，皆可直接信賴其內容即為 codebase 現況。

> 本文件反映 2024/06 codebase 現況，未來如有重大功能進展請持續更新。

- 所有程式碼註解（comments）必須使用英文撰寫，但團隊內部溝通可用中文。 