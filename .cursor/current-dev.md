# Current Development: 向量資料庫整合與 RAG context 強化

## 目標
- 導入語意向量資料庫，強化 RAG context 檢索與語意查詢能力。
- 支援 AI agent 跨專案、跨文章查詢語意相關內容，提升內容一致性與知識積累。

## 主要開發內容
- 串接向量資料庫（Milvus/Weaviate/Pinecone 等），設計 embedding 寫入/查詢 API。
- 抽象 RAG context 檢索邏輯為 service，支援多 agent 調用。
- 前後端 embedding/context 資料結構同步，並更新 .cursor/data-structure.md。
- 撰寫單元測試、E2E 測試，驗證語意查詢正確性。
- 文件、API spec、data-structure.md 同步維護。

## 依據
- 依 milestone.md、PRD.md、system-analysis.md、tech-architecture-and-refactor.md 規格執行。
- 交付物：向量資料庫串接、RAG context service、API 文件、測試案例、資料結構文件。

> 本階段開發唯一依據，所有規格、資料結構、API 需同步於 .cursor/ 相關文件。 