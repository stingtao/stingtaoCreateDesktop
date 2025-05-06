use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use std::fs;
use chrono::Local;
use crate::DB_NAME; // 導入 DB_NAME 常量
use std::collections::HashSet;

// Custom error type for database operations
#[derive(Debug)]
pub struct DbError(String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<rusqlite::Error> for DbError {
    fn from(error: rusqlite::Error) -> Self {
        DbError(error.to_string())
    }
}

impl From<DbError> for String {
    fn from(error: DbError) -> Self {
        error.0
    }
}

impl From<DbError> for rusqlite::Error {
    fn from(error: DbError) -> Self {
        rusqlite::Error::InvalidParameterName(error.0)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: Option<i64>,
    pub title: String,
    pub type_: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub time_commitment: Option<String>,
    pub publishing_frequency: Option<String>,
    pub custom_frequency: Option<String>,
    pub deadline: Option<String>,
    pub availability: Option<String>,
    pub reminder_frequency: Option<String>,
    pub publishing_platform: Option<String>,
    pub wordpress_url: Option<String>,
    pub substack_url: Option<String>,
    pub custom_platform: Option<String>,
    pub monetization_strategy: Option<String>,
    pub monetization_goals: Option<String>,
    pub keywords: Option<String>,
    pub target_audience: Option<String>,
    pub reference_links: Option<String>,
    pub structure: Option<String>,
    pub content_strategy: Option<String>,
    pub seo_strategy: Option<String>,
    pub goal: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub progress: Option<i32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub article_length: Option<String>,
    pub receive_notifications: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectContent {
    pub project_id: i64,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectSummary {
    pub id: i64,
    pub title: String,
    pub type_: String,
    pub created_at: String,
    pub updated_at: String,
    pub progress: i32,
    pub latest_content: Option<ContentSummary>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub time_commitment: Option<String>,
    pub publishing_frequency: Option<String>,
    pub custom_frequency: Option<String>,
    pub deadline: Option<String>,
    pub availability: Option<String>,
    pub reminder_frequency: Option<String>,
    pub publishing_platform: Option<String>,
    pub wordpress_url: Option<String>,
    pub substack_url: Option<String>,
    pub custom_platform: Option<String>,
    pub monetization_strategy: Option<String>,
    pub monetization_goals: Option<String>,
    pub keywords: Option<String>,
    pub target_audience: Option<String>,
    pub reference_links: Option<String>,
    pub structure: Option<String>,
    pub content_strategy: Option<String>,
    pub seo_strategy: Option<String>,
    pub goal: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub article_length: Option<String>,
    pub receive_notifications: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ContentSummary {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Blog {
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chapter {
    pub id: Option<i64>,
    pub project_id: i64,
    pub title: String,
    pub content: String,
    pub chapter_number: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub fn init_db() -> Result<Connection, String> {
    // Get the app data directory
    let app_dir = tauri::api::path::app_data_dir(&tauri::Config::default())
        .ok_or_else(|| String::from("Failed to get app data directory"))?;
    
    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&app_dir)
        .map_err(|e| e.to_string())?;
    
    // Open the database connection
    let db_path = app_dir.join(DB_NAME);
    println!("Opening database at: {:?}", db_path);
    let conn = Connection::open(db_path)
        .map_err(|e| e.to_string())?;
    
    // Create projects table if it doesn't exist (initial creation)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            progress INTEGER DEFAULT 0
            -- Add more CORE, NON-NULL fields if absolutely necessary initially
            -- Other columns will be added dynamically below
        )",
        [],
    ).map_err(|e| e.to_string())?;
    println!("Ensured core projects table exists.");

    // --- Schema Synchronization Logic --- 
    ensure_project_columns(&conn)?;

    // Create other tables (like blogs) if they don't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS blogs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            content TEXT,
            created_at TEXT,
            updated_at TEXT,
            FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
        )",
        [],
    ).map_err(|e| e.to_string())?;
    println!("Ensured blogs table exists.");
    
    // Create chapters table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS chapters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            content TEXT,
            chapter_number INTEGER,
            created_at TEXT,
            updated_at TEXT,
            FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
        )",
        [],
    ).map_err(|e| e.to_string())?;
    println!("Ensured chapters table exists.");

    // Initialize AI agent tables using the connection
    // Assuming init_ai_agent_tables is defined elsewhere and takes a &Connection
    // crate::ai_agent::init_ai_agent_tables(&conn).map_err(|e| e.to_string())?; 
    // println!("Ensured AI agent tables exist.");

    Ok(conn)
}

// Helper function to ensure all expected columns exist in the projects table
fn ensure_project_columns(conn: &Connection) -> Result<(), String> {
    println!("Synchronizing projects table schema...");

    // Define the expected columns and their definitions (simplified)
    // Use a HashMap or Vec of tuples: (name, definition)
    let expected_columns = vec![
        //("id", "INTEGER PRIMARY KEY AUTOINCREMENT"), // Usually handled by CREATE TABLE
        ("title", "TEXT NOT NULL"),
        ("type_", "TEXT NOT NULL DEFAULT 'blog'"), // Added default
        ("description", "TEXT"),
        ("category", "TEXT"),
        ("time_commitment", "TEXT"),
        ("publishing_frequency", "TEXT"),
        ("custom_frequency", "TEXT"),
        ("deadline", "TEXT"),
        ("availability", "TEXT"),
        ("reminder_frequency", "TEXT"),
        ("publishing_platform", "TEXT"),
        ("wordpress_url", "TEXT"),
        ("substack_url", "TEXT"),
        ("custom_platform", "TEXT"),
        ("monetization_strategy", "TEXT"),
        ("monetization_goals", "TEXT"),
        ("keywords", "TEXT"),
        ("target_audience", "TEXT"),
        ("reference_links", "TEXT"),
        ("structure", "TEXT"),
        ("content_strategy", "TEXT"),
        ("seo_strategy", "TEXT"),
        ("goal", "TEXT"),
        ("start_date", "TEXT"),
        ("end_date", "TEXT"),
        ("created_at", "TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP"), // Added default
        ("updated_at", "TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP"), // Added default
        ("progress", "INTEGER DEFAULT 0"),
        ("article_length", "TEXT"),
        ("receive_notifications", "INTEGER DEFAULT 0")
    ];

    // Get existing columns
    let mut stmt = conn.prepare("PRAGMA table_info(projects)").map_err(|e| e.to_string())?;
    let existing_columns_iter = stmt.query_map([], |row| row.get::<_, String>(1)).map_err(|e| e.to_string())?;
    
    let mut existing_columns = HashSet::new();
    for column in existing_columns_iter {
        existing_columns.insert(column.map_err(|e| e.to_string())?);
    }
    println!("Existing columns: {:?}", existing_columns);

    // Add missing columns
    for (col_name, col_def) in expected_columns {
        if !existing_columns.contains(col_name) {
            let sql = format!("ALTER TABLE projects ADD COLUMN {} {}", col_name, col_def);
            println!("Adding missing column: {}", sql);
            match conn.execute(&sql, []) {
                Ok(_) => println!("Successfully added column: {}", col_name),
                Err(e) => {
                    // Log the error but attempt to continue
                    println!("Failed to add column {}: {}. This might indicate an older incompatible schema or an issue with the ALTER statement.", col_name, e);
                    // Depending on the error, you might want to return Err here
                    // return Err(format!("Failed to add column {}: {}", col_name, e));
                }
            }
        }
    }

    println!("Projects table schema synchronization complete.");
    Ok(())
}

#[tauri::command]
pub fn save_project(project: Project) -> Result<i64, String> {
    println!("Rust: save_project called with data: {:?}", project);
    
    // Validate project data
    if project.title.is_empty() {
        return Err("Project title cannot be empty".to_string());
    }
    
    if project.type_.is_empty() {
        return Err("Project type cannot be empty".to_string());
    }
    
    let conn = match init_db() {
        Ok(conn) => conn,
        Err(e) => {
            println!("Rust: Failed to initialize database: {}", e);
            return Err(format!("Database initialization error: {}", e));
        }
    };
    
    // Convert receive_notifications boolean to integer for SQLite
    let receive_notifications_int = project.receive_notifications.unwrap_or(false) as i32;
    
    // Insert the project data
    let result = conn.execute(
        "INSERT INTO projects (
            title, type_, description, category, time_commitment, 
            publishing_frequency, custom_frequency, deadline, availability, 
            reminder_frequency, publishing_platform, wordpress_url, substack_url, 
            custom_platform, monetization_strategy, monetization_goals, keywords, 
            target_audience, reference_links, structure, content_strategy, 
            seo_strategy, goal, start_date, end_date, created_at, updated_at, progress,
            article_length, receive_notifications
        )
        VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, 
            ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, datetime('now'), datetime('now'), 0,
            ?26, ?27
        )",
        params![
            project.title,
            project.type_,
            project.description,
            project.category,
            project.time_commitment,
            project.publishing_frequency,
            project.custom_frequency,
            project.deadline,
            project.availability,
            project.reminder_frequency,
            project.publishing_platform,
            project.wordpress_url,
            project.substack_url,
            project.custom_platform,
            project.monetization_strategy,
            project.monetization_goals,
            project.keywords,
            project.target_audience,
            project.reference_links,
            project.structure,
            project.content_strategy,
            project.seo_strategy,
            project.goal,
            project.start_date,
            project.end_date,
            project.article_length,
            receive_notifications_int,
        ],
    );
    
    match result {
        Ok(_) => {
            let project_id = conn.last_insert_rowid();
            println!("Rust: Project saved successfully with ID: {}", project_id);
            Ok(project_id)
        },
        Err(e) => {
            println!("Rust: Failed to save project: {}", e);
            println!("Rust: Error details: {:?}", e);
            Err(format!("Failed to save project: {}", e))
        }
    }
}

#[tauri::command]
pub fn save_project_content(content: ProjectContent) -> Result<i64, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    // Use the blogs table instead of contents
    conn.execute(
        "INSERT INTO blogs (project_id, title, content, created_at, updated_at)
         VALUES (?1, ?2, ?3, datetime('now'), datetime('now'))",
        params![
            content.project_id,
            content.title,
            content.content,
        ],
    ).map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn get_all_projects() -> Result<Vec<ProjectSummary>, String> {
    println!("Rust: get_all_projects called");
    
    let conn = match init_db() {
        Ok(conn) => conn,
        Err(e) => {
            println!("Rust: Failed to initialize database: {}", e);
            return Err(format!("Database initialization error: {}", e));
        }
    };
    
    // First check if the projects table exists and has data
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM projects",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    println!("Rust: Found {} projects in database", count);
    
    if count == 0 {
        return Ok(Vec::new());
    }
    
    // Prepare the SQL statement - Updated to use blogs table instead of contents
    let mut stmt = match conn.prepare(
        "SELECT p.id, p.title, p.type_, p.created_at, p.updated_at, p.progress,
                b.title as content_title, b.content,
                p.description, p.category, p.time_commitment, p.publishing_frequency,
                p.custom_frequency, p.deadline, p.availability, p.reminder_frequency,
                p.publishing_platform, p.wordpress_url, p.substack_url, p.custom_platform,
                p.monetization_strategy, p.monetization_goals, p.keywords, p.target_audience,
                p.reference_links, p.structure, p.content_strategy, p.seo_strategy,
                p.goal, p.start_date, p.end_date, p.article_length, p.receive_notifications
         FROM projects p
         LEFT JOIN (
             SELECT project_id, title, content
             FROM blogs b1
             WHERE id = (
                 SELECT MAX(id)
                 FROM blogs b2
                 WHERE b2.project_id = b1.project_id
             )
         ) b ON p.id = b.project_id
         ORDER BY p.created_at DESC"
    ) {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Rust: Failed to prepare SQL statement: {}", e);
            return Err(format!("Database query error: {}", e));
        }
    };
    
    // Execute the query
    let projects = match stmt.query_map([], |row| {
        // Convert receive_notifications integer to boolean
        let receive_notifications_int: i32 = row.get(32)?;
        let receive_notifications = Some(receive_notifications_int != 0);
        
        Ok(ProjectSummary {
            id: row.get(0)?,
            title: row.get(1)?,
            type_: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
            progress: row.get(5)?,
            latest_content: if let Ok(content_title) = row.get(6) {
                Some(ContentSummary {
                    title: content_title,
                    content: row.get(7)?,
                })
            } else {
                None
            },
            description: row.get(8)?,
            category: row.get(9)?,
            time_commitment: row.get(10)?,
            publishing_frequency: row.get(11)?,
            custom_frequency: row.get(12)?,
            deadline: row.get(13)?,
            availability: row.get(14)?,
            reminder_frequency: row.get(15)?,
            publishing_platform: row.get(16)?,
            wordpress_url: row.get(17)?,
            substack_url: row.get(18)?,
            custom_platform: row.get(19)?,
            monetization_strategy: row.get(20)?,
            monetization_goals: row.get(21)?,
            keywords: row.get(22)?,
            target_audience: row.get(23)?,
            reference_links: row.get(24)?,
            structure: row.get(25)?,
            content_strategy: row.get(26)?,
            seo_strategy: row.get(27)?,
            goal: row.get(28)?,
            start_date: row.get(29)?,
            end_date: row.get(30)?,
            article_length: row.get(31)?,
            receive_notifications,
        })
    }) {
        Ok(projects) => projects,
        Err(e) => {
            println!("Rust: Failed to execute query: {}", e);
            return Err(format!("Database query error: {}", e));
        }
    };
    
    // Collect the results
    let mut result = Vec::new();
    for project in projects {
        match project {
            Ok(project) => {
                println!("Rust: Found project: {}", project.title);
                result.push(project);
            },
            Err(e) => {
                println!("Rust: Error processing project row: {}", e);
                // Continue processing other rows
            }
        }
    }
    
    println!("Rust: Found {} projects", result.len());
    Ok(result)
}

#[tauri::command]
pub fn delete_project(id: i64) -> Result<(), String> {
    println!("[DELETE DB] Starting delete_project for ID: {}", id);
    
    let mut conn = match init_db() {
        Ok(conn) => {
            println!("[DELETE DB] Database connection established");
            conn
        },
        Err(e) => {
            println!("[DELETE DB] Failed to connect to database: {}", e);
            return Err(format!("Database connection error: {}", e));
        }
    };
    
    // Get all tables in the database
    let tables: Vec<String> = match conn.query_row(
        "SELECT name FROM sqlite_master WHERE type='table'",
        [],
        |row| row.get::<_, String>(0)
    ) {
        Ok(name) => vec![name],
        Err(e) => {
            println!("[DELETE DB] Error getting tables: {}", e);
            Vec::new()
        }
    };
    
    println!("[DELETE DB] Available tables: {:?}", tables);
    
    // Helper function to check if a table exists
    let table_exists = |table_name: &str| -> bool {
        match conn.query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1",
            [table_name],
            |_| Ok(())
        ) {
            Ok(_) => true,
            Err(_) => false
        }
    };
    
    // Check which tables exist before starting transaction
    let blogs_exists = table_exists("blogs");
    let chapters_exists = table_exists("chapters");
    let agent_suggestions_exists = table_exists("agent_suggestions");
    let agent_reasonings_exists = table_exists("agent_reasonings");
    let rag_retrievals_exists = table_exists("rag_retrievals");
    
    // Start a transaction
    let tx = match conn.transaction() {
        Ok(tx) => {
            println!("[DELETE DB] Transaction started");
            tx
        },
        Err(e) => {
            println!("[DELETE DB] Failed to start transaction: {}", e);
            return Err(format!("Transaction error: {}", e));
        }
    };
    
    // Delete from blogs table if it exists
    if blogs_exists {
        match tx.execute("DELETE FROM blogs WHERE project_id = ?1", [id]) {
            Ok(rows_affected) => {
                println!("[DELETE DB] Deleted {} rows from blogs table", rows_affected);
            },
            Err(e) => {
                println!("[DELETE DB] Failed to delete from blogs table: {}", e);
                return Err(format!("Failed to delete blogs: {}", e));
            }
        };
    } else {
        println!("[DELETE DB] Blogs table does not exist, skipping");
    }
    
    // Delete from chapters table if it exists
    if chapters_exists {
        match tx.execute("DELETE FROM chapters WHERE project_id = ?1", [id]) {
            Ok(rows_affected) => {
                println!("[DELETE DB] Deleted {} rows from chapters table", rows_affected);
            },
            Err(e) => {
                println!("[DELETE DB] Failed to delete from chapters table: {}", e);
                // Continue with deletion even if this fails
            }
        };
    } else {
        println!("[DELETE DB] Chapters table does not exist, skipping");
    }
    
    // Delete from agent_suggestions table if it exists
    if agent_suggestions_exists {
        match tx.execute(
            "DELETE FROM agent_suggestions WHERE reasoning_id IN (SELECT id FROM agent_reasonings WHERE blog_id IN (SELECT id FROM blogs WHERE project_id = ?1))",
            [id]
        ) {
            Ok(rows_affected) => {
                println!("[DELETE DB] Deleted {} rows from agent_suggestions table", rows_affected);
            },
            Err(e) => {
                println!("[DELETE DB] Failed to delete from agent_suggestions table: {}", e);
                // Continue with deletion even if this fails
            }
        };
    } else {
        println!("[DELETE DB] Agent_suggestions table does not exist, skipping");
    }
    
    // Delete from agent_reasonings table if it exists
    if agent_reasonings_exists {
        match tx.execute(
            "DELETE FROM agent_reasonings WHERE blog_id IN (SELECT id FROM blogs WHERE project_id = ?1)",
            [id]
        ) {
            Ok(rows_affected) => {
                println!("[DELETE DB] Deleted {} rows from agent_reasonings table", rows_affected);
            },
            Err(e) => {
                println!("[DELETE DB] Failed to delete from agent_reasonings table: {}", e);
                // Continue with deletion even if this fails
            }
        };
    } else {
        println!("[DELETE DB] Agent_reasonings table does not exist, skipping");
    }
    
    // Delete from rag_retrievals table if it exists
    if rag_retrievals_exists {
        match tx.execute(
            "DELETE FROM rag_retrievals WHERE blog_id IN (SELECT id FROM blogs WHERE project_id = ?1)",
            [id]
        ) {
            Ok(rows_affected) => {
                println!("[DELETE DB] Deleted {} rows from rag_retrievals table", rows_affected);
            },
            Err(e) => {
                println!("[DELETE DB] Failed to delete from rag_retrievals table: {}", e);
                // Continue with deletion even if this fails
            }
        };
    } else {
        println!("[DELETE DB] Rag_retrievals table does not exist, skipping");
    }
    
    // Delete from projects table
    match tx.execute("DELETE FROM projects WHERE id = ?1", [id]) {
        Ok(rows_affected) => {
            println!("[DELETE DB] Deleted {} rows from projects table", rows_affected);
            if rows_affected == 0 {
                println!("[DELETE DB] WARNING: No project was deleted. Project ID {} may not exist.", id);
            }
        },
        Err(e) => {
            println!("[DELETE DB] Failed to delete from projects table: {}", e);
            return Err(format!("Failed to delete project: {}", e));
        }
    };
    
    // Commit the transaction
    match tx.commit() {
        Ok(_) => {
            println!("[DELETE DB] Transaction committed successfully");
        },
        Err(e) => {
            println!("[DELETE DB] Failed to commit transaction: {}", e);
            println!("[DELETE DB] Attempting direct deletion as fallback");
            
            // Try direct deletion as a fallback
            match conn.execute("DELETE FROM projects WHERE id = ?1", [id]) {
                Ok(rows_affected) => {
                    println!("[DELETE DB] Fallback: Deleted {} rows from projects table", rows_affected);
                },
                Err(e) => {
                    println!("[DELETE DB] Fallback: Failed to delete from projects table: {}", e);
                    return Err(format!("Failed to delete project: {}", e));
                }
            };
        }
    };
    
    // Verify that the project was actually deleted
    let project_exists: bool = match conn.query_row(
        "SELECT COUNT(*) FROM projects WHERE id = ?1",
        [id],
        |row| row.get::<_, i64>(0)
    ) {
        Ok(count) => count > 0,
        Err(e) => {
            println!("[DELETE DB] Error verifying project deletion: {}", e);
            false
        }
    };
    
    if project_exists {
        println!("[DELETE DB] ERROR: Project still exists after deletion! ID: {}", id);
        return Err(format!("Project still exists after deletion. ID: {}", id));
    } else {
        println!("[DELETE DB] Verification successful - project no longer exists");
    }
    
    println!("[DELETE DB] Project deletion completed successfully for ID: {}", id);
    Ok(())
}

// Utility function to validate database schema
fn validate_db_schema(conn: &Connection) -> Result<(), String> {
    println!("Rust: Validating database schema");
    
    // Check if the projects table exists
    let table_exists = match conn.query_row(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='projects'",
        [],
        |row| {
            let name: String = row.get(0)?;
            Ok(name == "projects")
        },
    ) {
        Ok(exists) => exists,
        Err(e) => {
            println!("Rust: Failed to check if projects table exists: {}", e);
            return Err(format!("Database schema error: {}", e));
        }
    };
    
    if !table_exists {
        println!("Rust: Projects table does not exist");
        return Err("Projects table does not exist".to_string());
    }
    
    // Check if the blogs table exists
    let blogs_table_exists = match conn.query_row(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='blogs'",
        [],
        |row| {
            let name: String = row.get(0)?;
            Ok(name == "blogs")
        },
    ) {
        Ok(exists) => exists,
        Err(e) => {
            println!("Rust: Failed to check if blogs table exists: {}", e);
            return Err(format!("Database schema error: {}", e));
        }
    };
    
    if !blogs_table_exists {
        println!("Rust: Blogs table does not exist");
        return Err("Blogs table does not exist".to_string());
    }
    
    // Get all columns in the projects table
    let mut stmt = match conn.prepare("PRAGMA table_info(projects)") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Rust: Failed to prepare PRAGMA statement: {}", e);
            return Err(format!("Database schema error: {}", e));
        }
    };
    
    let columns = match stmt.query_map([], |row| {
        let name: String = row.get(1)?;
        Ok(name)
    }) {
        Ok(columns) => columns,
        Err(e) => {
            println!("Rust: Failed to get column names: {}", e);
            return Err(format!("Database schema error: {}", e));
        }
    };
    
    let mut column_names = Vec::new();
    for column in columns {
        match column {
            Ok(name) => column_names.push(name),
            Err(e) => {
                println!("Rust: Error processing column: {}", e);
                // Continue processing other columns
            }
        }
    }
    
    println!("Rust: Projects table columns: {:?}", column_names);
    
    // Check for required columns
    let required_columns = [
        "id", "title", "type_", "created_at", "updated_at", "progress",
        "description", "category", "time_commitment", "publishing_frequency",
        "custom_frequency", "deadline", "availability", "reminder_frequency",
        "publishing_platform", "wordpress_url", "substack_url", "custom_platform",
        "monetization_strategy", "monetization_goals", "keywords", "target_audience",
        "reference_links", "structure", "content_strategy", "seo_strategy",
        "goal"
    ];
    
    let mut missing_columns = Vec::new();
    for column in required_columns.iter() {
        if !column_names.contains(&column.to_string()) {
            missing_columns.push(column);
        }
    }
    
    if !missing_columns.is_empty() {
        println!("Rust: Missing columns: {:?}", missing_columns);
        return Err(format!("Missing columns: {:?}", missing_columns));
    }
    
    println!("Rust: Database schema validation successful");
    Ok(())
}

#[tauri::command]
pub async fn export_database(app_handle: AppHandle) -> Result<String, String> {
    // Get the app data directory
    let app_dir = tauri::api::path::app_data_dir(&tauri::Config::default())
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&app_dir)
        .map_err(|e| e.to_string())?;
    
    let db_path = app_dir.join(DB_NAME);
    
    println!("Looking for database at: {:?}", db_path);
    
    if !db_path.exists() {
        println!("Database file does not exist at: {:?}", db_path);
        return Err("Database file does not exist".to_string());
    }
    
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let export_dir = app_dir.join("exports");
    
    println!("Export directory: {:?}", export_dir);
    
    if !export_dir.exists() {
        println!("Creating export directory: {:?}", export_dir);
        fs::create_dir_all(&export_dir).map_err(|e| e.to_string())?;
    }
    
    let export_path = export_dir.join(format!("stingtao_export_{}.db", timestamp));
    
    println!("Exporting database to: {:?}", export_path);
    
    fs::copy(&db_path, &export_path).map_err(|e| e.to_string())?;
    
    // Return the full path as a string
    Ok(export_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn open_export_location(app_handle: AppHandle) -> Result<(), String> {
    // Get the app data directory
    let app_dir = tauri::api::path::app_data_dir(&tauri::Config::default())
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    let export_dir = app_dir.join("exports");
    
    println!("Opening export directory: {:?}", export_dir);
    
    if !export_dir.exists() {
        println!("Export directory does not exist at: {:?}", export_dir);
        return Err("Export directory does not exist".to_string());
    }
    
    // Use the system's default file explorer to open the directory
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("explorer")
            .arg(export_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .arg(export_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open")
            .arg(export_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[tauri::command]
pub fn get_blogs_by_project(project_id: i64) -> Result<Vec<Blog>, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare(
        "SELECT id, project_id, title, content, created_at, updated_at 
         FROM blogs 
         WHERE project_id = ? 
         ORDER BY created_at DESC"
    ).map_err(|e| e.to_string())?;
    
    let blogs = stmt.query_map([project_id], |row| {
        Ok(Blog {
            id: row.get(0)?,
            project_id: row.get(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(blogs)
}

#[tauri::command]
pub fn get_chapters_by_project(project_id: i64) -> Result<Vec<Chapter>, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare(
        "SELECT id, project_id, title, content, chapter_number, created_at, updated_at 
         FROM chapters 
         WHERE project_id = ? 
         ORDER BY chapter_number ASC"
    ).map_err(|e| e.to_string())?;
    
    let chapters = stmt.query_map([project_id], |row| {
        Ok(Chapter {
            id: row.get(0)?,
            project_id: row.get(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            chapter_number: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(chapters)
}

#[tauri::command]
pub fn save_blog(blog: Blog) -> Result<i64, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    let now = Local::now().to_rfc3339();
    
    let id = conn.execute(
        "INSERT INTO blogs (project_id, title, content, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            blog.project_id,
            blog.title,
            blog.content,
            now,
            now
        ],
    ).map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn save_chapter(chapter: Chapter) -> Result<i64, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    let now = Local::now().to_rfc3339();

    let id = conn.execute(
        "INSERT INTO chapters (project_id, title, content, chapter_number, created_at, updated_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            chapter.project_id,
            chapter.title,
            chapter.content,
            chapter.chapter_number,
            chapter.created_at.clone().unwrap_or(now.clone()),
            chapter.updated_at.clone().unwrap_or(now),
        ],
    ).map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn update_blog(blog: Blog) -> Result<bool, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    let now = Local::now().to_rfc3339();
    
    conn.execute(
        "UPDATE blogs 
         SET title = ?1, content = ?2, updated_at = ?3 
         WHERE id = ?4",
        params![
            blog.title,
            blog.content,
            now,
            blog.id
        ],
    ).map_err(|e| e.to_string())?;
    
    Ok(true)
}

#[tauri::command]
pub fn update_chapter(chapter: Chapter) -> Result<bool, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    let now = Local::now().to_rfc3339();
    
    conn.execute(
        "UPDATE chapters 
         SET title = ?1, content = ?2, chapter_number = ?3, updated_at = ?4 
         WHERE id = ?5",
        params![
            chapter.title,
            chapter.content,
            chapter.chapter_number,
            now,
            chapter.id
        ],
    ).map_err(|e| e.to_string())?;
    
    Ok(true)
}

#[tauri::command]
pub fn delete_blog(id: i64) -> Result<bool, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    conn.execute(
        "DELETE FROM blogs WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    
    Ok(true)
}

#[tauri::command]
pub fn delete_chapter(id: i64) -> Result<bool, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    conn.execute(
        "DELETE FROM chapters WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    
    Ok(true)
}

#[tauri::command]
pub fn get_project(id: i64) -> Result<Project, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, title, type_, description, category, time_commitment, 
         publishing_frequency, custom_frequency, deadline, availability, 
         reminder_frequency, publishing_platform, wordpress_url, substack_url, 
         custom_platform, monetization_strategy, monetization_goals, keywords, 
         target_audience, reference_links, structure, content_strategy, 
         seo_strategy, goal, created_at, updated_at, progress, article_length, receive_notifications
         FROM projects WHERE id = ?"
    ).map_err(|e| e.to_string())?;
    
    let project = stmt.query_row([id], |row| {
        // Convert receive_notifications integer to boolean
        let receive_notifications_int: i32 = row.get(28)?;
        let receive_notifications = Some(receive_notifications_int != 0);
        
        Ok(Project {
            id: row.get(0)?,
            title: row.get(1)?,
            type_: row.get(2)?,
            description: row.get(3)?,
            category: row.get(4)?,
            time_commitment: row.get(5)?,
            publishing_frequency: row.get(6)?,
            custom_frequency: row.get(7)?,
            deadline: row.get(8)?,
            availability: row.get(9)?,
            reminder_frequency: row.get(10)?,
            publishing_platform: row.get(11)?,
            wordpress_url: row.get(12)?,
            substack_url: row.get(13)?,
            custom_platform: row.get(14)?,
            monetization_strategy: row.get(15)?,
            monetization_goals: row.get(16)?,
            keywords: row.get(17)?,
            target_audience: row.get(18)?,
            reference_links: row.get(19)?,
            structure: row.get(20)?,
            content_strategy: row.get(21)?,
            seo_strategy: row.get(22)?,
            goal: row.get(23)?,
            created_at: row.get(24)?,
            updated_at: row.get(25)?,
            progress: row.get(26)?,
            start_date: None,
            end_date: None,
            article_length: row.get(27)?,
            receive_notifications,
        })
    }).map_err(|e| e.to_string())?;
    
    Ok(project)
}

#[tauri::command]
pub fn update_project(project: Project) -> Result<bool, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    let now = chrono::Local::now().to_rfc3339();
    
    // Convert receive_notifications boolean to integer for SQLite
    let receive_notifications_int = project.receive_notifications.unwrap_or(false) as i32;
    
    conn.execute(
        "UPDATE projects SET 
         title = ?, type_ = ?, description = ?, category = ?, 
         time_commitment = ?, publishing_frequency = ?, custom_frequency = ?, 
         deadline = ?, availability = ?, reminder_frequency = ?, 
         publishing_platform = ?, wordpress_url = ?, substack_url = ?, 
         custom_platform = ?, monetization_strategy = ?, monetization_goals = ?, 
         keywords = ?, target_audience = ?, reference_links = ?, 
         structure = ?, content_strategy = ?, seo_strategy = ?, 
         goal = ?, updated_at = ?, progress = ?, end_date = ?, article_length = ?, receive_notifications = ? 
         WHERE id = ?",
        params![
            project.title, project.type_, project.description, project.category,
            project.time_commitment, project.publishing_frequency, project.custom_frequency,
            project.deadline, project.availability, project.reminder_frequency,
            project.publishing_platform, project.wordpress_url, project.substack_url,
            project.custom_platform, project.monetization_strategy, project.monetization_goals,
            project.keywords, project.target_audience, project.reference_links,
            project.structure, project.content_strategy, project.seo_strategy,
            project.goal, now, project.progress, project.end_date, project.article_length, receive_notifications_int,
            project.id
        ],
    ).map_err(|e| e.to_string())?;
    
    Ok(true)
}

#[tauri::command]
pub fn get_project_by_blog_id(blog_id: i64) -> Result<Project, String> {
    println!("Rust: get_project_by_blog_id called with blog_id: {}", blog_id);
    
    let conn = match init_db() {
        Ok(conn) => conn,
        Err(e) => {
            println!("Rust: Failed to initialize database: {}", e);
            return Err(format!("Database initialization error: {}", e));
        }
    };
    
    // First get the project_id from the blog
    let project_id: i64 = match conn.query_row(
        "SELECT project_id FROM blogs WHERE id = ?",
        [blog_id],
        |row| row.get(0)
    ) {
        Ok(id) => id,
        Err(e) => {
            println!("Rust: Failed to get project_id for blog {}: {}", blog_id, e);
            return Err(format!("Failed to get project_id for blog {}: {}", blog_id, e));
        }
    };
    
    println!("Rust: Found project_id {} for blog {}", project_id, blog_id);
    
    // Then get the project data using the existing get_project function
    match get_project(project_id) {
        Ok(project) => {
            println!("Rust: Successfully retrieved project data for blog {}", blog_id);
            Ok(project)
        },
        Err(e) => {
            println!("Rust: Failed to get project data: {}", e);
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn export_database_json(app_handle: AppHandle) -> Result<String, String> {
    // Get the app data directory
    let app_dir = tauri::api::path::app_data_dir(&tauri::Config::default())
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&app_dir)
        .map_err(|e| e.to_string())?;
    
    let db_path = app_dir.join(DB_NAME);
    
    println!("Looking for database at: {:?}", db_path);
    
    if !db_path.exists() {
        println!("Database file does not exist at: {:?}", db_path);
        return Err("Database file does not exist".to_string());
    }
    
    // Open the database connection
    let conn = Connection::open(&db_path)
        .map_err(|e| e.to_string())?;
    
    // Create a JSON structure to hold all the data
    let mut json_data = serde_json::Map::new();
    
    // Export projects
    let mut stmt = conn.prepare("SELECT * FROM projects")
        .map_err(|e| e.to_string())?;
    
    let projects = stmt.query_map([], |row| {
        let mut project = serde_json::Map::new();
        
        // Add each column to the project object
        if let Ok(id) = row.get::<_, i64>(0) { project.insert("id".to_string(), serde_json::Value::Number(id.into())); }
        if let Ok(title) = row.get::<_, String>(1) { project.insert("title".to_string(), serde_json::Value::String(title)); }
        if let Ok(type_) = row.get::<_, String>(2) { project.insert("type_".to_string(), serde_json::Value::String(type_)); }
        if let Ok(description) = row.get::<_, Option<String>>(3) { 
            project.insert("description".to_string(), 
                description.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(category) = row.get::<_, Option<String>>(4) { 
            project.insert("category".to_string(), 
                category.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(time_commitment) = row.get::<_, Option<String>>(5) { 
            project.insert("time_commitment".to_string(), 
                time_commitment.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(publishing_frequency) = row.get::<_, Option<String>>(6) { 
            project.insert("publishing_frequency".to_string(), 
                publishing_frequency.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(custom_frequency) = row.get::<_, Option<String>>(7) { 
            project.insert("custom_frequency".to_string(), 
                custom_frequency.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(deadline) = row.get::<_, Option<String>>(8) { 
            project.insert("deadline".to_string(), 
                deadline.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(availability) = row.get::<_, Option<String>>(9) { 
            project.insert("availability".to_string(), 
                availability.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(reminder_frequency) = row.get::<_, Option<String>>(10) { 
            project.insert("reminder_frequency".to_string(), 
                reminder_frequency.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(publishing_platform) = row.get::<_, Option<String>>(11) { 
            project.insert("publishing_platform".to_string(), 
                publishing_platform.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(wordpress_url) = row.get::<_, Option<String>>(12) { 
            project.insert("wordpress_url".to_string(), 
                wordpress_url.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(substack_url) = row.get::<_, Option<String>>(13) { 
            project.insert("substack_url".to_string(), 
                substack_url.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(custom_platform) = row.get::<_, Option<String>>(14) { 
            project.insert("custom_platform".to_string(), 
                custom_platform.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(monetization_strategy) = row.get::<_, Option<String>>(15) { 
            project.insert("monetization_strategy".to_string(), 
                monetization_strategy.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(monetization_goals) = row.get::<_, Option<String>>(16) { 
            project.insert("monetization_goals".to_string(), 
                monetization_goals.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(keywords) = row.get::<_, Option<String>>(17) { 
            project.insert("keywords".to_string(), 
                keywords.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(target_audience) = row.get::<_, Option<String>>(18) { 
            project.insert("target_audience".to_string(), 
                target_audience.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(reference_links) = row.get::<_, Option<String>>(19) { 
            project.insert("reference_links".to_string(), 
                reference_links.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(structure) = row.get::<_, Option<String>>(20) { 
            project.insert("structure".to_string(), 
                structure.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(content_strategy) = row.get::<_, Option<String>>(21) { 
            project.insert("content_strategy".to_string(), 
                content_strategy.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(seo_strategy) = row.get::<_, Option<String>>(22) { 
            project.insert("seo_strategy".to_string(), 
                seo_strategy.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(goal) = row.get::<_, Option<String>>(23) { 
            project.insert("goal".to_string(), 
                goal.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(created_at) = row.get::<_, String>(24) { 
            project.insert("created_at".to_string(), serde_json::Value::String(created_at)); 
        }
        if let Ok(updated_at) = row.get::<_, String>(25) { 
            project.insert("updated_at".to_string(), serde_json::Value::String(updated_at)); 
        }
        if let Ok(progress) = row.get::<_, i32>(26) { 
            project.insert("progress".to_string(), serde_json::Value::Number(progress.into())); 
        }
        if let Ok(article_length) = row.get::<_, Option<String>>(27) { 
            project.insert("article_length".to_string(), 
                article_length.map_or(serde_json::Value::Null, |v| serde_json::Value::String(v))); 
        }
        if let Ok(receive_notifications) = row.get::<_, i32>(28) { 
            project.insert("receive_notifications".to_string(), 
                serde_json::Value::Bool(receive_notifications != 0)); 
        }
        
        Ok(project)
    })
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    json_data.insert("projects".to_string(), serde_json::Value::Array(
        projects.into_iter().map(|p| serde_json::Value::Object(p)).collect()
    ));
    
    // Export blogs
    let mut stmt = conn.prepare("SELECT * FROM blogs")
        .map_err(|e| e.to_string())?;
    
    let blogs = stmt.query_map([], |row| {
        let mut blog = serde_json::Map::new();
        
        // Add each column to the blog object
        if let Ok(id) = row.get::<_, i64>(0) { blog.insert("id".to_string(), serde_json::Value::Number(id.into())); }
        if let Ok(project_id) = row.get::<_, i64>(1) { blog.insert("project_id".to_string(), serde_json::Value::Number(project_id.into())); }
        if let Ok(title) = row.get::<_, String>(2) { blog.insert("title".to_string(), serde_json::Value::String(title)); }
        if let Ok(content) = row.get::<_, String>(3) { blog.insert("content".to_string(), serde_json::Value::String(content)); }
        if let Ok(created_at) = row.get::<_, String>(4) { blog.insert("created_at".to_string(), serde_json::Value::String(created_at)); }
        if let Ok(updated_at) = row.get::<_, String>(5) { blog.insert("updated_at".to_string(), serde_json::Value::String(updated_at)); }
        
        Ok(blog)
    })
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    json_data.insert("blogs".to_string(), serde_json::Value::Array(
        blogs.into_iter().map(|b| serde_json::Value::Object(b)).collect()
    ));
    
    // Export chapters if the table exists
    if let Ok(mut stmt) = conn.prepare("SELECT * FROM chapters") {
        let chapters = stmt.query_map([], |row| {
            let mut chapter = serde_json::Map::new();
            
            // Add each column to the chapter object
            if let Ok(id) = row.get::<_, i64>(0) { chapter.insert("id".to_string(), serde_json::Value::Number(id.into())); }
            if let Ok(project_id) = row.get::<_, i64>(1) { chapter.insert("project_id".to_string(), serde_json::Value::Number(project_id.into())); }
            if let Ok(title) = row.get::<_, String>(2) { chapter.insert("title".to_string(), serde_json::Value::String(title)); }
            if let Ok(content) = row.get::<_, String>(3) { chapter.insert("content".to_string(), serde_json::Value::String(content)); }
            if let Ok(chapter_number) = row.get::<_, i32>(4) { chapter.insert("chapter_number".to_string(), serde_json::Value::Number(chapter_number.into())); }
            if let Ok(created_at) = row.get::<_, String>(5) { chapter.insert("created_at".to_string(), serde_json::Value::String(created_at)); }
            if let Ok(updated_at) = row.get::<_, String>(6) { chapter.insert("updated_at".to_string(), serde_json::Value::String(updated_at)); }
            
            Ok(chapter)
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
        
        json_data.insert("chapters".to_string(), serde_json::Value::Array(
            chapters.into_iter().map(|c| serde_json::Value::Object(c)).collect()
        ));
    }
    
    // Export settings if the table exists
    if let Ok(mut stmt) = conn.prepare("SELECT * FROM settings") {
        let settings = stmt.query_map([], |row| {
            let mut setting = serde_json::Map::new();
            
            // Add each column to the setting object
            if let Ok(id) = row.get::<_, i64>(0) { setting.insert("id".to_string(), serde_json::Value::Number(id.into())); }
            if let Ok(key) = row.get::<_, String>(1) { setting.insert("key".to_string(), serde_json::Value::String(key)); }
            if let Ok(value) = row.get::<_, String>(2) { setting.insert("value".to_string(), serde_json::Value::String(value)); }
            if let Ok(created_at) = row.get::<_, String>(3) { setting.insert("created_at".to_string(), serde_json::Value::String(created_at)); }
            if let Ok(updated_at) = row.get::<_, String>(4) { setting.insert("updated_at".to_string(), serde_json::Value::String(updated_at)); }
            
            Ok(setting)
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
        
        json_data.insert("settings".to_string(), serde_json::Value::Array(
            settings.into_iter().map(|s| serde_json::Value::Object(s)).collect()
        ));
    }
    
    // Convert to JSON string with pretty formatting
    let json_string = serde_json::to_string_pretty(&json_data)
        .map_err(|e| e.to_string())?;
    
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let export_dir = app_dir.join("exports");
    
    println!("Export directory: {:?}", export_dir);
    
    if !export_dir.exists() {
        println!("Creating export directory: {:?}", export_dir);
        fs::create_dir_all(&export_dir).map_err(|e| e.to_string())?;
    }
    
    let export_path = export_dir.join(format!("stingtao_export_{}.json", timestamp));
    
    println!("Exporting database to JSON: {:?}", export_path);
    
    // Write the JSON string to the file
    fs::write(&export_path, json_string)
        .map_err(|e| e.to_string())?;
    
    // Return the full path as a string
    Ok(export_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn get_chapter(id: i64) -> Result<Chapter, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, project_id, title, content, chapter_number, created_at, updated_at \
         FROM chapters \
         WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let chapter = stmt.query_row([id], |row| {
        Ok(Chapter {
            id: row.get(0)?,
            project_id: row.get(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            chapter_number: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    Ok(chapter)
} 