<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { onMounted } from 'vue'

const { t } = useI18n()

// 初始化主題
onMounted(() => {
  // 從 localStorage 加載主題設置
  const savedTheme = localStorage.getItem('theme') || 'light'
  document.body.className = savedTheme
})
</script>

<template>
  <div class="app">
    <router-view></router-view>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  height: 100vh;
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  transition: background-color 0.3s, color 0.3s;
}

/* 淺色主題 */
body.light {
  color: var(--color-text-primary);
  background-color: var(--color-bg-primary);
}

/* 深色主題 */
body.dark {
  color: var(--color-text-primary);
  background-color: var(--color-bg-primary);
}

.app {
  height: 100vh;
  overflow: hidden;
}

/* 全局滾動條樣式 - 淺色主題 */
body.light ::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

body.light ::-webkit-scrollbar-track {
  background: var(--color-bg-tertiary);
  border-radius: 4px;
}

body.light ::-webkit-scrollbar-thumb {
  background: var(--color-text-tertiary);
  border-radius: 4px;
}

body.light ::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-secondary);
}

/* 全局滾動條樣式 - 深色主題 */
body.dark ::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

body.dark ::-webkit-scrollbar-track {
  background: var(--color-bg-tertiary);
  border-radius: 4px;
}

body.dark ::-webkit-scrollbar-thumb {
  background: var(--color-text-tertiary);
  border-radius: 4px;
}

body.dark ::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-secondary);
}

/* 深色主題下的卡片和容器樣式 */
body.dark .card,
body.dark .settings-section,
body.dark .project-card,
body.dark .community-card,
body.dark .progress-card,
body.dark .news-ticker {
  background-color: var(--color-bg-secondary);
  box-shadow: var(--shadow-sm);
}

/* 深色主題下的文字顏色 */
body.dark h1,
body.dark h2,
body.dark h3,
body.dark h4,
body.dark h5,
body.dark h6 {
  color: var(--color-text-primary);
}

/* 深色主題下的邊框顏色 */
body.dark .border,
body.dark .settings-section,
body.dark .project-card,
body.dark .community-card {
  border-color: var(--color-border-primary);
}
</style>