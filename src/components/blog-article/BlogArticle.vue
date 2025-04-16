<template>
  <div class="blog-article">
    <div class="content-editing-area">
      <input v-model="title" type="text" class="title-input" placeholder="Enter title..." />
      <textarea v-model="content" class="content-input" placeholder="Enter content..."></textarea>
    </div>
    
    <DraftView
      v-model="draftView.isVisible"
      :draft-title="draftView.title"
      :draft-content="draftView.content"
      @accept="acceptDraft"
      @reject="rejectDraft"
    />
  </div>
</template>

<script setup lang="ts">
import { useAIConversation } from './composables/useAIConversation'
import DraftView from './DraftView.vue'
import { ref, onMounted, watch } from 'vue'

const title = ref('')
const content = ref('')

const {
  conversationHistory,
  userPrompt,
  isApiKeySet,
  showApiKeyWarning,
  formatTimestamp,
  checkApiKey,
  sendMessage,
  draftView,
  acceptDraft,
  rejectDraft,
  setUpdateCallbacks
} = useAIConversation()

// Add more detailed watcher for debugging
watch(draftView, (newValue) => {
  console.log('DraftView changed:', {
    isVisible: newValue.isVisible,
    title: newValue.title,
    content: newValue.content
  })
}, { deep: true, immediate: true })

// Add direct watcher for isVisible
watch(() => draftView.value.isVisible, (newValue) => {
  console.log('isVisible changed:', newValue)
}, { immediate: true })

onMounted(() => {
  // Set up callbacks for title and content updates
  setUpdateCallbacks(
    (newTitle) => title.value = newTitle,
    (newContent) => content.value = newContent
  )
  
  // Log initial state
  console.log('Initial DraftView state:', draftView.value)
})
</script>

<style scoped>
.blog-article {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem;
  position: relative;
}

.content-editing-area {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  transition: all 0.3s ease;
  z-index: 1;
}

.title-input {
  font-size: 1.5rem;
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.content-input {
  min-height: 300px;
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  resize: vertical;
}

.content-update-animation {
  animation: contentUpdate 1s ease;
}

@keyframes contentUpdate {
  0% {
    background-color: rgba(40, 167, 69, 0.1);
  }
  50% {
    background-color: rgba(40, 167, 69, 0.2);
  }
  100% {
    background-color: transparent;
  }
}
</style> 