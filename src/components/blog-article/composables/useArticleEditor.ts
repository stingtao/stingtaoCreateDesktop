import { ref, computed } from 'vue'
import { saveBlogArticle, getBlogsByProject, type BlogArticle } from '../../../lib/content'

export function useArticleEditor(projectId: number) {
  const article = ref<BlogArticle>({
    id: 0,
    title: '',
    content: '',
    keywords: [],
    project_id: projectId,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString()
  })

  const isSaving = ref(false)
  const isSaved = ref(false)
  const saveTimeout = ref<number | null>(null)

  const saveStatusText = computed(() => {
    if (isSaving.value) return 'Saving...'
    if (isSaved.value) return 'Saved'
    return 'Not saved'
  })

  const debounceSave = () => {
    isSaved.value = false
    
    if (saveTimeout.value) {
      clearTimeout(saveTimeout.value)
    }
    
    saveTimeout.value = window.setTimeout(() => {
      saveArticle()
    }, 10000)
  }

  const saveArticle = async () => {
    isSaving.value = true
    
    try {
      article.value.project_id = projectId
      const savedArticle = await saveBlogArticle(article.value)
      article.value = savedArticle
      isSaved.value = true
    } catch (error) {
      console.error('Failed to save article:', error)
    } finally {
      isSaving.value = false
    }
  }

  const loadExistingArticle = async (blogId: number) => {
    if (blogId && projectId) {
      try {
        console.log(`Loading blog with ID: ${blogId} from project: ${projectId}`)
        const blogs = await getBlogsByProject(projectId)
        const existingBlog = blogs.find(blog => blog.id === blogId)
        if (existingBlog) {
          console.log('Found existing blog:', existingBlog)
          article.value = existingBlog
          isSaved.value = true
        } else {
          console.warn(`Blog with ID ${blogId} not found in project ${projectId}`)
        }
      } catch (error) {
        console.error('Failed to load existing article:', error)
      }
    }
  }

  return {
    article,
    isSaving,
    isSaved,
    saveStatusText,
    debounceSave,
    saveArticle,
    loadExistingArticle
  }
} 