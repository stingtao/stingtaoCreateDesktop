# AI Writing Assistant API Spec (2024/06)

---

## 1. 現有 API

### 1.1 設定相關
- `init_db()`
- `get_setting(key: String)`
- `set_setting(key: String, value: String)`
- `delete_setting(key: String)`
- `get_all_settings()`

### 1.2 Project/Blog/Chapter CRUD
- `save_project(project: Project)`
- `get_project(id: i64)`
- `get_all_projects()`
- `update_project(project: Project)`
- `delete_project(id: i64)`
- `get_project_by_blog_id(blog_id: i64)`
- `save_project_content(content: ProjectContent)`
- `get_blogs_by_project(project_id: i64)`
- `get_chapters_by_project(project_id: i64)`
- `save_blog(blog: Blog)`
- `update_blog(blog: Blog)`
- `delete_blog(id: i64)`
- `save_chapter(chapter: Chapter)`
- `update_chapter(chapter: Chapter)`
- `delete_chapter(id: i64)`

### 1.3 匯出/備份
- `export_database()`
- `open_export_location()`
- `export_database_json()`

### 1.4 AI Agent/RAG
- `save_agent_reasoning(reasoning: AgentReasoning)`
- `save_agent_suggestion(suggestion: AgentSuggestion)`
- `save_rag_retrieval(retrieval: RagRetrieval)`
- `get_agent_reasonings_by_blog(blog_id: i64)`
- `get_agent_suggestions_by_reasoning(reasoning_id: i64)`
- `get_rag_retrievals_by_blog(blog_id: i64)`
- `update_suggestion_applied(id: i64, applied: bool)`

### 1.5 Gemini API Key
- `check_gemini_api_key()`
- `set_gemini_api_key(api_key: String)`
- `get_gemini_api_key()`

### 1.6 AI 產生/推理
- `generate_with_gemini(prompt, agent_type, blog_id, project_data)`
- `generate_article_draft(prompt, selected_text, blog_id, project_data, current_title, current_content)`
- `plan_article_content(prompt, selected_text, blog_id, project_data, current_title, current_content)`
- `analyze_article_content(prompt, selected_text, blog_id, project_data, current_title, current_content)`
- `adjust_article_style(prompt, selected_text, blog_id, project_data, current_title, current_content)`
- `review_article_final(prompt, selected_text, blog_id, project_data, current_title, current_content)`
- `inline_edit_text(selected_text, article_title, article_content, user_prompt, blog_id)`
- `get_article_data(blog_id)`

---

## 2. 建議新增 API（依據 PRD.md / system-analysis.md）

### 2.1 版本管理（Book/Chapter Versioning）
- `GET /api/books/:book_id/versions`
- `GET /api/books/:book_id/versions/:version_id`
- `POST /api/books/:book_id/versions`
- `POST /api/books/:book_id/restore/:version_id`
- `GET /api/chapters/:chapter_id/versions`
- `GET /api/chapters/:chapter_id/versions/:version_id`
- `POST /api/chapters/:chapter_id/versions`
- `POST /api/chapters/:chapter_id/restore/:version_id`
- `GET /api/chapters/:chapter_id/diff?from=:ver1&to=:ver2`

### 2.2 拖曳排序/章節重組
- `POST /api/books/:book_id/chapters/reorder`

### 2.3 MCP/個人化/外部資料
- `POST /api/mcp/invoke_file`  
- `POST /api/external/summarize_url`
- `POST /api/external/import_youtube`

### 2.4 協作/同步
- `GET /api/projects/:project_id/collaborators`
- `POST /api/projects/:project_id/collaborators/invite`
- `DELETE /api/projects/:project_id/collaborators/:user_id`

### 2.5 進階搜尋/導航
- `GET /api/search/projects?query=...`
- `GET /api/search/chapters?query=...&book_id=...`
- `GET /api/search/articles?query=...&blog_id=...`

### 2.6 通知/提醒
- `GET /api/notifications?user_id=...`
- `POST /api/notifications/:notification_id/read`

---

> 本文件分為現有 API 及建議新增 API，請於前後端協作、API 設計、文件同步時參考與維護。 