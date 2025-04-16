<template>
  <div v-if="modelValue" class="draft-view" :style="draftViewPosition">
    <div class="draft-view-header" @mousedown="startDragging">
      <span class="draft-view-title">Suggested Draft</span>
      <button class="close-button" @click="handleReject">×</button>
    </div>
    <div class="draft-view-content">
      <div class="draft-title">
        <div class="text-label">Title</div>
        <div class="text-content">{{ formattedTitle }}</div>
      </div>
      <div class="draft-body">
        <div class="text-label">Content</div>
        <div class="text-content" v-html="formattedContent"></div>
      </div>
    </div>
    <div class="buttons">
      <button @click="handleReject" class="reject-button">Reject</button>
      <button @click="handleAccept" class="accept-button">Accept</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { marked } from 'marked'

const props = defineProps<{
  modelValue: boolean;
  draftTitle: string;
  draftContent: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
  (e: 'accept'): void;
  (e: 'reject'): void;
}>();

// Add more detailed debug logs
onMounted(() => {
  console.log('DraftView mounted with props:', {
    modelValue: props.modelValue,
    draftTitle: props.draftTitle,
    draftContent: props.draftContent
  });
  if (props.modelValue) {
    positionDraftView();
  }
});

// Add watch for modelValue changes
watch(() => props.modelValue, (newValue) => {
  console.log('DraftView modelValue changed:', newValue);
  if (newValue) {
    // Use nextTick to ensure DOM is updated
    nextTick(() => {
      positionDraftView();
    });
  }
});

// Add watch for all props
watch(props, (newProps) => {
  console.log('DraftView props changed:', {
    modelValue: newProps.modelValue,
    draftTitle: newProps.draftTitle,
    draftContent: newProps.draftContent
  });
}, { deep: true });

// 拖動相關
const isDragging = ref(false);
const dragOffset = ref({ x: 0, y: 0 });
const draftViewPosition = ref({
  top: '0px',
  left: '0px'
});

// Add function to position the draft view
const positionDraftView = () => {
  console.log('Attempting to position DraftView');
  const contentEditor = document.querySelector('.content-editor') as HTMLElement;
  if (contentEditor) {
    const rect = contentEditor.getBoundingClientRect();
    console.log('Found .content-editor element:', contentEditor);
    console.log('.content-editor getBoundingClientRect():', rect);
    draftViewPosition.value = {
      left: `${rect.left + (rect.width / 2)}px`,
      top: `${rect.top + 20}px` // 20px below the top of the content editor
    };
    console.log('Calculated draftViewPosition:', draftViewPosition.value);
  } else {
    console.log('.content-editor element not found, falling back to window center');
    // Fallback to window center if content editor not found
    const windowWidth = window.innerWidth;
    const windowHeight = window.innerHeight;
    draftViewPosition.value = {
      left: `${windowWidth / 2}px`,
      top: `${windowHeight / 4}px`
    };
    console.log('Fallback draftViewPosition:', draftViewPosition.value);
  }
};

const startDragging = (event: MouseEvent) => {
  // Only start dragging if the left mouse button is pressed
  if (event.button !== 0) return
  
  isDragging.value = true
  
  // Calculate the offset from the mouse position to the draft view's top-left corner
  const draftView = document.querySelector('.draft-view') as HTMLElement
  if (draftView) {
    const rect = draftView.getBoundingClientRect()
    dragOffset.value = {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top
    }
  }
  
  // Add event listeners for mouse move and mouse up
  document.addEventListener('mousemove', handleDragging)
  document.addEventListener('mouseup', stopDragging)
}

const handleDragging = (event: MouseEvent) => {
  if (!isDragging.value) return
  
  // Update the draft view position based on the mouse position and offset
  draftViewPosition.value = {
    left: `${event.clientX - dragOffset.value.x}px`,
    top: `${event.clientY - dragOffset.value.y}px`
  }
}

const stopDragging = () => {
  isDragging.value = false
  
  // Remove event listeners
  document.removeEventListener('mousemove', handleDragging)
  document.removeEventListener('mouseup', stopDragging)
}

// Add cleanup on component unmount
onUnmounted(() => {
  document.removeEventListener('mousemove', handleDragging)
  document.removeEventListener('mouseup', stopDragging)
})

const handleAccept = () => {
  console.log('Accept clicked');
  emit('accept');
  emit('update:modelValue', false);
}

const handleReject = () => {
  console.log('Reject clicked');
  emit('reject');
  emit('update:modelValue', false);
}

// Add renderMarkdown function
const renderMarkdown = (text: string) => {
  try {
    return marked(text, { 
      breaks: true,
      gfm: true
    })
  } catch (error) {
    console.error('Error rendering markdown:', error)
    return text
  }
}

// Add computed properties for formatted content
const formattedTitle = computed(() => {
  // Remove markdown syntax from title
  return props.draftTitle.replace(/[#*`]/g, '').trim()
})

const formattedContent = computed(() => {
  return renderMarkdown(props.draftContent)
})
</script>

<style scoped>
.draft-view {
  position: fixed;
  z-index: 2000;
  background-color: var(--color-bg-secondary);
  border: 2px solid var(--color-success);
  border-radius: 4px;
  box-shadow: var(--shadow-md);
  padding: 0;
  max-width: 800px;
  width: 90%;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  min-width: 300px;
  cursor: default;
  animation: slideIn 0.3s ease-out;
  transition: background-color 0.3s ease, border-color 0.3s ease;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(-20px); /* Only vertical transform */
  }
  to {
    opacity: 1;
    transform: translateY(0); /* Only vertical transform */
  }
}

.draft-view-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background-color: var(--color-bg-tertiary);
  border-bottom: 1px solid var(--color-border-primary);
  border-radius: 4px 4px 0 0;
  cursor: move;
  -webkit-user-select: none;
  user-select: none;
  position: sticky;
  top: 0;
  z-index: 2;
  transition: background-color 0.3s ease, border-color 0.3s ease;
}

.draft-view-title {
  font-weight: 500;
  font-size: 14px;
  color: var(--color-text-primary);
  transition: color 0.3s ease;
}

.draft-view-content {
  flex: 1;
  overflow-y: auto;
  padding: 12px 16px;
  padding-bottom: 60px;
  background-color: var(--color-bg-secondary);
  transition: background-color 0.3s ease;
}

.draft-title, .draft-body {
  margin-bottom: 16px;
}

.text-label {
  font-weight: 500;
  margin-bottom: 8px;
  color: var(--color-text-secondary);
  transition: color 0.3s ease;
}

.text-content {
  background: var(--color-bg-tertiary);
  padding: 12px;
  border-radius: 4px;
  border: 1px solid var(--color-border-primary);
  white-space: pre-wrap;
  word-break: break-word;
  font-size: 14px;
  line-height: 1.5;
  color: var(--color-text-primary);
  transition: background-color 0.3s ease, border-color 0.3s ease, color 0.3s ease;
}

.buttons {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  padding: 12px 16px;
  background-color: var(--color-bg-secondary);
  border-top: 1px solid var(--color-border-primary);
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 2;
  transition: background-color 0.3s ease, border-color 0.3s ease;
}

.buttons button {
  padding: 6px 12px;
  border-radius: 4px;
  border: none;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
}

.reject-button {
  background-color: var(--color-bg-tertiary);
  color: var(--color-text-secondary);
}

.reject-button:hover {
  background-color: var(--color-bg-secondary);
}

.accept-button {
  background-color: var(--color-success);
  color: var(--color-text-inverse);
}

.accept-button:hover {
  background-color: var(--color-success-dark);
}

.close-button {
  background: none;
  border: none;
  font-size: 18px;
  cursor: pointer;
  color: var(--color-text-tertiary);
  padding: 0 4px;
  transition: color 0.2s ease;
}

.close-button:hover {
  color: var(--color-text-primary);
}

/* Add styles for markdown content */
.content-preview :deep(p) {
  margin-bottom: 0.5rem;
  color: var(--color-text-primary);
  transition: color 0.3s ease;
}

.content-preview :deep(h1),
.content-preview :deep(h2),
.content-preview :deep(h3),
.content-preview :deep(h4),
.content-preview :deep(h5),
.content-preview :deep(h6) {
  margin-top: 1rem;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: var(--color-text-primary);
  transition: color 0.3s ease;
}

.content-preview :deep(h1) { font-size: 1.5rem; }
.content-preview :deep(h2) { font-size: 1.25rem; }
.content-preview :deep(h3) { font-size: 1.125rem; }
.content-preview :deep(h4) { font-size: 1rem; }
.content-preview :deep(h5) { font-size: 0.875rem; }
.content-preview :deep(h6) { font-size: 0.75rem; }

.content-preview :deep(ul),
.content-preview :deep(ol) {
  margin-bottom: 0.5rem;
  padding-left: 1.5rem;
  color: var(--color-text-primary);
  transition: color 0.3s ease;
}

.content-preview :deep(li) {
  margin-bottom: 0.25rem;
  color: var(--color-text-primary);
  transition: color 0.3s ease;
}

.content-preview :deep(code) {
  background-color: var(--color-bg-tertiary);
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-family: monospace;
  font-size: 0.875em;
  color: var(--color-text-primary);
  transition: background-color 0.3s ease, color 0.3s ease;
}

.content-preview :deep(pre) {
  background-color: var(--color-bg-tertiary);
  padding: 0.75rem;
  border-radius: 0.375rem;
  overflow-x: auto;
  margin-bottom: 0.5rem;
  transition: background-color 0.3s ease;
}

.content-preview :deep(pre code) {
  background-color: transparent;
  padding: 0;
  border-radius: 0;
  font-size: 0.875rem;
  line-height: 1.5;
  color: var(--color-text-primary);
  transition: color 0.3s ease;
}

.content-preview :deep(blockquote) {
  border-left: 4px solid var(--color-border-primary);
  padding-left: 1rem;
  margin-left: 0;
  margin-bottom: 0.5rem;
  color: var(--color-text-secondary);
  transition: border-color 0.3s ease, color 0.3s ease;
}

.content-preview :deep(a) {
  color: var(--color-link);
  text-decoration: underline;
  transition: color 0.3s ease;
}

.content-preview :deep(table) {
  border-collapse: collapse;
  width: 100%;
  margin-bottom: 0.5rem;
}

.content-preview :deep(th),
.content-preview :deep(td) {
  border: 1px solid var(--color-border-primary);
  padding: 0.5rem;
  text-align: left;
  color: var(--color-text-primary);
  transition: border-color 0.3s ease, color 0.3s ease;
}

.content-preview :deep(th) {
  background-color: var(--color-bg-tertiary);
  font-weight: 600;
  transition: background-color 0.3s ease;
}

.content-preview :deep(img) {
  max-width: 100%;
  height: auto;
  border-radius: 0.375rem;
  margin-bottom: 0.5rem;
}

.content-preview :deep(hr) {
  border: 0;
  border-top: 1px solid var(--color-border-primary);
  margin: 1rem 0;
  transition: border-color 0.3s ease;
}
</style> 