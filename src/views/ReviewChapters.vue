<template>
  <TwoColumnLayout>
    <div class="review-chapters">
      <h2>審查章節草稿</h2>
      <button v-if="drafts.length" class="accept-all-btn" @click="acceptAll" :disabled="loadingIdx!==-1">✔ 全部接受</button>
      <div v-if="drafts.length === 0">所有章節已處理完畢！</div>
      <div v-for="(ch, i) in drafts" :key="i" class="chapter-draft" :class="{ loading: loadingIdx===i }">
        <input v-model="ch.title" class="chapter-title-input" placeholder="章節標題" />
        <textarea v-model="ch.content" rows="4" style="width:100%" placeholder="章節內容建議..." />
        <div class="actions">
          <button :disabled="loadingIdx===i" @click="acceptChapter(ch, i)">
            <span v-if="loadingIdx===i">處理中...</span>
            <span v-else>✔ 接受</span>
          </button>
          <button :disabled="loadingIdx===i" @click="removeDraft(ch, i)">🗑 刪除</button>
        </div>
      </div>
      <div v-if="toast" class="toast">{{ toast }}</div>
    </div>
  </TwoColumnLayout>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/tauri'
import TwoColumnLayout from '../components/layouts/TwoColumnLayout.vue'
import { useI18n } from 'vue-i18n'

const route = useRoute()
const router = useRouter()
const { t } = useI18n()
const projectId = Number(route.query.project_id)
const drafts = ref(JSON.parse(localStorage.getItem('chapterDrafts') || '[]'))
const loadingIdx = ref(-1)
const toast = ref('')
function showToast(msg: string) {
  toast.value = msg
  setTimeout(() => (toast.value = ''), 1800)
}

const acceptChapter = async (ch: any, idx: number) => {
  loadingIdx.value = idx
  try {
    if (ch.id) {
      await invoke('update_chapter', {
        chapter: {
          id: ch.id,
          project_id: projectId,
          title: ch.title,
          content: ch.content,
          chapter_number: idx + 1
        }
      })
    } else {
      await invoke('save_chapter', {
        chapter: {
          project_id: projectId,
          title: ch.title,
          content: ch.content,
          chapter_number: idx + 1
        }
      })
    }
    drafts.value.splice(idx, 1)
    showToast(t('chapter.accepted'))
    window.dispatchEvent(new CustomEvent('refresh-sidebar'))
  } catch (e: any) {
    showToast(t('chapter.save_failed') + (e?.message || e))
    console.error(e)
  }
  loadingIdx.value = -1
}
const removeDraft = async (ch: any, idx: number) => {
  loadingIdx.value = idx
  try {
    if (ch.id) {
      await invoke('delete_chapter', { id: ch.id })
    }
    drafts.value.splice(idx, 1)
    showToast(t('chapter.deleted'))
  } catch (e: any) {
    showToast(t('chapter.delete_failed') + (e?.message || e))
    console.error(e)
  }
  loadingIdx.value = -1
}
const acceptAll = async () => {
  loadingIdx.value = -2
  for (let i = drafts.value.length - 1; i >= 0; i--) {
    await acceptChapter(drafts.value[i], i)
  }
  showToast(t('chapter.all_accepted'))
  loadingIdx.value = -1
  window.dispatchEvent(new CustomEvent('refresh-sidebar'))
}
watch(drafts, (val) => {
  if (val.length === 0) {
    setTimeout(() => router.push('/projects'), 1200)
  }
})
</script>

<style scoped>
.review-chapters {
  max-width: 95%;
  margin: 5px auto;
}
.chapter-draft {
  background: #f9f9f9;
  border-radius: 8px;
  padding: 1rem;
  margin-bottom: 1.5rem;
  box-shadow: 0 2px 8px rgba(0,0,0,0.04);
  transition: box-shadow 0.2s, border 0.2s;
}
.chapter-draft:hover {
  box-shadow: 0 4px 16px rgba(25, 118, 210, 0.08);
  border: 1.5px solid #1976d2;
}
.chapter-title-input {
  width: 100%;
  font-size: 1.1rem;
  font-weight: bold;
  margin-bottom: 0.5rem;
  padding: 0.4rem 0.6rem;
  border-radius: 4px;
  border: 1px solid #ccc;
  transition: border 0.2s, box-shadow 0.2s;
}
.chapter-title-input:focus, textarea:focus {
  border-color: #1976d2;
  outline: none;
  box-shadow: 0 0 0 2px #1976d233;
}
.actions {
  margin-top: 0.5rem;
  display: flex;
  gap: 1rem;
}
button {
  padding: 0.4rem 1.2rem;
  border-radius: 4px;
  border: none;
  background: #1976d2;
  color: #fff;
  cursor: pointer;
  font-size: 1rem;
  display: flex;
  align-items: center;
  gap: 0.4rem;
  transition: background 0.2s;
}
button:last-child {
  background: #e53935;
}
button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
.accept-all-btn {
  margin-bottom: 1.2rem;
  background: #388e3c;
}
.toast {
  position: fixed;
  top: 2rem; left: 50%; transform: translateX(-50%);
  background: #323232; color: #fff; padding: 0.7rem 2rem;
  border-radius: 6px; z-index: 2000; font-size: 1.1rem;
  box-shadow: 0 2px 8px rgba(0,0,0,0.18);
  pointer-events: none;
}
</style> 