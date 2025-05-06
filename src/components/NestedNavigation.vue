<template>
  <div class="nested-nav" ref="navListRef">
    <ul class="nav-list">
      <!-- Blog Projects Section -->
      <li class="section-header" v-if="hasBlogProjects">
        <span class="section-title">Blog Projects</span>
        <span class="toggle-section-icon" @click="toggleAllBlogProjects">
          <span v-if="isBlogSectionCollapsed">▶</span>
          <span v-else>▼</span>
        </span>
      </li>
      <li v-for="project in blogProjects" :key="project.id" class="nav-item" v-show="!isBlogSectionCollapsed" :class="{ 'active-project': isProjectActive(project) }">
        <div class="nav-item-header" :class="{ 'active-project': isProjectActive(project) }">
          <div class="nav-item-left" @click="toggleProject(project.id!)">
            <span class="nav-item-icon" :class="getProjectTypeIcon(project.type_)"></span>
            <span class="nav-item-title" :title="project.title">{{ truncateText(project.title, 20) }}</span>
            <span class="nav-item-arrow" :class="{ 'expanded': expandedProjects.includes(project.id!) && !isBlogSectionCollapsed }">▶</span>
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
          <li v-for="blog in getBlogsByProject(project.id!)" :key="blog.id" class="nav-sub-item" :class="{ active: currentBlogId === blog.id }">
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
        <span class="toggle-section-icon" @click="toggleAllBookProjects">
          <span v-if="isBookSectionCollapsed">▶</span>
          <span v-else>▼</span>
        </span>
      </li>
      <li v-for="project in bookProjects" :key="project.id" class="nav-item" v-show="!isBookSectionCollapsed" :class="{ 'active-project': isProjectActive(project) }">
        <div class="nav-item-header" :class="{ 'active-project': isProjectActive(project) }">
          <div class="nav-item-left" @click="toggleProject(project.id!)">
            <span class="nav-item-icon" :class="getProjectTypeIcon(project.type_)"></span>
            <span class="nav-item-title" :title="project.title">{{ truncateText(project.title, 20) }}</span>
            <span class="nav-item-arrow" :class="{ 'expanded': expandedProjects.includes(project.id!) && !isBookSectionCollapsed }">▶</span>
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
          <li v-for="chapter in getChaptersByProject(project.id!)" :key="chapter.id" class="nav-sub-item" :class="{ active: currentChapterId === chapter.id }">
            <router-link 
              :to="`/projects/${project.id}/chapters/${chapter.id}`" 
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
import { ref, onMounted, computed, watch, nextTick } from 'vue'
import { getAllProjects, type ProjectSummary } from '../lib/project'
import { 
  getBlogsByProject as fetchBlogsByProject, 
  getChaptersByProject as fetchChaptersByProject, 
  type Blog, 
  type Chapter 
} from '../lib/content'
import { useRoute } from 'vue-router'

const projects = ref<ProjectSummary[]>([])
const expandedProjects = ref<number[]>([])
const blogsByProject = ref<Record<number, Blog[]>>({})
const chaptersByProject = ref<Record<number, Chapter[]>>({})
const isBlogSectionCollapsed = ref(false)
const isBookSectionCollapsed = ref(false)
const navListRef = ref<HTMLElement | null>(null)
const route = useRoute()

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

// Blog 點擊
const handleBlogClick = (projectId: number, blogId: number) => {
  console.log(`Navigating to blog ${blogId} in project ${projectId}`)
  // 僅確保該 project 展開，不自動收合其他 project
  if (!expandedProjects.value.includes(projectId)) {
    expandedProjects.value.push(projectId)
    saveExpandedState()
  }
  // 不再自動收合其他項目
}

// 新建 Blog 點擊
const handleNewBlogClick = (projectId: number) => {
  console.log(`Creating new blog in project ${projectId}`)
  if (!expandedProjects.value.includes(projectId)) {
    expandedProjects.value.push(projectId)
    saveExpandedState()
  }
}

// Chapter 點擊
const handleChapterClick = (projectId: number, chapterId: number) => {
  console.log(`Navigating to chapter ${chapterId} in project ${projectId}`)
  // 僅確保該 project 展開，不自動收合其他 project
  if (!expandedProjects.value.includes(projectId)) {
    expandedProjects.value.push(projectId)
    saveExpandedState()
  }
}

// 新建 Chapter 點擊
const handleNewChapterClick = (projectId: number) => {
  console.log(`Creating new chapter in project ${projectId}`)
  if (!expandedProjects.value.includes(projectId)) {
    expandedProjects.value.push(projectId)
    saveExpandedState()
  }
}

const toggleAllBlogProjects = () => {
  isBlogSectionCollapsed.value = !isBlogSectionCollapsed.value
  if (isBlogSectionCollapsed.value) {
    // 收合：移除所有 blog project id
    expandedProjects.value = expandedProjects.value.filter(id => {
      const p = projects.value.find(p => p.id === id)
      return p && p.type_ !== 'blog'
    })
  } else {
    // 展開：不自動展開任何 project
    // expandedProjects.value 不變
  }
  saveExpandedState()
}

const toggleAllBookProjects = () => {
  isBookSectionCollapsed.value = !isBookSectionCollapsed.value
  if (isBookSectionCollapsed.value) {
    // 收合：移除所有 book project id
    expandedProjects.value = expandedProjects.value.filter(id => {
      const p = projects.value.find(p => p.id === id)
      return p && p.type_ !== 'book'
    })
  } else {
    // 展開：不自動展開任何 project
    // expandedProjects.value 不變
  }
  saveExpandedState()
}

// 取得目前選中的 blogId/chapterId
const currentBlogId = computed(() => {
  const blogId = route.query.blog_id
  return blogId ? Number(blogId) : null
})
const currentChapterId = computed(() => {
  const match = route.path.match(/\/chapters\/(\d+)/)
  return match ? Number(match[1]) : null
})

// 自動捲動到高亮項目，最多重試 10 次
async function scrollToActiveNavItem(retry = 0) {
  await nextTick()
  const activeEl = navListRef.value?.querySelector('.nav-sub-item.active') as HTMLElement
  console.log('[scrollToActiveNavItem]', { retry, activeEl, navListRef: !!navListRef.value })
  if (activeEl && navListRef.value) {
    const navRect = navListRef.value.getBoundingClientRect()
    const elRect = activeEl.getBoundingClientRect()
    if (elRect.top < navRect.top || elRect.bottom > navRect.bottom) {
      console.log('[scrollToActiveNavItem] scrollIntoView', { elRect, navRect })
      activeEl.scrollIntoView({ block: 'center', behavior: 'auto' })
    } else {
      console.log('[scrollToActiveNavItem] already in view')
    }
  } else if (retry < 10) {
    setTimeout(() => scrollToActiveNavItem(retry + 1), 50)
  } else {
    console.log('[scrollToActiveNavItem] activeEl not found after max retries')
  }
}

// 監聽 route 變化、expandedProjects 變化都觸發 scroll
watch(
  () => route.fullPath,
  () => scrollToActiveNavItem(),
  { immediate: true }
)
watch(expandedProjects, () => scrollToActiveNavItem())

// 新增：判斷 project 是否 active
const isProjectActive = (project: ProjectSummary) => {
  if (project.type_ === 'blog') {
    return getBlogsByProject(project.id!).some(blog => currentBlogId.value === blog.id)
  } else if (project.type_ === 'book') {
    return getChaptersByProject(project.id!).some(chapter => currentChapterId.value === chapter.id)
  }
  return false
}

onMounted(() => {
  loadProjects().then(() => scrollToActiveNavItem())
  window.addEventListener('refresh-sidebar', () => {
    loadProjects().then(() => scrollToActiveNavItem())
  })
})
</script>

<style scoped>
.nested-nav {
  width: 100%;
  max-height: 100vh;
  overflow-y: auto;
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

.toggle-section-icon {
  cursor: pointer;
  margin-left: 0.5rem;
  font-size: 1.1rem;
  user-select: none;
}

.toggle-section-icon:hover {
  color: var(--color-text-primary);
}

.nav-sub-item.active {
  background: #e3f2fd;
}
.nav-sub-item.active .nav-sub-link {
  color: var(--color-primary);
  font-weight: bold;
}

.active-project,
.active-project .nav-item-header,
.active-project .nav-sub-list,
.active-project .nav-sub-item {
  background: #e3f2fd;
}
</style> 