import { invoke } from '@tauri-apps/api/tauri'

export interface Project {
  id?: number
  title: string
  type_: string
  description?: string
  category?: string
  time_commitment?: string
  publishing_frequency?: string
  custom_frequency?: string
  deadline?: string
  availability?: string
  reminder_frequency?: string
  publishing_platform?: string
  wordpress_url?: string
  substack_url?: string
  custom_platform?: string
  monetization_strategy?: string
  monetization_goals?: string
  keywords?: string
  target_audience?: string
  reference_links?: string
  structure?: string
  content_strategy?: string
  seo_strategy?: string
  goal?: string
  article_length?: string
  receive_notifications?: boolean
  created_at?: string
  updated_at?: string
  progress?: number
}

export interface ProjectContent {
  id?: number
  project_id: number
  title: string
  content: string
  created_at?: string
  updated_at?: string
}

export interface ProjectSummary extends Project {
  latest_content?: {
    title: string
    content: string
  }
  writing_frequency?: string
  start_date?: string
  end_date?: string
}

// Save a new project
export const saveProject = async (projectData: Omit<Project, 'id' | 'created_at' | 'updated_at' | 'progress'>): Promise<number> => {
  try {
    console.log('saveProject called with data:', projectData)
    
    // Send data directly as the backend expects type_
    console.log('Invoking Tauri save_project command with data:', projectData)
    
    const projectId = await invoke<number>('save_project', { project: projectData });
    
    console.log('Project saved successfully, ID:', projectId)
    return projectId
  } catch (error) {
    console.error('Failed to save project:', error)
    return -1 // Indicate failure
  }
}

// Save project content
export const saveProjectContent = async (content: Omit<ProjectContent, 'id' | 'created_at' | 'updated_at'>): Promise<number> => {
  try {
    console.log('saveProjectContent called with data:', JSON.stringify(content, null, 2))
    
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('Tauri is not available, cannot save project content')
      return -1
    }
    
    console.log('Invoking Tauri save_project_content command...')
    const contentId = await invoke<number>('save_project_content', { content })
    console.log(`Project content saved successfully with ID: ${contentId}`)
    return contentId
  } catch (error) {
    console.error('Failed to save project content:', error)
    // Log more details about the error
    if (error instanceof Error) {
      console.error('Error name:', error.name)
      console.error('Error message:', error.message)
      console.error('Error stack:', error.stack)
    }
    throw error
  }
}

// Get all projects
export async function getAllProjects(): Promise<ProjectSummary[]> {
  try {
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('Tauri is not available, cannot fetch projects')
      return []
    }
    
    console.log('Fetching all projects...')
    const projects = await invoke<ProjectSummary[]>('get_all_projects')
    console.log(`Found ${projects.length} projects`)
    return projects
  } catch (error) {
    console.error('Error fetching projects:', error)
    // Log more details about the error
    if (error instanceof Error) {
      console.error('Error name:', error.name)
      console.error('Error message:', error.message)
      console.error('Error stack:', error.stack)
    }
    return []
  }
}

// Get a single project by ID
export async function getProject(id: number): Promise<Project | null> {
  try {
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('Tauri is not available, cannot fetch project')
      return null
    }
    
    console.log(`Fetching project with ID: ${id}...`)
    const project = await invoke<Project>('get_project', { id })
    console.log(`Project found: ${project.title}`)
    return project
  } catch (error) {
    console.error('Error fetching project:', error)
    // Log more details about the error
    if (error instanceof Error) {
      console.error('Error name:', error.name)
      console.error('Error message:', error.message)
      console.error('Error stack:', error.stack)
    }
    return null
  }
}

// Update a project
export async function updateProject(project: Project): Promise<boolean> {
  try {
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('Tauri is not available, cannot update project')
      return false
    }
    
    console.log(`Updating project with ID: ${project.id}...`)
    const result = await invoke<boolean>('update_project', { project })
    console.log(`Project update result: ${result}`)
    return result
  } catch (error) {
    console.error('Error updating project:', error)
    // Log more details about the error
    if (error instanceof Error) {
      console.error('Error name:', error.name)
      console.error('Error message:', error.message)
      console.error('Error stack:', error.stack)
    }
    return false
  }
}

// Delete a project
export async function deleteProject(id: number): Promise<boolean> {
  try {
    console.log(`[DELETE API] Starting deleteProject for ID: ${id}`);
    
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('[DELETE API] Tauri is not available, cannot delete project');
      return false;
    }
    
    console.log(`[DELETE API] Invoking Tauri delete_project command for ID: ${id}`);
    // The backend returns Result<(), String> which is converted to void or error
    await invoke('delete_project', { id });
    console.log(`[DELETE API] Project deletion successful for ID: ${id}`);
    return true;
  } catch (error) {
    console.error('[DELETE API] Error deleting project:', error);
    // Log more details about the error
    if (error instanceof Error) {
      console.error('[DELETE API] Error name:', error.name);
      console.error('[DELETE API] Error message:', error.message);
      console.error('[DELETE API] Error stack:', error.stack);
    }
    return false;
  }
}

// Get project data by blog ID
export async function getProjectByBlogId(blogId: number): Promise<Project | null> {
  try {
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('Tauri is not available, cannot fetch project by blog ID')
      return null
    }
    
    console.log(`Fetching project for blog ID: ${blogId}...`)
    const project = await invoke<Project>('get_project_by_blog_id', { blogId })
    console.log(`Project found: ${project.title}`)
    return project
  } catch (error) {
    console.error('Error fetching project by blog ID:', error)
    // Log more details about the error
    if (error instanceof Error) {
      console.error('Error name:', error.name)
      console.error('Error message:', error.message)
      console.error('Error stack:', error.stack)
    }
    return null
  }
} 