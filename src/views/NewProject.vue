<template>
  <TwoColumnLayout>
    <div class="new-project-container">
      <h1>{{ t('newProject.title') }}</h1>
      
      <!-- Progress indicator -->
      <div class="progress-indicator">
        <div 
          v-for="(step, index) in steps" 
          :key="index" 
          class="step" 
          :class="{ active: currentStep === index, completed: currentStep > index }"
        >
          <div class="step-number">{{ index + 1 }}</div>
          <div class="step-label">{{ t(`newProject.steps.${step}`) }}</div>
        </div>
      </div>
      
      <!-- Step content -->
      <div class="step-content">
        <!-- Step 1: Project Type -->
        <div v-if="currentStep === 0" class="step-form">
          <h2>{{ t('newProject.projectType.title') }}</h2>
          <p class="step-description">{{ t('newProject.projectType.description') }}</p>
          
          <div class="project-type-options">
            <div 
              v-for="type in projectTypes" 
              :key="type.value" 
              class="project-type-card"
              :class="{ selected: projectData.type === type.value }"
              @click="projectData.type = type.value"
            >
              <div class="type-icon">{{ type.icon }}</div>
              <h3>{{ t(`project.type.${type.value}`) }}</h3>
              <p>{{ t(`newProject.projectType.${type.value}Description`) }}</p>
            </div>
          </div>
        </div>
        
        <!-- Step 2: Project Details -->
        <div v-if="currentStep === 1" class="step-form">
          <h2>{{ t('newProject.details.title') }}</h2>
          <p class="step-description">{{ t('newProject.details.description') }}</p>
          
          <div class="form-group">
            <label for="project-title">{{ t('newProject.details.projectTitle') }}</label>
            <input 
              id="project-title" 
              v-model="projectData.title" 
              type="text" 
              :placeholder="t('newProject.details.titlePlaceholder')"
            />
          </div>
          
          <div class="form-group">
            <label for="project-description">{{ t('newProject.details.description') }}</label>
            <textarea 
              id="project-description" 
              v-model="projectData.description" 
              :placeholder="t('newProject.details.descriptionPlaceholder')"
              rows="4"
            ></textarea>
          </div>
          
          <div class="form-group">
            <label for="project-goal">{{ t('newProject.details.goal') }}</label>
            <textarea 
              id="project-goal" 
              v-model="projectData.goal" 
              :placeholder="t('newProject.details.goalPlaceholder')"
              rows="3"
            ></textarea>
          </div>
        </div>
        
        <!-- Step 3: Time Commitment -->
        <div v-if="currentStep === 2" class="step-form">
          <h2>{{ t('newProject.time.title') }}</h2>
          <p class="step-description">{{ t('newProject.time.description') }}</p>
          
          <div class="form-group">
            <label for="time-commitment">{{ t('newProject.time.commitment') }}</label>
            <select id="time-commitment" v-model="projectData.timeCommitment">
              <option value="casual">{{ t('newProject.time.options.casual') }}</option>
              <option value="partTime">{{ t('newProject.time.options.partTime') }}</option>
              <option value="fullTime">{{ t('newProject.time.options.fullTime') }}</option>
            </select>
          </div>
          
          <div class="form-group">
            <label for="deadline">{{ t('newProject.time.deadline') }}</label>
            <input 
              id="deadline" 
              v-model="projectData.deadline" 
              type="date" 
              :min="today"
            />
          </div>
          
          <div class="form-group">
            <label>{{ t('newProject.time.availability') }}</label>
            <div class="availability-options">
              <div 
                v-for="day in weekDays" 
                :key="day.value" 
                class="day-option"
                :class="{ selected: projectData.availability.includes(day.value) }"
                @click="toggleAvailability(day.value)"
              >
                {{ day.label }}
              </div>
            </div>
          </div>
        </div>
        
        <!-- Step 4: Resources -->
        <div v-if="currentStep === 3" class="step-form">
          <h2>{{ t('newProject.resources.title') }}</h2>
          <p class="step-description">{{ t('newProject.resources.description') }}</p>
          
          <div class="form-group">
            <label for="keywords">{{ t('newProject.resources.keywords') }}</label>
            <div class="keywords-input">
              <input 
                id="keywords" 
                v-model="keywordInput" 
                type="text" 
                :placeholder="t('newProject.resources.keywordsPlaceholder')"
                @keydown.enter="addKeyword"
              />
              <button class="add-keyword-btn" @click="addKeyword">+</button>
            </div>
            <div class="keywords-list">
              <div 
                v-for="(keyword, index) in projectData.keywords" 
                :key="index" 
                class="keyword-tag"
              >
                {{ keyword }}
                <span class="remove-keyword" @click="removeKeyword(index)">×</span>
              </div>
            </div>
          </div>
          
          <div class="form-group">
            <label for="resources">{{ t('newProject.resources.existingResources') }}</label>
            <div class="resources-upload">
              <div class="upload-area" @click="triggerFileUpload">
                <div class="upload-icon">📄</div>
                <p>{{ t('newProject.resources.uploadText') }}</p>
                <input 
                  type="file" 
                  ref="fileInput" 
                  multiple 
                  @change="handleFileUpload" 
                  style="display: none"
                />
              </div>
              <div class="uploaded-files" v-if="projectData.resources.length > 0">
                <div 
                  v-for="(resource, index) in projectData.resources" 
                  :key="index" 
                  class="resource-item"
                >
                  <span class="resource-name">{{ resource.name }}</span>
                  <span class="resource-type">{{ resource.type }}</span>
                  <button class="remove-resource" @click="removeResource(index)">×</button>
                </div>
              </div>
            </div>
          </div>
          
          <div class="form-group">
            <label for="reference_links">{{ t('newProject.resources.references') }}</label>
            <textarea 
              id="reference_links" 
              v-model="projectData.reference_links" 
              :placeholder="t('newProject.resources.referencesPlaceholder')"
              rows="3"
            ></textarea>
          </div>
        </div>
        
        <!-- Step 5: Project Plan -->
        <div v-if="currentStep === 4" class="step-form">
          <h2>{{ t('newProject.plan.title') }}</h2>
          <p class="step-description">{{ t('newProject.plan.description') }}</p>
          
          <div class="plan-preview">
            <div class="plan-section">
              <h3>{{ t('newProject.plan.timeline') }}</h3>
              <div class="timeline">
                <div class="timeline-item">
                  <div class="timeline-date">{{ formatDate(new Date()) }}</div>
                  <div class="timeline-content">
                    <h4>{{ t('newProject.plan.start') }}</h4>
                    <p>{{ t('newProject.plan.startDescription') }}</p>
                  </div>
                </div>
                <div class="timeline-item">
                  <div class="timeline-date">{{ projectData.deadline ? formatDate(new Date(projectData.deadline)) : '' }}</div>
                  <div class="timeline-content">
                    <h4>{{ t('newProject.plan.deadline') }}</h4>
                    <p>{{ t('newProject.plan.deadlineDescription') }}</p>
                  </div>
                </div>
              </div>
            </div>
            
            <div class="plan-section">
              <h3>{{ t('newProject.plan.actionItems') }}</h3>
              <div class="action-items">
                <div class="action-item">
                  <div class="action-checkbox">☐</div>
                  <div class="action-content">
                    <h4>{{ t('newProject.plan.outline') }}</h4>
                    <p>{{ t('newProject.plan.outlineDescription') }}</p>
                  </div>
                </div>
                <div class="action-item">
                  <div class="action-checkbox">☐</div>
                  <div class="action-content">
                    <h4>{{ t('newProject.plan.research') }}</h4>
                    <p>{{ t('newProject.plan.researchDescription') }}</p>
                  </div>
                </div>
                <div class="action-item">
                  <div class="action-checkbox">☐</div>
                  <div class="action-content">
                    <h4>{{ t('newProject.plan.draft') }}</h4>
                    <p>{{ t('newProject.plan.draftDescription') }}</p>
                  </div>
                </div>
                <div class="action-item">
                  <div class="action-checkbox">☐</div>
                  <div class="action-content">
                    <h4>{{ t('newProject.plan.review') }}</h4>
                    <p>{{ t('newProject.plan.reviewDescription') }}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Navigation buttons -->
      <div class="step-navigation">
        <button 
          v-if="currentStep > 0" 
          class="nav-btn prev-btn" 
          @click="prevStep"
        >
          {{ t('newProject.navigation.prev') }}
        </button>
        <button 
          v-if="currentStep < steps.length - 1" 
          class="nav-btn next-btn" 
          @click="nextStep"
          :disabled="!canProceed"
        >
          {{ t('newProject.navigation.next') }}
        </button>
        <button 
          v-if="currentStep === steps.length - 1" 
          class="nav-btn create-btn" 
          @click="createProject"
        >
          {{ t('newProject.navigation.create') }}
        </button>
      </div>
    </div>
  </TwoColumnLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import TwoColumnLayout from '../components/layouts/TwoColumnLayout.vue'

const { t } = useI18n()
const router = useRouter()
const fileInput = ref<HTMLInputElement | null>(null)

// Project creation steps
const steps = ['goals', 'details', 'time', 'publishing', 'plan']
const currentStep = ref(0)

// Project data
const projectData = ref({
  type: '',
  title: '',
  description: '',
  goal: '',
  timeCommitment: 'casual',
  deadline: '',
  availability: [] as string[],
  keywords: [] as string[],
  resources: [] as { name: string, type: string }[],
  reference_links: ''
})

// Project types
const projectTypes = [
  { value: 'blog', icon: '📝' },
  { value: 'book', icon: '📚' }
]

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

// Today's date for deadline min value
const today = computed(() => {
  const date = new Date()
  return date.toISOString().split('T')[0]
})

// Keyword input
const keywordInput = ref('')

// Check if can proceed to next step
const canProceed = computed(() => {
  if (currentStep.value === 0) return projectData.value.type !== ''
  if (currentStep.value === 1) return projectData.value.title !== '' && projectData.value.description !== ''
  if (currentStep.value === 2) return projectData.value.deadline !== ''
  return true
})

// Navigation methods
const nextStep = () => {
  if (currentStep.value === 0 && projectData.value.type) {
    // If we're on the first step and a project type is selected, route to the appropriate creation page
    if (projectData.value.type === 'blog') {
      router.push('/create-blog-project')
    } else if (projectData.value.type === 'book') {
      router.push('/create-book-project')
    }
  } else if (currentStep.value < steps.length - 1) {
    currentStep.value++
  }
}

const prevStep = () => {
  if (currentStep.value > 0) {
    currentStep.value--
  }
}

// Keyword methods
const addKeyword = () => {
  if (keywordInput.value.trim() && !projectData.value.keywords.includes(keywordInput.value.trim())) {
    projectData.value.keywords.push(keywordInput.value.trim())
    keywordInput.value = ''
  }
}

const removeKeyword = (index: number) => {
  projectData.value.keywords.splice(index, 1)
}

// Availability methods
const toggleAvailability = (day: string) => {
  const index = projectData.value.availability.indexOf(day)
  if (index === -1) {
    projectData.value.availability.push(day)
  } else {
    projectData.value.availability.splice(index, 1)
  }
}

// File upload methods
const triggerFileUpload = () => {
  fileInput.value?.click()
}

const handleFileUpload = (event: Event) => {
  const input = event.target as HTMLInputElement
  if (input.files) {
    for (let i = 0; i < input.files.length; i++) {
      const file = input.files[i]
      const fileType = file.name.split('.').pop()?.toLowerCase() || ''
      projectData.value.resources.push({
        name: file.name,
        type: fileType
      })
    }
  }
}

const removeResource = (index: number) => {
  projectData.value.resources.splice(index, 1)
}

// Format date for display
const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('en-US', { 
    year: 'numeric', 
    month: 'short', 
    day: 'numeric' 
  }).format(date)
}

// Create project
const createProject = () => {
  // Route to the appropriate project creation view based on project type
  if (projectData.value.type === 'blog') {
    router.push('/create-blog-project')
  } else if (projectData.value.type === 'book') {
    router.push('/create-book-project')
  }
}
</script>

<style scoped>
.new-project-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
  width: 100%;
  box-sizing: border-box;
}

h1 {
  margin-bottom: 2rem;
  font-size: 2rem;
  color: var(--color-text-primary);
  transition: color 0.3s;
}

/* Progress indicator */
.progress-indicator {
  display: flex;
  justify-content: space-between;
  margin-bottom: 3rem;
  position: relative;
  flex-wrap: wrap;
}

.progress-indicator::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 0;
  right: 0;
  height: 2px;
  background-color: var(--color-border-primary);
  z-index: 1;
}

.step {
  display: flex;
  flex-direction: column;
  align-items: center;
  position: relative;
  z-index: 2;
  flex: 1;
  min-width: 80px;
}

.step-number {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background-color: var(--color-bg-secondary);
  border: 2px solid var(--color-border-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  margin-bottom: 0.5rem;
  transition: all 0.3s;
}

.step.active .step-number {
  background-color: var(--color-primary);
  border-color: var(--color-primary);
  color: var(--color-text-on-primary);
}

.step.completed .step-number {
  background-color: var(--color-success);
  border-color: var(--color-success);
  color: var(--color-text-on-primary);
}

.step-label {
  font-size: 0.9rem;
  color: var(--color-text-secondary);
  transition: color 0.3s;
  text-align: center;
}

.step.active .step-label {
  color: var(--color-primary);
  font-weight: bold;
}

/* Step content */
.step-content {
  margin-bottom: 2rem;
}

.step-form {
  background-color: var(--color-bg-secondary);
  padding: 2rem;
  border-radius: 8px;
  box-shadow: var(--shadow-sm);
  transition: background-color 0.3s, box-shadow 0.3s;
}

h2 {
  margin-bottom: 1rem;
  font-size: 1.5rem;
  color: var(--color-text-primary);
  transition: color 0.3s;
}

.step-description {
  margin-bottom: 2rem;
  color: var(--color-text-secondary);
  line-height: 1.5;
  transition: color 0.3s;
}

/* Project type options */
.project-type-options {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
}

.project-type-card {
  background-color: var(--color-bg-tertiary);
  border: 2px solid var(--color-border-primary);
  border-radius: 8px;
  padding: 1.5rem;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s;
}

.project-type-card:hover {
  transform: translateY(-5px);
  box-shadow: var(--shadow-md);
}

.project-type-card.selected {
  border-color: var(--color-primary);
  background-color: var(--color-primary-light);
}

.type-icon {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.project-type-card h3 {
  margin-bottom: 0.5rem;
  color: var(--color-text-primary);
  transition: color 0.3s;
}

.project-type-card p {
  color: var(--color-text-secondary);
  font-size: 0.9rem;
  transition: color 0.3s;
}

/* Form groups */
.form-group {
  margin-bottom: 1.5rem;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: var(--color-text-primary);
  transition: color 0.3s;
}

input, textarea, select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--color-border-primary);
  border-radius: 4px;
  font-size: 1rem;
  transition: all 0.3s;
  box-sizing: border-box;
  background-color: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

input:focus, textarea:focus, select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px var(--color-primary-light);
}

/* Availability options */
.availability-options {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.day-option {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background-color: var(--color-bg-tertiary);
  border: 1px solid var(--color-border-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s;
  color: var(--color-text-primary);
}

.day-option:hover {
  background-color: var(--color-bg-hover);
}

.day-option.selected {
  background-color: var(--color-primary);
  border-color: var(--color-primary);
  color: var(--color-text-on-primary);
}

/* Keywords */
.keywords-input {
  display: flex;
  gap: 0.5rem;
}

.add-keyword-btn {
  padding: 0 1rem;
  background-color: var(--color-primary);
  color: var(--color-text-on-primary);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1.2rem;
  transition: background-color 0.3s;
  white-space: nowrap;
}

.add-keyword-btn:hover {
  background-color: var(--color-primary-dark);
}

.keywords-list {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-top: 0.5rem;
}

.keyword-tag {
  background-color: var(--color-primary-light);
  color: var(--color-primary);
  padding: 0.25rem 0.75rem;
  border-radius: 20px;
  font-size: 0.9rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: background-color 0.3s, color 0.3s;
}

.remove-keyword {
  cursor: pointer;
  font-size: 1.2rem;
  line-height: 1;
}

/* Resources upload */
.resources-upload {
  margin-top: 0.5rem;
}

.upload-area {
  border: 2px dashed var(--color-border-primary);
  border-radius: 8px;
  padding: 2rem;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s;
}

.upload-area:hover {
  border-color: var(--color-primary);
  background-color: var(--color-primary-light);
}

.upload-icon {
  font-size: 2rem;
  margin-bottom: 1rem;
}

.uploaded-files {
  margin-top: 1rem;
}

.resource-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.75rem;
  background-color: var(--color-bg-tertiary);
  border-radius: 4px;
  margin-bottom: 0.5rem;
  transition: background-color 0.3s;
  flex-wrap: wrap;
}

.resource-name {
  flex: 1;
  font-weight: 500;
  min-width: 150px;
  color: var(--color-text-primary);
}

.resource-type {
  color: var(--color-text-secondary);
  font-size: 0.9rem;
  transition: color 0.3s;
}

.remove-resource {
  background: none;
  border: none;
  color: var(--color-danger);
  cursor: pointer;
  font-size: 1.2rem;
}

/* Plan preview */
.plan-preview {
  background-color: var(--color-bg-tertiary);
  border-radius: 8px;
  padding: 1.5rem;
  transition: background-color 0.3s;
}

.plan-section {
  margin-bottom: 2rem;
}

.plan-section h3 {
  margin-bottom: 1rem;
  color: var(--color-text-primary);
  transition: color 0.3s;
}

.timeline {
  position: relative;
  padding-left: 2rem;
}

.timeline::before {
  content: '';
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  width: 2px;
  background-color: var(--color-border-primary);
  transition: background-color 0.3s;
}

.timeline-item {
  position: relative;
  margin-bottom: 1.5rem;
}

.timeline-item::before {
  content: '';
  position: absolute;
  top: 0.5rem;
  left: -2.4rem;
  width: 1rem;
  height: 1rem;
  border-radius: 50%;
  background-color: var(--color-primary);
}

.timeline-date {
  font-weight: 500;
  margin-bottom: 0.5rem;
  color: var(--color-text-primary);
  transition: color 0.3s;
}

.timeline-content h4 {
  margin-bottom: 0.5rem;
  color: var(--color-text-primary);
  transition: color 0.3s;
}

.timeline-content p {
  color: var(--color-text-secondary);
  transition: color 0.3s;
}

.action-items {
  display: grid;
  gap: 1rem;
}

.action-item {
  display: flex;
  gap: 1rem;
  padding: 1rem;
  background-color: var(--color-bg-secondary);
  border-radius: 4px;
  box-shadow: var(--shadow-sm);
  transition: background-color 0.3s, box-shadow 0.3s;
}

.action-checkbox {
  font-size: 1.2rem;
  color: var(--color-primary);
  flex-shrink: 0;
}

.action-content h4 {
  margin-bottom: 0.25rem;
  color: var(--color-text-primary);
  transition: color 0.3s;
}

.action-content p {
  color: var(--color-text-secondary);
  font-size: 0.9rem;
  transition: color 0.3s;
}

/* Navigation buttons */
.step-navigation {
  display: flex;
  justify-content: space-between;
  margin-top: 2rem;
  flex-wrap: wrap;
  gap: 1rem;
}

.nav-btn {
  padding: 0.75rem 1.5rem;
  border-radius: 4px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s;
  flex: 1;
  min-width: 120px;
  text-align: center;
}

.prev-btn {
  background-color: var(--color-bg-tertiary);
  border: 1px solid var(--color-border-primary);
  color: var(--color-text-primary);
}

.prev-btn:hover {
  background-color: var(--color-bg-hover);
}

.next-btn, .create-btn {
  background-color: var(--color-primary);
  border: none;
  color: var(--color-text-on-primary);
}

.next-btn:hover, .create-btn:hover {
  background-color: var(--color-primary-dark);
}

.next-btn:disabled {
  background-color: var(--color-text-tertiary);
  cursor: not-allowed;
}

/* Dark theme styles */
:global(body.dark) h1 {
  color: #e0e0e0;
}

:global(body.dark) .step-label {
  color: #aaa;
}

:global(body.dark) .step.active .step-label {
  color: #64b5f6;
}

:global(body.dark) .step-form {
  background-color: #1e1e1e;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

:global(body.dark) h2 {
  color: #e0e0e0;
}

:global(body.dark) .step-description {
  color: #aaa;
}

:global(body.dark) .project-type-card {
  background-color: #2c2c2c;
  border-color: #444;
}

:global(body.dark) .project-type-card.selected {
  border-color: #64b5f6;
  background-color: rgba(100, 181, 246, 0.1);
}

:global(body.dark) .project-type-card h3 {
  color: #e0e0e0;
}

:global(body.dark) .project-type-card p {
  color: #aaa;
}

:global(body.dark) label {
  color: #e0e0e0;
}

:global(body.dark) input, :global(body.dark) textarea, :global(body.dark) select {
  background-color: #2c2c2c;
  border-color: #444;
  color: #e0e0e0;
}

:global(body.dark) input:focus, :global(body.dark) textarea:focus, :global(body.dark) select:focus {
  border-color: #64b5f6;
  box-shadow: 0 0 0 2px rgba(100, 181, 246, 0.2);
}

:global(body.dark) .day-option {
  background-color: #2c2c2c;
  border-color: #444;
  color: #e0e0e0;
}

:global(body.dark) .day-option:hover {
  background-color: #3c3c3c;
}

:global(body.dark) .day-option.selected {
  background-color: #64b5f6;
  border-color: #64b5f6;
  color: #1e1e1e;
}

:global(body.dark) .keyword-tag {
  background-color: rgba(100, 181, 246, 0.2);
  color: #64b5f6;
}

:global(body.dark) .upload-area {
  border-color: #444;
}

:global(body.dark) .upload-area:hover {
  border-color: #64b5f6;
  background-color: rgba(100, 181, 246, 0.05);
}

:global(body.dark) .resource-item {
  background-color: #2c2c2c;
}

:global(body.dark) .resource-type {
  color: #aaa;
}

:global(body.dark) .plan-preview {
  background-color: #2c2c2c;
}

:global(body.dark) .plan-section h3 {
  color: #e0e0e0;
}

:global(body.dark) .timeline::before {
  background-color: #444;
}

:global(body.dark) .timeline-item::before {
  background-color: #64b5f6;
}

:global(body.dark) .timeline-date {
  color: #e0e0e0;
}

:global(body.dark) .timeline-content h4 {
  color: #e0e0e0;
}

:global(body.dark) .timeline-content p {
  color: #aaa;
}

:global(body.dark) .action-item {
  background-color: #1e1e1e;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}

:global(body.dark) .action-checkbox {
  color: #64b5f6;
}

:global(body.dark) .action-content h4 {
  color: #e0e0e0;
}

:global(body.dark) .action-content p {
  color: #aaa;
}

:global(body.dark) .prev-btn {
  background-color: #2c2c2c;
  border-color: #444;
  color: #e0e0e0;
}

:global(body.dark) .prev-btn:hover {
  background-color: #3c3c3c;
}

/* Responsive styles */
@media (max-width: 1200px) {
  .new-project-container {
    padding: 1.75rem;
    max-width: 95%;
  }
  
  h1 {
    font-size: 1.75rem;
  }
  
  .step-form {
    padding: 1.75rem;
  }

  .progress-indicator {
    gap: 1rem;
  }

  .step {
    min-width: 100px;
  }
}

@media (max-width: 992px) {
  .new-project-container {
    padding: 1.5rem;
    max-width: 100%;
  }
  
  h1 {
    font-size: 1.5rem;
  }
  
  .step-form {
    padding: 1.5rem;
  }
  
  .step-number {
    width: 36px;
    height: 36px;
  }
  
  .step-label {
    font-size: 0.8rem;
  }
  
  h2 {
    font-size: 1.3rem;
  }
  
  .form-group {
    margin-bottom: 1.25rem;
  }
}

@media (max-width: 768px) {
  .new-project-container {
    padding: 1.25rem;
  }
  
  h1 {
    font-size: 1.3rem;
    margin-bottom: 1.5rem;
  }
  
  .progress-indicator {
    margin-bottom: 2rem;
    flex-wrap: wrap;
    justify-content: center;
    gap: 1.5rem;
  }

  .progress-indicator::before {
    display: none;
  }
  
  .step-form {
    padding: 1.25rem;
  }
  
  .step-number {
    width: 32px;
    height: 32px;
    font-size: 0.9rem;
  }
  
  .step-label {
    font-size: 0.75rem;
  }
  
  h2 {
    font-size: 1.2rem;
  }
  
  .step-description {
    font-size: 0.9rem;
  }
  
  .day-option {
    width: 36px;
    height: 36px;
  }
  
  .upload-area {
    padding: 1.5rem;
  }
  
  .plan-preview {
    padding: 1.25rem;
  }
  
  .timeline {
    padding-left: 1.5rem;
  }
  
  .timeline-item::before {
    left: -1.9rem;
  }
  
  .action-item {
    padding: 0.75rem;
  }

  .keywords-input {
    flex-direction: column;
  }

  .add-keyword-btn {
    width: 100%;
    margin-top: 0.5rem;
  }
}

@media (max-width: 576px) {
  .new-project-container {
    padding: 1rem;
  }
  
  h1 {
    font-size: 1.2rem;
    margin-bottom: 1.25rem;
  }
  
  .progress-indicator {
    margin-bottom: 1.5rem;
    gap: 1rem;
  }
  
  .step {
    min-width: 60px;
  }
  
  .step-number {
    width: 28px;
    height: 28px;
    font-size: 0.8rem;
  }
  
  .step-label {
    font-size: 0.7rem;
  }
  
  .step-form {
    padding: 1rem;
    border-radius: 6px;
  }
  
  h2 {
    font-size: 1.1rem;
  }
  
  .step-description {
    font-size: 0.85rem;
    margin-bottom: 1.5rem;
  }
  
  .form-group {
    margin-bottom: 1.25rem;
  }
  
  input, textarea, select {
    padding: 0.6rem;
    font-size: 0.9rem;
  }
  
  .day-option {
    width: 32px;
    height: 32px;
    font-size: 0.8rem;
  }
  
  .add-keyword-btn {
    padding: 0 0.75rem;
    font-size: 1rem;
  }
  
  .keyword-tag {
    font-size: 0.8rem;
    padding: 0.2rem 0.6rem;
  }
  
  .upload-area {
    padding: 1.25rem;
  }
  
  .upload-icon {
    font-size: 1.5rem;
  }
  
  .resource-item {
    padding: 0.6rem;
    flex-direction: column;
    align-items: flex-start;
  }
  
  .resource-name {
    min-width: 120px;
  }
  
  .plan-preview {
    padding: 1rem;
    border-radius: 6px;
  }
  
  .timeline {
    padding-left: 1.25rem;
  }
  
  .timeline-item::before {
    left: -1.65rem;
    width: 0.75rem;
    height: 0.75rem;
  }
  
  .timeline-date {
    font-size: 0.9rem;
  }
  
  .timeline-content h4 {
    font-size: 0.9rem;
  }
  
  .timeline-content p {
    font-size: 0.8rem;
  }
  
  .action-item {
    padding: 0.6rem;
    gap: 0.75rem;
  }
  
  .action-checkbox {
    font-size: 1rem;
  }
  
  .action-content h4 {
    font-size: 0.9rem;
  }
  
  .action-content p {
    font-size: 0.8rem;
  }
  
  .step-navigation {
    margin-top: 1.5rem;
    flex-direction: column;
    gap: 0.75rem;
  }
  
  .nav-btn {
    padding: 0.6rem 1.25rem;
    font-size: 0.9rem;
    width: 100%;
  }
}

/* Add styles for very small devices */
@media (max-width: 360px) {
  .new-project-container {
    padding: 0.75rem;
  }

  .step {
    min-width: 50px;
  }

  .step-number {
    width: 24px;
    height: 24px;
    font-size: 0.7rem;
  }

  .step-label {
    font-size: 0.65rem;
  }

  .day-option {
    width: 28px;
    height: 28px;
    font-size: 0.7rem;
  }

  .keyword-tag {
    font-size: 0.75rem;
    padding: 0.15rem 0.5rem;
  }
}

/* Add styles for landscape orientation on mobile devices */
@media (max-height: 600px) and (orientation: landscape) {
  .new-project-container {
    padding: 1rem;
  }

  .progress-indicator {
    margin-bottom: 1rem;
  }

  .step-form {
    padding: 1rem;
  }

  .form-group {
    margin-bottom: 1rem;
  }

  textarea {
    min-height: 60px;
  }
}
</style> 