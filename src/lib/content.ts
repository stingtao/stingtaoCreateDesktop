import { invoke } from '@tauri-apps/api/tauri'

export interface Blog {
  id: number
  project_id: number
  title: string
  content: string
  created_at: string
  updated_at: string
}

export interface Chapter {
  id: number
  project_id: number
  title: string
  content: string
  chapter_number: number
  created_at: string
  updated_at: string
}

export interface BlogArticle {
  id: number
  title: string
  content: string
  keywords: string
  project_id: number
  created_at: string
  updated_at: string
}

// Get all blogs for a project
export async function getBlogsByProject(projectId: number): Promise<BlogArticle[]> {
  try {
    return await invoke<BlogArticle[]>('get_blogs_by_project', { projectId })
  } catch (error) {
    console.error('Failed to get blogs:', error)
    return []
  }
}

// Get all chapters for a project
export async function getChaptersByProject(projectId: number): Promise<Chapter[]> {
  try {
    return await invoke<Chapter[]>('get_chapters_by_project', { projectId })
  } catch (error) {
    console.error('Failed to get chapters:', error)
    return []
  }
}

// Save a new blog
export async function saveBlog(blog: Omit<Blog, 'id' | 'created_at' | 'updated_at'>): Promise<number> {
  try {
    return await invoke<number>('save_blog', { blog })
  } catch (error) {
    console.error('Error saving blog:', error)
    throw error
  }
}

// Save a new chapter
export async function saveChapter(chapter: Omit<Chapter, 'id' | 'created_at' | 'updated_at'>): Promise<number> {
  try {
    return await invoke<number>('save_chapter', { chapter })
  } catch (error) {
    console.error('Error saving chapter:', error)
    throw error
  }
}

// Update a blog
export async function updateBlog(blog: Blog): Promise<boolean> {
  try {
    return await invoke<boolean>('update_blog', { blog })
  } catch (error) {
    console.error('Error updating blog:', error)
    return false
  }
}

// Update a chapter
export async function updateChapter(chapter: Chapter): Promise<boolean> {
  try {
    return await invoke<boolean>('update_chapter', { chapter })
  } catch (error) {
    console.error('Error updating chapter:', error)
    return false
  }
}

// Delete a blog
export async function deleteBlog(id: number): Promise<boolean> {
  try {
    return await invoke<boolean>('delete_blog', { id })
  } catch (error) {
    console.error('Error deleting blog:', error)
    return false
  }
}

// Delete a chapter
export async function deleteChapter(id: number): Promise<boolean> {
  try {
    return await invoke<boolean>('delete_chapter', { id })
  } catch (error) {
    console.error('Error deleting chapter:', error)
    return false
  }
}

// Save a blog article
export async function saveBlogArticle(article: BlogArticle): Promise<BlogArticle> {
  try {
    // 準備要發送到後端的數據
    const blogData = {
      id: article.id || 0,
      project_id: article.project_id,
      title: article.title,
      content: article.content,
      created_at: article.created_at || new Date().toISOString(),
      updated_at: article.updated_at || new Date().toISOString()
    }
    
    let savedArticle: BlogArticle;
    
    if (article.id && article.id > 0) {
      // 如果文章已有 ID，則更新現有文章
      const success = await invoke<boolean>('update_blog', { blog: blogData });
      if (!success) {
        throw new Error('Failed to update blog article');
      }
      savedArticle = { ...article };
    } else {
      // 如果文章沒有 ID，則創建新文章
      const blogId = await invoke<number>('save_blog', { blog: blogData });
      savedArticle = {
        ...article,
        id: blogId
      };
    }
    
    return savedArticle;
  } catch (error) {
    console.error('Failed to save blog article:', error);
    throw error;
  }
}

// Get a single blog article by ID
export async function getBlogArticle(id: number): Promise<BlogArticle | null> {
  try {
    const blogs = await getBlogsByProject(0) // We'll filter by ID in the backend
    const blog = blogs.find(b => b.id === id)
    if (blog) {
      return {
        ...blog,
        keywords: '' // Add default keywords if not present
      }
    }
    return null
  } catch (error) {
    console.error('Failed to get blog article:', error)
    return null
  }
} 