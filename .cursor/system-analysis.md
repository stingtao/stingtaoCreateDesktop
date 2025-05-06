# AI Writing Assistant 系統分析文件（2024/06）

## 1. 系統架構總覽

- **前端**：Vue3 + TypeScript，Tauri 桌面應用，三欄式 UI（Navigation / Editor / AI Assistant）。
- **後端**：Rust（Tauri），本地 SQLite 資料庫，Gemini API、未來將支援 Gemma 小模型。
- **AI 服務**：多 agent 協作（DraftGenerator, Planning, Research, Editor, Reviewer, InlineEditor），RAG（Retrieval-Augmented Generation）語意檢索。
- **資料流**：前端觸發 AI 請求 → Rust 後端組合 prompt/context → 呼叫 LLM → 儲存 reasoning/suggestion/RAG → 回傳前端顯示。

## 2. 主要模組與資料流

### 前端
- **Editor**：Markdown 編輯、split view、行號、語法高亮、Undo/Redo、auto-save、diff view、inline AI 編輯。
- **AI Assistant**：多 agent 選擇、對話歷史、引用選取文字、建議接受/拒絕。
- **Project/Article 管理**：專案設定、文章版本、歷史、匯出/匯入。
- **Book Project 管理**：
  - 支援多書本專案，每本書可包含多章節（Chapter）。
  - 書本專案有獨立 metadata（如主題、目標、關鍵字、描述、進度等）。
  - 書本層級可呼叫 AI 產生全書大綱、風格建議、內容一致性檢查。
  - ChapterListComponent：章節清單、拖曳排序、CRUD。
  - BookOutlineComponent：AI 產生/編輯大綱、章節標題、結構調整。
  - BookAIOrchestration：書本層級的 AI 協作（如全書風格建議、內容一致性檢查）。
  - 與 Editor、RAG、Prompt、MCP、AI Agent Orchestration 等元件可組合。

### 後端
- **AI Agent 管理**：每個 agent 有獨立推理、建議、RAG 檢索記錄，皆存本地 DB。
- **RAG**：根據專案設定、舊文章、關鍵字等 context，檢索語意相關內容輔助 LLM。
- **資料儲存**：所有文章、推理、建議、RAG 結果、MCP（未來）皆本地化儲存，支援匯出。
- **Book/Chapter 結構管理**：
  - Book（書本）與 Chapter（章節）一對多關係。
  - Book 可有 outline、progress、metadata、AI 建議等欄位。
  - Chapter 可有 AI 建議、推理、RAG 結果、版本歷史等欄位。

### 資料流
1. 使用者建立/管理書本專案，設定 metadata。
2. 新增/編輯章節，AI 可協助產生大綱、章節標題、初稿、brainstorming。
3. 書本層級可呼叫 AI 產生全書大綱、風格建議、內容一致性檢查。
4. 章節內容可呼叫 AI 產生草稿、重寫、批改、風格調整。
5. 所有 AI 推理、建議、RAG 結果皆存 DB，支援查詢/回溯。

## 3. AI Agent 協作流程

- **DraftGenerator**：根據專案目標/關鍵字產生草稿
- **Planning**：分析結構、建議大綱
- **Research**：批改、找出問題、給建議
- **Editor**：調整風格、長度、可讀性
- **Reviewer**：整合所有建議，產生最終出版建議
- **InlineEditor**：針對選取段落/標題即時 AI 改寫
- **RAG**：每個 agent 可檢索專案/舊文 context
- **MCP（即將開發）**：每次修改自動萃取進化型 prompt，未來可用於個人化微調
- **Book/Chapter 協作**：AI agent 可於書本/章節層級協作，提升內容一致性與知識積累。

## 4. 資料管理與儲存

- **本地 SQLite**：所有書本、章節、文章、推理、建議、RAG 結果、MCP、版本歷史皆本地儲存
- **向量資料庫（即將開發）**：語意向量儲存與檢索，強化 RAG
- **進度追蹤**：progress.md、PRD.md 為規格與進度唯一依據

## 5. 未來擴充性與最佳化建議

- **書本/章節/草稿的版本管理與 AI 協作記錄**。
- **明確區分 Book/Blog/Project 結構，提升可維護性。**
- **支援 outline/ai_suggestions 欄位，方便大綱、AI 草稿、brainstorming 記錄。**
- **資料結構同步**：TS/Rust schema 需同步，並於 .cursor/data-structure.md 明確標註。
- **版本管理**：可考慮為 Book/Chapter 增加 version/history 支援。

## 6. 元件化設計與可重用模組（Componentization & Reusable Modules）

### 1. 編輯元件（Editor Component）
- 提供 Markdown 編輯、split view、行號、語法高亮、Undo/Redo、auto-save、diff view、inline AI 編輯等功能。
- 支援「文章」、「章節」、「書本」等不同層級的內容編輯。
- 可根據 props/參數切換模式（如：章節編輯、全書大綱編輯、單篇文章編輯）。

### 2. RAG 元件（RAG/Context Retrieval Component）
- 封裝語意檢索、向量查詢、context 組合等功能。
- 可被 DraftGenerator、Planning、Editor、Reviewer 等 agent 調用，支援跨文章、跨章節、跨專案 context。
- 支援自定義檢索來源（如：本書所有章節、同主題舊文章、用戶自定知識庫）。

### 3. Prompt 元件（Prompt Builder/Manager）
- 封裝 prompt 組合、MCP（Meta-Chain Prompt）進化、作者風格學習等功能。
- 可根據不同 agent、不同內容類型（章節/文章/書本）自動組合最適 prompt。
- 支援 prompt 記錄、查詢、可視化，方便後續微調與個人化。

### 4. AI Agent 協作元件（Agent Orchestration）
- 封裝多 agent 協作流程，支援 DraftGenerator、Planning、Editor、Reviewer、InlineEditor 等 agent 的串接與切換。
- 可根據不同畫面（如章節、全書、文章）自動分配適合的 agent 流程。

### 5. 章節/書本結構元件（Book/Chapter Structure Component）
- 支援書本多章節結構的建立、編輯、重組、拖曳排序。
- 可與 Editor、RAG、AI Agent 元件組合，實現章節級 AI 輔助。
- **新增：**
  - BookProjectComponent：管理書本專案 CRUD、顯示進度、設定 metadata。
  - ChapterListComponent：顯示/管理章節清單、拖曳排序、章節 CRUD。
  - BookOutlineComponent：AI 產生/編輯大綱、章節標題、結構調整。
  - BookAIOrchestration：書本層級的 AI 協作（如全書風格建議、內容一致性檢查）。

## 7. 進階資料調用與內容擴充（Advanced Data Retrieval & Content Enrichment）

### 1. MCP 檔案調用
- 作者可透過 MCP（Meta-Chain Prompt）調用過去寫作檔案（Google Cloud 或 local storage），供 LLM 參考，提升個人化生成能力。
- MCP 調用流程：用戶選擇/指定檔案 → 系統檢索並摘要內容 → LLM 生成時自動引用該內容。
- 元件化設計：MCP 檔案選擇器、內容摘要器、context 組合器，皆可於不同寫作場景（書本/章節/文章）重用。

### 2. URL 匯入與摘要
- 支援用戶輸入 YouTube 或外部網站 URL，自動擷取內容、摘要重點，並存入本地資料庫。
- 匯入流程：用戶輸入 URL → 系統爬取/解析 → AI 摘要 → 存檔並可標註為靈感來源。
- 元件化設計：URL 匯入元件、內容爬取與摘要服務、資料管理模組，支援多場景調用。

---

（請將本章節與第 6 章元件化設計互相參照，確保 MCP 調用與 URL 匯入皆為可重用模組）

---

**設計理念補充：**
- 所有內容（書本、章節、文章、brainstorming）皆可透過上述元件組合實現，方便擴充與維護。
- 各元件應有明確 API/props 定義，方便跨畫面、跨流程調用。
- 未來如有新 agent 或新內容類型，只需擴充元件，不需重複開發。
- 元件化設計有助於 UI/UX 一致性、測試自動化、團隊協作。

## 8. Book/Chapter 版本管理 API 介面設計

### Book Version API
| Method | Endpoint | 說明 |
|--------|----------|------|
| GET    | `/api/books/:book_id/versions`         | 取得書本所有版本列表 |
| GET    | `/api/books/:book_id/versions/:ver_id` | 取得指定版本內容 |
| POST   | `/api/books/:book_id/versions`         | 新增一個版本（通常由系統自動呼叫）|
| POST   | `/api/books/:book_id/restore/:ver_id`  | 還原到指定版本 |

### Chapter Version API
| Method | Endpoint | 說明 |
|--------|----------|------|
| GET    | `/api/chapters/:chapter_id/versions`         | 取得章節所有版本列表 |
| GET    | `/api/chapters/:chapter_id/versions/:ver_id` | 取得指定版本內容 |
| POST   | `/api/chapters/:chapter_id/versions`         | 新增一個版本（通常由系統自動呼叫）|
| POST   | `/api/chapters/:chapter_id/restore/:ver_id`  | 還原到指定版本 |

### 進階功能建議
- 支援 diff API：`GET /api/chapters/:chapter_id/diff?from=:ver1&to=:ver2`
- 支援版本標註（如「重要」、「AI 產生」、「人工修改」等 tag）

### UI/UX 建議
- 書本/章節編輯頁面可提供「版本歷史」側欄，支援瀏覽、比較、還原。
- 版本列表可顯示 version_number、created_at、change_summary、標註。
- 支援一鍵還原、版本 diff、AI 產生版本摘要。 