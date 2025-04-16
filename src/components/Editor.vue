<template>
  <div class="h-full flex flex-col">
    <!-- 工具欄 -->
    <div class="border-b p-2 bg-white">
      <div class="flex space-x-2">
        <button class="px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 rounded">
          <i class="fas fa-bold"></i>
        </button>
        <button class="px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 rounded">
          <i class="fas fa-italic"></i>
        </button>
        <button class="px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 rounded">
          <i class="fas fa-list"></i>
        </button>
        <button class="px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 rounded">
          <i class="fas fa-link"></i>
        </button>
      </div>
    </div>

    <!-- 編輯區域 -->
    <div class="flex-1 flex">
      <!-- Markdown 編輯區 -->
      <div class="w-1/2 p-4">
        <textarea
          v-model="content"
          class="w-full h-full resize-none focus:outline-none"
          placeholder="開始寫作..."
          @input="handleInput"
        ></textarea>
      </div>

      <!-- 預覽區 -->
      <div class="w-1/2 p-4 border-l overflow-auto">
        <div class="prose max-w-none" v-html="renderedContent"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { marked } from 'marked'

const content = ref('')
const renderedContent = computed(() => {
  return marked(content.value)
})

const handleInput = () => {
  // 這裡可以添加自動保存邏輯
}
</script>

<style>
.prose {
  max-width: none;
}
</style> 