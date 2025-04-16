-- StingTao Database Schema
-- This file defines the structure of the app.db database

-- Projects table
-- Stores information about all projects (blog, book, etc.)
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    type_ TEXT NOT NULL,
    description TEXT,
    category TEXT,
    time_commitment TEXT,
    publishing_frequency TEXT,
    custom_frequency TEXT,
    deadline TEXT,
    availability TEXT,
    reminder_frequency TEXT,
    publishing_platform TEXT,
    wordpress_url TEXT,
    substack_url TEXT,
    custom_platform TEXT,
    monetization_strategy TEXT,
    monetization_goals TEXT,
    keywords TEXT,
    target_audience TEXT,
    reference_links TEXT,
    structure TEXT,
    content_strategy TEXT,
    seo_strategy TEXT,
    goal TEXT,
    start_date TEXT,
    end_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    progress INTEGER DEFAULT 0
);

-- Settings table
-- Stores application settings
CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT UNIQUE NOT NULL,
    value TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Blogs table
-- Stores blog content associated with projects
CREATE TABLE IF NOT EXISTS blogs (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
);

-- Indexes for better performance
CREATE INDEX IF NOT EXISTS idx_projects_type ON projects(type_);
CREATE INDEX IF NOT EXISTS idx_blogs_project_id ON blogs(project_id);
CREATE INDEX IF NOT EXISTS idx_settings_key ON settings(key); 