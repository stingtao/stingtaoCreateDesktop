# StingTao Database Documentation

This directory contains documentation and TypeScript interfaces for the StingTao application database.

## Database Structure

The StingTao application uses SQLite for data storage, with the database file located at `app.db` in the application data directory. The database contains the following tables:

### Projects Table

Stores information about all projects (blog, book, etc.).

**Key Fields:**
- `id`: Primary key
- `title`: Project title
- `type_`: Project type (blog, book, etc.)
- `description`: Project description
- `category`: Project category
- `created_at`: Creation timestamp
- `updated_at`: Last update timestamp
- `progress`: Project progress (0-100)

### Blogs Table

Stores blog content associated with projects.

**Key Fields:**
- `id`: Primary key
- `project_id`: Foreign key to projects table
- `title`: Blog post title
- `content`: Blog post content
- `created_at`: Creation timestamp
- `updated_at`: Last update timestamp

### Settings Table

Stores application settings.

**Key Fields:**
- `id`: Primary key
- `key`: Setting key (unique)
- `value`: Setting value
- `created_at`: Creation timestamp
- `updated_at`: Last update timestamp

## Usage

### Frontend

In the frontend, you can use the TypeScript interfaces defined in `schema.ts` to work with the database:

```typescript
import { Project, Blog, Setting } from './db/schema';

// Example: Creating a new project
const newProject: Project = {
  title: 'My Blog Project',
  type_: 'blog',
  description: 'A personal blog about technology',
  // ... other fields
};

// Save the project
const projectId = await saveProject(newProject);

// Create a blog post for the project
const newBlog: Blog = {
  project_id: projectId,
  title: 'My First Blog Post',
  content: 'This is the content of my first blog post...',
  // created_at and updated_at will be set automatically
};

// Save the blog post
const blogId = await saveBlog(newBlog);
```

### Backend

The backend uses Rust with the `rusqlite` crate to interact with the SQLite database. The database schema is defined in `schema.sql` and is used to initialize the database when the application starts.

## Database Schema

The complete database schema is defined in:

1. Backend: `src-tauri/src/schema.sql`
2. Frontend: `src/lib/db/schema.ts`

## Migrations

When adding new tables or modifying existing ones, update both the backend schema.sql file and the frontend schema.ts file to maintain consistency. 