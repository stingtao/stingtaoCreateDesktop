<template>
  <BaseLayout>
    <!-- 中間編輯區 -->
    <div class="editor-area">
      <slot name="editor"></slot>
    </div>

    <!-- 右側分隔線 -->
    <div 
      v-if="!isAiAssistantCollapsed"
      class="resizer right-resizer"
      @mousedown="(e) => startResize('right', e)"
      @touchstart="(e) => startResize('right', e)"
    ></div>

    <!-- 右側 AI 助手區 -->
    <div class="ai-assistant" :class="{ 'ai-assistant-collapsed': isAiAssistantCollapsed }" :style="{ width: aiAssistantWidth + 'px' }">
      <div class="ai-assistant-header">
        <h2 v-if="!isAiAssistantCollapsed">{{ t('ai.assistant') }}</h2>
        <button class="collapse-btn" @click="toggleAiAssistant" :title="isAiAssistantCollapsed ? 'Expand AI Assistant' : 'Collapse AI Assistant'">
          <svg v-if="isAiAssistantCollapsed" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="3" y1="12" x2="21" y2="12"></line><line x1="3" y1="6" x2="21" y2="6"></line><line x1="3" y1="18" x2="21" y2="18"></line></svg>
          <svg v-else xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"></polyline></svg>
        </button>
      </div>
      <div class="ai-assistant-content" v-if="!isAiAssistantCollapsed">
        <slot name="ai-assistant"></slot>
      </div>
    </div>
  </BaseLayout>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ref, onMounted, onUnmounted } from 'vue'
import BaseLayout from './BaseLayout.vue'
import '../../styles/ThreeColumnLayout.css'

const { t } = useI18n()

// 區域寬度狀態
const aiAssistantWidth = ref(300)

// 折疊狀態
const isAiAssistantCollapsed = ref(false)

// 拖動相關狀態
const isResizing = ref(false)
const currentResizer = ref<'right' | null>(null)
const startX = ref(0)
const startWidth = ref(0)

// 從 localStorage 加載保存的寬度和折疊狀態
onMounted(() => {
  const savedAiAssistantWidth = localStorage.getItem('aiAssistantWidth')
  const savedAiAssistantCollapsed = localStorage.getItem('aiAssistantCollapsed')
  
  if (savedAiAssistantWidth) aiAssistantWidth.value = parseInt(savedAiAssistantWidth)
  if (savedAiAssistantCollapsed) isAiAssistantCollapsed.value = savedAiAssistantCollapsed === 'true'
  
  // 檢查螢幕寬度，在小螢幕上自動折疊 AI 助手
  checkScreenSize()
  
  // 監聽視窗大小變化
  window.addEventListener('resize', checkScreenSize)
})

// 檢查螢幕大小並調整佈局
const checkScreenSize = () => {
  if (window.innerWidth < 768) {
    isAiAssistantCollapsed.value = true
  } else if (window.innerWidth < 1024) {
    isAiAssistantCollapsed.value = true
  } else {
    isAiAssistantCollapsed.value = false
  }
  
  // 保存折疊狀態
  localStorage.setItem('aiAssistantCollapsed', isAiAssistantCollapsed.value.toString())
}

// 切換 AI 助手折疊狀態
const toggleAiAssistant = () => {
  isAiAssistantCollapsed.value = !isAiAssistantCollapsed.value
  localStorage.setItem('aiAssistantCollapsed', isAiAssistantCollapsed.value.toString())
}

// 開始調整大小
const startResize = (resizer: 'right', event: MouseEvent | TouchEvent) => {
  isResizing.value = true
  currentResizer.value = resizer
  startX.value = event instanceof MouseEvent ? event.clientX : event.touches[0].clientX
  startWidth.value = aiAssistantWidth.value
  
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
  
  if (currentResizer.value === 'right') {
    const newWidth = Math.max(250, Math.min(500, startWidth.value - diff))
    aiAssistantWidth.value = newWidth
    localStorage.setItem('aiAssistantWidth', newWidth.toString())
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
/* All styles moved to src/styles/ThreeColumnLayout.css */
</style> 