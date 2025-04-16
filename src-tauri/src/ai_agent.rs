use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Mutex;
use chrono::Local;
use std::error::Error;
use std::fmt;
use reqwest::Client;
use serde_json::json;
use crate::db;

// Import SqliteState from main.rs
use crate::SqliteState;

// Import Project type from db.rs
use crate::db::Project;

// ADDING DRAFT RESPONSE STRUCT
#[derive(Deserialize, Debug)] // Add Deserialize
struct DraftGeneratorResponse {
    explanation: String,
    #[serde(rename = "draftTitle")] // Handle potential casing difference
    draft_title: String,
    #[serde(rename = "draftContent")]
    draft_content: String,
    suggestions: Vec<String>,
}

// 自定義錯誤類型
#[derive(Debug)]
pub enum AgentError {
    DatabaseError(rusqlite::Error),
    InvalidAgentType(String),
    ApiKeyNotSet,
    ApiError(String),
}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgentError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AgentError::InvalidAgentType(t) => write!(f, "Invalid agent type: {}", t),
            AgentError::ApiKeyNotSet => write!(f, "Gemini API key is not set"),
            AgentError::ApiError(e) => write!(f, "API error: {}", e),
        }
    }
}

impl Error for AgentError {}

// AI 代理類型枚舉
#[derive(Debug, Serialize, Deserialize)]
pub enum AgentType {
    DraftGenerator,
    Planning,
    Research,
    Editor,
    Reviewer,
    InlineEditor,
}

impl AgentType {
    fn to_string(&self) -> String {
        match self {
            AgentType::DraftGenerator => "draft_generator".to_string(),
            AgentType::Planning => "planning".to_string(),
            AgentType::Research => "research".to_string(),
            AgentType::Editor => "editor".to_string(),
            AgentType::Reviewer => "reviewer".to_string(),
            AgentType::InlineEditor => "inline_editor".to_string(),
        }
    }

    fn from_string(s: &str) -> Result<Self, AgentError> {
        match s {
            "draft_generator" => Ok(AgentType::DraftGenerator),
            "planning" => Ok(AgentType::Planning),
            "research" => Ok(AgentType::Research),
            "editor" => Ok(AgentType::Editor),
            "reviewer" => Ok(AgentType::Reviewer),
            "inline_editor" => Ok(AgentType::InlineEditor),
            _ => Err(AgentError::InvalidAgentType(s.to_string())),
        }
    }
}

// AI 代理推理步驟
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentReasoning {
    pub id: Option<i64>,
    pub blog_id: i64,
    pub agent_type: AgentType,
    pub title: String,
    pub reasoning: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

// AI 代理建議
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentSuggestion {
    pub id: Option<i64>,
    pub reasoning_id: i64,
    pub suggestion: String,
    pub applied: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

// RAG 檢索結果
#[derive(Debug, Serialize, Deserialize)]
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

// 初始化 AI 代理相關的表
pub fn init_ai_agent_tables(conn: &Connection) -> Result<(), String> {
    // 創建 agent_reasoning 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS agent_reasoning (
            id INTEGER PRIMARY KEY,
            blog_id INTEGER NOT NULL,
            agent_type TEXT NOT NULL,
            title TEXT NOT NULL,
            reasoning TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (blog_id) REFERENCES blogs (id) ON DELETE CASCADE
        )",
        [],
    ).map_err(|e| e.to_string())?;
    
    // 創建 agent_suggestions 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS agent_suggestions (
            id INTEGER PRIMARY KEY,
            reasoning_id INTEGER NOT NULL,
            suggestion TEXT NOT NULL,
            applied BOOLEAN DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (reasoning_id) REFERENCES agent_reasoning (id) ON DELETE CASCADE
        )",
        [],
    ).map_err(|e| e.to_string())?;
    
    // 創建 rag_retrievals 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS rag_retrievals (
            id INTEGER PRIMARY KEY,
            blog_id INTEGER NOT NULL,
            agent_type TEXT NOT NULL,
            source_type TEXT NOT NULL,
            source_id INTEGER NOT NULL,
            relevance_score REAL NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (blog_id) REFERENCES blogs (id) ON DELETE CASCADE
        )",
        [],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

// 保存代理推理
#[tauri::command]
pub fn save_agent_reasoning(reasoning: AgentReasoning, state: tauri::State<'_, SqliteState>) -> Result<i64, String> {
    let conn = state.0.lock().unwrap();
    
    let now = Local::now().to_rfc3339();
    
    let agent_type_str = match reasoning.agent_type {
        AgentType::DraftGenerator => "draft_generator",
        AgentType::Planning => "planning",
        AgentType::Research => "research",
        AgentType::Editor => "editor",
        AgentType::Reviewer => "reviewer",
        AgentType::InlineEditor => "inline_editor",
    };
    
    let id = conn.execute(
        "INSERT INTO agent_reasoning (blog_id, agent_type, title, reasoning, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            reasoning.blog_id,
            agent_type_str,
            reasoning.title,
            reasoning.reasoning,
            now,
            now
        ],
    ).map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid())
}

// 保存代理建議
#[tauri::command]
pub fn save_agent_suggestion(suggestion: AgentSuggestion, state: tauri::State<'_, SqliteState>) -> Result<i64, String> {
    let conn = state.0.lock().unwrap();
    
    let now = Local::now().to_rfc3339();
    
    let id = conn.execute(
        "INSERT INTO agent_suggestions (reasoning_id, suggestion, applied, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            suggestion.reasoning_id,
            suggestion.suggestion,
            suggestion.applied,
            now,
            now
        ],
    ).map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid())
}

// 保存 RAG 檢索結果
#[tauri::command]
pub fn save_rag_retrieval(retrieval: RagRetrieval, state: tauri::State<'_, SqliteState>) -> Result<i64, String> {
    let conn = state.0.lock().unwrap();
    
    let now = Local::now().to_rfc3339();
    
    let agent_type_str = match retrieval.agent_type {
        AgentType::DraftGenerator => "draft_generator",
        AgentType::Planning => "planning",
        AgentType::Research => "research",
        AgentType::Editor => "editor",
        AgentType::Reviewer => "reviewer",
        AgentType::InlineEditor => "inline_editor",
    };
    
    let id = conn.execute(
        "INSERT INTO rag_retrievals (blog_id, agent_type, source_type, source_id, relevance_score, content, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            retrieval.blog_id,
            agent_type_str,
            retrieval.source_type,
            retrieval.source_id,
            retrieval.relevance_score,
            retrieval.content,
            now,
            now
        ],
    ).map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid())
}

// 獲取博客的所有代理推理
#[tauri::command]
pub fn get_agent_reasonings_by_blog(blog_id: i64, state: tauri::State<'_, SqliteState>) -> Result<Vec<AgentReasoning>, String> {
    let conn = state.0.lock().unwrap();
    
    let mut stmt = conn.prepare(
        "SELECT id, blog_id, agent_type, title, reasoning, created_at, updated_at 
         FROM agent_reasoning 
         WHERE blog_id = ? 
         ORDER BY created_at DESC"
    ).map_err(|e| e.to_string())?;
    
    let reasonings = stmt.query_map([blog_id], |row| {
        let agent_type_str: String = row.get(2)?;
        let agent_type = match agent_type_str.as_str() {
            "draft_generator" => AgentType::DraftGenerator,
            "planning" => AgentType::Planning,
            "research" => AgentType::Research,
            "editor" => AgentType::Editor,
            "reviewer" => AgentType::Reviewer,
            "inline_editor" => AgentType::InlineEditor,
            _ => AgentType::DraftGenerator, // 默認值
        };
        
        Ok(AgentReasoning {
            id: Some(row.get(0)?),
            blog_id: row.get(1)?,
            agent_type,
            title: row.get(3)?,
            reasoning: row.get(4)?,
            created_at: Some(row.get(5)?),
            updated_at: Some(row.get(6)?),
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(reasonings)
}

// 獲取推理的所有建議
#[tauri::command]
pub fn get_agent_suggestions_by_reasoning(reasoning_id: i64, state: tauri::State<'_, SqliteState>) -> Result<Vec<AgentSuggestion>, String> {
    let conn = state.0.lock().unwrap();
    
    let mut stmt = conn.prepare(
        "SELECT id, reasoning_id, suggestion, applied, created_at, updated_at 
         FROM agent_suggestions 
         WHERE reasoning_id = ? 
         ORDER BY created_at ASC"
    ).map_err(|e| e.to_string())?;
    
    let suggestions = stmt.query_map([reasoning_id], |row| {
        Ok(AgentSuggestion {
            id: Some(row.get(0)?),
            reasoning_id: row.get(1)?,
            suggestion: row.get(2)?,
            applied: row.get(3)?,
            created_at: Some(row.get(4)?),
            updated_at: Some(row.get(5)?),
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(suggestions)
}

// 獲取博客的所有 RAG 檢索結果
#[tauri::command]
pub fn get_rag_retrievals_by_blog(blog_id: i64, state: tauri::State<'_, SqliteState>) -> Result<Vec<RagRetrieval>, String> {
    let conn = state.0.lock().unwrap();
    
    let mut stmt = conn.prepare(
        "SELECT id, blog_id, agent_type, source_type, source_id, relevance_score, content, created_at, updated_at 
         FROM rag_retrievals 
         WHERE blog_id = ? 
         ORDER BY relevance_score DESC"
    ).map_err(|e| e.to_string())?;
    
    let retrievals = stmt.query_map([blog_id], |row| {
        let agent_type_str: String = row.get(2)?;
        let agent_type = match agent_type_str.as_str() {
            "draft_generator" => AgentType::DraftGenerator,
            "planning" => AgentType::Planning,
            "research" => AgentType::Research,
            "editor" => AgentType::Editor,
            "reviewer" => AgentType::Reviewer,
            "inline_editor" => AgentType::InlineEditor,
            _ => AgentType::DraftGenerator, // 默認值
        };
        
        Ok(RagRetrieval {
            id: Some(row.get(0)?),
            blog_id: row.get(1)?,
            agent_type,
            source_type: row.get(3)?,
            source_id: row.get(4)?,
            relevance_score: row.get(5)?,
            content: row.get(6)?,
            created_at: Some(row.get(7)?),
            updated_at: Some(row.get(8)?),
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(retrievals)
}

// 更新建議的應用狀態
#[tauri::command]
pub fn update_suggestion_applied(id: i64, applied: bool, state: tauri::State<'_, SqliteState>) -> Result<bool, String> {
    let conn = state.0.lock().unwrap();
    
    let now = Local::now().to_rfc3339();
    
    conn.execute(
        "UPDATE agent_suggestions 
         SET applied = ?1, updated_at = ?2 
         WHERE id = ?3",
        params![
            applied,
            now,
            id
        ],
    ).map_err(|e| e.to_string())?;
    
    Ok(true)
}

// 檢查 Gemini API 金鑰是否已設定
#[tauri::command]
pub fn check_gemini_api_key(state: tauri::State<'_, SqliteState>) -> Result<bool, String> {
    println!("Checking if Gemini API key is set");
    
    let conn = state.0.lock().unwrap();
    println!("Database path: {:?}", conn.path().unwrap_or("unknown"));
    
    // Check if settings table exists using a more reliable method
    let table_exists: bool = match conn.query_row("SELECT 1 FROM settings LIMIT 1", [], |_| Ok(())) {
        Ok(_) => true,
        Err(e) => {
            // If the error is "no such table", then the table doesn't exist
            // Otherwise, it's a different error that we should log
            if e.to_string().contains("no such table") {
                println!("Settings table does not exist: {}", e);
                false
            } else {
                println!("Error checking settings table: {}", e);
                false
            }
        }
    };
    
    println!("Settings table exists: {}", table_exists);
    
    if !table_exists {
        println!("Settings table does not exist!");
        return Ok(false);
    }
    
    // Check if gemini_api_key record exists
    let key_exists: bool = conn.query_row(
        "SELECT COUNT(*) FROM settings WHERE key = 'gemini_api_key'",
        [],
        |row| row.get(0)
    ).unwrap_or(0) > 0;
    
    println!("Gemini API key exists in database: {}", key_exists);
    
    if !key_exists {
        return Ok(false);
    }
    
    let api_key: Result<Option<String>, _> = conn.query_row(
        "SELECT value FROM settings WHERE key = 'gemini_api_key'",
        [],
        |row| row.get(0)
    );
    
    match api_key {
        Ok(Some(key)) => {
            let is_valid = !key.is_empty();
            println!("API key is {}", if is_valid { "valid" } else { "empty" });
            if is_valid {
                println!("API key value: {}", key);
            }
            Ok(is_valid)
        },
        _ => {
            println!("No API key found or error retrieving it");
            Ok(false)
        }
    }
}

// 設定 Gemini API 金鑰
#[tauri::command]
pub fn set_gemini_api_key(api_key: String, state: tauri::State<'_, SqliteState>) -> Result<bool, String> {
    println!("Setting Gemini API key");
    
    let conn = state.0.lock().unwrap();
    let now = Local::now().to_rfc3339();
    
    // Insert or update the API key
    conn.execute(
        "INSERT INTO settings (key, value, created_at, updated_at) 
         VALUES ('gemini_api_key', ?1, ?2, ?2)
         ON CONFLICT(key) DO UPDATE SET value = ?1, updated_at = ?2",
        params![api_key, now],
    ).map_err(|e| {
        println!("Error saving API key: {}", e);
        e.to_string()
    })?;
    
    println!("Gemini API key saved successfully");
    Ok(true)
}

// 獲取 Gemini API 金鑰
#[tauri::command]
pub fn get_gemini_api_key(state: tauri::State<'_, SqliteState>) -> Result<String, String> {
    println!("Attempting to get Gemini API key");
    
    let conn = state.0.lock().unwrap();
    println!("Database path: {:?}", conn.path().unwrap_or("unknown"));
    
    // Check if settings table exists using a more reliable method
    let table_exists: bool = match conn.query_row("SELECT 1 FROM settings LIMIT 1", [], |_| Ok(())) {
        Ok(_) => true,
        Err(e) => {
            // If the error is "no such table", then the table doesn't exist
            // Otherwise, it's a different error that we should log
            if e.to_string().contains("no such table") {
                println!("Settings table does not exist: {}", e);
                false
            } else {
                println!("Error checking settings table: {}", e);
                false
            }
        }
    };
    
    println!("Settings table exists: {}", table_exists);
    
    if !table_exists {
        println!("Settings table does not exist!");
        return Err("Settings table does not exist".to_string());
    }
    
    // Check if gemini_api_key record exists
    let key_exists: bool = conn.query_row(
        "SELECT COUNT(*) FROM settings WHERE key = 'gemini_api_key'",
        [],
        |row| row.get(0)
    ).unwrap_or(0) > 0;
    
    println!("Gemini API key exists in database: {}", key_exists);
    
    if !key_exists {
        println!("No Gemini API key found in settings table");
        return Err("No Gemini API key found".to_string());
    }
    
    // Query the API key directly
    let api_key = conn.query_row(
        "SELECT value FROM settings WHERE key = 'gemini_api_key'",
        [],
        |row| row.get::<_, Option<String>>(0)
    ).map_err(|e| {
        println!("Error querying API key: {}", e);
        e.to_string()
    })?;
    
    match api_key {
        Some(key) if !key.is_empty() => {
            println!("API key found successfully");
            Ok(key)
        },
        Some(_) => {
            println!("API key found but is empty");
            Err("API key is empty".to_string())
        },
        None => {
            println!("No API key found in database");
            Err("No API key found".to_string())
        }
    }
}

// 使用 Gemini API 生成內容
#[tauri::command]
pub async fn generate_with_gemini(prompt: String, agent_type: AgentType, blog_id: i64, project_data: Option<serde_json::Value>, state: tauri::State<'_, SqliteState>) -> Result<String, String> {
    println!("generate_with_gemini called with agent_type: {:?}, blog_id: {}", agent_type, blog_id);
    
    // Get API key
    let api_key = match get_gemini_api_key(state.clone()) {
        Ok(key) => {
            println!("Successfully retrieved API key");
            key
        },
        Err(e) => {
            println!("Failed to get API key: {}", e);
            return Err("Gemini API key not set. Please set it in Settings.".to_string());
        }
    };
    
    // 構建請求 URL
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent";
    println!("Using Gemini API URL: {}", url);
    
    // 構建請求體
    let request_body = serde_json::json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": prompt
                    }
                ]
            }
        ],
        "generationConfig": {
            "temperature": 0.7,
            "topK": 40,
            "topP": 0.95,
            "maxOutputTokens": 6000,
        }
    });
    
    println!("Sending request to Gemini API with prompt: {}", prompt);
    println!("Request body: {}", serde_json::to_string_pretty(&request_body).unwrap_or_else(|_| "Failed to format JSON".to_string()));
    
    // 如果有 project_data，將其添加到請求中
    let mut request_body = request_body;
    if let Some(project_data) = project_data {
        println!("Including project data in the request");
        // 這裡可以根據需要處理 project_data
        // 例如，將其添加到 prompt 中或作為單獨的參數
    }
    
    // 發送請求
    let client = reqwest::Client::new();
    let response = client.post(format!("{}?key={}", url, api_key))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            println!("Request error: {}", e);
            format!("Failed to send request to Gemini API: {}", e)
        })?;
    
    // 檢查響應狀態
    let status = response.status();
    println!("Gemini API response status: {}", status);
    
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        println!("API error response: {}", error_text);
        return Err(format!("Gemini API returned error: {}", status));
    }
    
    // 解析響應
    let response_json = response.json::<serde_json::Value>().await.map_err(|e| {
        println!("JSON parsing error: {}", e);
        format!("Failed to parse Gemini API response: {}", e)
    })?;
    
    println!("Received response from Gemini API");
    println!("Full response JSON: {}", serde_json::to_string_pretty(&response_json).unwrap_or_else(|_| "Failed to format JSON".to_string()));
    
    // 提取生成的文本
    let generated_text = match response_json.get("candidates") {
        Some(candidates) => {
            println!("Found candidates in response");
            match candidates.get(0) {
                Some(candidate) => {
                    println!("Found first candidate");
                    match candidate.get("content") {
                        Some(content) => {
                            println!("Found content in candidate");
                            match content.get("parts") {
                                Some(parts) => {
                                    println!("Found parts in content");
                                    match parts.get(0) {
                                        Some(part) => {
                                            println!("Found first part");
                                            match part.get("text") {
                                                Some(text) => {
                                                    println!("Found text in part");
                                                    match text.as_str() {
                                                        Some(text_str) => {
                                                            println!("Successfully extracted text: {}", text_str);
                                                            Ok(text_str.to_string())
                                                        },
                                                        None => {
                                                            println!("Text is not a string: {:?}", text);
                                                            Err("Text in response is not a string".to_string())
                                                        }
                                                    }
                                                },
                                                None => {
                                                    println!("No text field found in part: {:?}", part);
                                                    Err("No text field found in response part".to_string())
                                                }
                                            }
                                        },
                                        None => {
                                            println!("No parts found in content: {:?}", content);
                                            Err("No parts found in response content".to_string())
                                        }
                                    }
                                },
                                None => {
                                    println!("No parts field found in content: {:?}", content);
                                    Err("No parts field found in response content".to_string())
                                }
                            }
                        },
                        None => {
                            println!("No content field found in candidate: {:?}", candidate);
                            Err("No content field found in response candidate".to_string())
                        }
                    }
                },
                None => {
                    println!("No candidates found in response: {:?}", candidates);
                    Err("No candidates found in response".to_string())
                }
            }
        },
        None => {
            println!("No candidates field found in response: {:?}", response_json);
            Err("No candidates field found in response".to_string())
        }
    }?;
    
    println!("Extracted generated text: {}", generated_text);
    
    // 保存代理推理
    let reasoning = AgentReasoning {
        id: None,
        blog_id,  // 使用從前端傳遞過來的 blog_id
        agent_type,
        title: "AI Generation".to_string(),
        reasoning: prompt,
        created_at: None,
        updated_at: None,
    };
    
    match save_agent_reasoning(reasoning, state) {
        Ok(_) => println!("Saved agent reasoning"),
        Err(e) => println!("Failed to save agent reasoning: {}", e),
    }
    
    Ok(generated_text)
}

// 使用 Gemini API 生成文章草稿
#[tauri::command]
pub async fn generate_article_draft(
    prompt: String,
    selected_text: String,
    blog_id: i64,
    project_data: Option<serde_json::Value>,
    current_title: String,
    current_content: String,
    state: tauri::State<'_, SqliteState>
) -> Result<String, String> {
    println!("generate_article_draft called with:");
    println!("prompt: {}", prompt);
    println!("selected_text: {}", selected_text);
    println!("blog_id: {}", blog_id);
    println!("current_title: {}", current_title);
    
    // 從 project_data 中提取信息，如果有的話
    let project_goal = match &project_data {
        Some(data) => data.get("goal").and_then(|v| v.as_str()).unwrap_or(""),
        None => ""
    };
    
    let project_description = match &project_data {
        Some(data) => data.get("description").and_then(|v| v.as_str()).unwrap_or(""),
        None => ""
    };
    
    let target_audience = match &project_data {
        Some(data) => data.get("target_audience").and_then(|v| v.as_str()).unwrap_or(""),
        None => ""
    };
    
    let keywords = match &project_data {
        Some(data) => data.get("keywords").and_then(|v| v.as_str()).unwrap_or(""),
        None => ""
    };
    
    // 構建 AI 提示
    let ai_prompt = format!(
        "You are an AI writing assistant for a content creator. I need your help drafting an article with the following details:

Project Goal: {}
Project Description: {}
Target Audience: {}
Keywords: {}

User's Request: 
{}

Selected Text Reference (if any): 
{}

I've already started working on a draft:

Draft Article Title: {}
Draft Article Content: {}

Please help me create a comprehensive, well-structured article based on these details. The article should:

1. Be tailored to the target audience
2. Focus on achieving the project goal
3. Incorporate the provided keywords naturally
4. Maintain a coherent structure with clear sections
5. Expand on my existing draft while preserving its key ideas

Include an engaging introduction, well-developed main sections with appropriate subheadings, and a conclusion that reinforces the key points.

IMPORTANT: Your response MUST be formatted as a JSON object wrapped in a markdown code block with the language specified as 'json'. The JSON should have the following structure:
```json
{{
    \"explanation\": \"Your explanation of how you made this draft\",
    \"draftTitle\": \"The title of the draft\",
    \"draftContent\": \"The content of the draft\",
    \"suggestions\": [
        \"Suggestion 1\",
        \"Suggestion 2\",
        \"etc.\"
    ]
}}
```

Do not include any text outside of this JSON code block. Keep your response concise and focused on the task.",
        project_goal, project_description, target_audience, keywords, 
        prompt, selected_text, current_title, current_content
    );
    
    let response = generate_with_gemini(ai_prompt, AgentType::DraftGenerator, blog_id, project_data, state).await?;
    
    // --- REVISED LOGIC --- 
    // 1. Extract JSON string FIRST from the raw response
    let json_str_extracted: &str = if response.starts_with("```json") && response.ends_with("```") {
        response.trim_start_matches("```json").trim_end_matches("```").trim()
    } else if let Some(json_start) = response.find('{') {
         if let Some(json_end) = response.rfind('}') {
            if json_end > json_start {
                &response[json_start..=json_end]
            } else {
                &response // Return original if end is before start
            }
         } else {
            &response // Return original if no closing brace
         }
    } else {
        &response // Return original if no opening brace
    };
    
    // 2. Clean control characters from the EXTRACTED string
    let cleaned_json_str: String = json_str_extracted.chars().filter(|c| {
        match *c {
            // Remove ALL control characters (U+0000 to U+001F) and DEL (U+007F)
            '\u{0000}'..='\u{001F}' | '\u{007F}' => false, 
            _ => true, // Keep all other characters
        }
    }).collect();

    // 3. Attempt to FIX truncated JSON before parsing
    let potentially_fixed_json_str = fix_truncated_json(&cleaned_json_str);

    // 4. Attempt to parse the potentially FIXED and CLEANED extracted string
    match serde_json::from_str::<serde_json::Value>(&potentially_fixed_json_str) { // Use potentially_fixed_json_str
        Ok(_) => {
            // PARSE TO VALUE SUCCEEDED: Now try parsing into the specific struct.
            match serde_json::from_str::<DraftGeneratorResponse>(&potentially_fixed_json_str) { // Use potentially_fixed_json_str
                Ok(_) => {
                    // PARSE TO STRUCT SUCCEEDED: Final check for expected structure markers.
                    // Check the potentially FIXED string for markers
                    if potentially_fixed_json_str.contains("\"explanation\":") &&
                       potentially_fixed_json_str.contains("\"draftTitle\":") &&
                       potentially_fixed_json_str.contains("\"draftContent\":") &&
                       potentially_fixed_json_str.contains("\"suggestions\":") {

                        println!("BACKEND: JSON parsed to struct AND contains key markers. Returning POTENTIALLY FIXED JSON string.");
                        Ok(potentially_fixed_json_str) // Return the potentially FIXED string
                    } else {
                        // STRUCTURE MARKERS MISSING: Serde parse was likely wrong.
                        println!("BACKEND: Parsed to struct BUT MISSING key markers (like \":\") in POTENTIALLY FIXED string. Malformed JSON detected. Returning fallback error.");
                        println!("POTENTIALLY FIXED JSON string missing markers: {}", potentially_fixed_json_str.chars().take(200).collect::<String>());

                        // Construct and return the structured error JSON.
                        let error_response = format!(
                            "{{\n    \"error\": \"Malformed JSON structure from AI.\",\n    \"explanation\": \"The AI response passed initial parsing but lacked expected structure (e.g., missing colons after keys). Please report this issue.\",\n    \"draftTitle\": \"{}\",\n    \"draftContent\": \"{}\",\n    \"suggestions\": [\n        \"Try rephrasing your request.\",\n        \"Check AI model status or API key.\"\n    ]\n}}",
                            current_title,
                            current_content
                        );
                        println!("BACKEND: Returning fallback error JSON string due to missing structure markers.");
                        Ok(error_response)
                    }
                }
                Err(struct_err) => {
                    // PARSE TO STRUCT FAILED: The initial parse to Value was misleading.
                    println!("BACKEND: Successfully parsed to Value, but FAILED to parse CLEANED string into DraftGeneratorResponse struct: {}. Returning fallback error.", struct_err);
                    println!("CLEANED JSON string snippet causing struct parse failure: {}", cleaned_json_str.chars().take(200).collect::<String>());
                    
                    // Construct and return the structured error JSON.
                    let error_response = format!(
                        "{{\n    \"error\": \"Failed to deserialize AI response into expected structure.\",\n    \"explanation\": \"The AI response was parsed as generic JSON but didn't match the required fields/types. Error: {}. Raw response might be malformed.\",\n    \"draftTitle\": \"{}\",\n    \"draftContent\": \"{}\",\n    \"suggestions\": [\n        \"Try rephrasing your request.\",\n        \"Check AI model status or API key.\",\n        \"Review the raw AI response for structural errors (missing colons, incorrect types etc.).\"\n    ]\n}}",
                        struct_err, // Include the specific struct deserialization error
                        current_title,
                        current_content
                    );
                    println!("BACKEND: Returning fallback error JSON string due to struct mismatch.");
                    Ok(error_response)
                }
            }
        },
        Err(e) => {
            // PARSE TO VALUE FAILED: The string is invalid JSON (likely syntax error).
            println!("BACKEND: Initial JSON parse of POTENTIALLY FIXED string failed: {}. Constructing fallback error response.", e);
            println!("Raw response snippet (original): {}", response.chars().take(200).collect::<String>()); // Log original
            println!("Extracted JSON string snippet (before cleaning): {}", json_str_extracted.chars().take(200).collect::<String>()); // Log extracted
            println!("Cleaned JSON string snippet: {}", cleaned_json_str.chars().take(200).collect::<String>()); // Log cleaned
            println!("Potentially fixed JSON string (full): {}", potentially_fixed_json_str); // Log potentially fixed (FULL)


            // Construct and return the structured error JSON immediately.
            let error_response = format!(
                "{{\n    \"error\": \"Failed to generate valid JSON response from AI.\",\n    \"explanation\": \"The AI response could not be parsed correctly. Error: {}. Raw response might be malformed.\",\n    \"draftTitle\": \"{}\",\n    \"draftContent\": \"{}\",\n    \"suggestions\": [\n        \"Try rephrasing your request.\",\n        \"Check AI model status or API key.\",\n        \"Review the raw AI response for structural errors.\"\n    ]\n}}",
                e, // Include the specific parse error message
                current_title,
                current_content // Use current content as fallback
            );
            println!("BACKEND: Returning fallback error JSON string.");
            Ok(error_response) // Return the error JSON as a string
        }
    }
}

// Helper function to fix truncated JSON
fn fix_truncated_json(json_str: &str) -> String {
    // Check if the JSON is truncated (missing closing brackets)
    let mut fixed_json = json_str.to_string();
    
    // Count opening and closing braces
    let open_braces = fixed_json.matches('{').count();
    let close_braces = fixed_json.matches('}').count();
    
    // Add missing closing braces
    for _ in 0..(open_braces - close_braces) {
        fixed_json.push('}');
    }
    
    // Count opening and closing brackets
    let open_brackets = fixed_json.matches('[').count();
    let close_brackets = fixed_json.matches(']').count();
    
    // Add missing closing brackets
    for _ in 0..(open_brackets - close_brackets) {
        fixed_json.push(']');
    }
    
    // Check if the JSON ends with a comma (invalid)
    if fixed_json.ends_with(',') {
        fixed_json.pop();
    }
    
    fixed_json
}

// 使用 Gemini API 規劃文章內容
#[tauri::command]
pub async fn plan_article_content(
    prompt: String,
    selected_text: String,
    blog_id: i64,
    project_data: Option<serde_json::Value>,
    current_title: String,
    current_content: String,
    state: tauri::State<'_, SqliteState>
) -> Result<String, String> {
    println!("plan_article_content called with prompt: {}", prompt);
    
    // 從 project_data 中提取信息，如果有的話
    let project_title = match &project_data {
        Some(data) => data.get("title").and_then(|v| v.as_str()).unwrap_or(""),
        None => ""
    };
    
    let keywords = match &project_data {
        Some(data) => data.get("keywords").and_then(|v| v.as_str()).unwrap_or(""),
        None => ""
    };
    
    // 構建 AI 提示
    let ai_prompt = format!(
        "Create a detailed content plan for a blog article with the following details:
        
Current Article Title: {}
Keywords: {}

User's Request:
{}

Selected Text Reference (if any):
{}

Current Article Content (if any):
{}

Include a suggested outline with main sections and subsections, key points to cover, and any relevant research topics.
Provide a clear structure that addresses the user's request and incorporates the keywords naturally.",
        current_title, keywords, prompt, selected_text, current_content
    );
    
    generate_with_gemini(ai_prompt, AgentType::Planning, blog_id, project_data, state).await
}

// 使用 Gemini API 分析文章內容
#[tauri::command]
pub async fn analyze_article_content(
    prompt: String,
    selected_text: String,
    blog_id: i64,
    project_data: Option<serde_json::Value>,
    current_title: String,
    current_content: String,
    state: tauri::State<'_, SqliteState>
) -> Result<String, String> {
    println!("analyze_article_content called with prompt: {}", prompt);
    
    // 構建 AI 提示
    let ai_prompt = format!(
        "Analyze the following blog article content and provide feedback based on the user's request:

Article Title: {}

User's Request:
{}

Selected Text to Focus on (if any):
{}

Article Content:
{}

Provide detailed analysis on clarity, coherence, argument strength, and areas for improvement.
Offer specific suggestions that address the user's request.",
        current_title, prompt, selected_text, current_content
    );
    
    generate_with_gemini(ai_prompt, AgentType::Research, blog_id, project_data, state).await
}

// 使用 Gemini API 調整文章風格
#[tauri::command]
pub async fn adjust_article_style(
    prompt: String,
    selected_text: String,
    blog_id: i64,
    project_data: Option<serde_json::Value>,
    current_title: String,
    current_content: String,
    state: tauri::State<'_, SqliteState>
) -> Result<String, String> {
    println!("adjust_article_style called with prompt: {}", prompt);
    
    // 構建 AI 提示
    let ai_prompt = format!(
        "Review the following blog article and suggest improvements to style, tone, and readability based on the user's request:

Article Title: {}

User's Request:
{}

Selected Text to Focus on (if any):
{}

Article Content:
{}

Provide specific suggestions for enhancing the writing style while preserving the original meaning and intent.
Focus on making the content more engaging, clear, and effective for the target audience.",
        current_title, prompt, selected_text, current_content
    );
    
    generate_with_gemini(ai_prompt, AgentType::Editor, blog_id, project_data, state).await
}

// 使用 Gemini API 進行最終審查
#[tauri::command]
pub async fn review_article_final(
    prompt: String,
    selected_text: String,
    blog_id: i64,
    project_data: Option<serde_json::Value>,
    current_title: String, 
    current_content: String,
    state: tauri::State<'_, SqliteState>
) -> Result<String, String> {
    println!("review_article_final called with prompt: {}", prompt);
    
    // 構建 AI 提示
    let ai_prompt = format!(
        "Perform a comprehensive final review of the following blog article, addressing the user's specific request:

Article Title: {}

User's Request:
{}

Selected Text to Focus on (if any):
{}

Article Content:
{}

Check for grammar, spelling, style consistency, and overall quality.
Provide a thorough evaluation and specific recommendations based on the user's request.
Include both strengths and areas for improvement in your assessment.",
        current_title, prompt, selected_text, current_content
    );
    
    generate_with_gemini(ai_prompt, AgentType::Reviewer, blog_id, project_data, state).await
}

// 使用 Gemini API 進行內聯編輯
#[tauri::command]
pub async fn inline_edit_text(
    selected_text: String, 
    article_title: String, 
    article_content: String, 
    user_prompt: String,
    blog_id: i64,
    state: tauri::State<'_, SqliteState>
) -> Result<String, String> {
    println!("inline_edit_text called with prompt: {}", user_prompt);
    println!("Selected text: {}", selected_text);
    println!("Blog ID: {}", blog_id);
    
    let prompt = format!(
        "You are an inline editor AI assistant. I need you to improve the following selected text from a blog article.
        
        Article Title: {}
        
        Selected Text: {}
        
        User Request: {}
        
        Please provide your response in the following JSON format:
        {{
            \"explanation\": \"Your explanation of the changes you made\",
            \"improved_text\": \"The improved version of the selected text\",
            \"suggestions\": [
                \"Suggestion 1\",
                \"Suggestion 2\",
                \"etc.\"
            ]
        }}
        
        Focus on improving clarity, grammar, style, and impact while maintaining the original meaning.
        If the selected text is a title, focus on making it more engaging and SEO-friendly.
        If the selected text is content, ensure it flows well with the surrounding text.",
        article_title, selected_text, user_prompt
    );
    
    println!("Sending prompt to Gemini API");
    match generate_with_gemini(prompt, AgentType::InlineEditor, blog_id, None, state).await {
        Ok(response) => {
            println!("Received response from Gemini API: {}", response);
            Ok(response)
        },
        Err(e) => {
            println!("Error from Gemini API: {}", e);
            Err(e)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ArticleData {
    pub description: Option<String>,
    pub category: Option<String>,
    pub keywords: Option<String>,
    pub target_audience: Option<String>,
    pub goal: Option<String>,
    pub word_count: Option<i32>,
}

#[tauri::command]
pub async fn get_article_data(blog_id: i64) -> Result<ArticleData, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    
    // First get the project_id from the blog
    let project_id: i64 = conn.query_row(
        "SELECT project_id FROM blogs WHERE id = ?",
        [blog_id],
        |row| row.get(0)
    ).map_err(|e| format!("Failed to get project_id for blog {}: {}", blog_id, e))?;
    
    // Then get the project data
    let project = db::get_project(project_id)?;
    
    // Create and return the ArticleData struct
    Ok(ArticleData {
        description: project.description,
        category: project.category,
        keywords: project.keywords,
        target_audience: project.target_audience,
        goal: project.goal,
        word_count: Some(500), // Default word count
    })
}

#[derive(Debug, Serialize)]
struct ProjectSummary {
    id: i64,
    title: String,
    type_: String,
    description: Option<String>,
    category: Option<String>,
    time_commitment: Option<String>,
    publishing_frequency: Option<String>,
    custom_frequency: Option<String>,
    deadline: Option<String>,
    availability: Option<String>,
    reminder_frequency: Option<String>,
    publishing_platform: Option<String>,
    wordpress_url: Option<String>,
    substack_url: Option<String>,
    custom_platform: Option<String>,
    monetization_strategy: Option<String>,
    monetization_goals: Option<String>,
    keywords: Option<String>,
    target_audience: Option<String>,
    reference_links: Option<String>,
    structure: Option<String>,
    content_strategy: Option<String>,
    seo_strategy: Option<String>,
    goal: Option<String>,
    created_at: String,
    updated_at: String,
    progress: i32,
}

fn generate_project_summary(project: &Project) -> ProjectSummary {
    ProjectSummary {
        id: project.id.unwrap_or(0),
        title: project.title.clone(),
        type_: project.type_.clone(),
        description: project.description.clone(),
        category: project.category.clone(),
        time_commitment: project.time_commitment.clone(),
        publishing_frequency: project.publishing_frequency.clone(),
        custom_frequency: project.custom_frequency.clone(),
        deadline: project.deadline.clone(),
        availability: project.availability.clone(),
        reminder_frequency: project.reminder_frequency.clone(),
        publishing_platform: project.publishing_platform.clone(),
        wordpress_url: project.wordpress_url.clone(),
        substack_url: project.substack_url.clone(),
        custom_platform: project.custom_platform.clone(),
        monetization_strategy: project.monetization_strategy.clone(),
        monetization_goals: project.monetization_goals.clone(),
        keywords: project.keywords.clone(),
        target_audience: project.target_audience.clone(),
        reference_links: project.reference_links.clone(),
        structure: project.structure.clone(),
        content_strategy: project.content_strategy.clone(),
        seo_strategy: project.seo_strategy.clone(),
        goal: project.goal.clone(),
        created_at: project.created_at.clone().unwrap_or_default(),
        updated_at: project.updated_at.clone().unwrap_or_default(),
        progress: project.progress.unwrap_or(0),
    }
} 