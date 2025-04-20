<template>
  <TwoColumnLayout>
    <div class="new-project-container">
      <h1>{{ t('newProject.title') }} - {{ t('project.type.book') }}</h1>
      
      <!-- Progress indicator -->
      <div class="progress-indicator">
        <div 
          v-for="(step, index) in steps" 
          :key="index" 
          class="step" 
          :class="{ active: currentStep === index, completed: currentStep > index }"
        >
          <div class="step-number">
            <span v-if="currentStep > index">✓</span>
            <span v-else>{{ index + 1 }}</span>
          </div>
          <div class="step-label">{{ t(`newProject.steps.${step}`) }}</div>
        </div>
      </div>
      
      <!-- Step content -->
      <div class="step-content">
        <!-- Step 1: Goal Setting -->
        <div v-if="currentStep === 0" class="step-form">
          <div class="step-form-header">
            <h2>Transform Your Story into Reality</h2>
            <p class="step-description">Every great book begins with a dream. Whether you're crafting a thrilling novel, sharing valuable knowledge, or telling your life story, this is where your author's journey begins. Let's turn your literary vision into a masterpiece that touches hearts and minds.</p>
            <div class="required-field-hint">{{ t('newProject.requiredFields') }}</div>
          </div>
          
          <div class="validation-status" v-if="!projectData.goal" :class="{ error: showErrors }">
            <span class="validation-status-icon">ℹ️</span>
            <span class="validation-message">{{ t('newProject.goals.validationMessage') }}</span>
          </div>
          
          <div class="form-group required" :class="{ 'has-error': showErrors && !projectData.goal }">
            <div class="goal-header">
              <label for="project-goal" :title="t('newProject.tooltips.goal')">
                {{ t('newProject.goals.mainGoal') }}
              </label>
              <button class="inspiration-btn" @click="openInspirationModal">
                <span class="inspiration-icon">💡</span>
                {{ t('newProject.goals.inspirationTitle') }}
              </button>
            </div>
            <textarea 
              id="project-goal" 
              v-model="projectData.goal" 
              placeholder="Share your book's purpose and vision. For example:
• I want to write a compelling fantasy novel that transports readers to a unique world
• I aim to create a practical guide that helps people master personal finance
• My goal is to share my life experiences through a memoir that inspires others
• I want to write a children's book series that sparks imagination and creativity"
              rows="8"
            ></textarea>
            <div class="form-hint">{{ t('newProject.goals.goalHint') }}</div>
            <div class="error-message" v-if="showErrors && !projectData.goal">
              {{ t('newProject.goals.errorMessage') }}
            </div>
          </div>
        </div>
        
        <!-- Goal Inspiration Modal -->
        <div v-if="showInspirationModal" class="inspiration-modal">
          <div class="inspiration-modal-content">
            <h3>{{ t('newProject.goals.inspirationTitle') }}</h3>
            <p class="inspiration-prompt">{{ selectedPrompt }}</p>
            <div class="inspiration-actions">
              <button class="secondary-btn" @click="closeInspirationModal">
                {{ t('newProject.goals.inspirationCancel') }}
              </button>
              <button class="secondary-btn" @click="getAnotherPrompt">
                Another
              </button>
              <button class="primary-btn" @click="applyPromptToGoal">
                {{ t('newProject.goals.inspirationApply') }}
              </button>
            </div>
          </div>
        </div>
        
        <!-- Step 2: Book Details -->
        <div v-if="currentStep === 1" class="step-form">
          <div class="step-form-header">
            <h2>{{ t('newProject.details.title') }}</h2>
            <p class="step-description">{{ t('newProject.details.description-long') }}</p>
            <div class="required-field-hint">{{ t('newProject.requiredFields') }}</div>
          </div>
          
          <div class="validation-status" v-if="!isStep2Valid" :class="{ error: showErrors }">
            <span class="validation-status-icon">ℹ️</span>
            <span class="validation-message">{{ t('newProject.details.validationMessage') }}</span>
          </div>
          
          <div class="form-columns">
            <div class="form-group required" :class="{ 'has-error': showErrors && !projectData.title }">
              <label for="project-title" :title="t('newProject.tooltips.title')">
                {{ t('newProject.details.projectTitle') }}
              </label>
              <input 
                id="project-title" 
                v-model="projectData.title" 
                type="text" 
                :placeholder="t('newProject.details.titlePlaceholder')"
              />
              <div class="error-message" v-if="showErrors && !projectData.title">
                {{ t('newProject.details.titleError') }}
              </div>
            </div>
            
            <div class="form-group required" :class="{ 'has-error': showErrors && !projectData.genre }">
              <label for="book-genre" :title="t('newProject.tooltips.genre')">
                {{ t('newProject.book.genre') }}
              </label>
              <select id="book-genre" v-model="projectData.genre">
                <option value="" disabled selected>{{ t('newProject.book.selectGenre') }}</option>
                
                <!-- Fiction Categories -->
                <optgroup :label="t('newProject.book.genreCategories.fiction')">
                  <option value="fiction.romance">{{ t('newProject.book.genres.fiction.romance') }}</option>
                  <option value="fiction.scienceFiction">{{ t('newProject.book.genres.fiction.scienceFiction') }}</option>
                  <option value="fiction.fantasy">{{ t('newProject.book.genres.fiction.fantasy') }}</option>
                  <option value="fiction.mystery">{{ t('newProject.book.genres.fiction.mystery') }}</option>
                  <option value="fiction.horror">{{ t('newProject.book.genres.fiction.horror') }}</option>
                  <option value="fiction.historical">{{ t('newProject.book.genres.fiction.historical') }}</option>
                  <option value="fiction.literary">{{ t('newProject.book.genres.fiction.literary') }}</option>
                  <option value="fiction.contemporary">{{ t('newProject.book.genres.fiction.contemporary') }}</option>
                  <option value="fiction.action">{{ t('newProject.book.genres.fiction.action') }}</option>
                  <option value="fiction.classics">{{ t('newProject.book.genres.fiction.classics') }}</option>
                  <option value="fiction.other">{{ t('newProject.book.genres.fiction.other') }}</option>
                </optgroup>

                <!-- Nonfiction Categories -->
                <optgroup :label="t('newProject.book.genreCategories.nonfiction')">
                  <option value="nonfiction.biography">{{ t('newProject.book.genres.nonfiction.biography') }}</option>
                  <option value="nonfiction.business">{{ t('newProject.book.genres.nonfiction.business') }}</option>
                  <option value="nonfiction.selfHelp">{{ t('newProject.book.genres.nonfiction.selfHelp') }}</option>
                  <option value="nonfiction.history">{{ t('newProject.book.genres.nonfiction.history') }}</option>
                  <option value="nonfiction.science">{{ t('newProject.book.genres.nonfiction.science') }}</option>
                  <option value="nonfiction.technology">{{ t('newProject.book.genres.nonfiction.technology') }}</option>
                  <option value="nonfiction.philosophy">{{ t('newProject.book.genres.nonfiction.philosophy') }}</option>
                  <option value="nonfiction.politics">{{ t('newProject.book.genres.nonfiction.politics') }}</option>
                  <option value="nonfiction.psychology">{{ t('newProject.book.genres.nonfiction.psychology') }}</option>
                  <option value="nonfiction.education">{{ t('newProject.book.genres.nonfiction.education') }}</option>
                  <option value="nonfiction.reference">{{ t('newProject.book.genres.nonfiction.reference') }}</option>
                  <option value="nonfiction.other">{{ t('newProject.book.genres.nonfiction.other') }}</option>
                </optgroup>

                <!-- Specialty Categories -->
                <optgroup :label="t('newProject.book.genreCategories.specialty')">
                  <option value="specialty.children">{{ t('newProject.book.genres.specialty.children') }}</option>
                  <option value="specialty.youngAdult">{{ t('newProject.book.genres.specialty.youngAdult') }}</option>
                  <option value="specialty.comics">{{ t('newProject.book.genres.specialty.comics') }}</option>
                  <option value="specialty.cookbooks">{{ t('newProject.book.genres.specialty.cookbooks') }}</option>
                  <option value="specialty.art">{{ t('newProject.book.genres.specialty.art') }}</option>
                  <option value="specialty.travel">{{ t('newProject.book.genres.specialty.travel') }}</option>
                  <option value="specialty.religion">{{ t('newProject.book.genres.specialty.religion') }}</option>
                  <option value="specialty.health">{{ t('newProject.book.genres.specialty.health') }}</option>
                  <option value="specialty.sports">{{ t('newProject.book.genres.specialty.sports') }}</option>
                  <option value="specialty.crafts">{{ t('newProject.book.genres.specialty.crafts') }}</option>
                  <option value="specialty.parenting">{{ t('newProject.book.genres.specialty.parenting') }}</option>
                  <option value="specialty.lgbtqia">{{ t('newProject.book.genres.specialty.lgbtqia') }}</option>
                  <option value="specialty.other">{{ t('newProject.book.genres.specialty.other') }}</option>
                </optgroup>

                <!-- Other -->
                <option value="other">{{ t('newProject.book.genres.other') }}</option>
              </select>
              <div class="error-message" v-if="showErrors && !projectData.genre">
                {{ t('newProject.book.genreError') }}
              </div>
            </div>
          </div>
          
          <div class="form-group required" :class="{ 'has-error': showErrors && !projectData.description }">
            <div class="description-header">
              <label for="project-description" :title="t('newProject.book.tooltips.description')">
                {{ t('newProject.details.description') }}
              </label>
              <button class="sample-btn" @click="openSampleModal">
                <span class="sample-icon">📋</span>
                {{ t('newProject.details.sampleButton') }}
              </button>
            </div>
            <textarea 
              id="project-description" 
              v-model="projectData.description" 
              :placeholder="t('newProject.details.descriptionPlaceholder')"
              rows="4"
            ></textarea>
            <div class="form-hint">{{ t('newProject.book.tooltips.description') }}</div>
            <div class="error-message" v-if="showErrors && !projectData.description">
              {{ t('newProject.details.descriptionError') }}
            </div>
          </div>
          
          <div class="form-group">
            <label for="target-audience">{{ t('newProject.resources.targetAudience') }}</label>
            <textarea 
              id="target-audience" 
              v-model="projectData.targetAudience" 
              :placeholder="t('newProject.resources.targetAudiencePlaceholder')"
              rows="3"
            ></textarea>
          </div>
          
          <div class="form-group">
            <label for="keywords" :title="t('newProject.book.tooltips.keywords')">
              {{ t('newProject.resources.keywords') }}
            </label>
            <div class="keywords-input">
              <input 
                id="keywords" 
                v-model="keywordInput" 
                type="text" 
                :placeholder="t('newProject.resources.keywordsPlaceholder')"
                @keydown.enter.prevent="addKeyword"
              />
              <button class="add-keyword-btn" @click="addKeyword">
                <span class="btn-icon">+</span>
              </button>
            </div>
            <div class="form-hint">{{ t('newProject.book.tooltips.keywords') }}</div>
            <div class="keywords-list" v-if="projectData.keywords.length > 0">
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
        </div>
        
        <!-- Description Sample Modal -->
        <div v-if="showSampleModal" class="sample-modal">
          <div class="sample-modal-content">
            <h3>{{ t('newProject.details.sampleTitle') }}</h3>
            <p class="sample-prompt">{{ t(`newProject.book.samples.${selectedCategory}.${selectedSampleKey}`) }}</p>
            <div class="sample-actions">
              <button class="secondary-btn" @click="closeSampleModal">
                {{ t('newProject.details.sampleCancel') }}
              </button>
              <button class="secondary-btn" @click="getAnotherSample">
                {{ t('newProject.details.sampleAnother') }}
              </button>
              <button class="primary-btn" @click="applySampleToDescription">
                {{ t('newProject.details.sampleApply') }}
              </button>
            </div>
          </div>
        </div>
        
        <!-- Step 3: Time Commitment -->
        <div v-if="currentStep === 2" class="step-form">
          <h2>{{ t('newProject.time.title') }}</h2>
          <p class="step-description">{{ t('newProject.time.description') }}</p>
          
          <div class="form-group">
            <label for="time-commitment">{{ t('newProject.time.commitment') }}</label>
            <select id="time-commitment" v-model="projectData.timeCommitment">
              <option value="casual">{{ t('common.timeCommitment.casual') }}</option>
              <option value="partTime">{{ t('common.timeCommitment.partTime') }}</option>
              <option value="fullTime">{{ t('common.timeCommitment.fullTime') }}</option>
            </select>
          </div>
          
          <div class="form-group">
            <label for="project-duration">{{ t('newProject.time.duration') }}</label>
            <div class="duration-container">
              <select id="project-duration" v-model="projectData.duration" @change="updateDeadline">
                <option value="90">{{ t('newProject.time.durations.days90') }}</option>
                <option value="180">{{ t('newProject.time.durations.days180') }}</option>
                <option value="365">{{ t('newProject.time.durations.days365') }}</option>
              </select>
            </div>
          </div>
          
          <div class="form-group">
            <label>{{ t('common.availability.label') }}</label>
            <div class="availability-options">
              <button 
                v-for="day in weekDays" 
                :key="day.value"
                class="day-option"
                :class="{ selected: projectData.availability.includes(day.value) }"
                @click="toggleAvailability(day.value)"
              >
                {{ t(`common.availability.daysShort.${day.value}`) }}
              </button>
            </div>
          </div>
          
          <div class="time-commitment-display" v-if="projectData.availability.length > 0">
            <div class="commitment-message">
              {{ t('newProject.time.commitmentMessage', { hours: monthlyCommitment }) }}
            </div>
            <div class="deadline-display">
              {{ t('newProject.time.deadline') }}: {{ formattedDeadline }}
            </div>
          </div>
        </div>
        
        <!-- Step 4: Book Plan -->
        <div v-if="currentStep === 3" class="step-form">
          <h2>{{ t('newProject.plan.title') }}</h2>
          <p class="step-description">{{ t('newProject.plan.description') }}</p>
          
          <div class="project-summary">
            <h3>{{ t('newProject.plan.summary') }}</h3>
            <div class="summary-content">
              <div class="summary-item">
                <span class="summary-label">{{ t('newProject.goals.mainGoal') }}:</span>
                <span class="summary-value">{{ projectData.goal }}</span>
              </div>
              <div class="summary-item">
                <span class="summary-label">{{ t('newProject.details.projectTitle') }}:</span>
                <span class="summary-value">{{ projectData.title }}</span>
              </div>
              <div class="summary-item">
                <span class="summary-label">{{ t('newProject.details.description') }}:</span>
                <span class="summary-value">{{ projectData.description }}</span>
              </div>
            </div>
          </div>
          
          <div class="notification-option">
            <label class="checkbox-label">
              <input 
                type="checkbox" 
                v-model="projectData.receiveNotifications" 
                checked
              />
              <span>{{ t('newProject.notifications.reminder') }}</span>
            </label>
          </div>
          
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
                <div class="action-item">
                  <div class="action-checkbox">☐</div>
                  <div class="action-content">
                    <h4>{{ t('newProject.plan.publish') }}</h4>
                    <p>{{ t('newProject.plan.publishDescription') }}</p>
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
          <span class="btn-icon">←</span>
          {{ t('newProject.navigation.prev') }}
        </button>
        <button 
          v-if="currentStep < steps.length - 1" 
          class="nav-btn next-btn" 
          @click="nextStep"
          :disabled="!canProceed"
          :data-error="getStepErrorMessage"
        >
          {{ t('newProject.navigation.next') }}
          <span class="btn-icon">→</span>
        </button>
        <button 
          v-if="currentStep === steps.length - 1" 
          class="nav-btn create-btn" 
          @click="createProject"
          :disabled="!isFormValid"
          :data-error="getFormErrorMessage"
        >
          {{ t('newProject.navigation.create') }}
          <span class="btn-icon">✓</span>
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
import { saveProject, saveProjectContent } from '../lib/project'
import '../styles/CreateBookProject.css'

const { t } = useI18n()
const router = useRouter()
const fileInput = ref<HTMLInputElement | null>(null)

// Project creation steps
const steps = ['goals', 'details', 'time', 'plan']
const currentStep = ref(0)

// Project data
const projectData = ref({
  type: 'book',
  goal: '',
  title: '',
  description: '',
  genre: '',
  timeCommitment: 'casual',
  duration: '90', // Default to 90 days
  deadline: '',
  availability: [] as string[],
  keywords: [] as string[],
  targetAudience: '',
  structure: 'standard',
  receiveNotifications: true
})

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

// Goal inspiration data
const goalInspirationPrompts = [
  "I want to write a book that inspires others to pursue their dreams.",
  "I want to share my unique life experiences through storytelling.",
  "I want to create a comprehensive guide that helps people solve specific problems.",
  "I want to write a novel that entertains and makes people think.",
  "I want to document important historical events or cultural phenomena.",
  "I want to create a book series that captures readers' imagination.",
  "I want to share my expertise and knowledge in my field.",
  "I want to write a memoir that connects with readers on an emotional level.",
  "I want to create a self-help book that transforms people's lives.",
  "I want to write a book that challenges conventional thinking."
]

// Goal inspiration methods
const showInspirationModal = ref(false)
const selectedPrompt = ref('')

const openInspirationModal = () => {
  showInspirationModal.value = true
  selectedPrompt.value = goalInspirationPrompts[Math.floor(Math.random() * goalInspirationPrompts.length)]
}

const getAnotherPrompt = () => {
  let newPrompt
  do {
    newPrompt = goalInspirationPrompts[Math.floor(Math.random() * goalInspirationPrompts.length)]
  } while (newPrompt === selectedPrompt.value && goalInspirationPrompts.length > 1)
  selectedPrompt.value = newPrompt
}

const closeInspirationModal = () => {
  showInspirationModal.value = false
}

const applyPromptToGoal = () => {
  if (projectData.value.goal === '') {
    projectData.value.goal = selectedPrompt.value + '\n'
  } else {
    projectData.value.goal += `\n\n${selectedPrompt.value}\n\n`
  }
  closeInspirationModal()
}

// Description sample data
type SampleCategory = 'fiction' | 'nonfiction' | 'specialty'
type SampleKeys = {
  [K in SampleCategory]: string[]
}

const descriptionSampleKeys: SampleKeys = {
  fiction: ['fantasy', 'mystery', 'romance', 'literary', 'historical'],
  nonfiction: ['selfHelp', 'business', 'science', 'biography'],
  specialty: ['children', 'cookbook', 'youngAdult', 'health']
}

// Description sample methods
const showSampleModal = ref(false)
const selectedCategory = ref<SampleCategory>('fiction')
const selectedSampleKey = ref('')

const openSampleModal = () => {
  showSampleModal.value = true
  // Randomly select a category first
  const categories = Object.keys(descriptionSampleKeys) as SampleCategory[]
  selectedCategory.value = categories[Math.floor(Math.random() * categories.length)]
  // Then randomly select a sample from that category
  const samples = descriptionSampleKeys[selectedCategory.value]
  selectedSampleKey.value = samples[Math.floor(Math.random() * samples.length)]
}

const getAnotherSample = () => {
  let newCategory: SampleCategory
  let newSampleKey: string
  do {
    const categories = Object.keys(descriptionSampleKeys) as SampleCategory[]
    newCategory = categories[Math.floor(Math.random() * categories.length)]
    const samples = descriptionSampleKeys[newCategory]
    newSampleKey = samples[Math.floor(Math.random() * samples.length)]
  } while (
    (newCategory === selectedCategory.value && newSampleKey === selectedSampleKey.value) && 
    Object.keys(descriptionSampleKeys).length > 1
  )
  selectedCategory.value = newCategory
  selectedSampleKey.value = newSampleKey
}

const closeSampleModal = () => {
  showSampleModal.value = false
}

const applySampleToDescription = () => {
  projectData.value.description = t(`newProject.book.samples.${selectedCategory.value}.${selectedSampleKey.value}`)
  closeSampleModal()
}

// Keyword input
const keywordInput = ref('')

// Check if can proceed to next step
const canProceed = computed(() => {
  if (currentStep.value === 0) {
    return projectData.value.goal !== ''
  }
  if (currentStep.value === 1) {
    return projectData.value.title !== '' && 
           projectData.value.description !== '' && 
           projectData.value.genre !== ''
  }
  if (currentStep.value === 2) {
    return projectData.value.availability.length > 0
  }
  return true
})

// Navigation methods
const nextStep = () => {
  showErrors.value = true
  if (canProceed.value) {
    currentStep.value++
    showErrors.value = false
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

// Format date for display
const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('en-US', { 
    year: 'numeric', 
    month: 'short', 
    day: 'numeric' 
  }).format(date)
}

// Calculate and update deadline based on duration
const updateDeadline = () => {
  const today = new Date()
  const durationDays = parseInt(projectData.value.duration)
  const deadlineDate = new Date(today)
  deadlineDate.setDate(today.getDate() + durationDays)
  projectData.value.deadline = deadlineDate.toISOString().split('T')[0]
}

// Formatted deadline for display
const formattedDeadline = computed(() => {
  if (!projectData.value.deadline) {
    updateDeadline() // Initialize if not set
  }
  const date = new Date(projectData.value.deadline)
  return formatDate(date)
})

// Initialize deadline on component mount
onMounted(() => {
  updateDeadline()
})

// Calculate monthly commitment based on availability and time commitment
const monthlyCommitment = computed(() => {
  let hoursPerDay = 1 // Default for casual
  
  if (projectData.value.timeCommitment === 'casual') {
    hoursPerDay = 1
  } else if (projectData.value.timeCommitment === 'partTime') {
    hoursPerDay = 3
  } else if (projectData.value.timeCommitment === 'fullTime') {
    hoursPerDay = 5
  }
  
  const daysPerWeek = projectData.value.availability.length
  const weeksPerMonth = 4
  
  const totalHoursPerMonth = daysPerWeek * hoursPerDay * weeksPerMonth
  
  return Math.round(totalHoursPerMonth)
})

const showErrors = ref(false)

// Validation computed properties
const isStep2Valid = computed(() => {
  return projectData.value.title && 
         projectData.value.description && 
         projectData.value.genre
})

const isFormValid = computed(() => {
  return projectData.value.goal &&
         projectData.value.title &&
         projectData.value.description &&
         projectData.value.genre &&
         projectData.value.availability.length > 0
})

const getStepErrorMessage = computed(() => {
  if (currentStep.value === 0 && !projectData.value.goal) {
    return t('newProject.goals.errorMessage')
  }
  if (currentStep.value === 1 && !isStep2Valid.value) {
    return t('newProject.details.validationMessage')
  }
  if (currentStep.value === 2 && projectData.value.availability.length === 0) {
    return t('newProject.time.availabilityError')
  }
  return ''
})

const getFormErrorMessage = computed(() => {
  if (!isFormValid.value) {
    return t('newProject.form.incompleteError')
  }
  return ''
})

// Create project
const createProject = async () => {
  showErrors.value = true
  if (!isFormValid.value) {
    return
  }
  
  try {
    console.log('Starting project creation process...')
    
    // Prepare the project data
    const project = {
      title: projectData.value.title,
      type_: projectData.value.type,
      description: projectData.value.description,
      genre: projectData.value.genre,
      time_commitment: projectData.value.timeCommitment,
      deadline: projectData.value.deadline,
      availability: JSON.stringify(projectData.value.availability),
      keywords: JSON.stringify(projectData.value.keywords),
      target_audience: projectData.value.targetAudience,
      structure: projectData.value.structure,
      goal: projectData.value.goal,
      receive_notifications: projectData.value.receiveNotifications
    }
    
    console.log('Formatted project data for database:', project)
    
    // Save the project
    const projectId = await saveProject(project)
    console.log(`Project saved with ID: ${projectId}`)
    
    if (projectId !== -1) {
      // Navigate to the project page
      router.push('/projects')
    } else {
      console.error('Failed to save project: Invalid project ID returned')
      alert('Failed to create project. Please try again.')
    }
  } catch (error) {
    console.error('Error creating project:', error)
    alert('An error occurred while creating the project. Please try again.')
  }
}
</script>

<style scoped>
.new-project-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
}

h1 {
  margin-bottom: 2rem;
  font-size: 2rem;
  color: #2c3e50;
  transition: color 0.3s;
}

/* Progress indicator */
.progress-indicator {
  display: flex;
  justify-content: space-between;
  margin-bottom: 3rem;
  position: relative;
}

.progress-indicator::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 0;
  right: 0;
  height: 2px;
  background-color: #e0e0e0;
  z-index: 1;
}

.step {
  display: flex;
  flex-direction: column;
  align-items: center;
  position: relative;
  z-index: 2;
}

.step-number {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background-color: #f8f9fa;
  border: 2px solid #e0e0e0;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  margin-bottom: 0.5rem;
  transition: all 0.3s;
}

.step.active .step-number {
  background-color: #2196F3;
  border-color: #2196F3;
  color: white;
}

.step.completed .step-number {
  background-color: #4CAF50;
  border-color: #4CAF50;
  color: white;
}

.step-label {
  font-size: 0.9rem;
  color: #666;
  transition: color 0.3s;
}

.step.active .step-label {
  color: #2196F3;
  font-weight: bold;
}

/* Step content */
.step-content {
  margin-bottom: 2rem;
}

.step-form {
  background-color: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  transition: background-color 0.3s, box-shadow 0.3s;
}

h2 {
  margin-bottom: 1rem;
  font-size: 1.5rem;
  color: #2c3e50;
  transition: color 0.3s;
}

.step-description {
  margin-bottom: 2rem;
  color: #666;
  line-height: 1.5;
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
  color: #2c3e50;
  transition: color 0.3s;
}

input, textarea, select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
  transition: all 0.3s;
}

input:focus, textarea:focus, select:focus {
  outline: none;
  border-color: #2196F3;
  box-shadow: 0 0 0 2px rgba(33, 150, 243, 0.2);
}

/* Availability options */
.availability-options {
  display: flex;
  gap: 0.5rem;
}

.day-option {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background-color: #f8f9fa;
  border: 1px solid #ddd;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s;
}

.day-option:hover {
  background-color: #e9ecef;
}

.day-option.selected {
  background-color: #2196F3;
  border-color: #2196F3;
  color: white;
}

/* Keywords */
.keywords-input {
  display: flex;
  gap: 0.5rem;
}

.add-keyword-btn {
  padding: 0 1rem;
  background-color: #2196F3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1.2rem;
  transition: background-color 0.3s;
}

.add-keyword-btn:hover {
  background-color: #1976D2;
}

.keywords-list {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-top: 0.5rem;
}

.keyword-tag {
  background-color: #e3f2fd;
  color: #1976D2;
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
  border: 2px dashed #ddd;
  border-radius: 8px;
  padding: 2rem;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s;
}

.upload-area:hover {
  border-color: #2196F3;
  background-color: rgba(33, 150, 243, 0.05);
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
  background-color: #f8f9fa;
  border-radius: 4px;
  margin-bottom: 0.5rem;
  transition: background-color 0.3s;
}

.resource-name {
  flex: 1;
  font-weight: 500;
}

.resource-type {
  color: #666;
  font-size: 0.9rem;
  transition: color 0.3s;
}

.remove-resource {
  background: none;
  border: none;
  color: #dc3545;
  cursor: pointer;
  font-size: 1.2rem;
}

/* Plan preview */
.plan-preview {
  background-color: #f8f9fa;
  border-radius: 8px;
  padding: 1.5rem;
  transition: background-color 0.3s;
}

.plan-section {
  margin-bottom: 2rem;
}

.plan-section h3 {
  margin-bottom: 1rem;
  color: #2c3e50;
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
  background-color: #e0e0e0;
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
  background-color: #2196F3;
}

.timeline-date {
  font-weight: 500;
  margin-bottom: 0.5rem;
  color: #2c3e50;
  transition: color 0.3s;
}

.timeline-content h4 {
  margin-bottom: 0.5rem;
  color: #2c3e50;
  transition: color 0.3s;
}

.timeline-content p {
  color: #666;
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
  background-color: white;
  border-radius: 4px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: background-color 0.3s, box-shadow 0.3s;
}

.action-checkbox {
  font-size: 1.2rem;
  color: #2196F3;
}

.action-content h4 {
  margin-bottom: 0.25rem;
  color: #2c3e50;
  transition: color 0.3s;
}

.action-content p {
  color: #666;
  font-size: 0.9rem;
  transition: color 0.3s;
}

/* Navigation buttons */
.step-navigation {
  display: flex;
  justify-content: space-between;
  margin-top: 2rem;
}

.nav-btn {
  padding: 0.75rem 1.5rem;
  border-radius: 4px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s;
}

.prev-btn {
  background-color: #f8f9fa;
  border: 1px solid #ddd;
  color: #2c3e50;
}

.prev-btn:hover {
  background-color: #e9ecef;
}

.next-btn, .create-btn {
  background-color: #2196F3;
  border: none;
  color: white;
}

.next-btn:hover, .create-btn:hover {
  background-color: #1976D2;
}

.next-btn:disabled {
  background-color: #b0bec5;
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
  
  .availability-options {
    flex-wrap: wrap;
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