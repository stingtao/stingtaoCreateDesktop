<template>
  <div class="layout">
    <!-- 左側選單 -->
    <div class="sidebar" :class="{ 'sidebar-collapsed': isSidebarCollapsed }" :style="{ width: sidebarWidth + 'px' }">
      <div class="sidebar-header">
        <div class="app-brand" :class="{ 'collapsed': isSidebarCollapsed }">
          <div class="app-logo">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"></path>
            </svg>
          </div>
          <div class="app-info" v-if="!isSidebarCollapsed">
            <div class="app-name">stingtaoCreate</div>
            <div class="app-version">v0.1</div>
          </div>
        </div>
        <button class="collapse-btn" @click="toggleSidebar" :title="isSidebarCollapsed ? 'Expand Sidebar' : 'Collapse Sidebar'">
          <svg v-if="isSidebarCollapsed" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="3" y1="12" x2="21" y2="12"></line><line x1="3" y1="6" x2="21" y2="6"></line><line x1="3" y1="18" x2="21" y2="18"></line></svg>
          <svg v-else xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="15 18 9 12 15 6"></polyline></svg>
        </button>
      </div>
      <nav class="sidebar-nav" v-if="!isSidebarCollapsed">
        <ul>
          <li>
            <router-link to="/" class="nav-item">
              {{ t('nav.home') }}
            </router-link>
          </li>
          <NestedNavigation />
          <li>
            <router-link to="/settings" class="nav-item">
              {{ t('nav.settings') }}
            </router-link>
          </li>
          <li>
            <router-link to="/help" class="nav-item">
              {{ t('nav.help') }}
            </router-link>
          </li>
        </ul>
      </nav>
    </div>

    <!-- 左側分隔線 -->
    <div 
      v-if="!isSidebarCollapsed"
      class="resizer left-resizer"
      @mousedown="(e) => startResize('left', e)"
      @touchstart="(e) => startResize('left', e)"
    ></div>

    <!-- 主要內容區域 -->
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ref, onMounted, onUnmounted } from 'vue'
import NestedNavigation from '../NestedNavigation.vue'

const { t } = useI18n()

// 區域寬度狀態
const sidebarWidth = ref(250)

// 折疊狀態
const isSidebarCollapsed = ref(false)

// 拖動相關狀態
const isResizing = ref(false)
const currentResizer = ref<'left' | null>(null)
const startX = ref(0)
const startWidth = ref(0)

// 從 localStorage 加載保存的寬度和折疊狀態
onMounted(() => {
  const savedSidebarWidth = localStorage.getItem('sidebarWidth')
  const savedSidebarCollapsed = localStorage.getItem('sidebarCollapsed')
  
  if (savedSidebarWidth) sidebarWidth.value = parseInt(savedSidebarWidth)
  if (savedSidebarCollapsed) isSidebarCollapsed.value = savedSidebarCollapsed === 'true'
  
  // 檢查螢幕寬度，在小螢幕上自動折疊側邊欄
  checkScreenSize()
  
  // 監聽視窗大小變化
  window.addEventListener('resize', checkScreenSize)
})

// 檢查螢幕大小並調整佈局
const checkScreenSize = () => {
  if (window.innerWidth < 768) {
    isSidebarCollapsed.value = true
  } else if (window.innerWidth < 1024) {
    isSidebarCollapsed.value = false
  } else {
    isSidebarCollapsed.value = false
  }
  
  // 保存折疊狀態
  localStorage.setItem('sidebarCollapsed', isSidebarCollapsed.value.toString())
}

// 切換側邊欄折疊狀態
const toggleSidebar = () => {
  isSidebarCollapsed.value = !isSidebarCollapsed.value
  localStorage.setItem('sidebarCollapsed', isSidebarCollapsed.value.toString())
}

// 開始調整大小
const startResize = (resizer: 'left', event: MouseEvent | TouchEvent) => {
  isResizing.value = true
  currentResizer.value = resizer
  startX.value = event instanceof MouseEvent ? event.clientX : event.touches[0].clientX
  startWidth.value = sidebarWidth.value
  
  document.addEventListener('mousemove', handleResize)
  document.addEventListener('mouseup', stopResize)
  document.addEventListener('touchmove', handleResize)
  document.addEventListener('touchend', stopResize)
}

// 處理調整大小
const handleResize = (event: MouseEvent | TouchEvent) => {
  if (!isResizing.value) return
  
  const currentX = event instanceof MouseEvent ? event.clientX : event.touches[0].clientX
  const diff = currentX - startX.value
  
  if (currentResizer.value === 'left') {
    const newWidth = Math.max(200, Math.min(400, startWidth.value + diff))
    sidebarWidth.value = newWidth
    localStorage.setItem('sidebarWidth', newWidth.toString())
  }
}

// 停止調整大小
const stopResize = () => {
  isResizing.value = false
  currentResizer.value = null
  document.removeEventListener('mousemove', handleResize)
  document.removeEventListener('mouseup', stopResize)
  document.removeEventListener('touchmove', handleResize)
  document.removeEventListener('touchend', stopResize)
}

// 清理事件監聽器
onUnmounted(() => {
  document.removeEventListener('mousemove', handleResize)
  document.removeEventListener('mouseup', stopResize)
  document.removeEventListener('touchmove', handleResize)
  document.removeEventListener('touchend', stopResize)
  window.removeEventListener('resize', checkScreenSize)
})
</script>

<style scoped>
/* Base Layout Styles */
.layout {
  display: flex;
  height: 100vh;
  width: 100%;
  background-color: var(--color-bg-primary);
  position: relative;
  overflow: hidden;
  transition: background-color 0.3s ease;
}

/* Sidebar Styles */
.sidebar {
  height: 100%;
  background-color: var(--color-bg-tertiary);
  border-right: 1px solid var(--color-border-primary);
  transition: width 0.3s ease, background-color 0.3s ease;
  display: flex;
  flex-direction: column;
  position: relative;
  z-index: 10;
  flex-shrink: 0;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.1);
}

.sidebar-collapsed {
  width: 64px !important;
}

.sidebar-header {
  padding: 1rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--color-border-primary);
  background-color: var(--color-bg-secondary);
  transition: background-color 0.3s ease;
}

/* Adjust header justification when collapsed */
.sidebar-collapsed .sidebar-header {
  justify-content: center;
  padding: 1rem 0.5rem;
}

.sidebar-nav {
  flex: 1;
  overflow-y: auto;
  padding: 1rem 0;
}

.sidebar-nav ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.nav-item {
  display: block;
  padding: 0.75rem 1rem;
  color: var(--color-text-primary);
  text-decoration: none;
  transition: all 0.3s ease;
  border-radius: 4px;
  margin: 0 0.5rem;
  font-weight: 500;
  position: relative;
  overflow: hidden;
}

.nav-item:hover {
  background-color: var(--color-bg-secondary);
  color: var(--color-primary);
  transform: translateX(4px);
}

.nav-item.router-link-active {
  background-color: var(--color-bg-secondary);
  color: var(--color-primary);
  font-weight: 600;
  box-shadow: 0 2px 4px var(--color-shadow);
}

.nav-item.router-link-active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  width: 3px;
  background-color: var(--color-primary);
}

/* App Brand Styles */
.app-brand {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem;
  border-radius: 6px;
  transition: all 0.3s ease;
}

.app-brand.collapsed {
  justify-content: center;
  padding: 0.5rem;
}

.app-logo {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 6px;
  background-color: var(--color-bg-tertiary);
  color: var(--color-primary);
  transition: all 0.3s ease;
}

.app-info {
  display: flex;
  flex-direction: column;
}

.app-name {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--color-text-primary);
  line-height: 1.2;
}

.app-version {
  font-size: 0.75rem;
  color: var(--color-text-tertiary);
  line-height: 1;
}

/* Collapse Button Styles */
.collapse-btn {
  background: none;
  border: none;
  padding: 0.5rem;
  border-radius: 6px;
  cursor: pointer;
  color: var(--color-text-secondary);
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
}

.collapse-btn:hover {
  background-color: var(--color-bg-secondary);
  color: var(--color-primary);
  transform: scale(1.1);
}

/* Resizer Styles */
.resizer {
  width: 4px;
  background-color: var(--color-border-primary);
  cursor: col-resize;
  transition: background-color 0.3s ease;
  flex-shrink: 0;
  z-index: 5;
}

.resizer:hover {
  background-color: var(--color-primary);
  width: 6px;
}

.left-resizer {
  margin-right: -2px;
}

/* Nested Navigation Styles */
:deep(.nested-nav) {
  margin-top: 0.5rem;
}

:deep(.nested-nav-item) {
  padding: 0.5rem 1rem 0.5rem 2.5rem;
  color: var(--color-text-secondary);
  font-size: 0.9rem;
  transition: all 0.3s ease;
}

:deep(.nested-nav-item:hover) {
  color: var(--color-text-primary);
  background-color: var(--color-bg-secondary);
  transform: translateX(4px);
}

:deep(.nested-nav-item.router-link-active) {
  color: var(--color-primary);
  font-weight: 500;
  background-color: var(--color-bg-secondary);
}

:deep(.nested-nav-item.router-link-active::before) {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  width: 2px;
  background-color: var(--color-primary);
}

/* Scrollbar Styles */
.sidebar-nav::-webkit-scrollbar {
  width: 6px;
}

.sidebar-nav::-webkit-scrollbar-track {
  background-color: var(--color-bg-tertiary);
}

.sidebar-nav::-webkit-scrollbar-thumb {
  background-color: var(--color-border-primary);
  border-radius: 3px;
}

.sidebar-nav::-webkit-scrollbar-thumb:hover {
  background-color: var(--color-border-secondary);
}

/* Responsive Styles */
@media (max-width: 1200px) {
  .sidebar {
    width: 220px;
  }
}

@media (max-width: 992px) {
  .sidebar {
    width: 200px;
  }
  
  .sidebar-header {
    padding: 1.25rem;
  }
  
  .nav-item {
    padding: 0.6rem 1.25rem;
  }
}

@media (max-width: 768px) {
  .sidebar {
    position: fixed;
    left: 0;
    top: 0;
    height: 100vh;
    transform: translateX(0);
    transition: transform 0.3s ease, width 0.3s ease;
  }
  
  .sidebar-collapsed {
    transform: translateX(-100%);
  }
  
  .sidebar-header {
    padding: 1rem;
  }
  
  .nav-item {
    padding: 0.5rem 1rem;
  }
  
  h2 {
    font-size: 1.1rem;
  }
}

@media (max-width: 576px) {
  .sidebar-collapsed {
    width: 50px !important;
  }
  
  .sidebar-header {
    padding: 0.75rem;
  }
  
  .collapse-btn {
    font-size: 1rem;
  }
}

/* Title Styles */
h2 {
  margin: 0;
  font-size: 1.1rem; /* Slightly smaller title */
  font-weight: 600;
  color: var(--color-text-primary); /* Use CSS variable instead of hardcoded color */
  white-space: nowrap; /* Prevent title wrapping */
  overflow: hidden;
  text-overflow: ellipsis;
}

/* Remove global user-select: none; */
/* Let JS handle user-select during drag */

/* Add styles for nested navigation if needed */
:deep(.nested-nav) {
  /* Styles for NestedNavigation.vue might need adjustment here */
  /* Example: */
  margin-left: 1rem;
}

/* 響應式設計 */
@media (max-width: 1200px) {
  .sidebar {
    width: 220px;
  }
}

@media (max-width: 992px) {
  .sidebar {
    width: 200px;
  }
  
  .sidebar-header {
    padding: 1.25rem;
  }
  
  .nav-item {
    padding: 0.6rem 1.25rem;
  }
}

@media (max-width: 768px) {
  .sidebar {
    position: fixed;
    left: 0;
    top: 0;
    height: 100vh;
    transform: translateX(0);
    transition: transform 0.3s ease, width 0.3s ease;
  }
  
  .sidebar-collapsed {
    transform: translateX(-100%);
  }
  
  .sidebar-header {
    padding: 1rem;
  }
  
  .nav-item {
    padding: 0.5rem 1rem;
  }
  
  h2 {
    font-size: 1.1rem;
  }
}

@media (max-width: 576px) {
  .sidebar-collapsed {
    width: 50px !important;
  }
  
  .sidebar-header {
    padding: 0.75rem;
  }
  
  .collapse-btn {
    font-size: 1rem;
  }
}

/* App Brand Styles */
.app-brand {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  color: var(--color-text-primary);
  transition: color 0.3s ease;
}

.app-brand.collapsed {
  justify-content: center;
}

.app-logo {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-primary);
  transition: color 0.3s ease;
}

.app-info {
  display: flex;
  flex-direction: column;
}

.app-name {
  font-weight: 600;
  font-size: 1.1rem;
  color: var(--color-text-primary);
  transition: color 0.3s ease;
}

.app-version {
  font-size: 0.8rem;
  color: var(--color-text-secondary);
  transition: color 0.3s ease;
}

/* Adjust sidebar header for collapsed state */
.sidebar-collapsed .sidebar-header {
  justify-content: center;
  padding: 1rem 0.5rem;
}

.sidebar-collapsed .app-logo {
  margin: 0;
}

.sidebar-collapsed .app-info {
  display: none;
}

.sidebar-collapsed .collapse-btn {
  margin-left: auto;
}

.section-header {
  padding: 0.5rem 1rem;
  margin-top: 0.5rem;
  border-top: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style> 