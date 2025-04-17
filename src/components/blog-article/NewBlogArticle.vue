<template>
  <ThreeColumnLayout>
    <!-- 中間編輯區 -->
    <template #editor>
      <div class="editor-container">
        <div class="editor-header">
          <div class="breadcrumb">
            <span class="breadcrumb-item">Blog Projects</span>
            <span class="breadcrumb-separator">></span>
            <span class="breadcrumb-item">{{ projectName }}</span>
            <span class="breadcrumb-separator">></span>
            <span class="breadcrumb-item">{{ article.title || 'New Article' }}</span>
          </div>
        </div>
        
        <div class="editor-form">
          <div class="form-group">
            <label for="title">Title</label>
            <div class="input-container">
              <input 
                id="title" 
                v-model="article.title" 
                type="text" 
                placeholder="Enter article title"
                @input="debounceSave"
                @mouseup="() => titleInputRef && handleTextSelection('title', titleInputRef)"
                @keyup="() => titleInputRef && handleTextSelection('title', titleInputRef)"
                @select="() => titleInputRef && handleTextSelection('title', titleInputRef)"
                @focus="checkTitleSelection"
                ref="titleInputRef"
              />
              
              <!-- Floating hint for selected text in title -->
              <div v-if="showTitleFloatingHint" class="floating-hint" :style="titleFloatingHintPosition">
                <button class="hint-button add-to-chat" @click="() => addToChat('title')">
                  <span class="button-icon">💬</span> Add to Chat
                </button>
                <button class="hint-button edit" @click="() => showInlineEdit('title')">
                  <span class="button-icon">✏️</span> Edit
                </button>
              </div>
            </div>
          </div>
          
          <div class="form-group content-group">
            <div class="content-header">
              <label for="content">Content</label>
              <div class="editor-controls">
                <div class="history-controls">
                  <button 
                    class="history-button undo-button" 
                    @click="undo" 
                    :disabled="!canUndo"
                    title="Undo (Ctrl+Z)"
                  >
                    <span class="button-icon">↩️</span>
                  </button>
                  <button 
                    class="history-button redo-button" 
                    @click="redo" 
                    :disabled="!canRedo"
                    title="Redo (Ctrl+Y)"
                  >
                    <span class="button-icon">↪️</span>
                  </button>
                </div>
                <div class="save-status" :class="{ 'saving': isSaving, 'saved': isSaved }">
                  {{ saveStatusText }}
                </div>
              </div>
            </div>
            <div class="content-editor">
              <div class="editor-split-view">
                <!-- Markdown source editor -->
                <div class="editor-source">
              <div class="line-numbers" ref="lineNumbersRef">
                <div v-for="n in lineCount" :key="n" class="line-number">{{ n }}</div>
              </div>
              <div class="textarea-container">
                <textarea 
                  id="content" 
                  v-model="article.content" 
                  placeholder="Write your article content here..."
                  @input="handleContentInput"
                  @scroll="syncScroll"
                  @mouseup="() => contentTextareaRef && lineNumbersRef && handleTextSelection('content', contentTextareaRef, lineNumbersRef)"
                  @keyup="() => contentTextareaRef && lineNumbersRef && handleTextSelection('content', contentTextareaRef, lineNumbersRef)"
                  @select="() => contentTextareaRef && lineNumbersRef && handleTextSelection('content', contentTextareaRef, lineNumbersRef)"
                  @focus="checkContentSelection"
                  ref="contentTextareaRef"
                ></textarea>
                  </div>
                </div>
                
                <!-- HTML preview -->
                <div class="editor-preview" ref="previewContentRef">
                  <div class="preview-content" v-html="renderedContent"></div>
                </div>
              </div>
                
                <!-- Floating hint for selected text in content -->
                <div v-if="showContentFloatingHint" class="floating-hint" :style="contentFloatingHintPosition">
                  <button class="hint-button add-to-chat" @click="() => addToChat('content')">
                    <span class="button-icon">💬</span> Add to Chat
                  </button>
                  <button class="hint-button edit" @click="() => showInlineEdit('content')">
                    <span class="button-icon">✏️</span> Edit
                  </button>
                </div>
                
                <!-- Inline editing area -->
                <div v-if="showInlineEditor" class="inline-editor" :style="inlineEditorPosition">
                  <div class="inline-editor-header">
                    <span class="inline-editor-title">Edit with AI</span>
                    <button class="close-button" @click="hideInlineEdit">×</button>
                  </div>
                  <textarea 
                    v-model="inlineEditPrompt" 
                    placeholder="hint: input your editing instruction"
                    class="inline-edit-prompt"
                    rows="2"
                  ></textarea>
                  <div class="inline-editor-footer">
                    <button class="cancel-button" @click="hideInlineEdit">Cancel</button>
                    <button 
                      class="submit-button" 
                      :disabled="!inlineEditPrompt.trim() || !selectedTextInfo?.text"
                      @click="() => selectedTextInfo?.text && submitInlineEdit(selectedTextInfo.text, article.title, article.content, inlineEditPrompt, blogId, conversationHistoryRef || undefined)"
                    >
                      Submit
                    </button>
                  </div>
                </div>
                
                <!-- Diff view for improved text -->
                <div v-if="showDiffView && inlineEditResponse" class="diff-view" :style="diffViewPosition">
                  <div class="diff-view-header" @mousedown="startDragging">
                    <span class="diff-view-title">Compare Changes</span>
                    <button class="close-button" @click="handleRejectImprovedText">×</button>
                  </div>
                <div class="diff-view-content">
                  <div class="original-text">
                    <div class="text-label">Original:</div>
                    <div class="text-content">{{ selectedTextInfo?.text }}</div>
                  </div>
                  <div class="improved-text">
                    <div class="text-label">Improved:</div>
                    <div class="text-content">{{ inlineEditResponse.improved_text }}</div>
                  </div>
                  </div>
                  <div class="buttons">
                    <button class="reject-button" @click="handleRejectImprovedText">Reject</button>
                    <button class="accept-button" @click="handleAcceptImprovedText">Accept</button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      
      <!-- Move DraftView here, inside the #editor slot -->
      <DraftView
        v-model="draftView.isVisible"
        :draft-title="draftView.title"
        :draft-content="draftView.content"
        @accept="handleAcceptDraft"
        @reject="handleRejectDraft"
      />
    </template>
    
    <!-- 右側 AI 對話區 -->
    <template #ai-assistant>
      <div class="ai-conversation-container">
        <!-- API 金鑰警告 -->
        <div v-if="showApiKeyWarning" class="api-key-warning">
          <div class="warning-content">
            <p>Gemini API key is not set. Please set it in Settings to use AI features.</p>
            <button @click="goToSettings" class="settings-button">Go to Settings</button>
          </div>
        </div>
        
        <!-- 對話歷史區域 -->
        <div class="conversation-history" ref="conversationHistoryRef">
          <div v-for="(message, index) in conversationHistory" :key="index" class="message" :class="[message.role, { 'typing': message.role === 'assistant' && message.isTyping }]">
            <div class="message-header">
              <span class="role">{{ message.role === 'user' ? 'You' : 'AI' }}</span>
              <span class="timestamp">{{ formatTimestamp(message.timestamp) }}</span>
            </div>
            <div v-if="message.referencedText" class="referenced-text">
              <div class="referenced-text-header">Referenced Text:</div>
              <div class="referenced-text-content">{{ message.referencedText }}</div>
            </div>
            <div class="message-content" v-html="renderMarkdown(message.content)"></div>
          </div>
        </div>
        
        <!-- 用戶輸入區域 -->
        <div class="user-input-area">
          <!-- @ 區域 -->
          <div class="at-section">
            <div class="at-header">
              <span class="at-label">@ Selected Text</span>
              <button v-if="selectedTextInfo" @click="clearSelection" class="clear-button">Clear</button>
            </div>
            <div v-if="selectedTextInfo" class="selected-text">
              <template v-if="textAddedToChat">
                {{ selectedTextInfo.start.toString() }} - {{ selectedTextInfo.end.toString() }}
              </template>
              <template v-else>
                Text selected
              </template>
            </div>
            <div v-else class="no-selection">
              Select text in the editor to reference it here
            </div>
          </div>
          
          <!-- 提示輸入區域 -->
          <div class="prompt-section">
            <textarea 
              v-model="userPrompt" 
              placeholder="Tell me how you want me to help you draft the article..."
              @keydown="handlePromptKeyDown"
              @compositionstart="isComposing = true"
              @compositionend="handleCompositionEnd"
            ></textarea>
            <div class="prompt-actions">
              <select v-model="agentType" class="agent-type-selector">
                <option v-for="type in agentTypes" :key="type" :value="type">{{ type }}</option>
              </select>
            <button 
                @click="handleSendMessage" 
                :disabled="!userPrompt.trim() || isSending" 
              class="send-button"
                :class="{ 'sending': isSending }"
              >
                <span v-if="isSending" class="loading">
                  <span class="loading-text">Sending</span>
                  <span class="loading-dots">
                    <span class="dot">.</span>
                    <span class="dot">.</span>
                    <span class="dot">.</span>
                  </span>
                </span>
                <span v-else>Send</span>
            </button>
            </div>
          </div>
        </div>
      </div>
    </template>
  </ThreeColumnLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'
import { useRouter, useRoute, onBeforeRouteLeave, type RouteLocationNormalized } from 'vue-router'
import ThreeColumnLayout from '../layouts/ThreeColumnLayout.vue'
import { getAllProjects, getProject, getProjectByBlogId, type ProjectSummary } from '../../lib/project'
import { useArticleEditor } from './composables/useArticleEditor'
import { useTextEditor } from './composables/useTextEditor'
import { useAIConversation } from './composables/useAIConversation'
import { useInlineEditor } from './composables/useInlineEditor'
import { getBlogArticle, type BlogArticle } from '../../lib/content'
import { marked } from 'marked'
import './NewBlogArticle.css'
import DraftView from './DraftView.vue'

const router = useRouter()
const route = useRoute()

// Define props
const props = defineProps<{
  key?: string
}>()

// 從 URL 參數獲取 project_id 和 blog_id
const projectId = computed(() => {
  // 先檢查路徑參數
  const pathId = route.params.project_id
  if (pathId) {
    return parseInt(pathId as string, 10)
  }
  
  // 再檢查查詢參數
  const queryId = route.query.project_id
  return queryId ? parseInt(queryId as string, 10) : 0
})

const blogId = computed(() => {
  // 先檢查路徑參數
  const pathId = route.params.blog_id
  if (pathId) {
    return parseInt(pathId as string, 10)
  }
  
  // 再檢查查詢參數
  const queryId = route.query.blog_id
  return queryId ? parseInt(queryId as string, 10) : 0
})

// 從 URL 參數獲取 project_name
const projectName = computed(() => {
  // 先檢查查詢參數
  if (route.query.project_name) {
    return route.query.project_name as string
  }
  
  // 如果沒有查詢參數，嘗試從路由名稱獲取
  if (route.params.project_id) {
    // 從 projects 中查找項目名稱
    const project = projects.value.find((p: ProjectSummary) => p.id === parseInt(route.params.project_id as string, 10))
    if (project) {
      return project.title
    }
  }
  
  return 'Unknown Project'
})

// Initialize composables
const {
  article,
  isSaving,
  isSaved,
  saveStatusText,
  debounceSave,
  saveArticle,
  loadExistingArticle
} = useArticleEditor(projectId.value)

const {
  selectedText,
  selectedTextSource,
  selectedTextStartLine,
  selectedTextEndLine,
  selectedTextStart,
  selectedTextEnd,
  showContentFloatingHint,
  showTitleFloatingHint,
  contentFloatingHintPosition,
  titleFloatingHintPosition,
  calculateSelectionPosition,
  handleTextSelection: handleTextSelectionFromEditor,
  clearSelectedText
} = useTextEditor()

const {
  conversationHistory,
  userPrompt,
  isApiKeySet,
  showApiKeyWarning,
  formatTimestamp,
  checkApiKey,
  sendMessage,
  draftView,
  setUpdateCallbacks
} = useAIConversation()

const {
  showInlineEditor,
  inlineEditorPosition,
  inlineEditPrompt,
  inlineEditResponse,
  showDiffView,
  diffViewPosition,
  selectedTextInfo,
  showInlineEdit: showInlineEditUI,
  hideInlineEdit,
  submitInlineEdit,
  acceptImprovedText,
  rejectImprovedText,
  setSelectedTextInfo,
  clearSelectedTextInfo,
  updateContent: updateInlineContent,
  setAddToConversationHistory,
  updateMousePosition: updateInlineEditorMousePosition
} = useInlineEditor()

// 加載項目列表
const projects = ref<ProjectSummary[]>([])

// 行號相關
const lineCount = ref(1)
const contentTextareaRef = ref<HTMLTextAreaElement | null>(null)
const lineNumbersRef = ref<HTMLDivElement | null>(null)
const titleInputRef = ref<HTMLInputElement | null>(null)
const conversationHistoryRef = ref<HTMLElement | null>(null)
const previewContentRef = ref<HTMLElement | null>(null)

// Add these variables to the script section
const isDragging = ref(false)
const dragOffset = ref({ x: 0, y: 0 })

// Add history tracking variables
const blogHistories = ref<Map<number, string[]>>(new Map())
const currentHistoryIndex = ref(-1)
const maxHistorySize = 50; // Limit history size to prevent memory issues
const isHistoryAction = ref(false); // Flag to prevent recursive history updates

// Add computed properties for undo/redo state
const canUndo = computed(() => currentHistoryIndex.value > 0)
const canRedo = computed(() => {
  const currentHistory = getCurrentBlogHistory()
  return currentHistoryIndex.value < currentHistory.length - 1
})

// Get the current blog's history
const getCurrentBlogHistory = () => {
  if (!blogId.value) return []
  
  if (!blogHistories.value.has(blogId.value)) {
    blogHistories.value.set(blogId.value, [])
  }
  
  return blogHistories.value.get(blogId.value) || []
}

// Add history management functions
const updateCanUndoRedo = () => {
  // These are computed properties, so they'll update automatically
}

const addToHistory = (newContent: string) => {
  if (!blogId.value) return
  
  const currentHistory = getCurrentBlogHistory()
  
  // Remove any future states if we're not at the end of history
  if (currentHistoryIndex.value < currentHistory.length - 1) {
    const updatedHistory = currentHistory.slice(0, currentHistoryIndex.value + 1)
    blogHistories.value.set(blogId.value, updatedHistory)
  }

  // Add new state
  const updatedHistory = [...getCurrentBlogHistory(), newContent]
  blogHistories.value.set(blogId.value, updatedHistory)
  currentHistoryIndex.value = updatedHistory.length - 1
}

// Add these constants at the top of the script section
const AUTO_SAVE_DELAY = 10000 // 10 seconds

// Add saveTimeout ref
const saveTimeout = ref<number | null>(null)

// Add word counting function
const countWords = (text: string): number => {
  // 使用正則表達式匹配空白字符和標點符號分隔的詞
  return text.trim().split(/[\s,.!?;:，。！？；：]+/).filter(word => word.length > 0).length;
}

// Add variables to track accumulated word count
const accumulatedWordCount = ref(0)
const lastContentLength = ref(0)

// Add function to check if we should record history
const shouldRecordHistory = (oldText: string, newText: string): boolean => {
  // 計算新增的字數
  const oldCount = countWords(oldText)
  const newCount = countWords(newText)
  const addedWords = newCount - oldCount
  
  // 如果是刪除操作，重置累計
  if (addedWords < 0) {
    accumulatedWordCount.value = 0
    return false
  }
  
  // 累計新增的字數
  accumulatedWordCount.value += addedWords
  
  // 當累計字數達到或超過3個時，記錄歷史並重置計數
  if (accumulatedWordCount.value >= 3) {
    accumulatedWordCount.value = 0
    return true
  }
  
  return false
}

// Modify the undo function
const undo = () => {
  if (!blogId.value) return
  
  const currentHistory = getCurrentBlogHistory()
  
  if (canUndo.value) {
    isHistoryAction.value = true; // 設置標誌
    currentHistoryIndex.value--
    const previousState = currentHistory[currentHistoryIndex.value]
    // 確保使用完整的歷史狀態
    article.value.content = previousState
    isHistoryAction.value = false; // 重置標誌
    
    // 使用普通的 debounceSave，而不是立即保存
    debounceSave();
  }
}

// Modify the redo function
const redo = () => {
  if (!blogId.value) return
  
  const currentHistory = getCurrentBlogHistory()
  
  if (canRedo.value) {
    isHistoryAction.value = true; // 設置標誌
    currentHistoryIndex.value++
    const nextState = currentHistory[currentHistoryIndex.value]
    // 確保使用完整的歷史狀態
    article.value.content = nextState
    isHistoryAction.value = false; // 重置標誌
    
    // 使用普通的 debounceSave，而不是立即保存
    debounceSave();
  }
}

// Add isComposing ref to track IME composition state
const isComposing = ref(false)
const pendingEnter = ref(false)

// Add handlePromptKeyDown function for the prompt textarea
const handlePromptKeyDown = (event: KeyboardEvent) => {
  if (event.key === 'Enter') {
    if (isComposing.value) {
      // If composing, mark that we have a pending Enter
      pendingEnter.value = true
    } else {
      // If not composing, send the message
      event.preventDefault()
      handleSendMessage()
    }
  }
}

// Add handleCompositionEnd function
const handleCompositionEnd = () => {
  isComposing.value = false
  // If we have a pending Enter, send the message after a short delay
  if (pendingEnter.value) {
    setTimeout(() => {
      handleSendMessage()
      pendingEnter.value = false
    }, 100)
  }
}

// Keep the original handleKeyDown for editor shortcuts
const handleKeyDown = (event: KeyboardEvent) => {
  // Check if the target is the content textarea or title input
  const target = event.target as HTMLElement;
  const isEditor = target.tagName === 'TEXTAREA' || target.id === 'title';
  
  if (isEditor) {
    // Ctrl+Z for undo
    if (event.ctrlKey && event.key === 'z' && !event.shiftKey) {
      event.preventDefault();
      undo();
    }
    
    // Ctrl+Shift+Z or Ctrl+Y for redo
    if ((event.ctrlKey && event.shiftKey && event.key === 'z') || 
        (event.ctrlKey && event.key === 'y')) {
      event.preventDefault();
      redo();
    }
  }
};

// 加載項目列表
const loadProjects = async () => {
  try {
    projects.value = await getAllProjects()
  } catch (error) {
    console.error('Failed to load projects:', error)
  }
}

// 導航到設定頁面
const goToSettings = () => {
  router.push('/settings')
}

// Add mousePosition ref after other refs
const mousePosition = ref({ x: 0, y: 0 })

// Update handleTextSelection function
const handleTextSelection = (source: 'title' | 'content', element: HTMLInputElement | HTMLTextAreaElement, lineNumbers?: HTMLElement) => {
  console.log('handleTextSelection called with source:', source)
  
  // Get selection directly from the element
  const start = element.selectionStart ?? 0
  const end = element.selectionEnd ?? 0
  console.log('Element selection range:', { start, end })
  
  if (start === end) {
    console.log('No text selected (start equals end), clearing selected text')
    clearSelectedText()
    return
  }
  
  // Get the selected text directly from the element's value
  const selectedText = element.value.substring(start, end).trim()
  console.log('Selected text:', selectedText)
  
  if (selectedText) {
    // Update selected text info
    setSelectedTextInfo({
      start,
      end,
      text: selectedText,
      source
    })
    
    // Calculate position relative to viewport
    const OFFSET_X = 10 // Offset from mouse pointer
    const OFFSET_Y = 10 // Offset from mouse pointer
    
    // Get viewport dimensions
    const viewportWidth = window.innerWidth
    const viewportHeight = window.innerHeight
    
    // Get floating hint dimensions (approximate if not yet rendered)
    const HINT_WIDTH = 200 // Approximate width of floating hint
    const HINT_HEIGHT = 80 // Approximate height of floating hint
    
    // Calculate position, ensuring it stays within viewport
    let left = Math.min(mousePosition.value.x + OFFSET_X, viewportWidth - HINT_WIDTH)
    let top = Math.min(mousePosition.value.y + OFFSET_Y, viewportHeight - HINT_HEIGHT)
    
    // Ensure minimum distance from viewport edges
    left = Math.max(0, left)
    top = Math.max(0, top)
    
    const position = {
      top: `${top}px`,
      left: `${left}px`
    }
    
    // Update floating hint position
    if (source === 'title') {
      titleFloatingHintPosition.value = position
      showTitleFloatingHint.value = true
    } else {
      contentFloatingHintPosition.value = position
      showContentFloatingHint.value = true
    }
  } else {
    console.log('No text selected, clearing selected text')
    clearSelectedText()
  }
}

// 檢查標題選擇
const checkTitleSelection = () => {
  if (titleInputRef.value) {
    const start = titleInputRef.value.selectionStart ?? 0
    const end = titleInputRef.value.selectionEnd ?? 0
    
    if (start !== end) {
      handleTextSelection('title', titleInputRef.value)
    }
  }
}

// 檢查內容選擇
const checkContentSelection = () => {
  if (contentTextareaRef.value && lineNumbersRef.value) {
    const start = contentTextareaRef.value.selectionStart ?? 0
    const end = contentTextareaRef.value.selectionEnd ?? 0
    
    if (start !== end) {
      handleTextSelection('content', contentTextareaRef.value, lineNumbersRef.value)
    }
  }
}

// Add a new ref to track if text has been added to chat
const textAddedToChat = ref(false)

// 添加到聊天
const addToChat = (source: 'title' | 'content') => {
  if (selectedTextInfo.value) {
    // 格式化選中的文本
    let formattedText = selectedTextInfo.value.text
      if (source === 'content') {
      formattedText = `[Lines ${selectedTextStartLine.value}-${selectedTextEndLine.value}]\n${selectedTextInfo.value.text}`
    } else {
      formattedText = `[Title]\n${selectedTextInfo.value.text}`
    }
    
    // 更新選中的文本
    setSelectedTextInfo({
      ...selectedTextInfo.value,
      text: formattedText
    })
    
    // Set flag to show count in AI Assistant
    textAddedToChat.value = true
    
    // 隱藏浮動提示
    showContentFloatingHint.value = false
    showTitleFloatingHint.value = false
  }
}

// 顯示內聯編輯器
const showInlineEdit = (source: 'title' | 'content') => {
  showContentFloatingHint.value = false
  showTitleFloatingHint.value = false
  
  // Calculate position relative to viewport
  const OFFSET_X = 10 // Offset from mouse pointer
  const OFFSET_Y = 10 // Offset from mouse pointer
  
  // Get viewport dimensions
  const viewportWidth = window.innerWidth
  const viewportHeight = window.innerHeight
  
  // Get inline editor dimensions
  const EDITOR_WIDTH = 400 // Width from CSS
  const EDITOR_HEIGHT = 200 // Approximate height
  
  // Calculate position, ensuring the editor stays within viewport
  let left = mousePosition.value.x
  let top = mousePosition.value.y + OFFSET_Y
  
  // Adjust horizontal position if it would go off screen
  if (left + EDITOR_WIDTH + OFFSET_X > viewportWidth) {
    // If it would go off the right edge, position it to the left of the mouse
    left = left - EDITOR_WIDTH - OFFSET_X
  } else {
    // Otherwise, position it to the right of the mouse
    left = left + OFFSET_X
  }
  
  // Adjust vertical position if it would go off screen
  if (top + EDITOR_HEIGHT > viewportHeight) {
    // If it would go off the bottom, position it above the mouse
    top = mousePosition.value.y - EDITOR_HEIGHT - OFFSET_Y
  }
  
  // Ensure minimum distances from viewport edges
  left = Math.max(OFFSET_X, left)
  top = Math.max(OFFSET_Y, top)
  
  const position = {
    left: `${left}px`,
    top: `${top}px`
  }
  
  showInlineEditUI(source, position)
}

// Add hasUnsavedChanges ref
const hasUnsavedChanges = ref(false)

// Add immediate save function that handles the unsaved changes state
const saveImmediately = async () => {
  if (hasUnsavedChanges.value) {
    if (saveTimeout.value) {
      clearTimeout(saveTimeout.value)
    }
    try {
      await saveArticle()
      hasUnsavedChanges.value = false
    } catch (error) {
      console.error('Failed to save article:', error)
      // Keep hasUnsavedChanges true if save failed
      throw error
    }
  }
}

// Create a wrapper for debounceSave that tracks unsaved changes
const debounceSaveWithTracking = () => {
  hasUnsavedChanges.value = true
  debounceSave()
}

// Add navigation guard
onBeforeRouteLeave(async (to: RouteLocationNormalized, from: RouteLocationNormalized) => {
  if (hasUnsavedChanges.value) {
    console.log('Saving changes before navigation...')
    await saveImmediately()
  }
  return true // Allow navigation after saving
})

// Watch for content changes
watch(() => article.value.content, async (newContent) => {
  // Keep track of unsaved changes and trigger save
  hasUnsavedChanges.value = true;
  debounceSaveWithTracking(); // Assuming this handles saving logic
}, { immediate: true }); // Keep immediate: true for initial render

// Watch for title changes
watch(() => article.value.title, () => {
  hasUnsavedChanges.value = true
  debounceSaveWithTracking()
})

// Modify handleContentInput to set hasUnsavedChanges
const handleContentInput = async (event: Event) => {
  const target = event.target as HTMLTextAreaElement;
  const newContent = target.value;

  // Update content
  article.value.content = newContent;
  hasUnsavedChanges.value = true;

  // Add to history
  addToHistory(newContent);

  // Update line count immediately
  updateLineCount();

  // Use the wrapped version of debounceSave
  debounceSaveWithTracking(); // Assuming this handles saving logic
};

// 更新行數 (基於 scrollHeight 近似計算視覺行數)
const updateLineCount = () => {
  if (!contentTextareaRef.value) return;

  const textarea = contentTextareaRef.value;
  const computedStyle = getComputedStyle(textarea);

  // 獲取 line-height
  let numericLineHeight: number;
  const lineHeightStyle = computedStyle.lineHeight;
  if (lineHeightStyle === 'normal') {
    // 如果 line-height 是 'normal'，則基於 font-size 估算
    numericLineHeight = parseFloat(computedStyle.fontSize) * 1.2; // 常用估算值
    console.warn("Textarea line-height is 'normal'. Approximating line count based on font-size. Consider setting a specific line-height in CSS for better accuracy.");
  } else {
    numericLineHeight = parseFloat(lineHeightStyle);
  }

  // 檢查 line-height 是否有效
  if (isNaN(numericLineHeight) || numericLineHeight <= 0) {
    console.error("Could not determine a valid line height for line counting. Falling back to newline counting.");
    // 回退到基於換行符的計算
    const text = textarea.value;
    const normalizedText = text.replace(/\r\n/g, '\n').replace(/\r/g, '\n');
    const lines = normalizedText.split('\n');
    lineCount.value = Math.max(1, lines.length);
    return;
  }

  // 獲取垂直 padding
  const paddingTop = parseFloat(computedStyle.paddingTop);
  const paddingBottom = parseFloat(computedStyle.paddingBottom);

  // 計算內容的實際高度
  const contentHeight = textarea.scrollHeight - paddingTop - paddingBottom;

  // 向上取整計算近似行數
  const calculatedLines = Math.ceil(contentHeight / numericLineHeight);

  // 確保行數至少為 1
  lineCount.value = Math.max(1, calculatedLines);

  console.log(`Line count updated (using scrollHeight: ${textarea.scrollHeight}, lineHeight: ${numericLineHeight}, contentHeight: ${contentHeight}): ${lineCount.value}`);
}

// 同步滾動
const syncScroll = () => {
  // 檢查必要的元素是否存在
  if (!contentTextareaRef.value || !lineNumbersRef.value || !previewContentRef.value) return

  const editor = contentTextareaRef.value
  const lineNumbers = lineNumbersRef.value
  const preview = previewContentRef.value

  // 同步行號滾動
  lineNumbers.scrollTop = editor.scrollTop

  // 同步預覽區域滾動
  const editorScrollRatio = editor.scrollTop / (editor.scrollHeight - editor.clientHeight || 1)
  const previewScrollTop = editorScrollRatio * (preview.scrollHeight - preview.clientHeight)
  preview.scrollTop = previewScrollTop
}

// Function to scroll the conversation history to the bottom
const scrollToBottom = async () => {
  await nextTick(); // Wait for DOM updates
  const container = conversationHistoryRef.value;
  if (container) {
    container.scrollTop = container.scrollHeight;
  }
};

// Watch for changes in conversation history and scroll to bottom
watch(
  conversationHistory,
  () => {
    scrollToBottom();
  },
  { deep: true }
);

// 移除預覽區滾動事件監聽器
onMounted(async () => {
  // Check API key
  checkApiKey()

  // Load project data
  await loadProjectData()

  // Load existing article if blogId is provided
      if (blogId.value) {
    await loadExistingArticle(blogId.value) // await here
        
        // Initialize history for this blog if it doesn't exist
        if (!blogHistories.value.has(blogId.value)) {
          blogHistories.value.set(blogId.value, [article.value.content])
          currentHistoryIndex.value = 0
        } else {
          // Reset to the latest state in the history
          const currentHistory = getCurrentBlogHistory()
          currentHistoryIndex.value = currentHistory.length - 1
    }

    // Update line count and alignment after loading article
    updateLineCount()
      nextTick(() => {
      ensureLineNumberAlignment()
    })
  } else {
     // Also update for new articles (initial state)
        updateLineCount()
     nextTick(() => {
       ensureLineNumberAlignment()
     })
  }

  // Add keyboard event listener
  window.addEventListener('keydown', handleKeyDown)

  // Scroll conversation history to bottom on mount
  scrollToBottom(); // ADDED HERE

  // Add mouse move event listener
  document.addEventListener('mousemove', updateMousePosition)
})

// 在組件卸載時移除事件監聽器
onUnmounted(() => {
  document.removeEventListener('mousemove', handleDragging)
  document.removeEventListener('mouseup', stopDragging)

  // 移除鍵盤事件監聽器
  window.removeEventListener('keydown', handleKeyDown)

  if (saveTimeout.value) {
    clearTimeout(saveTimeout.value)
  }

  // 清理累計字數
  accumulatedWordCount.value = 0
  lastContentLength.value = 0

  // Save any pending changes
  if (hasUnsavedChanges.value) {
    saveImmediately()
  }

  // Remove mouse move event listener
  document.removeEventListener('mousemove', updateMousePosition)
})

// Add project data ref
const projectData = ref<any>(null)

// Add function to load project data
const loadProjectData = async () => {
  if (projectId.value) {
    try {
      const project = await getProject(projectId.value)
      projectData.value = project
    } catch (error) {
      console.error('Failed to load project data:', error)
    }
  }
}

// 確保行號和文本區域對齊
const ensureLineNumberAlignment = () => {
  if (!contentTextareaRef.value || !lineNumbersRef.value) return
  
  // 獲取文本區域的計算樣式
  const textareaStyle = getComputedStyle(contentTextareaRef.value)
  const lineHeight = textareaStyle.lineHeight
  const fontSize = textareaStyle.fontSize
  const paddingTop = textareaStyle.paddingTop
  const paddingBottom = textareaStyle.paddingBottom
  
  console.log('Textarea computed style:', {
    lineHeight,
    fontSize,
    paddingTop,
    paddingBottom
  })
  
  // 設置行號容器的樣式
  lineNumbersRef.value.style.lineHeight = lineHeight
  lineNumbersRef.value.style.fontSize = fontSize
  lineNumbersRef.value.style.paddingTop = paddingTop
  lineNumbersRef.value.style.paddingBottom = paddingBottom
  
  // 設置每個行號元素的樣式
  const lineNumberElements = lineNumbersRef.value.querySelectorAll('.line-number')
  lineNumberElements.forEach(el => {
    const lineNumberEl = el as HTMLElement
    lineNumberEl.style.lineHeight = lineHeight
    lineNumberEl.style.fontSize = fontSize
  })
  
  // 確保行號容器的高度與文本區域相同
  lineNumbersRef.value.style.height = contentTextareaRef.value.clientHeight + 'px'
}

// 接受改進的文本
const handleAcceptImprovedText = () => {
  if (!inlineEditResponse.value || !selectedTextInfo.value) return
  
  const { start, end, source } = selectedTextInfo.value
  const improvedText = inlineEditResponse.value.improved_text
  
  // 更新內容
  if (source === 'title' && titleInputRef.value) {
    // 更新標題
    const beforeText = article.value.title.substring(0, start)
    const afterText = article.value.title.substring(end)
    article.value.title = beforeText + improvedText + afterText
    
    // 更新選中範圍
    const newEnd = start + improvedText.length
    titleInputRef.value.setSelectionRange(start, newEnd)
    
    // 添加高亮效果
    highlightInsertedText(titleInputRef.value, start, newEnd)
  } else if (source === 'content' && contentTextareaRef.value) {
    // 更新內容
    const beforeText = article.value.content.substring(0, start)
    const afterText = article.value.content.substring(end)
    article.value.content = beforeText + improvedText + afterText
    
    // 更新選中範圍
    const newEnd = start + improvedText.length
    contentTextareaRef.value.setSelectionRange(start, newEnd)
    
    // 添加高亮效果
    highlightInsertedText(contentTextareaRef.value, start, newEnd)
  }
  
  // 保存文章
  debounceSave()
  
 // 更新行數
  updateLineCount()
  
  // 隱藏 diff 視圖
  showDiffView.value = false
  inlineEditResponse.value = null
  clearSelectedTextInfo()
}

// 高亮插入的文本
const highlightInsertedText = (element: HTMLInputElement | HTMLTextAreaElement, start: number, end: number) => {
  // 簡化的方法：直接使用 CSS 動畫來高亮文本區域
  element.classList.add('highlight-textarea')
  
  // 創建一個臨時的 span 元素來包裹選中的文本
  const tempSpan = document.createElement('span')
  tempSpan.className = 'highlight-inserted-text'
  
  // 獲取選中的文本
  const selectedText = element.value.substring(start, end)
  
  // 設置 span 的內容為選中的文本
  tempSpan.textContent = selectedText
  
  // 獲取元素的樣式
  const computedStyle = window.getComputedStyle(element)
  
  // 設置 span 的樣式以匹配元素
  tempSpan.style.font = computedStyle.font
  tempSpan.style.fontSize = computedStyle.fontSize
  tempSpan.style.fontFamily = computedStyle.fontFamily
  tempSpan.style.lineHeight = computedStyle.lineHeight
  tempSpan.style.padding = computedStyle.padding
  tempSpan.style.margin = computedStyle.margin
  tempSpan.style.whiteSpace = 'pre-wrap'
  tempSpan.style.wordWrap = 'break-word'
  tempSpan.style.overflowWrap = 'break-word'
  
  // 將 span 添加到 body 中以便測量
  document.body.appendChild(tempSpan)
  
  // 獲取 span 的尺寸
  const spanRect = tempSpan.getBoundingClientRect()
  
  // 移除 span
  document.body.removeChild(tempSpan)
  
  // 創建高亮元素
  const highlightElement = document.createElement('div')
  highlightElement.className = 'highlight-inserted-text'
  
  // 獲取元素的位置
  const elementRect = element.getBoundingClientRect()
  
  // 計算高亮元素的位置
  // 對於 textarea，我們需要考慮滾動位置
  const scrollTop = element.scrollTop
  const scrollLeft = element.scrollLeft
  
  // 計算文本在 textarea 中的位置
  const textBeforeSelection = element.value.substring(0, start)
  const linesBeforeSelection = textBeforeSelection.split('\n')
  const lastLineBeforeSelection = linesBeforeSelection[linesBeforeSelection.length - 1]
  
  // 創建一個臨時的 span 來測量最後一行文本的寬度
  const measureSpan = document.createElement('span')
  measureSpan.style.visibility = 'hidden'
  measureSpan.style.position = 'absolute'
  measureSpan.style.whiteSpace = 'pre-wrap'
  measureSpan.style.wordWrap = 'break-word'
  measureSpan.style.width = `${element.clientWidth}px`
  measureSpan.style.font = computedStyle.font
  document.body.appendChild(measureSpan)
  
  // 設置 span 的內容為最後一行文本
  measureSpan.textContent = lastLineBeforeSelection
  
  // 獲取最後一行文本的寬度
  const lastLineWidth = measureSpan.offsetWidth
  
  // 移除 span
  document.body.removeChild(measureSpan)
  
  // 計算高亮元素的位置
  const lineHeight = parseInt(computedStyle.lineHeight)
  const topPosition = elementRect.top + (linesBeforeSelection.length - 1) * lineHeight - scrollTop
  const leftPosition = elementRect.left + lastLineWidth - scrollLeft
  
  // 設置高亮元素的位置和尺寸
  highlightElement.style.position = 'fixed'
  highlightElement.style.top = `${topPosition}px`
  highlightElement.style.left = `${leftPosition}px`
  highlightElement.style.width = `${spanRect.width}px`
  highlightElement.style.height = `${spanRect.height}px`
  highlightElement.style.zIndex = '1000'
  
  // 將高亮元素添加到 body
  document.body.appendChild(highlightElement)
  
  // 1秒後移除高亮元素
  setTimeout(() => {
    highlightElement.classList.add('fade-out')
    setTimeout(() => {
      document.body.removeChild(highlightElement)
    }, 500) // 等待淡出動畫完成
  }, 1000)
  
  // 1秒後移除 CSS 類
  setTimeout(() => {
    element.classList.remove('highlight-textarea')
  }, 1000)
}

// 拒絕改進的文本
const handleRejectImprovedText = () => {
  // 隱藏 diff 視圖
  showDiffView.value = false
  inlineEditResponse.value = null
  clearSelectedTextInfo()
}

// Add these methods to the script section
const startDragging = (event: MouseEvent) => {
  // Only start dragging if the left mouse button is pressed
  if (event.button !== 0) return
  
  isDragging.value = true
  
  // Calculate the offset from the mouse position to the diff view's top-left corner
  const diffView = document.querySelector('.diff-view') as HTMLElement
  if (diffView) {
    const rect = diffView.getBoundingClientRect()
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
  
  // Calculate potential new position
  let newTop = event.clientY - dragOffset.value.y
  let newLeft = event.clientX - dragOffset.value.x

  // Prevent dragging the header completely off the top of the viewport
  // Allow some small negative value if needed, but 0 is usually safe
  const minTop = 0; 
  if (newTop < minTop) {
    newTop = minTop;
  }
  
  // Optional: Prevent dragging too far left/right if needed
  // const minLeft = 0;
  // const maxLeft = window.innerWidth - diffViewWidth; // Requires knowing diff view width
  // if (newLeft < minLeft) newLeft = minLeft;
  // if (newLeft > maxLeft) newLeft = maxLeft;

  // Update the diff view position
  diffViewPosition.value = {
    left: `${newLeft}px`,
    top: `${newTop}px`
  }
}

const stopDragging = () => {
  isDragging.value = false
  
  // Remove event listeners
  document.removeEventListener('mousemove', handleDragging)
  document.removeEventListener('mouseup', stopDragging)
}

// Add a function to render markdown
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

// Update the clearSelectedTextInfo function to reset the textAddedToChat flag
const clearSelection = () => {
  clearSelectedTextInfo()
  textAddedToChat.value = false
}

// Add agentType ref with default value 'DraftGenerator'
const agentType = ref('DraftGenerator')

// Define available agent types
const agentTypes = ref([
  'DraftGenerator'
])

// Add handleAcceptDraft and handleRejectDraft functions
const handleAcceptDraft = async () => {
  // Add animation class to content area
  const contentArea = document.querySelector('.content-editing-area');
  if (contentArea) {
    contentArea.classList.add('content-update-animation');
    setTimeout(() => {
      contentArea.classList.remove('content-update-animation');
    }, 1000);
  }
  
  // Convert markdown to plain text for title (remove markdown symbols)
  const processedTitle = draftView.value.title.replace(/[#*`]/g, '').trim();
  
  // Keep the original markdown content
  const originalContent = draftView.value.content;
  
  // Update title and content
  article.value.title = processedTitle;
  article.value.content = originalContent;
  
  // Save changes
  debounceSave();
}

const handleRejectDraft = () => {
  // No action needed, DraftView will be hidden automatically
}

// Add watch for draftView
watch(draftView, (newValue) => {
  console.log('DraftView state changed:', {
    isVisible: newValue.isVisible,
    title: newValue.title,
    content: newValue.content
  })
}, { deep: true })

// Add computed property for rendered content
const renderedContent = computed(() => {
  try {
    return marked(article.value.content, {
      breaks: true,
      gfm: true
    });
  } catch (error) {
    console.error('Error rendering markdown:', error);
    // Provide fallback content in case of rendering error
    const errorMessage = error instanceof Error ? error.message : String(error);
    return `<p style="color: red;">Error rendering markdown: ${errorMessage}</p><pre><code>${article.value.content}</code></pre>`;
  }
})

// Add isSending ref
const isSending = ref(false)

// Add handleSendMessage function
const handleSendMessage = async () => {
  if (!userPrompt.value.trim()) return
  
  isSending.value = true
  try {
    await sendMessage(
      userPrompt.value,
      selectedTextInfo.value?.text,
      agentType.value,
      blogId.value,
      conversationHistoryRef.value || undefined,
      projectData.value,
      article.value.title,
      article.value.content
    )
  } catch (error) {
    console.error('Error sending message:', error)
  } finally {
    isSending.value = false
  }
}

// Add mouse position tracking
const updateMousePosition = (event: MouseEvent) => {
  mousePosition.value = {
    x: event.clientX,
    y: event.clientY
  }
  // Also update the inline editor's mouse position
  updateInlineEditorMousePosition(event.clientX, event.clientY)
}
</script> 

<style scoped>
/* Styles moved to NewBlogArticle.css */
</style> 