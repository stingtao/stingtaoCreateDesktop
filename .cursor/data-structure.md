# Data Structure Overview (2024/06)

---

## TypeScript (Frontend)

### Book
```ts
export interface Book {
  id: number;
  title: string;
  description?: string;
  keywords?: string;
  outline?: string; // 全書大綱
  progress?: number;
  created_at: string;
  updated_at: string;
  // 其他 metadata 欄位
}
```

### Chapter
```ts
export interface Chapter {
  id?: number;
  project_id: number;
  title: string;
  content: string;
  chapter_number: number;
  created_at?: string;
  updated_at?: string;
}
```

### Project
```ts
export interface Project {
  id?: number;
  title: string;
  type_: string; // 'blog' | 'book' | ...
  description?: string;
  category?: string;
  time_commitment?: string;
  publishing_frequency?: string;
  custom_frequency?: string;
  deadline?: string;
  availability?: string;
  reminder_frequency?: string;
  publishing_platform?: string;
  wordpress_url?: string;
  substack_url?: string;
  custom_platform?: string;
  monetization_strategy?: string;
  monetization_goals?: string;
  keywords?: string;
  target_audience?: string;
  reference_links?: string;
  structure?: string;
  content_strategy?: string;
  seo_strategy?: string;
  goal?: string;
  article_length?: string;
  receive_notifications?: boolean;
  created_at?: string;
  updated_at?: string;
  progress?: number;
}
```

### Blog / BlogArticle
```ts
export interface Blog {
  id: number;
  project_id: number;
  title: string;
  content: string;
  created_at: string;
  updated_at: string;
}

export interface BlogArticle {
  id: number;
  title: string;
  content: string;
  keywords: string;
  project_id: number;
  created_at: string;
  updated_at: string;
}
```

### Settings
```ts
export interface Settings {
  id: number;
  key: string;
  value: string;
  created_at: string;
  updated_at: string;
}
```

### BookVersion
```ts
export interface BookVersion {
  id: number;
  book_id: number;
  version_number: number;
  title: string;
  description?: string;
  keywords?: string;
  outline?: string;
  content_snapshot?: string; // 書本當下所有章節內容快照（可選）
  created_at: string;
  updated_at: string;
  change_summary?: string; // 本次變更摘要
}
```

### ChapterVersion
```ts
export interface ChapterVersion {
  id: number;
  chapter_id: number;
  book_id: number;
  version_number: number;
  title: string;
  content: string;
  outline?: string;
  ai_suggestions?: string;
  created_at: string;
  updated_at: string;
  change_summary?: string;
}
```

---

## Rust (Backend)

### Book / Chapter
```rust
pub struct Book {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub outline: Option<String>,
    pub progress: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
    // 其他 metadata 欄位
}

pub struct Chapter {
    pub id: i64,
    pub book_id: i64,
    pub title: String,
    pub content: String,
    pub chapter_number: i32,
    pub outline: Option<String>,
    pub ai_suggestions: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
```

### Project / ProjectContent / ProjectSummary
```rust
pub struct Project { ... }
pub struct ProjectContent { ... }
pub struct ProjectSummary { ... }
```
（欄位與 TS 端對應，詳見 src-tauri/src/db.rs）

### Blog
```rust
pub struct Blog { ... }
```

### AI Agent 相關
```rust
pub enum AgentType {
    DraftGenerator,
    Planning,
    Research,
    Editor,
    Reviewer,
    InlineEditor,
}

pub struct AgentReasoning {
    pub id: Option<i64>,
    pub blog_id: i64,
    pub agent_type: AgentType,
    pub title: String,
    pub reasoning: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub struct AgentSuggestion {
    pub id: Option<i64>,
    pub reasoning_id: i64,
    pub suggestion: String,
    pub applied: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub struct RagRetrieval {
    pub id: Option<i64>,
    pub blog_id: i64,
    pub agent_type: AgentType,
    pub source_type: String,
    pub source_id: i64,
    pub relevance_score: f64,
    pub content: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
```

### BookVersion (Rust)
```rust
pub struct BookVersion {
    pub id: i64,
    pub book_id: i64,
    pub version_number: i32,
    pub title: String,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub outline: Option<String>,
    pub content_snapshot: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub change_summary: Option<String>,
}
```

### ChapterVersion (Rust)
```rust
pub struct ChapterVersion {
    pub id: i64,
    pub chapter_id: i64,
    pub book_id: i64,
    pub version_number: i32,
    pub title: String,
    pub content: String,
    pub outline: Option<String>,
    pub ai_suggestions: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub change_summary: Option<String>,
}
```

---

## 關聯與用途
- Book/Chapter：書本與章節的主結構，支援多層級內容管理、AI 協作、版本管理。
- Project/Blog/BlogArticle：其他專案、部落格、文章結構。
- Settings：全域設定（如 API 金鑰、偏好等）。
- AgentReasoning/AgentSuggestion/RagRetrieval：AI agent 推理、建議、RAG 檢索結果，支援多 agent 協作與追蹤。

## 改善建議
- 明確區分 Book/Blog/Project 結構，避免混淆。
- Chapter 需有 book_id 關聯，並支援 outline、ai_suggestions 欄位。
- 支援 outline/ai_suggestions 欄位，方便大綱、AI 草稿、brainstorming 記錄。
- TS/Rust schema 需同步，並於本文件明確標註。
- 可考慮為 Book/Chapter 增加 version/history 支援。

## Version/History 設計重點
- 每次 Book/Chapter 內容有重大變更時，產生一筆 version 記錄。
- version_number 可自動遞增。
- 可支援「回溯」、「版本比較」、「一鍵還原」等功能。
- change_summary 方便記錄本次修改重點（可由 AI 自動產生）。

> 本文件僅彙整主要資料結構，詳細 schema 以 src/lib/db.ts、src/lib/content.ts、src-tauri/src/db.rs、src-tauri/src/ai_agent.rs 為準。 