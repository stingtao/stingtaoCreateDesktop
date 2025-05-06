# 技術元件架構與 Refactor 建議（2024/06）

---

## 1. 前端（src/）主要元件/模組結構

- **Editor 元件**：Markdown 編輯、split view、行號、語法高亮、Undo/Redo、auto-save、diff view、inline AI 編輯。建議拆分為：
  - EditorCore（純編輯）
  - EditorToolbar（工具列/快捷操作）
  - EditorHistory（歷史/Undo/Redo）
  - EditorAI（AI 相關操作）
- **AI Assistant 元件**：多 agent 協作、對話歷史、引用選取文字、建議接受/拒絕。建議抽象為可插拔 agent 模組。
- **RAG/Prompt 元件**：context 檢索、prompt 組合、MCP 進化、作者風格學習。建議封裝為 composable 或 service，供各 agent/元件調用。
- **Project/Book/Chapter 管理**：專案、書本、章節、文章 CRUD 與結構管理，建議統一 interface，支援多層級內容。
- **Settings/Config**：全域設定、API 金鑰、偏好等，建議集中管理。
- **資料結構**：所有 interface/type 應集中於 lib/db.ts、lib/content.ts、lib/project.ts，並與後端 schema 對齊。

## 2. 後端（src-tauri/）主要模組/服務

- **DB 模組**：Project、Blog、Chapter、AgentReasoning、AgentSuggestion、RagRetrieval 等 struct 與 CRUD。
- **AI Agent 模組**：DraftGenerator、Planning、Research、Editor、Reviewer、InlineEditor，皆有推理、建議、RAG 檢索、MCP 支援。
- **RAG/Context 檢索**：支援向量資料庫、語意檢索、context 組合。
- **MCP/Prompt 進化**：每次修改自動萃取 prompt，支援個人化學習。
- **外部 API 整合**：Gemini、Gemma、YouTube/網站爬取與摘要。
- **資料結構**：Rust struct 應與前端 interface 對齊，便於序列化與資料同步。

## 3. 前後端資料結構對應

- Project/Blog/Chapter/BlogArticle/Settings/AgentReasoning/AgentSuggestion/RagRetrieval 皆有 TS interface 與 Rust struct 對應，建議維持 schema 一致性。
- 建議所有資料結構異動時，前後端同步更新 .cursor/data-structure.md。

## 4. Refactor/重構建議

- **元件拆分與共用**：將大型元件（如 Editor、AI Assistant）拆分為小型可重用元件，提升維護性。
- **composables/service 抽象**：RAG、Prompt、AI agent 流程抽象為 composable/service，減少重複邏輯。
- **API 統一**：前後端 API 命名、參數、回傳格式統一，便於自動化測試與型別檢查。
- **資料流優化**：明確劃分資料流向（單向/雙向），減少 props drilling，善用 context/provider。
- **型別/Schema 驗證**：引入 schema 驗證工具（如 zod、serde），確保資料正確性。
- **測試覆蓋**：針對元件、service、API 增加單元測試與整合測試。
- **文件自動化**：每次 schema/元件異動自動更新 .cursor/data-structure.md。

## 5. 維護、擴充、減少技術債設計原則

- **高內聚、低耦合**：元件/模組職責單一，依賴明確。
- **可插拔/可擴充**：新 agent、新內容類型可輕鬆擴充，不影響現有功能。
- **資料結構/型別同步**：前後端 schema 變更需同步維護。
- **集中設定/規範**：所有規範、設計、進度、資料結構皆集中於 .cursor/ 目錄。
- **自動化流程**：測試、文件、型別檢查自動化，減少人為疏漏。

## 6. 自動化型別/文件同步與架構最佳化建議

### 1. schema/type 變更自動同步
- 撰寫 Node.js/TS 腳本，掃描 src/lib/*.ts、src-tauri/src/*.rs，自動萃取 interface/struct，產生/覆蓋 .cursor/data-structure.md。
- API 文件同步：如有 API 變更，自動產生/更新 .cursor/api-spec.md（可結合 OpenAPI 產生器）。
- 在 CI/CD 加入自動執行（如 node .cursor/scripts/gen-data-structure.js）。
- PR/merge 時自動比對 data-structure.md 是否同步。

### 2. API 文件標準化（OpenAPI/Swagger）
- 將 .cursor/api-spec.md 內容轉為 openapi.yaml 或 openapi.json。
- 自動產生前端 client、API mock、測試（swagger-codegen、openapi-generator）。
- API 文件自動化展示（Swagger UI、Redoc）。
- 在 CI/CD 加入 OpenAPI 檢查、同步、產生 client/mock/test 步驟。

### 3. 前後端型別/schema 同步（zod + schemars）
- TypeScript 端用 zod 定義 schema，並用 zod-to-json-schema 產生 JSON Schema。
- Rust 端用 schemars derive schema，產生 jsonschema 檔案與前端同步。
- CI/CD 比對 schema 差異，提醒同步。
- schema 變更時自動產生/覆蓋 .cursor/data-structure.md。

### 4. 元件/模組細分與重用
- Editor/AI Assistant 拆分為 EditorCore、EditorToolbar、EditorHistory、EditorAI 等。
- 抽象共用邏輯為 composable/service（如 RAG、Prompt、AI agent 流程）。
- 每次元件重構/細分時，更新 .cursor/UIUX-Guide.md。

### 5. 狀態管理最佳化（Pinia/Vuex）
- 引入 Pinia，建立 store（如 src/stores/project.ts、src/stores/ai.ts）。
- 將全域狀態遷移到 store，組件間透過 store 讀寫。
- 在 UIUX-Guide、tech-architecture-and-refactor.md 補充狀態管理設計。

### 6. 自動化與 CI/CD 整合
- 在 CI/CD（如 GitHub Actions）加入：
  - schema/type 變更自動產生/同步腳本
  - OpenAPI 檢查與 client/mock 產生
  - 型別/schema 差異比對
  - linter、測試、build、文件同步檢查

> 本章節為全域自動化、型別/文件同步、API 標準化、元件細分、狀態集中管理等最佳化建議，請於每次架構調整、CI/CD 優化時參考。

---

> 本文件為全局技術架構與 refactor 建議，請於每次重大開發/重構/新功能時參考與同步維護。 