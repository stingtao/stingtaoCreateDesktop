<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import TwoColumnLayout from '../components/layouts/TwoColumnLayout.vue'
import { useRouter } from 'vue-router'
import { getAllProjects, type ProjectSummary } from '../lib/project'
import { getBlogsByProject, type BlogArticle } from '../lib/content'

const { t } = useI18n()
const router = useRouter()

// 模擬數據
const totalProjects = ref(0)
const totalBlogs = ref(0)
const totalWords = ref(0)

const newsItems = ref([
  '🎉 stingtaoCreate 0.1.0 Released - Your AI-Powered Writing Companion',
  '📝 Start Your Writing Journey: Create a Project and Define Clear Writing Goals',
  '✨ Pro Tip: Select Any Text, Click Edit, and Let AI Enhance Your Writing Inline',
  '💡 Need Inspiration? Try Draft Generator to Create Articles Tailored to Your Expertise',
  '🤖 Writing Block? Use AI Conversation to Brainstorm and Refine Your Ideas in Real-Time'
])

const currentNewsIndex = ref(0)
const currentNews = computed(() => newsItems.value[currentNewsIndex.value])

// 定義專案和文章類型
interface RecentArticle {
  id: number
  title: string
  date: Date
}

interface RecentProject {
  id: number
  title: string
  description: string
  targetAudience: string
  type: string
  lastEdited: Date
  progress: number
  recentArticles: RecentArticle[]
}

// 自動輪換新聞
onMounted(() => {
  setInterval(() => {
    currentNewsIndex.value = (currentNewsIndex.value + 1) % newsItems.value.length
  }, 5000)
  
  // 加載專案和部落格數據
  loadProjectsAndBlogs()
})

// 加載專案和部落格數據
const loadProjectsAndBlogs = async () => {
  try {
    // 獲取所有專案
    const projects = await getAllProjects()
    
    // 更新專案總數
    totalProjects.value = projects.length
    
    // 獲取最近的專案（最多3個）
    const recentProjectsData = projects.slice(0, 3)
    
    // 為每個專案獲取最近的部落格文章
    const projectsWithBlogs = await Promise.all(
      recentProjectsData.map(async (project) => {
        // 獲取該專案的所有部落格文章
        const blogs = await getBlogsByProject(project.id || 0)
        
        // 更新部落格總數
        totalBlogs.value += blogs.length
        
        // 計算總字數（簡單估算：每個字符算一個字）
        blogs.forEach(blog => {
          totalWords.value += blog.content.length
        })
        
        // 獲取最近的兩篇文章
        const recentArticles = blogs.slice(0, 2).map(blog => ({
          id: blog.id,
          title: blog.title,
          date: new Date(blog.created_at || new Date())
        }))
        
        // 返回格式化後的專案數據
        return {
          id: project.id || 0,
          title: project.title || '',
          description: project.description || '',
          targetAudience: project.target_audience || '',
          type: project.type_ || '',
          lastEdited: new Date(project.updated_at || new Date()),
          progress: project.progress || 0,
          recentArticles
        } as RecentProject
      })
    )
    
    // 更新最近的專案數據
    recentProjects.value = projectsWithBlogs
  } catch (error) {
    console.error('Failed to load projects and blogs:', error)
  }
}

const recentProjects = ref<RecentProject[]>([])

// 格式化日期
const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('zh-TW').format(date)
}

// 創建新專案
const createNewProject = () => {
  // Navigate to the new project creation view
  router.push('/new-project')
}

// 打開專案
const openProject = (projectId: number) => {
  router.push(`/projects/${projectId}`)
}

// 打開文章
const openArticle = (projectId: number, articleId: number) => {
  router.push(`/projects/${projectId}/articles/${articleId}`)
}

// 導航到專案列表頁面
const navigateToProjects = () => {
  router.push('/projects')
}
</script>

<template>
  <TwoColumnLayout>
    <div class="home-content">
      <!-- 最新消息跑馬燈 -->
      <div class="news-ticker">
        <div class="ticker-content">
          <span class="news-icon">📢</span>
          <span class="news-label">{{ t('home.latestNews') }}:</span>
          <span class="news-text">{{ currentNews }}</span>
        </div>
      </div>

      <!-- 寫作進度和新建按鈕 -->
      <div class="action-section">
        <div class="progress-card" @click="navigateToProjects">
          <h3 class="section-title">{{ t('home.writingProgress') }}</h3>
          <div class="progress-stats">
            <div class="stat-item">
              <span class="stat-icon">📚</span>
              <span class="stat-label">{{ t('home.totalProjects') }}</span>
              <span class="stat-value">{{ totalProjects }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-icon">📝</span>
              <span class="stat-label">{{ t('home.totalBlogs') }}</span>
              <span class="stat-value">{{ totalBlogs }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-icon">📊</span>
              <span class="stat-label">{{ t('home.totalWords') }}</span>
              <span class="stat-value">{{ totalWords.toLocaleString() }}</span>
            </div>
          </div>
        </div>
        <button class="new-project-btn" @click="createNewProject">
          <span class="icon">+</span>
          <span class="btn-text">{{ t('project.new') }}</span>
        </button>
      </div>

      <!-- 最近的專案 -->
      <div class="recent-projects">
        <h3 class="section-title">{{ t('home.recentProjects') }}</h3>
        <div v-if="recentProjects.length === 0" class="no-projects">
          <div class="empty-state">
            <span class="empty-icon">📋</span>
            <p>{{ t('home.noProjects') }}</p>
            <button class="create-project-btn" @click="createNewProject">
              {{ t('project.new') }}
            </button>
          </div>
        </div>
        <div v-else class="project-list">
          <div v-for="project in recentProjects" :key="project.id" class="project-card">
            <div class="project-header" @click="openProject(project.id)">
              <h4 class="project-title">{{ project.title }}</h4>
              <div class="project-meta">
                <span class="project-type">{{ t(`project.type.${project.type}`) }}</span>
                <span class="last-edited">{{ t('home.lastEdited') }}: {{ formatDate(project.lastEdited) }}</span>
              </div>
            </div>
            
            <div class="project-details">
              <div class="project-description">
                <div class="description-item">
                  <span class="description-label">{{ t('home.description') }}:</span>
                  <span class="description-text">{{ project.description }}</span>
                </div>
                <div class="description-item">
                  <span class="description-label">{{ t('home.targetAudience') }}:</span>
                  <span class="description-text">{{ project.targetAudience }}</span>
                </div>
              </div>
              
              <div class="project-progress">
                <div class="progress-label">
                  <span>Progress</span>
                  <span class="progress-text">{{ project.progress }}%</span>
                </div>
                <div class="progress-bar">
                  <div :style="{ width: project.progress + '%' }" class="progress-fill"></div>
                </div>
              </div>
              
              <div class="recent-articles">
                <h5 class="articles-title">{{ t('home.recentArticles') }}</h5>
                <ul v-if="project.recentArticles.length > 0" class="articles-list">
                  <li v-for="article in project.recentArticles" :key="article.id" @click="openArticle(project.id, article.id)" class="article-item">
                    <span class="article-title">{{ article.title }}</span>
                    <span class="article-date">{{ formatDate(article.date) }}</span>
                  </li>
                </ul>
                <p v-else class="no-articles">{{ t('home.noArticles') }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </TwoColumnLayout>
</template>

<style scoped>
.home-content {
  padding: 2rem;
  transition: padding 0.3s ease;
  max-width: 1200px;
  margin: 0 auto;
}

/* 最新消息跑馬燈 */
.news-ticker {
  background-color: var(--color-bg-secondary);
  padding: 0.75rem 1rem;
  border-radius: 8px;
  margin-bottom: 2rem;
  overflow: hidden;
  transition: all 0.3s ease;
  box-shadow: var(--shadow-sm);
  border-left: 4px solid var(--color-primary);
  position: relative;
}

.ticker-content {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  animation: ticker 20s linear infinite;
  white-space: nowrap;
  padding-right: 2rem;
}

.news-icon {
  margin-right: 0.5rem;
  font-size: 1.2rem;
  color: var(--color-primary);
  animation: pulse 2s infinite;
}

.news-label {
  font-weight: 600;
  margin-right: 0.5rem;
  color: var(--color-text-on-primary);
  background-color: var(--color-primary);
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
}

.news-text {
  color: var(--color-text-primary);
  font-weight: 500;
}

@keyframes ticker {
  0% {
    transform: translateX(100%);
  }
  100% {
    transform: translateX(-100%);
  }
}

@keyframes pulse {
  0% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.1);
  }
  100% {
    transform: scale(1);
  }
}

/* 寫作進度和新建按鈕 */
.action-section {
  display: flex;
  gap: 2rem;
  margin-bottom: 2.5rem;
  transition: flex-direction 0.3s ease, gap 0.3s ease, margin-bottom 0.3s ease;
}

.progress-card {
  flex: 1;
  background-color: var(--color-bg-secondary);
  padding: 1.5rem;
  border-radius: 12px;
  box-shadow: var(--shadow-sm);
  transition: background-color 0.3s, box-shadow 0.3s, padding 0.3s, transform 0.2s;
  cursor: pointer;
  border: 1px solid var(--color-border-primary);
}

.progress-card:hover {
  transform: translateY(-3px);
  box-shadow: var(--shadow-md);
}

.section-title {
  color: var(--color-text-primary);
  margin-bottom: 1.25rem;
  font-size: 1.25rem;
  font-weight: 600;
  transition: color 0.3s, font-size 0.3s;
}

.progress-stats {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1.5rem;
  margin-top: 1rem;
  transition: grid-template-columns 0.3s ease, gap 0.3s ease;
}

.stat-item {
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 0.75rem;
  border-radius: 8px;
  background-color: var(--color-bg-tertiary);
  transition: background-color 0.3s;
}

.stat-item:hover {
  background-color: var(--color-bg-hover);
}

.stat-icon {
  font-size: 1.5rem;
  margin-bottom: 0.5rem;
}

.stat-label {
  display: block;
  color: var(--color-text-secondary);
  font-size: 0.9rem;
  margin-bottom: 0.5rem;
  transition: color 0.3s, font-size 0.3s;
}

.stat-value {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--color-text-primary);
  transition: color 0.3s, font-size 0.3s;
}

.new-project-btn {
  padding: 1rem 2rem;
  background-color: var(--color-primary);
  color: var(--color-text-on-primary);
  border: none;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 1rem;
  font-weight: 500;
  transition: all 0.2s, padding 0.3s;
  white-space: nowrap;
  box-shadow: var(--shadow-sm);
  align-self: flex-start;
}

.new-project-btn:hover {
  background-color: var(--color-primary-dark);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.icon {
  font-size: 1.2rem;
  font-weight: bold;
}

/* 最近的專案 */
.recent-projects {
  margin-bottom: 2rem;
  transition: margin-bottom 0.3s ease;
}

.no-projects {
  text-align: center;
  padding: 3rem 2rem;
  background-color: var(--color-bg-secondary);
  border-radius: 12px;
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--color-border-primary);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}

.empty-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
  color: var(--color-text-tertiary);
}

.no-projects p {
  margin-bottom: 1.5rem;
  color: var(--color-text-secondary);
  font-size: 1.1rem;
}

.create-project-btn {
  padding: 0.75rem 1.5rem;
  background-color: var(--color-primary);
  color: var(--color-text-on-primary);
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 500;
  transition: all 0.2s;
  box-shadow: var(--shadow-sm);
}

.create-project-btn:hover {
  background-color: var(--color-primary-dark);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.project-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 1.5rem;
  margin-top: 1rem;
  transition: grid-template-columns 0.3s ease, gap 0.3s ease;
}

.project-card {
  background-color: var(--color-bg-secondary);
  border-radius: 12px;
  box-shadow: var(--shadow-sm);
  overflow: hidden;
  transition: all 0.2s, background-color 0.3s, box-shadow 0.3s;
  border: 1px solid var(--color-border-primary);
}

.project-card:hover {
  transform: translateY(-3px);
  box-shadow: var(--shadow-md);
}

.project-header {
  padding: 1.5rem;
  border-bottom: 1px solid var(--color-border-primary);
  cursor: pointer;
  transition: background-color 0.2s;
}

.project-header:hover {
  background-color: var(--color-bg-hover);
}

.project-title {
  margin: 0 0 0.75rem 0;
  color: var(--color-text-primary);
  font-size: 1.25rem;
  font-weight: 600;
  transition: color 0.3s, font-size 0.3s;
}

.project-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.project-type {
  color: var(--color-text-on-primary);
  font-size: 0.9rem;
  font-weight: 500;
  background-color: var(--color-primary);
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  transition: color 0.3s, font-size 0.3s;
}

.last-edited {
  color: var(--color-text-secondary);
  font-size: 0.8rem;
  transition: color 0.3s, font-size 0.3s;
}

.project-details {
  padding: 1.5rem;
}

.project-description {
  margin-bottom: 1.5rem;
}

.description-item {
  margin-bottom: 0.75rem;
  display: flex;
  flex-direction: column;
}

.description-label {
  font-weight: 600;
  color: var(--color-text-secondary);
  margin-bottom: 0.25rem;
  font-size: 0.9rem;
}

.description-text {
  color: var(--color-text-primary);
  line-height: 1.5;
}

.project-progress {
  margin: 1.5rem 0;
}

.progress-label {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
  color: var(--color-text-secondary);
}

.progress-text {
  font-weight: 600;
  color: var(--color-primary);
}

.progress-bar {
  height: 8px;
  background-color: var(--color-bg-tertiary);
  border-radius: 4px;
  overflow: hidden;
  transition: background-color 0.3s, height 0.3s;
}

.progress-fill {
  height: 100%;
  background-color: var(--color-success);
  transition: width 0.3s ease;
}

.recent-articles {
  margin-top: 1.5rem;
}

.articles-title {
  margin: 0 0 0.75rem 0;
  color: var(--color-text-primary);
  font-size: 1rem;
  font-weight: 600;
}

.no-articles {
  color: var(--color-text-secondary);
  font-style: italic;
  font-size: 0.9rem;
  text-align: center;
  padding: 0.75rem 0;
  background-color: var(--color-bg-tertiary);
  border-radius: 4px;
}

.articles-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.article-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  border-bottom: 1px solid var(--color-border-primary);
  cursor: pointer;
  transition: background-color 0.2s;
}

.article-item:last-child {
  border-bottom: none;
}

.article-item:hover {
  background-color: var(--color-bg-hover);
}

.article-title {
  color: var(--color-primary);
  font-weight: 500;
  flex: 1;
  margin-right: 1rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.article-date {
  color: var(--color-text-secondary);
  font-size: 0.8rem;
  white-space: nowrap;
}

/* 響應式設計 */
@media (max-width: 1200px) {
  .home-content {
    padding: 1.75rem;
  }
  
  .action-section {
    gap: 1.5rem;
  }
  
  .progress-card {
    padding: 1.25rem;
  }
  
  .stat-value {
    font-size: 1.3rem;
  }
  
  .project-list {
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  }
}

@media (max-width: 992px) {
  .home-content {
    padding: 1.5rem;
  }
  
  .news-ticker {
    padding: 0.6rem 0.75rem;
    margin-bottom: 1.5rem;
  }
  
  .action-section {
    gap: 1rem;
    margin-bottom: 1.5rem;
  }
  
  .progress-card {
    padding: 1.25rem;
  }
  
  .progress-stats {
    gap: 0.75rem;
  }
  
  .stat-label {
    font-size: 0.85rem;
  }
  
  .stat-value {
    font-size: 1.2rem;
  }
  
  .new-project-btn {
    padding: 0.875rem 1.5rem;
  }
  
  .project-list {
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  }
  
  .project-header, .project-details {
    padding: 1.25rem;
  }
}

@media (max-width: 768px) {
  .home-content {
    padding: 1.25rem;
  }
  
  .news-ticker {
    padding: 0.5rem 0.75rem;
    margin-bottom: 1.25rem;
  }
  
  .action-section {
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 1.25rem;
  }
  
  .new-project-btn {
    width: 100%;
    justify-content: center;
  }
  
  .progress-stats {
    grid-template-columns: repeat(3, 1fr);
  }
  
  .project-list {
    grid-template-columns: 1fr;
  }
  
  .project-header, .project-details {
    padding: 1rem;
  }
  
  .project-title {
    font-size: 1.1rem;
  }
  
  .section-title {
    font-size: 1.2rem;
  }
}

@media (max-width: 576px) {
  .home-content {
    padding: 1rem;
  }
  
  .news-ticker {
    padding: 0.4rem 0.75rem;
    margin-bottom: 1rem;
    border-radius: 6px;
  }
  
  .action-section {
    gap: 0.75rem;
    margin-bottom: 1rem;
  }
  
  .progress-card {
    padding: 1rem;
    border-radius: 8px;
  }
  
  .progress-stats {
    gap: 0.5rem;
  }
  
  .stat-label {
    font-size: 0.8rem;
  }
  
  .stat-value {
    font-size: 1.1rem;
  }
  
  .new-project-btn {
    padding: 0.75rem 1rem;
  }
  
  .project-header, .project-details {
    padding: 0.875rem;
  }
  
  .project-title {
    font-size: 1rem;
  }
  
  .project-type {
    font-size: 0.8rem;
  }
  
  .last-edited {
    font-size: 0.75rem;
  }
  
  .progress-bar {
    height: 6px;
  }
  
  .progress-text {
    font-size: 0.75rem;
  }
  
  .section-title {
    font-size: 1.1rem;
    margin-bottom: 0.75rem;
  }
}
</style> 