// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod ai_agent;

use rusqlite::{Connection, Result};
use std::sync::Mutex;
use tauri::Manager;

// 定義資料庫名稱常量
pub const DB_NAME: &str = "stingtao.db";

pub struct SqliteState(pub Mutex<Connection>);

#[tauri::command]
async fn init_db(state: tauri::State<'_, SqliteState>) -> Result<(), String> {
    let conn = state.0.lock().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT UNIQUE NOT NULL,
            value TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| e.to_string())?;
    println!("Database initialized successfully");
    Ok(())
}

#[tauri::command]
async fn get_setting(key: String, state: tauri::State<'_, SqliteState>) -> Result<Option<String>, String> {
    let conn = state.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?")
        .map_err(|e| e.to_string())?;
    let mut rows = stmt.query([key]).map_err(|e| e.to_string())?;
    
    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let value: String = row.get(0).map_err(|e| e.to_string())?;
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

#[tauri::command]
async fn set_setting(key: String, value: String, state: tauri::State<'_, SqliteState>) -> Result<(), String> {
    let conn = state.0.lock().unwrap();
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = CURRENT_TIMESTAMP",
        [&key, &value],
    ).map_err(|e| e.to_string())?;
    println!("Setting {} saved successfully", key);
    Ok(())
}

#[tauri::command]
async fn delete_setting(key: String, state: tauri::State<'_, SqliteState>) -> Result<(), String> {
    let conn = state.0.lock().unwrap();
    conn.execute("DELETE FROM settings WHERE key = ?", [key])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_all_settings(state: tauri::State<'_, SqliteState>) -> Result<Vec<(String, String)>, String> {
    let conn = state.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT key, value FROM settings")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?))
    }).map_err(|e| e.to_string())?;
    
    let mut settings = Vec::new();
    for row in rows {
        settings.push(row.map_err(|e| e.to_string())?);
    }
    println!("Retrieved {} settings", settings.len());
    Ok(settings)
}

fn main() {
    // Get the app data directory
    let app_dir = tauri::api::path::app_data_dir(&tauri::Config::default())
        .expect("Failed to get app data directory");
    
    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");
    
    // Open the database connection - use app.db instead of settings.db
    let db_path = app_dir.join(DB_NAME);
    println!("Opening database at: {:?}", db_path);
    let conn = Connection::open(db_path).expect("Failed to open database");
    
    tauri::Builder::default()
        .manage(SqliteState(Mutex::new(conn)))
        .invoke_handler(tauri::generate_handler![
            init_db,
            get_setting,
            set_setting,
            delete_setting,
            get_all_settings,
            db::save_project,
            db::get_project,
            db::get_all_projects,
            db::get_project_by_blog_id,
            db::update_project,
            db::delete_project,
            db::get_blogs_by_project,
            db::get_chapters_by_project,
            db::get_chapter,
            db::save_blog,
            db::save_chapter,
            db::update_blog,
            db::update_chapter,
            db::delete_blog,
            db::delete_chapter,
            db::export_database,
            db::open_export_location,
            db::export_database_json,
            ai_agent::save_agent_reasoning,
            ai_agent::save_agent_suggestion,
            ai_agent::save_rag_retrieval,
            ai_agent::get_agent_reasonings_by_blog,
            ai_agent::get_agent_suggestions_by_reasoning,
            ai_agent::get_rag_retrievals_by_blog,
            ai_agent::update_suggestion_applied,
            ai_agent::check_gemini_api_key,
            ai_agent::set_gemini_api_key,
            ai_agent::get_gemini_api_key,
            ai_agent::generate_with_gemini,
            ai_agent::generate_article_draft,
            ai_agent::plan_article_content,
            ai_agent::analyze_article_content,
            ai_agent::adjust_article_style,
            ai_agent::review_article_final,
            ai_agent::inline_edit_text,
            ai_agent::get_article_data,
        ])
        .setup(|app| {
            // Initialize database tables
            db::init_db().expect("Failed to initialize database");
            
            // Get the state for AI agent tables initialization
            let state = app.state::<SqliteState>();
            let conn = state.0.lock().unwrap();
            
            // Initialize AI agent tables
            ai_agent::init_ai_agent_tables(&conn).expect("Failed to initialize AI agent tables");
            
            // Ensure settings table exists
            conn.execute(
                "CREATE TABLE IF NOT EXISTS settings (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    key TEXT UNIQUE NOT NULL,
                    value TEXT NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )",
                [],
            ).expect("Failed to create settings table");
            
            println!("Database initialized in setup");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
