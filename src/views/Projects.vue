<template>
  <TwoColumnLayout>
    <div class="projects-container">
      <div class="projects-header">
        <h1>{{ t('projects.title') }}</h1>
        <router-link to="/new-project" class="new-project-btn">
          <span class="btn-icon">+</span>
          {{ t('projects.newProject') }}
        </router-link>
      </div>

      <!-- Projects Grid -->
      <div class="projects-grid">
        <div 
          v-for="project in projects" 
          :key="project.id" 
          class="project-card"
          :class="[project.type_ === 'blog' ? 'blog-bg' : project.type_ === 'book' ? 'book-bg' : '', { 'deleting': project.id === deletingProjectId }]"
        >
          <div class="project-header">
            <h3>{{ project.title }}</h3>
            <div class="header-actions">
              <div class="project-type">{{ getProjectTypeLabel(project.type_) }}</div>
            </div>
          </div>

          <div class="project-info">
            <div class="info-section">
              <h4>{{ t('projects.goal') }}</h4>
              <p>{{ truncateText(project.goal || '', 50) }}</p>
            </div>
            <div class="info-section">
              <h4>{{ t('projects.description') }}</h4>
              <p>{{ truncateText(project.description || '', 100) }}</p>
            </div>
            <div class="info-section">
              <h4>{{ t('projects.frequency') }}</h4>
              <p>{{ project.publishing_frequency || t('projects.notSet') }}</p>
            </div>
            <div class="info-section">
              <h4>{{ t('projects.targetAudience') }}</h4>
              <p>{{ truncateText(project.target_audience || '', 50) }}</p>
            </div>
          </div>

          <div class="project-footer">
            <div class="project-dates">
              <div class="date-item">
                <span class="date-label">{{ t('projects.startDate') }}:</span>
                <span class="date-value">{{ formatDate(project.created_at || '') }}</span>
              </div>
              <div class="date-item">
                <span class="date-label">{{ t('projects.endDate') }}:</span>
                <span class="date-value">{{ formatDate(project.deadline || '') }}</span>
              </div>
              <div class="date-item">
                <span class="date-label">{{ t('projects.updated') }}:</span>
                <span class="date-value">{{ formatDate(project.updated_at || '') }}</span>
              </div>
            </div>

            <div class="project-actions">
              <button 
                class="action-btn edit"
                @click="editProject(project.id || 0)"
                :title="t('projects.tooltips.edit')"
              >
                {{ t('projects.actions.edit') }}
              </button>
              <button 
                class="action-btn delete"
                @click="confirmDelete(project)"
                :title="t('projects.tooltips.delete')"
              >
                {{ t('projects.actions.delete') }}
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div v-if="projects.length === 0" class="empty-state">
        <div class="empty-icon">📝</div>
        <h2>{{ t('projects.empty.title') }}</h2>
        <p>{{ t('projects.empty.description') }}</p>
        <router-link to="/new-project" class="start-btn">
          {{ t('projects.empty.start') }}
        </router-link>
      </div>

      <!-- Delete Confirmation Modal -->
      <div v-if="showDeleteModal" class="modal-overlay">
        <div class="modal-content">
          <h3>{{ t('projects.deleteModal.title') }}</h3>
          <p>{{ t('projects.deleteModal.confirmation', { title: projectToDelete?.title }) }}</p>
          <div class="modal-actions">
            <button 
              class="cancel-btn"
              @click="showDeleteModal = false"
            >
              {{ t('projects.deleteModal.cancel') }}
            </button>
            <button 
              class="confirm-btn"
              @click="showSecondConfirmation = true"
            >
              {{ t('projects.deleteModal.confirm') }}
            </button>
          </div>
        </div>
      </div>

      <!-- Second Delete Confirmation Modal -->
      <div v-if="showSecondConfirmation" class="modal-overlay">
        <div class="modal-content">
          <h3>{{ t('projects.deleteModal.secondTitle') }}</h3>
          <p>{{ t('projects.deleteModal.secondConfirmation', { title: projectToDelete?.title }) }}</p>
          <div class="modal-actions">
            <button 
              class="cancel-btn"
              @click="showSecondConfirmation = false"
            >
              {{ t('projects.deleteModal.cancel') }}
            </button>
            <button 
              class="confirm-btn"
              @click="deleteProjectConfirmed"
            >
              {{ t('projects.deleteModal.confirm') }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </TwoColumnLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import TwoColumnLayout from '../components/layouts/TwoColumnLayout.vue'
import { getAllProjects, deleteProject, updateProject, type ProjectSummary } from '../lib/project'
import '../styles/Projects.css'

const { t } = useI18n()
const router = useRouter()

const projects = ref<ProjectSummary[]>([])
const showDeleteModal = ref(false)
const showSecondConfirmation = ref(false)
const projectToDelete = ref<ProjectSummary | null>(null)
const deletingProjectId = ref<number | null>(null)

// Load projects
const loadProjects = async () => {
  projects.value = await getAllProjects()
}

// Format date
const formatDate = (date: string) => {
  if (!date) return t('projects.notSet');
  const parsedDate = new Date(date);
  return isNaN(parsedDate.getTime()) ? t('projects.notSet') : parsedDate.toLocaleDateString();
}

// Edit project
const editProject = (id: number) => {
  router.push(`/projects/${id}`)
}

// Delete project
const confirmDelete = (project: ProjectSummary) => {
  projectToDelete.value = project
  showDeleteModal.value = true
}

const deleteProjectConfirmed = async () => {
  if (projectToDelete.value && projectToDelete.value.id) {
    try {
      console.log(`[DELETE] Starting deletion process for project ID: ${projectToDelete.value.id}, Title: "${projectToDelete.value.title}"`);
      deletingProjectId.value = projectToDelete.value.id
      
      const success = await deleteProject(projectToDelete.value.id)
      console.log(`[DELETE] API response - success: ${success}`);
      
      if (success) {
        console.log(`[DELETE] Project deletion successful, updating UI`);
        setTimeout(() => {
          projects.value = projects.value.filter(p => p.id !== projectToDelete.value?.id)
          console.log(`[DELETE] Projects remaining in UI: ${projects.value.length}`);
          deletingProjectId.value = null
          
          loadProjects().then(() => {
            console.log(`[DELETE] Projects refreshed from server, count: ${projects.value.length}`);
          }).catch(err => {
            console.error(`[DELETE] Error refreshing projects:`, err);
          });
        }, 2000)
      } else {
        console.error(`[DELETE] Project deletion failed - API returned false`);
        throw new Error(t('projects.deleteError'))
      }
    } catch (error) {
      console.error('[DELETE] Error deleting project:', error)
      alert(t('projects.deleteError'))
    } finally {
      console.log(`[DELETE] Cleanup - closing modals and resetting state`);
      showDeleteModal.value = false
      showSecondConfirmation.value = false
      projectToDelete.value = null
    }
  }
}

const getProjectTypeIcon = (type: string) => {
  switch (type) {
    case 'blog':
      return 'i-heroicons-document-text'
    case 'book':
      return 'i-heroicons-book-open'
    case 'course':
      return 'i-heroicons-academic-cap'
    default:
      return 'i-heroicons-document'
  }
}

const getProjectTypeLabel = (type: string) => {
  switch (type) {
    case 'blog':
      return t('projects.types.blog')
    case 'book':
      return t('projects.types.book')
    case 'course':
      return t('projects.types.course')
    default:
      return t('projects.types.blog')
  }
}

const truncateText = (text: string, maxLength: number) => {
  if (!text) return ''
  return text.length <= maxLength ? text : text.slice(0, maxLength) + '...'
}

// Set end date
const setEndDate = async (project: ProjectSummary) => {
  const currentDate = project.end_date 
    ? new Date(project.end_date).toISOString().split('T')[0]
    : new Date().toISOString().split('T')[0];
    
  const date = prompt(t('projects.setEndDate'), currentDate);
  if (date) {
    try {
      // Validate the date
      const parsedDate = new Date(date);
      if (isNaN(parsedDate.getTime())) {
        throw new Error('Invalid date');
      }
      
      const updatedProject = { ...project, end_date: date };
      const success = await updateProject(updatedProject);
      if (success) {
        await loadProjects();
      }
    } catch (error) {
      alert(t('projects.invalidDate'));
    }
  }
}

onMounted(() => {
  loadProjects()
})
</script>

<style scoped>
/* All styles moved to src/styles/Projects.css */
/* Blog/Book 專案不同背景色 */
.project-card.blog-bg {
  background-color: #f3f7fa;
}
.project-card.book-bg {
  background-color: #f9f6f2;
}
</style> 