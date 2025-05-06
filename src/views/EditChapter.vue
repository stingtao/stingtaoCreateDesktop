<template>
  <ThreeColumnLayout>
    <template #editor>
      <div class="editor-container">
        <div class="editor-header">
          <div class="breadcrumb">
            <span class="breadcrumb-item">Book Projects</span>
            <span class="breadcrumb-separator">></span>
            <span class="breadcrumb-item">{{ projectName }}</span>
            <span class="breadcrumb-separator">></span>
            <span class="breadcrumb-item">{{ chapter.title || 'New Chapter' }}</span>
          </div>
        </div>
        <div class="editor-form">
          <div class="form-group">
            <label for="title">Title</label>
            <div class="input-container">
              <input
                id="title"
                v-model="chapter.title"
                type="text"
                placeholder="Enter chapter title"
                @input="checkTitleSelection"
                @mouseup="() => titleInputRef && handleTextSelection('title', titleInputRef)"
                @keyup="() => titleInputRef && handleTextSelection('title', titleInputRef)"
                @select="() => titleInputRef && handleTextSelection('title', titleInputRef)"
                ref="titleInputRef"
              />
              <div v-if="showTitleFloatingHint" class="floating-hint" :style="titleFloatingHintPosition">
                <button class="hint-button add-to-chat" @click="addToChat('title')">
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
                  <button class="history-button undo-button" @click="undo" :disabled="!canUndo" title="Undo (Ctrl+Z)">
                    <span class="button-icon">↩️</span>
                  </button>
                  <button class="history-button redo-button" @click="redo" :disabled="!canRedo" title="Redo (Ctrl+Y)">
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
                <div class="editor-source">
                  <div class="line-numbers" ref="lineNumbersRef">
                    <div v-for="n in lineCount" :key="n" class="line-number">{{ n }}</div>
                  </div>
                  <div class="textarea-container">
                    <textarea
                      id="content"
                      v-model="chapter.content"
                      placeholder="Write your chapter content here..."
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
                <div class="editor-preview" ref="previewContentRef">
                  <div class="preview-content" v-html="renderedContent"></div>
                </div>
              </div>
              <div v-if="showContentFloatingHint" class="floating-hint" :style="contentFloatingHintPosition">
                <button class="hint-button add-to-chat" @click="addToChat('content')">
                  <span class="button-icon">💬</span> Add to Chat
                </button>
                <button class="hint-button edit" @click="() => showInlineEdit('content')">
                  <span class="button-icon">✏️</span> Edit
                </button>
              </div>
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
                    @click="handleInlineEditSubmit"
                  >
                    Submit
                  </button>
                </div>
              </div>
              <div v-if="showDiffView && inlineEditResponse" class="diff-view" :style="diffViewPosition">
                <div class="diff-view-header" @mousedown="startDragging">
                  <span class="diff-view-title">Compare Changes</span>
                  <button class="close-button" @click="rejectImprovedText">×</button>
                </div>
                <div class="diff-view-content">
                  <div class="original-text">
                    <div class="text-label">Original:</div>
                    <div class="text-content" v-html="renderDiffHtml(selectedTextInfo?.text || '', inlineEditResponse?.improved_text || '')"></div>
                  </div>
                  <div class="improved-text">
                    <div class="text-label">Improved:</div>
                    <div class="text-content" v-html="renderDiffHtml(inlineEditResponse?.improved_text || '', selectedTextInfo?.text || '')"></div>
                  </div>
                </div>
                <div class="buttons">
                  <button class="reject-button" @click="rejectImprovedText">Reject</button>
                  <button class="accept-button" @click="acceptImprovedText">Accept</button>
                </div>
              </div>
            </div>
          </div>
        </div>
        <DraftView
          v-model="draftView.isVisible"
          :draft-title="draftView.title"
          :draft-content="draftView.content"
          @accept="handleAcceptDraft"
          @reject="handleRejectDraft"
        />
        <!-- 刪除按鈕與確認對話框 -->
        <div class="editor-footer">
          <button class="delete-chapter-btn" @click="showDeleteModal = true">Delete Chapter</button>
        </div>
        <div v-if="showDeleteModal" class="modal-overlay">
          <div class="modal-content">
            <h3>Delete Chapter</h3>
            <p>Are you sure you want to delete this chapter?</p>
            <div class="modal-actions">
              <button @click="showDeleteModal = false">Cancel</button>
              <button @click="showSecondDeleteModal = true">Confirm</button>
            </div>
          </div>
        </div>
        <div v-if="showSecondDeleteModal" class="modal-overlay">
          <div class="modal-content">
            <h3>Final Confirmation</h3>
            <p>This will permanently delete the chapter. Are you absolutely sure?</p>
            <div class="modal-actions">
              <button @click="showSecondDeleteModal = false">Cancel</button>
              <button @click="deleteChapter">Delete</button>
            </div>
          </div>
        </div>
      </div>
    </template>
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
              <button v-if="selectedTextInfoForChat" @click="clearSelection" class="clear-button">Clear</button>
            </div>
            <div v-if="selectedTextInfoForChat" class="selected-text">
              <template v-if="textAddedToChat">
                <div v-html="formatSelectedTextHtml(selectedTextInfoForChat.text)"></div>
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
              placeholder="Tell me how you want me to help you draft the chapter..."
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
import { ref, computed, onMounted, nextTick, watch, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import ThreeColumnLayout from '../components/layouts/ThreeColumnLayout.vue'
import DraftView from '../components/blog-article/DraftView.vue'
import AIAssistant from '../components/AIAssistant.vue'
import { getChapter, updateChapter, deleteChapter as deleteChapterApi, getChaptersByProject } from '../lib/content'
import { getProject } from '../lib/project'
import { marked } from 'marked'
import { useInlineEditor } from '../components/blog-article/composables/useInlineEditor'
import { useTextEditor } from '../components/blog-article/composables/useTextEditor'
import { useAIConversation } from '../components/blog-article/composables/useAIConversation'
// @ts-ignore
import { diffWords, Change } from 'diff'
import { message as tauriMessage } from '@tauri-apps/api/dialog'
import { useI18n } from 'vue-i18n'
// TODO: 可複用 useAIConversation/useTextEditor 等 composable

const route = useRoute()
const projectId = computed(() => Number(route.params.project_id))
const chapterId = computed(() => Number(route.params.chapter_id))

const chapter = ref({
  id: chapterId.value,
  project_id: projectId.value,
  title: '',
  content: '',
  chapter_number: 1,
  created_at: new Date().toISOString(),
  updated_at: new Date().toISOString()
})
const project = ref({})
const projectName = ref('')
const aiPrompt = ref('')

// Inline editor composable
const {
  showInlineEditor,
  inlineEditorPosition,
  inlineEditPrompt,
  inlineEditResponse,
  showDiffView,
  diffViewPosition,
  selectedTextInfo,
  showInlineEdit: showInlineEditRaw,
  hideInlineEdit,
  submitInlineEdit,
  rejectImprovedText
} = useInlineEditor()

// Text editor composable for selection/floating hints
const {
  showTitleFloatingHint,
  showContentFloatingHint,
  titleFloatingHintPosition,
  contentFloatingHintPosition,
  handleTextSelection,
  clearSelectedText,
  selectedText,
  selectedTextStart,
  selectedTextEnd,
  selectedTextStartLine,
  selectedTextEndLine
} = useTextEditor()

// AI conversation and draft
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

// Refs for DOM elements
const titleInputRef = ref<HTMLInputElement | null>(null)
const contentTextareaRef = ref<HTMLTextAreaElement | null>(null)
const lineNumbersRef = ref<HTMLElement | null>(null)
const previewContentRef = ref<HTMLElement | null>(null)

// Undo/Redo and history logic for chapter editing
const chapterHistories = ref<Map<number, string[]>>(new Map())
const currentHistoryIndex = ref(-1)
const isHistoryAction = ref(false)

const canUndo = computed(() => currentHistoryIndex.value > 0)
const canRedo = computed(() => {
  const currentHistory = getCurrentChapterHistory()
  return currentHistoryIndex.value < currentHistory.length - 1
})

function getCurrentChapterHistory() {
  if (!chapter.value.id) return []
  if (!chapterHistories.value.has(chapter.value.id)) {
    chapterHistories.value.set(chapter.value.id, [])
  }
  return chapterHistories.value.get(chapter.value.id) || []
}

function addToHistory(newContent: string) {
  if (!chapter.value.id) return
  const currentHistory = getCurrentChapterHistory()
  if (currentHistoryIndex.value < currentHistory.length - 1) {
    const updatedHistory = currentHistory.slice(0, currentHistoryIndex.value + 1)
    chapterHistories.value.set(chapter.value.id, updatedHistory)
  }
  const updatedHistory = [...getCurrentChapterHistory(), newContent]
  chapterHistories.value.set(chapter.value.id, updatedHistory)
  currentHistoryIndex.value = updatedHistory.length - 1
}

function undo() {
  if (!chapter.value.id) return
  const currentHistory = getCurrentChapterHistory()
  if (canUndo.value) {
    isHistoryAction.value = true
    currentHistoryIndex.value--
    const previousState = currentHistory[currentHistoryIndex.value]
    chapter.value.content = previousState
    isHistoryAction.value = false
  }
}

function redo() {
  if (!chapter.value.id) return
  const currentHistory = getCurrentChapterHistory()
  if (canRedo.value) {
    isHistoryAction.value = true
    currentHistoryIndex.value++
    const nextState = currentHistory[currentHistoryIndex.value]
    chapter.value.content = nextState
    isHistoryAction.value = false
  }
}

const isSaving = ref(false)
const isSaved = ref(false)
const saveTimeout = ref<number | null>(null)

const saveStatusText = computed(() => {
  if (isSaving.value) return 'Saving...'
  if (isSaved.value) return 'Saved'
  return 'Not saved'
})

function debounceSave() {
  isSaved.value = false
  if (saveTimeout.value) {
    clearTimeout(saveTimeout.value)
  }
  saveTimeout.value = window.setTimeout(() => {
    saveChapter()
  }, 10000)
}

async function saveChapter() {
  isSaving.value = true
  try {
    // Ensure all required Chapter fields are present
    const now = new Date().toISOString()
    const chapterToSave = {
      ...chapter.value,
      chapter_number: chapter.value.chapter_number ?? 1,
      created_at: chapter.value.created_at ?? now,
      updated_at: now
    }
    await updateChapter(chapterToSave)
    isSaved.value = true
  } catch (error) {
    console.error('Failed to save chapter:', error)
  } finally {
    isSaving.value = false
  }
}

const lineCount = ref(1)

function updateLineCount() {
  if (!contentTextareaRef.value) return
  const textarea = contentTextareaRef.value
  const computedStyle = getComputedStyle(textarea)
  let numericLineHeight: number
  const lineHeightStyle = computedStyle.lineHeight
  if (lineHeightStyle === 'normal') {
    numericLineHeight = parseFloat(computedStyle.fontSize) * 1.2
  } else {
    numericLineHeight = parseFloat(lineHeightStyle)
  }
  if (isNaN(numericLineHeight) || numericLineHeight <= 0) {
    const text = textarea.value
    const normalizedText = text.replace(/\r\n/g, '\n').replace(/\r/g, '\n')
    const lines = normalizedText.split('\n')
    lineCount.value = Math.max(1, lines.length)
    return
  }
  const paddingTop = parseFloat(computedStyle.paddingTop)
  const paddingBottom = parseFloat(computedStyle.paddingBottom)
  const contentHeight = textarea.scrollHeight - paddingTop - paddingBottom
  const calculatedLines = Math.ceil(contentHeight / numericLineHeight)
  lineCount.value = Math.max(1, calculatedLines)
  ensureLineNumberAlignment()
}

function ensureLineNumberAlignment() {
  if (!contentTextareaRef.value || !lineNumbersRef.value) return
  const textareaStyle = getComputedStyle(contentTextareaRef.value)
  const lineHeight = textareaStyle.lineHeight
  const fontSize = textareaStyle.fontSize
  const paddingTop = textareaStyle.paddingTop
  const paddingBottom = textareaStyle.paddingBottom
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
  // 保證高度一致
  lineNumbersRef.value.style.height = contentTextareaRef.value.clientHeight + 'px'
}

function handleContentInput(event: Event) {
  const target = event.target as HTMLTextAreaElement
  const newContent = target.value
  chapter.value.content = newContent
  addToHistory(newContent)
  debounceSave()
  updateLineCount()
  ensureLineNumberAlignment()
}

// Selection helpers
const checkTitleSelection = () => {
  if (titleInputRef.value) {
    const start = titleInputRef.value.selectionStart ?? 0
    const end = titleInputRef.value.selectionEnd ?? 0
    if (start !== end) {
      handleTextSelection('title', titleInputRef.value)
    }
  }
}
const checkContentSelection = () => {
  if (contentTextareaRef.value && lineNumbersRef.value) {
    const start = contentTextareaRef.value.selectionStart ?? 0
    const end = contentTextareaRef.value.selectionEnd ?? 0
    if (start !== end) {
      handleTextSelection('content', contentTextareaRef.value, lineNumbersRef.value)
    }
  }
}

const mousePosition = ref({ x: 0, y: 0 })

document.addEventListener('mousemove', (event) => {
  mousePosition.value = { x: event.clientX, y: event.clientY }
})

const textAddedToChat = ref(false)

const selectedTextInfoForChat = ref<{ text: string, start: number, end: number, source: 'title' | 'content' } | null>(null)

function addToChat(source: 'title' | 'content') {
  console.log('addToChat called', source, selectedText.value, selectedTextStart.value, selectedTextEnd.value)
  if (selectedText.value) {
    let formattedText = selectedText.value
    let rangeLabel = ''
    if (source === 'content') {
      rangeLabel = `[Lines ${selectedTextStartLine.value}-${selectedTextEndLine.value}]\n`
    } else {
      rangeLabel = `[Title]\n`
    }
    formattedText = rangeLabel + formattedText
    selectedTextInfoForChat.value = {
      text: formattedText,
      start: selectedTextStart.value,
      end: selectedTextEnd.value,
      source
    }
    textAddedToChat.value = true
    showContentFloatingHint.value = false
    showTitleFloatingHint.value = false
  }
}

function clearSelection() {
  clearSelectedText()
  textAddedToChat.value = false
  selectedTextInfoForChat.value = null
}

function showInlineEdit(source: 'title' | 'content') {
  showContentFloatingHint.value = false
  showTitleFloatingHint.value = false
  textAddedToChat.value = false
  // ...原有位置計算...
  // 改進：避免超出視窗
  const OFFSET_X = 10
  const OFFSET_Y = 10
  const viewportWidth = window.innerWidth
  const viewportHeight = window.innerHeight
  const HINT_WIDTH = 200
  const HINT_HEIGHT = 80
  let left = mousePosition.value.x
  let top = mousePosition.value.y + OFFSET_Y
  if (left + HINT_WIDTH + OFFSET_X > viewportWidth) {
    left = left - HINT_WIDTH - OFFSET_X
  } else {
    left = left + OFFSET_X
  }
  if (top + HINT_HEIGHT > viewportHeight) {
    top = mousePosition.value.y - HINT_HEIGHT - OFFSET_Y
  }
  left = Math.max(OFFSET_X, left)
  top = Math.max(OFFSET_Y, top)
  const position = { left: `${left}px`, top: `${top}px` }
  // 新增：同步選取內容給 useInlineEditor
  showInlineEditRaw(source, position)
  if (selectedText.value) {
    selectedTextInfo.value = {
      start: selectedTextStart.value,
      end: selectedTextEnd.value,
      text: selectedText.value,
      source
    }
  } else {
    selectedTextInfo.value = null
  }
}

// 監聽 chapterId 變化，自動載入新章節
watch(
  () => chapterId.value,
  async (newChapterId) => {
    if (!newChapterId) return
    const data = await getChapter(newChapterId)
    if (data) {
      chapter.value = {
        ...data,
        chapter_number: data.chapter_number ?? 1,
        created_at: data.created_at ?? new Date().toISOString(),
        updated_at: data.updated_at ?? new Date().toISOString()
      }
      // 重置 undo/redo 歷史
      chapterHistories.value.set(newChapterId, [data.content])
      currentHistoryIndex.value = 0
    }
    nextTick(() => {
      updateLineCount()
    })
  }
)

onMounted(async () => {
  const data = await getChapter(chapterId.value)
  if (data) {
    chapter.value = {
      ...data,
      chapter_number: data.chapter_number ?? 1,
      created_at: data.created_at ?? new Date().toISOString(),
      updated_at: data.updated_at ?? new Date().toISOString()
    }
    chapterHistories.value.set(chapterId.value, [data.content])
    currentHistoryIndex.value = 0
  }
  // TODO: 呼叫 API 取得專案資料
  const proj = await getProject(projectId.value)
  if (proj) {
    project.value = proj
    projectName.value = proj.title
  }
  nextTick(() => {
    updateLineCount()
    ensureLineNumberAlignment()
    scrollToBottom()
  })
  window.addEventListener('resize', ensureLineNumberAlignment)
})

function handleAcceptDraft() {
  // 章節草稿接受邏輯
  const contentArea = document.querySelector('.content-editor');
  if (contentArea) {
    contentArea.classList.add('content-update-animation');
    setTimeout(() => {
      contentArea.classList.remove('content-update-animation');
    }, 1000);
  }
  chapter.value.title = draftView.value.title.replace(/[#*`]/g, '').trim();
  chapter.value.content = draftView.value.content;
  debounceSave();
  draftView.value.isVisible = false;
}

function handleRejectDraft() {
  draftView.value.isVisible = false
}

watch(draftView, (newValue) => {
  // 可加日誌或其他副作用
}, { deep: true })

const renderedContent = computed(() => {
  try {
    return marked(chapter.value.content, {
      breaks: true,
      gfm: true
    })
  } catch (error) {
    console.error('Error rendering markdown:', error)
    const errorMessage = error instanceof Error ? error.message : String(error)
    return `<p style="color: red;">Error rendering markdown: ${errorMessage}</p><pre><code>${chapter.value.content}</code></pre>`
  }
})

function syncScroll() {
  if (!contentTextareaRef.value || !lineNumbersRef.value || !previewContentRef.value) return
  const editor = contentTextareaRef.value
  const lineNumbers = lineNumbersRef.value
  const preview = previewContentRef.value
  lineNumbers.scrollTop = editor.scrollTop
  const editorScrollRatio = editor.scrollTop / (editor.scrollHeight - editor.clientHeight || 1)
  const previewScrollTop = editorScrollRatio * (preview.scrollHeight - preview.clientHeight)
  preview.scrollTop = previewScrollTop
}

// AI 對話區移植所需變數與方法
const router = useRouter()
const conversationHistoryRef = ref<HTMLElement | null>(null)
const isComposing = ref(false)
const pendingEnter = ref(false)
const agentType = ref('DraftGenerator')
const agentTypes = ref(['DraftGenerator'])
const isSending = ref(false)

const { t } = useI18n()

function goToSettings() {
  router.push('/settings')
}
function handlePromptKeyDown(event: KeyboardEvent) {
  if (event.key === 'Enter') {
    if (isComposing.value) {
      pendingEnter.value = true
    } else {
      event.preventDefault()
      handleSendMessage()
    }
  }
}
function handleCompositionEnd() {
  isComposing.value = false
  if (pendingEnter.value) {
    setTimeout(() => {
      handleSendMessage()
      pendingEnter.value = false
    }, 100)
  }
}
function renderMarkdown(text: string) {
  try {
    return marked(text, { breaks: true, gfm: true })
  } catch (error) {
    return text
  }
}
async function handleSendMessage() {
  if (!userPrompt.value.trim()) return
  isSending.value = true
  try {
    await sendMessage(
      userPrompt.value,
      selectedTextInfo.value?.text,
      agentType.value,
      chapterId.value,
      conversationHistoryRef.value || undefined,
      project.value,
      chapter.value.title,
      chapter.value.content
    )
  } catch (error) {
    console.error('Error sending message:', error)
  } finally {
    isSending.value = false
  }
}

const isDragging = ref(false)
const dragOffset = ref({ x: 0, y: 0 })

function startDragging(event: MouseEvent) {
  if (event.button !== 0) return
  isDragging.value = true
  const diffView = document.querySelector('.diff-view') as HTMLElement
  if (diffView) {
    const rect = diffView.getBoundingClientRect()
    dragOffset.value = {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top
    }
  }
  document.addEventListener('mousemove', handleDragging)
  document.addEventListener('mouseup', stopDragging)
}

function handleDragging(event: MouseEvent) {
  if (!isDragging.value) return
  let newTop = event.clientY - dragOffset.value.y
  let newLeft = event.clientX - dragOffset.value.x
  newTop = Math.max(0, newTop)
  diffViewPosition.value = {
    left: `${newLeft}px`,
    top: `${newTop}px`
  }
}

function stopDragging() {
  isDragging.value = false
  document.removeEventListener('mousemove', handleDragging)
  document.removeEventListener('mouseup', stopDragging)
}

onUnmounted(() => {
  window.removeEventListener('resize', ensureLineNumberAlignment)
})

function scrollToBottom() {
  nextTick(() => {
    const container = conversationHistoryRef.value
    if (container) {
      container.scrollTop = container.scrollHeight
    }
  })
}

watch(conversationHistory, () => {
  scrollToBottom()
}, { deep: true })

function handleInlineEditSubmit() {
  if (selectedTextInfo.value?.text && inlineEditPrompt.value.trim()) {
    submitInlineEdit(
      selectedTextInfo.value.text,
      chapter.value.title,
      chapter.value.content,
      inlineEditPrompt.value,
      chapterId.value,
      conversationHistoryRef.value || undefined
    )
  }
}

function acceptImprovedText() {
  if (!inlineEditResponse.value || !selectedTextInfo.value) return
  const { start, end, source } = selectedTextInfo.value
  const improvedText = inlineEditResponse.value.improved_text
  // 更新內容
  if (source === 'title' && titleInputRef.value) {
    const beforeText = chapter.value.title.substring(0, start)
    const afterText = chapter.value.title.substring(end)
    chapter.value.title = beforeText + improvedText + afterText
    // 更新選中範圍
    const newEnd = start + improvedText.length
    titleInputRef.value.setSelectionRange(start, newEnd)
    highlightInsertedText(titleInputRef.value, start, newEnd)
  } else if (source === 'content' && contentTextareaRef.value) {
    const beforeText = chapter.value.content.substring(0, start)
    const afterText = chapter.value.content.substring(end)
    chapter.value.content = beforeText + improvedText + afterText
    // 更新選中範圍
    const newEnd = start + improvedText.length
    contentTextareaRef.value.setSelectionRange(start, newEnd)
    highlightInsertedText(contentTextareaRef.value, start, newEnd)
    addToHistory(chapter.value.content)
    updateLineCount()
  }
  debounceSave()
  showDiffView.value = false
  inlineEditResponse.value = null
}

function highlightInsertedText(element: HTMLInputElement | HTMLTextAreaElement, start: number, end: number) {
  element.classList.add('highlight-textarea')
  setTimeout(() => {
    element.classList.remove('highlight-textarea')
  }, 1000)
}

function renderDiffHtml(oldText: string, newText: string) {
  const diff = diffWords(oldText || '', newText || '')
  return diff.map((part: Change) => {
    if (part.added) {
      return `<span class='diff-added'>${escapeHtml(part.value)}</span>`
    } else if (part.removed) {
      return `<span class='diff-removed'>${escapeHtml(part.value)}</span>`
    } else {
      return escapeHtml(part.value)
    }
  }).join('')
}

function escapeHtml(text: string) {
  return text.replace(/[&<>"']/g, function (c) {
    return {'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c] || c
  })
}

function formatSelectedTextHtml(text: string) {
  // 將 [Lines x-y] 或 [Title] 轉為粗體，並支援換行
  return text
    .replace(/^(\[Lines \d+-\d+\]|\[Title\])/m, '<b>$1</b>')
    .replace(/\n/g, '<br>')
}

const showDeleteModal = ref(false)
const showSecondDeleteModal = ref(false)
async function deleteChapterHandler() {
  if (!chapter.value.id) return
  try {
    await deleteChapterApi(chapter.value.id)
    showSecondDeleteModal.value = false
    showDeleteModal.value = false
    await tauriMessage(t('chapter.deleted', { title: chapter.value.title }))
    const others = (await getChaptersByProject(chapter.value.project_id)).filter(c => c.id !== chapter.value.id)
    if (others && others.length > 0) {
      router.push({ name: 'edit-chapter', params: { project_id: chapter.value.project_id, chapter_id: others[0].id } })
    } else {
      router.push('/')
    }
  } catch (error) {
    console.error('刪除章節失敗:', error)
    await tauriMessage(t('chapter.delete_failed'))
  }
}
const deleteChapter = deleteChapterHandler
</script>

<style scoped>
.editor-container { max-width: 95%; margin: 5px auto; }
.chapter-title-input { width: 100%; font-size: 1.3rem; font-weight: bold; margin-bottom: 1rem; }
.chapter-content-textarea { width: 100%; min-height: 300px; font-size: 1.1rem; }

/* DraftView 內容區動畫 */
.content-update-animation {
  animation: content-update-flash 1s;
}
@keyframes content-update-flash {
  0% { background: #ffe; }
  100% { background: transparent; }
}

/* 新增的 highlight-textarea 樣式 */
.highlight-textarea { background: #ffe; transition: background 0.5s; }

/* DiffView 內容區塊：
<div class="original-text">
  <div class="text-label">Original:</div>
  <div class="text-content" v-html="renderDiffHtml(selectedTextInfo?.text || '', inlineEditResponse?.improved_text || '')"></div>
</div>
<div class="improved-text">
  <div class="text-label">Improved:</div>
  <div class="text-content" v-html="renderDiffHtml(inlineEditResponse?.improved_text || '', selectedTextInfo?.text || '')"></div>
</div> */

/* style */
.diff-added { background: #d4f8d4; color: #228B22; }
.diff-removed { background: #ffe0e0; color: #b22222; text-decoration: line-through; }

.editor-footer {
  display: flex;
  justify-content: flex-end;
  margin-top: 2rem;
}
.delete-chapter-btn {
  background: #ff4d4f;
  color: #fff;
  border: none;
  border-radius: 6px;
  padding: 0.75rem 1.5rem;
  font-size: 1rem;
  cursor: pointer;
  transition: background 0.2s;
}
.delete-chapter-btn:hover {
  background: #d9363e;
}
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0,0,0,0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}
.modal-content {
  background: #fff;
  border-radius: 10px;
  padding: 2rem 2.5rem;
  box-shadow: 0 2px 16px rgba(0,0,0,0.15);
  min-width: 320px;
  max-width: 400px;
}
.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 1.5rem;
}
</style> 