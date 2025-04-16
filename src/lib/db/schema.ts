/**
 * StingTao Database Schema
 * This file documents the structure of the app.db database for frontend developers
 */

/**
 * Project interface representing a row in the projects table
 */
export interface Project {
  id?: number;
  title: string;
  type_: string;
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
  created_at?: string;
  updated_at?: string;
  progress?: number;
}

/**
 * Blog interface representing a row in the blogs table
 */
export interface Blog {
  id?: number;
  project_id: number;
  title: string;
  content: string;
  created_at?: string;
  updated_at?: string;
}

/**
 * Setting interface representing a row in the settings table
 */
export interface Setting {
  id?: number;
  key: string;
  value: string;
  created_at?: string;
  updated_at?: string;
}

/**
 * Database schema information
 */
export const DB_SCHEMA = {
  tables: {
    projects: {
      name: 'projects',
      description: 'Stores information about all projects (blog, book, etc.)',
      columns: {
        id: { type: 'INTEGER', primaryKey: true, autoIncrement: true },
        title: { type: 'TEXT', notNull: true },
        type_: { type: 'TEXT', notNull: true },
        description: { type: 'TEXT' },
        category: { type: 'TEXT' },
        time_commitment: { type: 'TEXT' },
        publishing_frequency: { type: 'TEXT' },
        custom_frequency: { type: 'TEXT' },
        deadline: { type: 'TEXT' },
        availability: { type: 'TEXT' },
        reminder_frequency: { type: 'TEXT' },
        publishing_platform: { type: 'TEXT' },
        wordpress_url: { type: 'TEXT' },
        substack_url: { type: 'TEXT' },
        custom_platform: { type: 'TEXT' },
        monetization_strategy: { type: 'TEXT' },
        monetization_goals: { type: 'TEXT' },
        keywords: { type: 'TEXT' },
        target_audience: { type: 'TEXT' },
        reference_links: { type: 'TEXT' },
        structure: { type: 'TEXT' },
        content_strategy: { type: 'TEXT' },
        seo_strategy: { type: 'TEXT' },
        goal: { type: 'TEXT' },
        created_at: { type: 'TEXT', notNull: true },
        updated_at: { type: 'TEXT', notNull: true },
        progress: { type: 'INTEGER', defaultValue: 0 }
      },
      indexes: [
        { name: 'idx_projects_type', columns: ['type_'] }
      ]
    },
    blogs: {
      name: 'blogs',
      description: 'Stores blog content associated with projects',
      columns: {
        id: { type: 'INTEGER', primaryKey: true, autoIncrement: true },
        project_id: { type: 'INTEGER', notNull: true, foreignKey: 'projects.id' },
        title: { type: 'TEXT', notNull: true },
        content: { type: 'TEXT', notNull: true },
        created_at: { type: 'TEXT', notNull: true },
        updated_at: { type: 'TEXT', notNull: true }
      },
      indexes: [
        { name: 'idx_blogs_project_id', columns: ['project_id'] }
      ]
    },
    settings: {
      name: 'settings',
      description: 'Stores application settings',
      columns: {
        id: { type: 'INTEGER', primaryKey: true, autoIncrement: true },
        key: { type: 'TEXT', notNull: true, unique: true },
        value: { type: 'TEXT', notNull: true },
        created_at: { type: 'DATETIME', defaultValue: 'CURRENT_TIMESTAMP' },
        updated_at: { type: 'DATETIME', defaultValue: 'CURRENT_TIMESTAMP' }
      },
      indexes: [
        { name: 'idx_settings_key', columns: ['key'] }
      ]
    }
  }
}; 