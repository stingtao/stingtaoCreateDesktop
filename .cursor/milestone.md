# AI Writing Assistant Milestones（2024/06）

## 近期開發里程碑

### Milestone 1：向量資料庫整合
- 目標說明：導入語意向量資料庫，強化 RAG 檢索與語意查詢能力。
- 對應 PRD 功能：強化 RAG 檢索能力，支援語意查詢過去文章、推理、建議（PRD: RAG、AI 檢索、內容推薦能力）
- 依賴元件/模組：RAG/Context Retrieval Component、向量資料庫、資料結構同步（TS/Rust）、本地 DB
- 主要開發內容：
  - 導入語意向量資料庫（如 Milvus、Weaviate、Pinecone 等）
  - 強化 RAG 檢索能力，支援語意查詢過去文章、推理、建議
  - 完成向量資料寫入、查詢、與現有 RAG 流程整合
- 重構/最佳化建議：RAG context 檢索邏輯抽象為可重用 service，前後端資料結構同步維護
- 預期交付物：
  - 向量資料庫串接與查詢 API
  - RAG context service 抽象
  - 文件與測試案例
- 驗收標準：
  - 可查詢語意相關內容，查詢結果正確
  - RAG context 可被多 agent 調用
  - 文件、API spec、data-structure.md 同步
- 負責人/協作人：
- 預計起訖日：

### Milestone 2：MCP（Meta-Chain Prompt）進化
- 目標說明：MCP 流程抽象與個人化 prompt 優化，支援作者風格學習。
- 對應 PRD 功能：MCP 進化、個人化 prompt 優化、作者風格學習（PRD: MCP、進化式建議產生、作者風格學習）
- 依賴元件/模組：Prompt Builder/Manager、MCP 記錄/查詢、AI Agent Orchestration、資料結構同步
- 主要開發內容：
  - 每次文章修改自動萃取進化型 prompt，記錄 MCP
  - MCP 可用於 prompt 優化、個人化學習
  - MCP 記錄可查詢、可視化
- 重構/最佳化建議：MCP 流程抽象為 composable/service，便於多 agent/多內容類型調用
- 預期交付物：
  - MCP 記錄/查詢/可視化元件
  - Prompt Builder/Manager service
  - 文件與測試案例
- 驗收標準：
  - MCP 記錄可查詢、可視化
  - Prompt 可根據作者風格自動優化
  - 文件、API spec、data-structure.md 同步
- 負責人/協作人：
- 預計起訖日：

### Milestone 3：跨文章內容讀取
- 目標說明：AI agent 可跨專案、跨文章讀取 context，提升內容一致性。
- 對應 PRD 功能：AI agent 可讀取、分析所有過去文章內容，提升內容一致性與知識積累（PRD: 跨文章內容讀取、內容一致性、知識積累）
- 依賴元件/模組：RAG/Context Retrieval Component、資料結構統一（多層級內容）、API 統一
- 主要開發內容：
  - AI agent 可讀取、分析所有過去文章內容
  - 支援跨專案、跨文章 context 檢索與引用
  - 提升內容一致性與知識積累
- 重構/最佳化建議：資料結構與 API 統一，支援多層級內容（書本/章節/文章）
- 預期交付物：
  - 跨專案 context 查詢 API
  - 多層級內容資料結構
  - 文件與測試案例
- 驗收標準：
  - 可跨專案查詢 context 並正確引用
  - API、data-structure.md、system-analysis.md 同步
- 負責人/協作人：
- 預計起訖日：

### Milestone 4：Gemma 小模型微調與應用
- 目標說明：導入 Gemma 小模型，支援個人化微調與 prompt 產生。
- 對應 PRD 功能：Gemma 小模型微調、個人化 AI 寫作助手、Gemma prompt 產生（PRD: Gemma 小模型、個人化 AI、prompt 多樣性）
- 依賴元件/模組：AI Agent 模組、Gemma API/本地模型、資料收集/微調流程、Prompt Builder
- 主要開發內容：
  - 導入 Gemma 小模型（本地或 API）
  - 根據每次文章 diff 自動收集資料，週期性微調 Gemma
  - 使用 Gemma 產生 prompt，提升 AI 回應多樣性與上下文適應性
- 重構/最佳化建議：AI 模型串接與微調流程模組化，便於未來更換/擴充模型
- 預期交付物：
  - Gemma 串接與微調流程
  - 個人化 prompt 產生元件
  - 文件與測試案例
- 驗收標準：
  - Gemma 可根據用戶資料微調並產生 prompt
  - 文件、API spec、data-structure.md 同步
- 負責人/協作人：
- 預計起訖日：

### Milestone 5：系統最佳化與協作
- 目標說明：強化多 agent 協作、UI/UX、多人同步與行動裝置支援。
- 對應 PRD 功能：多 agent 協作、UI/UX 優化、多人協作、行動裝置支援（PRD: 多 agent 協作、UI/UX、協作編輯、行動裝置）
- 依賴元件/模組：AI Agent Orchestration、Editor/AI Assistant 拆分元件、協作元件、UI/UX 元件
- 主要開發內容：
  - 強化多 agent 協作流程
  - 優化 UI/UX、diff view、inline 編輯體驗
  - 預備多人協作、同步、行動裝置支援
- 重構/最佳化建議：Editor/AI Assistant/協作元件拆分，提升可維護性與可測試性
- 預期交付物：
  - 協作/同步元件
  - 行動裝置 UI/UX
  - 文件與測試案例
- 驗收標準：
  - 多人協作、同步體驗順暢
  - UI/UX-Guide、system-analysis.md 同步
- 負責人/協作人：
- 預計起訖日：

### Milestone 6：MCP 檔案調用與個人化生成
- 目標說明：MCP 調用作者過去寫作檔案，支援個人化 draft 生成。
- 對應 PRD 功能：MCP 調用作者過去寫作檔案、個人化 draft 生成（PRD: MCP 檔案調用、個人化生成）
- 依賴元件/模組：MCP 檔案選擇器、內容摘要器、context 組合器、RAG/Prompt/AI Agent
- 主要開發內容：
  - 實作 MCP 調用作者過去寫作檔案（Google Cloud/local storage），供 LLM 參考與個人化 draft 生成。
  - 完成 MCP 檔案選擇器、內容摘要器、context 組合器元件。
- 重構/最佳化建議：MCP 檔案調用元件與 context 組合邏輯抽象，便於多場景重用
- 預期交付物：
  - MCP 檔案調用元件
  - 內容摘要與 context 組合器
  - 文件與測試案例
- 驗收標準：
  - LLM 可正確引用 MCP 檔案內容
  - 文件、API spec、data-structure.md 同步
- 負責人/協作人：
- 預計起訖日：

### Milestone 7：URL 匯入與 AI 摘要
- 目標說明：支援用戶輸入 YouTube/外部網站 URL，自動擷取內容、AI 摘要並存檔。
- 對應 PRD 功能：URL 匯入、AI 摘要、靈感來源管理（PRD: URL 匯入、AI 摘要、靈感來源）
- 依賴元件/模組：URL 匯入元件、內容爬取與摘要服務、資料管理模組、RAG/AI Agent
- 主要開發內容：
  - 支援用戶輸入 YouTube/外部網站 URL，自動擷取內容、AI 摘要並存檔。
  - 完成 URL 匯入元件、內容爬取與摘要服務、資料管理模組。
- 重構/最佳化建議：URL 匯入與摘要流程模組化，API 統一，便於擴充新資料來源
- 預期交付物：
  - URL 匯入與摘要元件
  - 內容爬取/摘要服務
  - 文件與測試案例
- 驗收標準：
  - 可正確擷取/摘要外部內容並存檔
  - 文件、API spec、data-structure.md 同步
- 負責人/協作人：
- 預計起訖日：

### Milestone X：自動化型別/文件同步與架構最佳化
- 目標說明：提升專案自動化、型別一致性、API 文件標準化與架構可維護性。
- 對應 PRD 功能：全域技術最佳化、開發效率提升、協作品質提升
- 依賴元件/模組：CI/CD、型別/文件產生腳本、Pinia store、UI/UX Guide
- 主要開發內容：
  1. **schema/type 變更自動同步 .cursor/data-structure.md、.cursor/api-spec.md**
     - 撰寫 Node.js/TS 腳本，掃描 src/lib/*.ts、src-tauri/src/*.rs，自動萃取 interface/struct，產生/覆蓋 .cursor/data-structure.md。
     - API 文件同步：如有 API 變更，自動產生/更新 .cursor/api-spec.md（可結合 OpenAPI 產生器）。
     - 在 CI/CD 加入自動執行（如 node .cursor/scripts/gen-data-structure.js）。
     - PR/merge 時自動比對 data-structure.md 是否同步。
  2. **API 文件標準化（OpenAPI/Swagger）**
     - 將 .cursor/api-spec.md 內容轉為 openapi.yaml 或 openapi.json。
     - 自動產生前端 client、API mock、測試（swagger-codegen、openapi-generator）。
     - API 文件自動化展示（Swagger UI、Redoc）。
     - 在 CI/CD 加入 OpenAPI 檢查、同步、產生 client/mock/test 步驟。
  3. **前後端型別/schema 同步（zod + schemars）**
     - TypeScript 端用 zod 定義 schema，並用 zod-to-json-schema 產生 JSON Schema。
     - Rust 端用 schemars derive schema，產生 jsonschema 檔案與前端同步。
     - CI/CD 比對 schema 差異，提醒同步。
     - schema 變更時自動產生/覆蓋 .cursor/data-structure.md。
  4. **元件/模組細分與重用**
     - Editor/AI Assistant 拆分為 EditorCore、EditorToolbar、EditorHistory、EditorAI 等。
     - 抽象共用邏輯為 composable/service（如 RAG、Prompt、AI agent 流程）。
     - 每次元件重構/細分時，更新 .cursor/UIUX-Guide.md。
  5. **狀態管理最佳化（Pinia/Vuex）**
     - 引入 Pinia，建立 store（如 src/stores/project.ts、src/stores/ai.ts）。
     - 將全域狀態遷移到 store，組件間透過 store 讀寫。
     - 在 UIUX-Guide、tech-architecture-and-refactor.md 補充狀態管理設計。
  6. **自動化與 CI/CD 整合**
     - 在 CI/CD（如 GitHub Actions）加入：
       - schema/type 變更自動產生/同步腳本
       - OpenAPI 檢查與 client/mock 產生
       - 型別/schema 差異比對
       - linter、測試、build、文件同步檢查
- 重構/最佳化建議：全程自動化、型別/文件同步、API 標準化、元件細分、狀態集中管理
- 預期交付物：
  - 自動產生/同步型別、API 文件腳本
  - OpenAPI 標準文件與自動產生 client/mock
  - 前後端 schema 差異比對與同步
  - 元件細分與共用邏輯抽象
  - Pinia 狀態管理 store
  - CI/CD 自動化流程
- 驗收標準：
  - PR/merge 時型別、API、文件自動同步
  - API 文件可自動產生 client/mock
  - 前後端 schema 差異自動檢查
  - 元件細分、狀態集中管理落地
  - CI/CD 全自動化檢查通過
- 負責人/協作人：
- 預計起訖日：

---

> 每回合開發請參考 milestone.md、PRD.md、system-analysis.md、tech-architecture-and-refactor.md，確保整體設計、規格、架構一致，並持續優化可維護性、擴充性、技術債減少。 