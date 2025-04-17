<template>
  <TwoColumnLayout>
    <div class="new-project-container">
      <h1>{{ t('newProject.title') }} - {{ t('project.type.blog') }}</h1>
      
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
            <h2>{{ t('newProject.goals.title') }}</h2>
            <p class="step-description">{{ t('newProject.goals.description') }}</p>
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
              :placeholder="t('newProject.goals.goalPlaceholder')"
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
        
        <!-- Step 2: Blog Details -->
        <div v-if="currentStep === 1" class="step-form">
          <div class="step-form-header">
            <h2>{{ t('newProject.details.title') }}</h2>
            <p class="step-description">{{ t('newProject.details.description') }}</p>
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
            
            <div class="form-group required" :class="{ 'has-error': showErrors && !projectData.category }">
              <label for="blog-category" :title="t('newProject.tooltips.category')">
                {{ t('newProject.blog.category') }}
              </label>
              <select id="blog-category" v-model="projectData.category">
                <option value="" disabled selected>{{ t('newProject.blog.selectCategory') }}</option>
                <option value="technology">{{ t('newProject.blog.categories.technology') }}</option>
                <option value="lifestyle">{{ t('newProject.blog.categories.lifestyle') }}</option>
                <option value="travel">{{ t('newProject.blog.categories.travel') }}</option>
                <option value="food">{{ t('newProject.blog.categories.food') }}</option>
                <option value="personal">{{ t('newProject.blog.categories.personal') }}</option>
                <option value="business">{{ t('newProject.blog.categories.business') }}</option>
                <option value="education">{{ t('newProject.blog.categories.education') }}</option>
                <option value="health">{{ t('newProject.blog.categories.health') }}</option>
              </select>
              <div class="error-message" v-if="showErrors && !projectData.category">
                {{ t('newProject.blog.categoryError') }}
              </div>
            </div>
          </div>
          
          <div class="form-group required" :class="{ 'has-error': showErrors && !projectData.description }">
            <div class="description-header">
              <label for="project-description" :title="t('newProject.tooltips.description')">
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
            <div class="error-message" v-if="showErrors && !projectData.description">
              {{ t('newProject.details.descriptionError') }}
            </div>
          </div>
          
          <div class="form-group">
            <label for="article-length">{{ t('newProject.blog.articleLength') }}</label>
            <select id="article-length" v-model="projectData.articleLength">
              <option value="" disabled selected>{{ t('newProject.blog.selectLength') }}</option>
              <option value="150">{{ t('newProject.blog.articleLengths.short') }} (150 {{ t('newProject.blog.words') }})</option>
              <option value="300">{{ t('newProject.blog.articleLengths.medium') }} (300 {{ t('newProject.blog.words') }})</option>
              <option value="600">{{ t('newProject.blog.articleLengths.long') }} (600 {{ t('newProject.blog.words') }})</option>
              <option value="1000">{{ t('newProject.blog.articleLengths.veryLong') }} (1000 {{ t('newProject.blog.words') }})</option>
              <option value="2000">{{ t('newProject.blog.articleLengths.extensive') }} (2000 {{ t('newProject.blog.words') }})</option>
            </select>
          </div>
          
          <div class="form-group">
            <label for="keywords">{{ t('newProject.resources.keywords') }}</label>
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
          
          <div class="form-group">
            <label for="target-audience">{{ t('newProject.resources.targetAudience') }}</label>
            <textarea 
              id="target-audience" 
              v-model="projectData.targetAudience" 
              :placeholder="t('newProject.resources.targetAudiencePlaceholder')"
              rows="3"
            ></textarea>
          </div>
        </div>
        
        <!-- Description Sample Modal -->
        <div v-if="showSampleModal" class="sample-modal">
          <div class="sample-modal-content">
            <h3>{{ t('newProject.details.sampleTitle') }}</h3>
            <p class="sample-prompt">{{ t(`newProject.details.samples.${selectedSampleKey}`) }}</p>
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
                <option value="30">{{ t('newProject.time.durations.days30') }}</option>
                <option value="60">{{ t('newProject.time.durations.days60') }}</option>
                <option value="90">{{ t('newProject.time.durations.days90') }}</option>
              </select>
              
            </div>
          </div>
          
          <div class="form-group">
            <label for="publishing-frequency">{{ t('newProject.time.publishingFrequency') }}</label>
            <select id="publishing-frequency" v-model="projectData.publishingFrequency">
              <option value="weekly">{{ t('newProject.time.frequency.weekly') }}</option>
              <option value="biweekly">{{ t('newProject.time.frequency.biweekly') }}</option>
              <option value="monthly">{{ t('newProject.time.frequency.monthly') }}</option>
            </select>
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
        
        <!-- Step 4: Publishing Platform -->
        <div v-if="currentStep === 3" class="step-form">
          <h2>{{ t('newProject.publishing.title') }}</h2>
          <p class="step-description">{{ t('newProject.publishing.description') }}</p>
          
          <div class="form-group">
            <label for="publishing-platform">{{ t('newProject.publishing.platform') }}</label>
            <select id="publishing-platform" v-model="projectData.publishingPlatform">
              <option value="wordpress">{{ t('newProject.publishing.platforms.wordpress') }}</option>
              <option value="substack">{{ t('newProject.publishing.platforms.substack') }}</option>
              <option value="medium">{{ t('newProject.publishing.platforms.medium') }}</option>
              <option value="custom">{{ t('newProject.publishing.platforms.custom') }}</option>
            </select>
          </div>
          
          <div class="form-group" v-if="projectData.publishingPlatform === 'wordpress'">
            <label for="wordpress-url">{{ t('newProject.publishing.wordpressUrl') }}</label>
            <input 
              id="wordpress-url" 
              v-model="projectData.wordpressUrl" 
              type="url" 
              :placeholder="t('newProject.publishing.wordpressUrlPlaceholder')"
            />
          </div>
          
          <div class="form-group" v-if="projectData.publishingPlatform === 'substack'">
            <label for="substack-url">{{ t('newProject.publishing.substackUrl') }}</label>
            <input 
              id="substack-url" 
              v-model="projectData.substackUrl" 
              type="url" 
              :placeholder="t('newProject.publishing.substackUrlPlaceholder')"
            />
          </div>
          
          <div class="form-group" v-if="projectData.publishingPlatform === 'custom'">
            <label for="custom-platform">{{ t('newProject.publishing.customPlatform') }}</label>
            <input 
              id="custom-platform" 
              v-model="projectData.customPlatform" 
              type="text" 
              :placeholder="t('newProject.publishing.customPlatformPlaceholder')"
            />
          </div>
          
          <div class="monetization-section">
            <h3>{{ t('newProject.publishing.monetization') }}</h3>
            <p class="monetization-description">{{ t('newProject.publishing.monetizationDescription') }}</p>
            
            <div class="form-group">
              <label for="monetization-strategy">{{ t('newProject.publishing.monetizationStrategy') }}</label>
              <select id="monetization-strategy" v-model="projectData.monetizationStrategy">
                <option value="free">{{ t('newProject.publishing.strategies.free') }}</option>
                <option value="subscription">{{ t('newProject.publishing.strategies.subscription') }}</option>
                <option value="ads">{{ t('newProject.publishing.strategies.ads') }}</option>
                <option value="affiliate">{{ t('newProject.publishing.strategies.affiliate') }}</option>
                <option value="products">{{ t('newProject.publishing.strategies.products') }}</option>
                <option value="mixed">{{ t('newProject.publishing.strategies.mixed') }}</option>
              </select>
            </div>
            
            <div class="form-group">
              <label for="monetization-goals">{{ t('newProject.publishing.monetizationGoals') }}</label>
              <select id="monetization-goals" v-model="projectData.monetizationGoals">
                <option value="0" selected>{{ t('newProject.publishing.goals.zero') }}</option>
                <option value="100">{{ t('newProject.publishing.goals.weekly100') }}</option>
                <option value="1000">{{ t('newProject.publishing.goals.weekly1000') }}</option>
                <option value="5000">{{ t('newProject.publishing.goals.weekly5000') }}</option>
              </select>
            </div>
          </div>
        </div>
        
        <!-- Step 5: Blog Plan -->
        <div v-if="currentStep === 4" class="step-form">
          
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
              <span>Remind me when it's time to write!</span>
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
import '../styles/CreateBlogProject.css'

const { t } = useI18n()
const router = useRouter()
const fileInput = ref<HTMLInputElement | null>(null)

// Project creation steps
const steps = ['goals', 'details', 'time', 'publishing', 'plan']
const currentStep = ref(0)

// Project data
const projectData = ref({
  type: 'blog',
  // Goal-related fields
  goal: '',
  // Other fields...
  title: '',
  description: '',
  category: 'technology',
  timeCommitment: 'casual',
  duration: '30', // Default to 30 days
  publishingFrequency: 'weekly',
  customFrequency: '',
  deadline: '',
  availability: [] as string[],
  reminderFrequency: 'weekly',
  publishingPlatform: 'wordpress',
  wordpressUrl: '',
  substackUrl: '',
  customPlatform: '',
  monetizationStrategy: 'free',
  monetizationGoals: '',
  keywords: [] as string[],
  targetAudience: '',
  structure: 'standard',
  contentStrategy: '',
  seoStrategy: '',
  articleLength: '150',
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

// Today's date for deadline min value
const today = computed(() => {
  const date = new Date()
  return date.toISOString().split('T')[0]
})

// Keyword input
const keywordInput = ref('')

// Goal inspiration data
const goalInspirationQuotes = [
  {
    quote: "A blog is only as valuable as the value it provides to its readers.",
    author: "Content Marketing Institute"
  },
  {
    quote: "The best time to start was yesterday. The next best time is now.",
    author: "Chinese Proverb"
  },
  {
    quote: "Your blog is your unedited version of yourself.",
    author: "Seth Godin"
  },
  {
    quote: "Don't focus on having a great blog. Focus on producing a blog that's great for your readers.",
    author: "Brian Clark"
  }
]

const goalInspirationPrompts = [
  "I want to share my unique perspective with my readers.",
  "I want to make a difference in people's lives through my blog.",
  "I want to create transformation for my readers.",
  "I want to share my expertise and experience with the world.",
  "I want to grow personally and professionally through my blog.",
  "I want to build a community of like-minded individuals through my writing.",
  "I want to leave a legacy of knowledge and insights for future generations.",
  "I want to influence or initiate important conversations in my field.",
  "I want to establish authority and credibility in my niche.",
  "I want to document my personal challenges and growth to inspire others.",
  "I want to create passive income while providing value to my readers.",
  "I want to address global or local issues to create positive change.",
  "I want to connect with industry leaders and expand my professional network.",
  "I want to fill knowledge gaps in my audience through my content.",
  "I want to showcase my expertise and thought leadership through my blog.",
  "I want to develop my writing skills and find my unique voice.",
  "I want to create a platform for underrepresented voices in my industry.",
  "I want to build a personal brand that opens doors to new opportunities.",
  "I want to document my journey and progress in a specific area of expertise.",
  "I want to create a resource library that helps others solve common problems.",
  "I want to establish myself as a go-to expert for a specific topic.",
  "I want to create content that stands the test of time and remains valuable.",
  "I want to build an email list of engaged readers who trust my recommendations.",
  "I want to create a blog that serves as a portfolio for my professional work.",
  "I want to develop a consistent writing habit that improves my communication skills.",
  "I want to create a blog that generates leads for my business or services.",
  "I want to build a platform that allows me to interview interesting people in my field.",
  "I want to create a blog that helps me process and share my learning journey.",
  "I want to establish a blog that can be monetized through multiple channels.",
  "I want to create content that helps others avoid the mistakes I've made.",
  "I want to build a blog that serves as a knowledge base for my future projects.",
  "I want to create a platform for collaboration with other creators in my niche.",
  "I want to develop a blog that can be turned into a book or course in the future.",
  "I want to create content that helps me stay accountable to my own goals."
]

// Goal inspiration methods
const currentQuoteIndex = ref(0)
const showInspirationModal = ref(false)
const selectedPrompt = ref('')

const rotateQuote = () => {
  currentQuoteIndex.value = (currentQuoteIndex.value + 1) % goalInspirationQuotes.length
}

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
const descriptionSampleKeys = [
  'technology',
  'travel',
  'business',
  'health',
  'education',
  'community',
  'learning',
  'currentEvents',
  'projects',
  'parenting',
  'automotive',
  'ai',
  'cooking',
  'sustainability',
  'mindfulness',
  'remoteWork',
  'urbanExploration',
  'minimalism',
  'digitalNomad',
  'creativeWriting',
  'personalFinance',
  'science',
  'photography'
]

// Description sample methods
const showSampleModal = ref(false)
const selectedSampleKey = ref('')

const openSampleModal = () => {
  showSampleModal.value = true
  selectedSampleKey.value = descriptionSampleKeys[Math.floor(Math.random() * descriptionSampleKeys.length)]
}

const getAnotherSample = () => {
  let newSampleKey
  do {
    newSampleKey = descriptionSampleKeys[Math.floor(Math.random() * descriptionSampleKeys.length)]
  } while (newSampleKey === selectedSampleKey.value && descriptionSampleKeys.length > 1)
  selectedSampleKey.value = newSampleKey
}

const closeSampleModal = () => {
  showSampleModal.value = false
}

const applySampleToDescription = () => {
  projectData.value.description = t(`newProject.details.samples.${selectedSampleKey.value}`)
  closeSampleModal()
}

// Check if can proceed to next step
const canProceed = computed(() => {
  if (currentStep.value === 0) {
    // For goals step, require goal to be filled
    return projectData.value.goal !== ''
  }
  if (currentStep.value === 1) return projectData.value.title !== '' && projectData.value.description !== ''
  if (currentStep.value === 2) {
    // For time step, require at least one available day to be selected
    return projectData.value.availability.length > 0
  }
  if (currentStep.value === 3) {
    if (projectData.value.publishingPlatform === 'wordpress') return projectData.value.wordpressUrl !== ''
    if (projectData.value.publishingPlatform === 'substack') return projectData.value.substackUrl !== ''
    if (projectData.value.publishingPlatform === 'custom') return projectData.value.customPlatform !== ''
    return true
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
      // No longer storing resources
    }
  }
}

const removeResource = (index: number) => {
  // No longer removing resources
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

// Calculate monthly commitment based on availability and publishing frequency
const monthlyCommitment = computed(() => {
  // Base hours per day based on time commitment level
  let hoursPerDay = 1 // Default for casual
  
  if (projectData.value.timeCommitment === 'casual') {
    hoursPerDay = 1
  } else if (projectData.value.timeCommitment === 'partTime') {
    hoursPerDay = 3
  } else if (projectData.value.timeCommitment === 'fullTime') {
    hoursPerDay = 5
  }
  
  // Number of available days per week
  const daysPerWeek = projectData.value.availability.length
  
  // Calculate weeks per month based on publishing frequency
  let weeksPerMonth = 4 // Default to 4 weeks per month
  
  if (projectData.value.publishingFrequency === 'weekly') {
    weeksPerMonth = 4
  } else if (projectData.value.publishingFrequency === 'biweekly') {
    weeksPerMonth = 4
    // Divide hours per day by 2 for bi-weekly frequency
    hoursPerDay = hoursPerDay / 2
  } else if (projectData.value.publishingFrequency === 'monthly') {
    weeksPerMonth = 4
    // Divide hours per day by 2 for monthly frequency
    hoursPerDay = hoursPerDay / 4
  }
  
  // Calculate total hours per month
  const totalHoursPerMonth = daysPerWeek * hoursPerDay * weeksPerMonth
  
  // Round to nearest whole number
  return Math.round(totalHoursPerMonth)
})

const showErrors = ref(false)

// Validation computed properties
const isStep2Valid = computed(() => {
  return projectData.value.title && 
         projectData.value.description && 
         projectData.value.category
})

const isFormValid = computed(() => {
  // Add validation for all required fields
  return projectData.value.goal &&
         projectData.value.title &&
         projectData.value.description &&
         projectData.value.category &&
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
  if (currentStep.value === 3) {
    if (!projectData.value.publishingPlatform) {
      return t('newProject.publishing.platformError')
    }
    if (projectData.value.publishingPlatform === 'wordpress' && !projectData.value.wordpressUrl) {
      return t('newProject.publishing.urlError.wordpress')
    }
    if (projectData.value.publishingPlatform === 'substack' && !projectData.value.substackUrl) {
      return t('newProject.publishing.urlError.substack')
    }
    if (projectData.value.publishingPlatform === 'custom' && !projectData.value.customPlatform) {
      return t('newProject.publishing.urlError.custom')
    }
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
    
    // Log all form elements
    console.log('Project Form Data:', {
      // Basic Info
      title: projectData.value.title,
      type: projectData.value.type,
      description: projectData.value.description,
      category: projectData.value.category,
      
      // Time and Schedule
      timeCommitment: projectData.value.timeCommitment,
      publishingFrequency: projectData.value.publishingFrequency,
      customFrequency: projectData.value.customFrequency,
      deadline: projectData.value.deadline,
      availability: projectData.value.availability,
      reminderFrequency: projectData.value.reminderFrequency,
      
      // Publishing Platform
      publishingPlatform: projectData.value.publishingPlatform,
      wordpressUrl: projectData.value.wordpressUrl,
      substackUrl: projectData.value.substackUrl,
      customPlatform: projectData.value.customPlatform,
      
      // Monetization
      monetizationStrategy: projectData.value.monetizationStrategy,
      monetizationGoals: projectData.value.monetizationGoals,
      
      // Content Strategy
      keywords: projectData.value.keywords,
      targetAudience: projectData.value.targetAudience,
      structure: projectData.value.structure,
      contentStrategy: projectData.value.contentStrategy,
      seoStrategy: projectData.value.seoStrategy,
      
      // Project Goals
      goal: projectData.value.goal,
      articleLength: projectData.value.articleLength,
      
      // Notifications
      receiveNotifications: projectData.value.receiveNotifications
    })
    
    // Prepare the project data
    const project = {
      title: projectData.value.title,
      type_: projectData.value.type,
      description: projectData.value.description,
      category: projectData.value.category,
      time_commitment: projectData.value.timeCommitment,
      publishing_frequency: projectData.value.publishingFrequency,
      custom_frequency: projectData.value.customFrequency,
      deadline: projectData.value.deadline,
      availability: JSON.stringify(projectData.value.availability),
      reminder_frequency: projectData.value.reminderFrequency,
      publishing_platform: projectData.value.publishingPlatform,
      wordpress_url: projectData.value.wordpressUrl,
      substack_url: projectData.value.substackUrl,
      custom_platform: projectData.value.customPlatform,
      monetization_strategy: projectData.value.monetizationStrategy,
      monetization_goals: projectData.value.monetizationGoals,
      keywords: JSON.stringify(projectData.value.keywords),
      target_audience: projectData.value.targetAudience,
      structure: projectData.value.structure,
      content_strategy: projectData.value.contentStrategy,
      seo_strategy: projectData.value.seoStrategy,
      goal: projectData.value.goal,
      article_length: projectData.value.articleLength,
      receive_notifications: projectData.value.receiveNotifications
    }
    
    console.log('Formatted project data for database:', project)
    
    // Save the project
    const projectId = await saveProject(project)
    console.log(`Project saved with ID: ${projectId}`)
    
    if (projectId !== -1) {
      // Save project content if needed
      if (projectData.value.contentStrategy) {
        try {
          const content = {
            project_id: projectId,
            title: 'Project Plan',
            content: JSON.stringify({
              contentStrategy: projectData.value.contentStrategy,
              seoStrategy: projectData.value.seoStrategy,
              keywords: projectData.value.keywords,
              targetAudience: projectData.value.targetAudience
            }),
            content_order: 1
          }
          console.log('Project content to be saved:', content)
          const contentId = await saveProjectContent(content)
          console.log(`Project content saved with ID: ${contentId}`)
        } catch (contentError) {
          console.error('Error saving project content:', contentError)
          // Continue even if content saving fails
        }
      }
      
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
/* All styles moved to CreateBlogProject.css */
</style> 