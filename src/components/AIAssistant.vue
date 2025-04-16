<template>
  <div class="ai-assistant h-full flex flex-col">
    <!-- 對話歷史 -->
    <div class="conversation-history flex-1 overflow-auto p-4 space-y-4">
      <div v-for="(message, index) in messages" :key="index" 
           :class="[
             'message p-3 rounded-lg transition-all duration-300',
             message.type === 'user' 
               ? 'user-message bg-primary-light dark:bg-primary-dark ml-4' 
               : 'assistant-message bg-secondary-light dark:bg-secondary-dark mr-4'
           ]">
        <div class="message-header flex justify-between items-center mb-2">
          <span class="role text-xs font-medium" :class="message.type === 'user' ? 'text-primary-dark dark:text-primary-light' : 'text-secondary-dark dark:text-secondary-light'">
            {{ message.type === 'user' ? 'You' : 'AI Assistant' }}
          </span>
          <span class="timestamp text-xs text-gray-500 dark:text-gray-400">
            {{ formatTime(new Date()) }}
          </span>
        </div>
        <div class="message-content text-sm" :class="message.type === 'user' ? 'text-gray-800 dark:text-gray-200' : 'text-gray-700 dark:text-gray-300'">
          {{ message.content }}
        </div>
      </div>
    </div>

    <!-- 輸入區域 -->
    <div class="input-area border-t border-gray-200 dark:border-gray-700 p-4 bg-white dark:bg-gray-800 transition-colors duration-300">
      <div class="flex space-x-2">
        <input
          v-model="userInput"
          type="text"
          class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 dark:focus:ring-primary-400 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 placeholder-gray-500 dark:placeholder-gray-400 transition-colors duration-300"
          placeholder="輸入您的問題..."
          @keyup.enter="sendMessage"
        >
        <button
          @click="sendMessage"
          class="send-button px-4 py-2 bg-primary-500 dark:bg-primary-600 text-white rounded-lg hover:bg-primary-600 dark:hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 dark:focus:ring-primary-400 transition-colors duration-300 flex items-center justify-center"
          :disabled="!userInput.trim()"
        >
          <span class="button-text">發送</span>
          <span v-if="isSending" class="loading-spinner ml-2"></span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Message {
  type: 'user' | 'assistant'
  content: string
  timestamp?: Date
}

const messages = ref<Message[]>([])
const userInput = ref('')
const isSending = ref(false)

const formatTime = (date: Date): string => {
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
}

const sendMessage = async () => {
  if (!userInput.value.trim()) return

  // 添加用戶消息
  messages.value.push({
    type: 'user',
    content: userInput.value,
    timestamp: new Date()
  })

  // 清空輸入框
  userInput.value = ''
  
  // 設置發送狀態
  isSending.value = true

  // TODO: 調用 AI API
  // 這裡將添加與後端的通信邏輯

  // 模擬 AI 回應
  setTimeout(() => {
    messages.value.push({
      type: 'assistant',
      content: '這是一個模擬的 AI 回應。在實際應用中，這裡將顯示來自 AI 模型的回應。',
      timestamp: new Date()
    })
    isSending.value = false
  }, 1000)
}
</script>

<style scoped>
.ai-assistant {
  background-color: var(--color-bg-secondary, #f9fafb);
  transition: background-color 0.3s ease;
}

:global(body.dark) .ai-assistant {
  background-color: var(--color-bg-secondary, #1f2937);
}

.conversation-history {
  scrollbar-width: thin;
  scrollbar-color: rgba(156, 163, 175, 0.5) transparent;
}

.conversation-history::-webkit-scrollbar {
  width: 6px;
}

.conversation-history::-webkit-scrollbar-track {
  background: transparent;
}

.conversation-history::-webkit-scrollbar-thumb {
  background-color: rgba(156, 163, 175, 0.5);
  border-radius: 3px;
}

:global(body.dark) .conversation-history::-webkit-scrollbar-thumb {
  background-color: rgba(156, 163, 175, 0.3);
}

.user-message {
  border-top-left-radius: 0;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.assistant-message {
  border-top-right-radius: 0;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.message {
  animation: fadeIn 0.3s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.loading-spinner {
  display: inline-block;
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  border-top-color: white;
  animation: spin 1s ease-in-out infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.send-button:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

/* Theme variables */
:root {
  --primary-light: #e6f0ff;
  --primary-dark: #1e3a8a;
  --secondary-light: #f3f4f6;
  --secondary-dark: #374151;
  --primary-500: #3b82f6;
  --primary-600: #2563eb;
  --primary-400: #60a5fa;
  --primary-700: #1d4ed8;
}

:global(body.dark) {
  --primary-light: #1e3a8a;
  --primary-dark: #e6f0ff;
  --secondary-light: #374151;
  --secondary-dark: #f3f4f6;
}
</style> 