<template>
  <TwoColumnLayout>
    <div class="chapter-draft-page">
      <ChapterDraftDialog
        :bookTitle="bookTitle"
        :defaultChapterCount="defaultChapterCount"
        :visible="true"
        @confirm="handleConfirm"
        @cancel="handleCancel"
      />
    </div>
  </TwoColumnLayout>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import TwoColumnLayout from '../components/layouts/TwoColumnLayout.vue'
import ChapterDraftDialog from '../components/book-project/ChapterDraftDialog.vue'

const route = useRoute()
const router = useRouter()

const bookTitle = ref(route.query.project_name ? decodeURIComponent(route.query.project_name as string) : '')
const defaultChapterCount = ref(8)

const handleConfirm = (data: any) => {
  localStorage.setItem('chapterDrafts', JSON.stringify(data.chapters))
  router.push({
    path: '/review-chapters',
    query: { project_id: route.query.project_id }
  })
}
const handleCancel = () => {
  router.back()
}
</script>

<style scoped>
.chapter-draft-page {
  max-width: 700px;
  margin: 2rem auto;
}
</style> 