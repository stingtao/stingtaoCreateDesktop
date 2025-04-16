<template>
  <div class="nested-nav">
    <ul class="nav-list">
      <!-- Blog Projects Section -->
      <li class="section-header" v-if="hasBlogProjects">
        <span class="section-title">Blog Projects</span>
      </li>
      <li v-for="project in blogProjects" :key="project.id" class="nav-item">
        <div class="nav-item-header">
          <div class="nav-item-left" @click="toggleProject(project.id!)">
            <span class="nav-item-icon" :class="getProjectTypeIcon(project.type_)"></span>
            <span class="nav-item-title" :title="project.title">{{ truncateText(project.title, 20) }}</span>
            <span class="nav-item-arrow" :class="{ 'expanded': expandedProjects.includes(project.id!) }">▶</span>
          </div>
          <router-link 
            :to="`/new-blog-article?project_id=${project.id}&project_name=${encodeURIComponent(project.title)}`" 
            class="add-btn" 
            title="New Blog Article"
            @click.native="handleNewBlogClick(project.id!)"
          >
            <span class="add-icon">+</span>
          </router-link>
        </div>
        
        <ul v-if="expandedProjects.includes(project.id!)" class="nav-sub-list">
          <li v-for="blog in getBlogsByProject(project.id!)" :key="blog.id" class="nav-sub-item">
            <router-link 
              :to="`/new-blog-article?project_id=${project.id}&blog_id=${blog.id}&project_name=${encodeURIComponent(project.title)}`" 
              class="nav-sub-link"
              @click.native="handleBlogClick(project.id!, blog.id)"
            >
              <span class="nav-sub-icon">📝</span>
              <span class="nav-sub-title" :title="blog.title">{{ truncateText(blog.title, 25) }}</span>
            </router-link>
          </li>
        </ul>
      </li>

      <!-- Book Projects Section -->
      <li class="section-header" v-if="hasBookProjects">
        <span class="section-title">Book Projects</span>
      </li>
      <li v-for="project in bookProjects" :key="project.id" class="nav-item">
        <div class="nav-item-header">
          <div class="nav-item-left" @click="toggleProject(project.id!)">
            <span class="nav-item-icon" :class="getProjectTypeIcon(project.type_)"></span>
            <span class="nav-item-title" :title="project.title">{{ truncateText(project.title, 20) }}</span>
            <span class="nav-item-arrow" :class="{ 'expanded': expandedProjects.includes(project.id!) }">▶</span>
          </div>
          <router-link 
            :to="`/new-chapter?project_id=${project.id}&project_name=${encodeURIComponent(project.title)}`" 
            class="add-btn" 
            title="New Chapter"
            @click.native="handleNewChapterClick(project.id!)"
          >
            <span class="add-icon">+</span>
          </router-link>
        </div>
        
        <ul v-if="expandedProjects.includes(project.id!)" class="nav-sub-list">
          <li v-for="chapter in getChaptersByProject(project.id!)" :key="chapter.id" class="nav-sub-item">
            <router-link 
              :to="`/new-chapter?project_id=${project.id}&chapter_id=${chapter.id}&project_name=${encodeURIComponent(project.title)}`" 
              class="nav-sub-link"
              @click.native="handleChapterClick(project.id!, chapter.id)"
            >
              <span class="nav-sub-icon">📑</span>
              <span class="nav-sub-title" :title="chapter.title">{{ truncateText(chapter.title, 25) }}</span>
            </router-link>
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { getAllProjects, type ProjectSummary } from '../lib/project'
import { 
  getBlogsByProject as fetchBlogsByProject, 
  getChaptersByProject as fetchChaptersByProject, 
  type Blog, 
  type Chapter 
} from '../lib/content'

const projects = ref<ProjectSummary[]>([])
const expandedProjects = ref<number[]>([])
const blogsByProject = ref<Record<number, Blog[]>>({})
const chaptersByProject = ref<Record<number, Chapter[]>>({})

// 從 localStorage 加載展開狀態
const loadExpandedState = () => {
  try {
    const savedState = localStorage.getItem('expandedProjects')
    if (savedState) {
      const parsedState = JSON.parse(savedState)
      expandedProjects.value = parsedState
      
      // 加載已展開項目的內容
      parsedState.forEach(async (projectId: number) => {
        const project = projects.value.find(p => p.id === projectId)
        if (project) {
          if (project.type_ === 'blog') {
            blogsByProject.value[projectId] = await fetchBlogsByProject(projectId)
          } else if (project.type_ === 'book') {
            chaptersByProject.value[projectId] = await fetchChaptersByProject(projectId)
          }
        }
      })
    }
  } catch (error) {
    console.error('Failed to load expanded state:', error)
  }
}

// 保存展開狀態到 localStorage
const saveExpandedState = () => {
  try {
    localStorage.setItem('expandedProjects', JSON.stringify(expandedProjects.value))
  } catch (error) {
    console.error('Failed to save expanded state:', error)
  }
}

// 計算屬性：分類專案，只包含有 ID 的專案
const blogProjects = computed(() => projects.value.filter(p => p.type_ === 'blog' && p.id !== undefined))
const bookProjects = computed(() => projects.value.filter(p => p.type_ === 'book' && p.id !== undefined))
const hasBlogProjects = computed(() => blogProjects.value.length > 0)
const hasBookProjects = computed(() => bookProjects.value.length > 0)

// 文字截斷函數
const truncateText = (text: string, maxLength: number) => {
  if (text.length <= maxLength) return text
  return text.slice(0, maxLength) + '...'
}

// Load projects
const loadProjects = async () => {
  projects.value = await getAllProjects()
  // 加載項目後，加載展開狀態
  loadExpandedState()
}

// Toggle project expansion
const toggleProject = async (projectId: number) => {
  const index = expandedProjects.value.indexOf(projectId)
  if (index === -1) {
    expandedProjects.value.push(projectId)
    // Load content when expanding
    const project = projects.value.find(p => p.id === projectId)
    if (project) {
      if (project.type_ === 'blog') {
        blogsByProject.value[projectId] = await fetchBlogsByProject(projectId)
      } else if (project.type_ === 'book') {
        chaptersByProject.value[projectId] = await fetchChaptersByProject(projectId)
      }
    }
  } else {
    expandedProjects.value.splice(index, 1)
  }
  
  // 保存展開狀態
  saveExpandedState()
}

// Get project type icon
const getProjectTypeIcon = (type: string) => {
  switch (type) {
    case 'blog':
      return 'i-heroicons-document-text'
    case 'book':
      return 'i-heroicons-book-open'
    default:
      return 'i-heroicons-document'
  }
}

// Get blogs by project
const getBlogsByProject = (projectId: number) => {
  return blogsByProject.value[projectId] || []
}

// Get chapters by project
const getChaptersByProject = (projectId: number) => {
  return chaptersByProject.value[projectId] || []
}

// 處理博客點擊
const handleBlogClick = (projectId: number, blogId: number) => {
  console.log(`Navigating to blog ${blogId} in project ${projectId}`)
  
  // 確保項目已展開
  if (!expandedProjects.value.includes(projectId)) {
    expandedProjects.value.push(projectId)
    saveExpandedState()
  }

  // 清除其他項目的展開狀態
  expandedProjects.value = expandedProjects.value.filter(id => id === projectId)
  saveExpandedState()
}

// 處理新建博客點擊
const handleNewBlogClick = (projectId: number) => {
  console.log(`Creating new blog in project ${projectId}`)
  
  // 確保項目已展開
  if (!expandedProjects.value.includes(projectId)) {
    expandedProjects.value.push(projectId)
    saveExpandedState()
  }
}

// 處理章節點擊
const handleChapterClick = (projectId: number, chapterId: number) => {
  console.log(`Navigating to chapter ${chapterId} in project ${projectId}`)
  
  // 確保項目已展開
  if (!expandedProjects.value.includes(projectId)) {
    expandedProjects.value.push(projectId)
    saveExpandedState()
  }
}

// 處理新建章節點擊
const handleNewChapterClick = (projectId: number) => {
  console.log(`Creating new chapter in project ${projectId}`)
  
  // 確保項目已展開
  if (!expandedProjects.value.includes(projectId)) {
    expandedProjects.value.push(projectId)
    saveExpandedState()
  }
}

onMounted(() => {
  loadProjects()
})
</script>

<style scoped>
.nested-nav {
  width: 100%;
}

.nav-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.section-header {
  padding: 0.5rem 1rem;
  margin-top: 0.5rem;
  border-top: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.section-title {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.nav-item {
  margin: 0;
  padding: 0;
}

.nav-item-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  transition: background-color 0.2s;
}

.nav-item-left {
  display: flex;
  align-items: center;
  flex: 1;
  cursor: pointer;
}

.nav-item-left:hover {
  background-color: var(--color-bg-secondary);
}

.nav-item-icon {
  margin-right: 0.5rem;
  font-size: 1.2rem;
}

.nav-item-title {
  flex: 1;
  font-size: 0.9rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.nav-item-arrow {
  font-size: 0.8rem;
  transition: transform 0.2s;
  margin-left: 0.5rem;
}

.nav-item-arrow.expanded {
  transform: rotate(90deg);
}

.nav-sub-list {
  list-style: none;
  padding: 0;
  margin: 0;
  padding-left: 1.5rem;
}

.nav-sub-item {
  margin: 0;
  padding: 0;
}

.nav-sub-link {
  display: flex;
  align-items: center;
  padding: 0.5rem 1rem;
  text-decoration: none;
  color: var(--color-text-primary);
  transition: background-color 0.2s, color 0.2s;
}

.nav-sub-link:hover {
  background-color: var(--color-bg-secondary);
}

.nav-sub-icon {
  margin-right: 0.5rem;
  font-size: 1rem;
}

.nav-sub-title {
  font-size: 0.85rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--color-text-primary);
}

/* 響應式設計 */
@media (max-width: 1200px) {
  .nav-item-title {
    max-width: 150px;
  }
  .nav-sub-title {
    max-width: 130px;
  }
}

@media (max-width: 992px) {
  .nav-item-title {
    max-width: 120px;
  }
  .nav-sub-title {
    max-width: 100px;
  }
}

@media (max-width: 768px) {
  .nav-item-title {
    max-width: 100px;
  }
  .nav-sub-title {
    max-width: 80px;
  }
}

.add-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background-color: var(--color-bg-secondary);
  color: var(--color-text-secondary);
  text-decoration: none;
  transition: background-color 0.2s, color 0.2s;
  margin-left: 0.5rem;
}

.add-btn:hover {
  background-color: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

.add-icon {
  font-size: 1.2rem;
  line-height: 1;
}
</style> 