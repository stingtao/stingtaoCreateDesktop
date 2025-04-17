<template>
  <TwoColumnLayout>
    <div class="edit-project-container">
      <div class="edit-project-header">
        <h1>{{ t('projects.editForm.title') }}</h1>
        <div class="header-actions">
          <button class="save-btn" @click="saveProject" :disabled="isSaving">
            <i class="fas fa-save"></i>
            {{ isSaving ? t('projects.editForm.saving') : t('projects.editForm.save') }}
          </button>
          <button class="cancel-btn" @click="goBack">
            <i class="fas fa-times"></i>
            {{ t('projects.editForm.cancel') }}
          </button>
        </div>
      </div>

      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>{{ t('projects.editForm.loading') }}</p>
      </div>

      <div v-else-if="error" class="error-state">
        <p>{{ error }}</p>
        <button class="retry-btn" @click="loadProject">
          <i class="fas fa-redo"></i>
          {{ t('projects.editForm.retry') }}
        </button>
      </div>

      <div v-else class="edit-project-form">
        <p class="required-fields-note">{{ t('projects.editForm.requiredFields') }}</p>
        
        <!-- Basic Information -->
        <div class="form-section">
          <h2><i class="fas fa-info-circle"></i> {{ t('projects.editForm.basicInfo') }}</h2>
          <div class="form-group">
            <label for="title" class="required">{{ t('projects.editForm.title') }}</label>
            <input 
              id="title"
              v-model="project.title"
              type="text"
              :placeholder="t('projects.editForm.titlePlaceholder')"
              required
            >
          </div>

          <div class="form-group">
            <label for="type">{{ t('projects.editForm.type') }}</label>
            <input 
              id="type"
              v-model="project.type_"
              type="text"
              readonly
              class="readonly-field"
            >
          </div>

          <div class="form-group">
            <label for="category">{{ t('projects.editForm.category') }}</label>
            <input 
              id="category"
              v-model="project.category"
              type="text"
              readonly
              class="readonly-field"
            >
          </div>

          <div class="form-group">
            <label for="description" class="required">{{ t('projects.editForm.description') }}</label>
            <textarea 
              id="description"
              v-model="project.description"
              :placeholder="t('projects.editForm.descriptionPlaceholder')"
              rows="4"
              required
            ></textarea>
          </div>

          <div class="form-group">
            <label for="goal" class="required">{{ t('projects.editForm.goal') }}</label>
            <textarea 
              id="goal"
              v-model="project.goal"
              :placeholder="t('projects.editForm.goalPlaceholder')"
              rows="4"
              required
            ></textarea>
          </div>
        </div>

        <!-- Time Management -->
        <div class="form-section">
          <h2><i class="fas fa-clock"></i> {{ t('projects.editForm.timeManagement') }}</h2>
          <div class="form-group">
            <label for="time_commitment" class="required">{{ t('projects.editForm.timeCommitment') }}</label>
            <select id="time_commitment" v-model="project.time_commitment">
              <option value="casual">{{ t('common.timeCommitment.casual') }}</option>
              <option value="partTime">{{ t('common.timeCommitment.partTime') }}</option>
              <option value="fullTime">{{ t('common.timeCommitment.fullTime') }}</option>
            </select>
          </div>

          <div class="form-group">
            <label for="receive_notifications">{{ t('projects.editForm.receiveNotifications') }}</label>
            <select id="receive_notifications" v-model="project.receive_notifications">
              <option value="true">{{ t('projects.editForm.notifications.yes') }}</option>
              <option value="false">{{ t('projects.editForm.notifications.no') }}</option>
            </select>
          </div>

          <div class="form-group">
            <label for="publishing_frequency" class="required">{{ t('projects.editForm.publishingFrequency') }}</label>
            <select id="publishing_frequency" v-model="project.publishing_frequency" required>
              <option value="daily">{{ t('projects.editForm.frequency.daily') }}</option>
              <option value="weekly">{{ t('projects.editForm.frequency.weekly') }}</option>
              <option value="biweekly">{{ t('projects.editForm.frequency.biweekly') }}</option>
              <option value="monthly">{{ t('projects.editForm.frequency.monthly') }}</option>
              <option value="custom">{{ t('projects.editForm.frequency.custom') }}</option>
            </select>
          </div>

          <div class="form-group" v-if="project.publishing_frequency === 'custom'">
            <label for="custom_frequency" class="required">{{ t('projects.editForm.customFrequency') }}</label>
            <input 
              id="custom_frequency"
              v-model="project.custom_frequency"
              type="text"
              :placeholder="t('projects.editForm.customFrequencyPlaceholder')"
              required
            >
          </div>

          <div class="form-group">
            <label for="deadline" class="required">{{ t('projects.editForm.deadline') }}</label>
            <input 
              id="deadline"
              v-model="project.deadline"
              type="date"
              required
            >
          </div>

          <div class="form-group">
            <label for="availability" class="required">{{ t('common.availability.label') }}</label>
            <div class="availability-options">
              <button 
                v-for="day in weekDays" 
                :key="day.value"
                class="day-option"
                :class="{ selected: isDaySelected(day.value) }"
                @click="toggleAvailability(day.value)"
              >
                {{ t(`common.availability.daysShort.${day.value}`) }}
              </button>
            </div>
          </div>

          <div class="form-group">
            <label for="reminder_frequency" class="required">{{ t('projects.editForm.reminderFrequency') }}</label>
            <select id="reminder_frequency" v-model="project.reminder_frequency" required>
              <option value="daily">{{ t('projects.editForm.frequency.daily') }}</option>
              <option value="weekly">{{ t('projects.editForm.frequency.weekly') }}</option>
              <option value="monthly">{{ t('projects.editForm.frequency.monthly') }}</option>
            </select>
          </div>
        </div>

        <!-- Publishing Details -->
        <div class="form-section">
          <h2><i class="fas fa-globe"></i> {{ t('projects.editForm.publishingDetails') }}</h2>
          <div class="form-group">
            <label for="publishing_platform">{{ t('projects.editForm.publishingPlatform') }}</label>
            <input 
              id="publishing_platform"
              v-model="project.publishing_platform"
              type="text"
              readonly
              class="readonly-field"
            >
          </div>

          <div class="form-group" v-if="project.wordpress_url">
            <label for="wordpress_url">{{ t('projects.editForm.wordpressUrl') }}</label>
            <input 
              id="wordpress_url"
              v-model="project.wordpress_url"
              type="text"
              readonly
              class="readonly-field"
            >
          </div>

          <div class="form-group" v-if="project.substack_url">
            <label for="substack_url">{{ t('projects.editForm.substackUrl') }}</label>
            <input 
              id="substack_url"
              v-model="project.substack_url"
              type="text"
              readonly
              class="readonly-field"
            >
          </div>

          <div class="form-group" v-if="project.custom_platform">
            <label for="custom_platform">{{ t('projects.editForm.customPlatform') }}</label>
            <input 
              id="custom_platform"
              v-model="project.custom_platform"
              type="text"
              readonly
              class="readonly-field"
            >
          </div>
        </div>

        <!-- Monetization -->
        <div class="form-section">
          <h2><i class="fas fa-dollar-sign"></i> {{ t('projects.editForm.monetization') }}</h2>
          <div class="form-group">
            <label for="monetization_strategy">{{ t('projects.editForm.monetizationStrategy') }}</label>
            <input 
              id="monetization_strategy"
              v-model="project.monetization_strategy"
              type="text"
              readonly
              class="readonly-field"
            >
          </div>

          <div class="form-group">
            <label for="monetization_goals">{{ t('projects.editForm.monetizationGoals') }}</label>
            <input 
              id="monetization_goals"
              v-model="project.monetization_goals"
              type="text"
              readonly
              class="readonly-field"
            >
          </div>
        </div>

        <!-- Content Strategy -->
        <div class="form-section">
          <h2><i class="fas fa-pen-fancy"></i> {{ t('projects.editForm.contentStrategy') }}</h2>
          <div class="form-group">
            <label for="keywords" class="required">{{ t('projects.editForm.keywords') }}</label>
            <div class="keywords-input">
              <input 
                v-model="keywordInput"
                type="text"
                :placeholder="t('projects.editForm.keywordsPlaceholder')"
                @keyup.enter="addKeyword"
                required
              >
              <button class="add-keyword-btn" @click="addKeyword">+</button>
            </div>
            <div class="keywords-list">
              <span 
                v-for="(keyword, index) in parsedKeywords" 
                :key="index"
                class="keyword-tag"
              >
                {{ keyword }}
                <span class="remove-keyword" @click="removeKeyword(index)">&times;</span>
              </span>
            </div>
          </div>

          <div class="form-group">
            <label for="target_audience" class="required">{{ t('projects.editForm.targetAudience') }}</label>
            <input 
              id="target_audience"
              v-model="project.target_audience"
              type="text"
              :placeholder="t('projects.editForm.targetAudiencePlaceholder')"
              required
            >
          </div>

          <div class="form-group">
            <label for="article_length" class="required">{{ t('projects.editForm.articleLengthLabel') }}</label>
            <select id="article_length" v-model="project.article_length" required>
              <option value="150">{{ t('projects.editForm.articleLength.short') }} (150 {{ t('newProject.blog.words') }})</option>
              <option value="300">{{ t('projects.editForm.articleLength.medium') }} (300 {{ t('newProject.blog.words') }})</option>
              <option value="600">{{ t('projects.editForm.articleLength.long') }} (600 {{ t('newProject.blog.words') }})</option>
              <option value="1000">{{ t('projects.editForm.articleLength.veryLong') }} (1000 {{ t('newProject.blog.words') }})</option>
              <option value="2000">{{ t('projects.editForm.articleLength.extensive') }} (2000 {{ t('newProject.blog.words') }})</option>
            </select>
          </div>
        </div>
      </div>
    </div>
  </TwoColumnLayout>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import TwoColumnLayout from '../components/layouts/TwoColumnLayout.vue'
import { getProject, updateProject, type Project } from '../lib/project'
import '../styles/EditProject.css'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const project = ref<Project>({
  id: 0,
  title: '',
  type_: 'blog',
  description: '',
  category: '',
  time_commitment: 'casual',
  publishing_frequency: 'weekly',
  custom_frequency: '',
  deadline: '',
  availability: '[]',
  reminder_frequency: 'weekly',
  publishing_platform: 'wordpress',
  wordpress_url: '',
  substack_url: '',
  custom_platform: '',
  monetization_strategy: 'free',
  monetization_goals: '0',
  keywords: '[]',
  target_audience: '',
  structure: 'standard',
  content_strategy: '',
  seo_strategy: '',
  goal: '',
  progress: 0,
  created_at: '',
  updated_at: '',
  article_length: '150',
  receive_notifications: true
})

const keywordInput = ref('')
const isLoading = ref(true)
const isSaving = ref(false)
const error = ref<string | null>(null)

// Week days for availability
const weekDays = [
  { value: 'monday', label: 'M' },
  { value: 'tuesday', label: 'T' },
  { value: 'wednesday', label: 'W' },
  { value: 'thursday', label: 'T' },
  { value: 'friday', label: 'F' },
  { value: 'saturday', label: 'S' },
  { value: 'sunday', label: 'S' }
]

// Computed property for parsed keywords
const parsedKeywords = computed(() => {
  try {
    return JSON.parse(project.value.keywords || '[]')
  } catch {
    return []
  }
})

const loadProject = async () => {
  isLoading.value = true
  error.value = null
  
  try {
    const projectId = Number(route.params.id)
    if (!projectId) {
      throw new Error(t('projects.editForm.invalidId'))
    }

    const loadedProject = await getProject(projectId)
    if (!loadedProject) {
      throw new Error(t('projects.editForm.notFound'))
    }

    project.value = loadedProject
  } catch (e) {
    error.value = e instanceof Error ? e.message : t('projects.editForm.loadError')
  } finally {
    isLoading.value = false
  }
}

const saveProject = async () => {
  if (!project.value.title || !project.value.type_) {
    error.value = t('projects.editForm.requiredFields')
    return
  }

  isSaving.value = true
  error.value = null

  try {
    const success = await updateProject(project.value)
    if (success) {
      router.push('/projects')
    } else {
      throw new Error(t('projects.editForm.saveError'))
    }
  } catch (e) {
    error.value = e instanceof Error ? e.message : t('projects.editForm.saveError')
  } finally {
    isSaving.value = false
  }
}

const goBack = () => {
  router.push('/projects')
}

// Keyword methods
const addKeyword = () => {
  if (keywordInput.value.trim()) {
    const keywords = parsedKeywords.value
    if (!keywords.includes(keywordInput.value.trim())) {
      keywords.push(keywordInput.value.trim())
      project.value.keywords = JSON.stringify(keywords)
      keywordInput.value = ''
    }
  }
}

const removeKeyword = (index: number) => {
  const keywords = parsedKeywords.value
  keywords.splice(index, 1)
  project.value.keywords = JSON.stringify(keywords)
}

// Availability methods
const isDaySelected = (day: string) => {
  try {
    const availability = JSON.parse(project.value.availability || '[]')
    return availability.includes(day)
  } catch {
    return false
  }
}

const toggleAvailability = (day: string) => {
  try {
    const availability = JSON.parse(project.value.availability || '[]')
    const index = availability.indexOf(day)
    if (index === -1) {
      availability.push(day)
    } else {
      availability.splice(index, 1)
    }
    project.value.availability = JSON.stringify(availability)
  } catch {
    project.value.availability = JSON.stringify([day])
  }
}

onMounted(() => {
  loadProject()
})
</script>

<style scoped>
/* All styles moved to src/styles/EditProject.css */
</style> 