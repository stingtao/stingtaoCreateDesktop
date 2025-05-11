<template>
  <Modal v-if="visible" @close="$emit('cancel')">
    <h2>{{ t('chapter.auto_generate_title') }}</h2>
    <form @submit.prevent="onGenerate">
      <label>{{ t('chapter.book_title') }}</label>
      <input v-model="localBookTitle" />
      <label>{{ t('chapter.chapter_count') }}</label>
      <input type="number" v-model.number="chapterCount" min="1" />
      <label>{{ t('chapter.extra_info') }}</label>
      <textarea v-model="extraInfo" rows="5" class="extra-info-textarea" />
      <button type="submit" :disabled="loading" class="big-button">{{ t('chapter.generate_draft') }}</button>
    </form>
    <div v-if="loading" class="loading-overlay">
      <div class="spinner"></div>
      <div class="loading-text">{{ t('chapter.generating_ai') }}</div>
    </div>
    <div v-if="chapters.length">
      <div v-for="(ch, i) in chapters" :key="i">
        <input v-model="ch.title" :placeholder="t('chapter.chapter_title')" />
        <textarea v-model="ch.content" :placeholder="t('chapter.chapter_content')" />
      </div>
      <button @click="onAccept">{{ t('common.accept') }}</button>
    </div>
    <button @click="$emit('cancel')" class="big-button cancel-button">{{ t('common.cancel') }}</button>
    <div v-if="errorMessage" class="error-toast">{{ errorMessage }}</div>
  </Modal>
</template>
<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import Modal from './Modal.vue'
import { getProject } from '../../lib/project'
import { invoke } from '@tauri-apps/api/tauri'
import { useI18n } from 'vue-i18n'
import { buildAIPrompt } from '../../composables/useAIPromptBuilder'
const props = defineProps<{ bookTitle: string, defaultChapterCount: number, visible: boolean }>()
const emit = defineEmits(['confirm', 'cancel'])
const localBookTitle = ref(props.bookTitle)
const chapterCount = ref(props.defaultChapterCount)
const extraInfo = ref('')
const loading = ref(false)
const chapters = ref<{ title: string, content: string }[]>([])
const router = useRouter()
const route = useRoute()
const { t } = useI18n()
const errorMessage = ref('')

watch(() => props.bookTitle, (val) => { localBookTitle.value = val })
watch(() => props.defaultChapterCount, (val) => { chapterCount.value = val })

const onGenerate = async () => {
  loading.value = true
  chapters.value = []
  try {
    // 1. 取得 project 詳細資料
    const projectId = Number(route.query.project_id)
    const project = projectId ? await getProject(projectId) : null
    // 2. 組合 prompt
    const prompt = buildAIPrompt('ChapterDraft', {
      bookTitle: localBookTitle.value,
      chapterCount: chapterCount.value,
      extraInfo: extraInfo.value,
      project: project || undefined
    })
    // 3. 呼叫 Gemini API
    const raw = await invoke<string>('generate_with_gemini', {
      prompt,
      agentType: 'DraftGenerator',
      blogId: 0,
      projectData: project || null
    })
    // 4. 解析 JSON code block
    const match = raw.match(/```json[\r\n]+([\s\S]+?)\r?\n```/)
    let arr = []
    if (match && match[1]) {
      try { arr = JSON.parse(match[1]) } catch (e) { arr = [] }
    }
    if (!Array.isArray(arr) || !arr.length) throw new Error('AI 回傳格式錯誤')
    chapters.value = arr.map((c: any) => ({ title: c.chapterTitle || '', content: c.chapterContent || '' }))
    // 存入 localStorage 並跳轉
    localStorage.setItem('chapterDrafts', JSON.stringify(chapters.value))
    router.push({
      path: '/review-chapters',
      query: { project_id: route.query.project_id }
    })
  } catch (e: any) {
    errorMessage.value = t('chapter.generate_failed') + (e?.message || e)
  } finally {
    loading.value = false
  }
}
const onAccept = () => {
  emit('confirm', { bookTitle: localBookTitle.value, chapterCount: chapterCount.value, extraInfo: extraInfo.value, chapters: chapters.value })
}
</script>
<style scoped>
.big-button {
  font-size: 1.15rem;
  padding: 0.7em 2em;
  border-radius: 8px;
  margin: 0.5em 0.5em 0 0;
  background: #3b82f6;
  color: #fff;
  border: none;
  cursor: pointer;
  transition: background 0.2s;
}
.big-button:hover {
  background: #2563eb;
}
.cancel-button {
  background: #e5e7eb;
  color: #222;
}
.cancel-button:hover {
  background: #cbd5e1;
}
.extra-info-textarea {
  width: 100%;
  min-height: 5em;
  font-size: 1rem;
  padding: 0.5em;
  border-radius: 6px;
  border: 1px solid #ccc;
  resize: vertical;
  margin-bottom: 1em;
}
.loading-overlay {
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(255,255,255,0.85);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  z-index: 100;
}
.spinner {
  border: 6px solid #e0e0e0;
  border-top: 6px solid #3b82f6;
  border-radius: 50%;
  width: 48px;
  height: 48px;
  animation: spin 1s linear infinite;
  margin-bottom: 1rem;
}
@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
.loading-text {
  font-size: 1.2rem;
  color: #1976d2;
  font-weight: bold;
  letter-spacing: 1px;
}
.error-toast {
  position: fixed;
  top: 2rem;
  left: 50%;
  transform: translateX(-50%);
  background: #e53935;
  color: #fff;
  padding: 0.7rem 2rem;
  border-radius: 6px;
  z-index: 2000;
  font-size: 1.1rem;
  box-shadow: 0 2px 8px rgba(0,0,0,0.18);
  pointer-events: none;
}
</style> 