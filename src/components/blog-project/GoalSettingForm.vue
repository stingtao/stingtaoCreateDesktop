<template>
  <div class="step-form">
    <h2>{{ t('newProject.goals.title') }}</h2>
    <p class="step-description">{{ t('newProject.goals.description') }}</p>
    
    <div class="form-group">
      <div class="goal-header">
        <label for="project-goal">{{ t('newProject.goals.mainGoal') }}</label>
        <button class="inspiration-btn" @click="openInspirationModal">
          <span class="inspiration-icon">💡</span>
          Need Inspiration?
        </button>
      </div>
      <textarea 
        id="project-goal" 
        v-model="projectData.goal" 
        :placeholder="t('newProject.goals.goalPlaceholder')"
        rows="8"
      ></textarea>
      <div class="form-hint">{{ t('newProject.goals.goalHint') }}</div>
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
          <button class="primary-btn" @click="applyPromptToGoal">
            {{ t('newProject.goals.inspirationApply') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface ProjectData {
  goal: string
}

const props = defineProps<{
  projectData: ProjectData
}>()

const emit = defineEmits<{
  (e: 'update:projectData', value: ProjectData): void
}>()

const showInspirationModal = ref(false)
const selectedPrompt = ref('')

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
  "I want to showcase my expertise and thought leadership through my blog."
]

const openInspirationModal = () => {
  showInspirationModal.value = true
  selectedPrompt.value = goalInspirationPrompts[Math.floor(Math.random() * goalInspirationPrompts.length)]
}

const closeInspirationModal = () => {
  showInspirationModal.value = false
}

const applyPromptToGoal = () => {
  const updatedData = { ...props.projectData }
  if (updatedData.goal === '') {
    updatedData.goal = `Inspired by the prompt: "${selectedPrompt.value}"\n\n`
  } else {
    updatedData.goal += `\n\nInspired by: "${selectedPrompt.value}"`
  }
  emit('update:projectData', updatedData)
  closeInspirationModal()
}
</script>

<style scoped>
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

.form-group {
  margin-bottom: 1.5rem;
}

.form-hint {
  font-size: 0.85rem;
  color: #666;
  margin-top: 0.5rem;
  font-style: italic;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #2c3e50;
  transition: color 0.3s;
}

textarea {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
  transition: all 0.3s;
  box-sizing: border-box;
  resize: vertical;
}

textarea:focus {
  outline: none;
  border-color: #2196F3;
  box-shadow: 0 0 0 2px rgba(33, 150, 243, 0.2);
}

.goal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.inspiration-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background-color: #fff;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.3s;
  white-space: nowrap;
}

.inspiration-btn:hover {
  background-color: #f8f9fa;
  border-color: #2196F3;
}

.inspiration-icon {
  font-size: 1.2rem;
}

.inspiration-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.inspiration-modal-content {
  background-color: white;
  border-radius: 8px;
  padding: 2rem;
  max-width: 500px;
  width: 90%;
}

.inspiration-prompt {
  font-size: 1.2rem;
  line-height: 1.6;
  margin: 1.5rem 0;
  text-align: center;
  color: #2196F3;
}

.inspiration-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 2rem;
  flex-wrap: wrap;
}

.secondary-btn {
  padding: 0.75rem 1.5rem;
  background-color: #f8f9fa;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.3s;
  flex: 1;
  min-width: 100px;
  text-align: center;
}

.secondary-btn:hover {
  background-color: #e9ecef;
}

.primary-btn {
  padding: 0.75rem 1.5rem;
  background-color: #2196F3;
  border: none;
  border-radius: 4px;
  color: white;
  cursor: pointer;
  transition: all 0.3s;
  flex: 1;
  min-width: 100px;
  text-align: center;
}

.primary-btn:hover {
  background-color: #1976D2;
}

/* Dark theme styles */
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

:global(body.dark) .form-hint {
  color: #aaa;
}

:global(body.dark) label {
  color: #e0e0e0;
}

:global(body.dark) textarea {
  background-color: #2c2c2c;
  border-color: #444;
  color: #e0e0e0;
}

:global(body.dark) textarea:focus {
  border-color: #64b5f6;
  box-shadow: 0 0 0 2px rgba(100, 181, 246, 0.2);
}

:global(body.dark) .inspiration-btn {
  background-color: #2c2c2c;
  border-color: #444;
  color: #e0e0e0;
}

:global(body.dark) .inspiration-btn:hover {
  background-color: #3c3c3c;
  border-color: #64b5f6;
}

:global(body.dark) .inspiration-modal-content {
  background-color: #1e1e1e;
}

:global(body.dark) .inspiration-prompt {
  color: #64b5f6;
}

:global(body.dark) .secondary-btn {
  background-color: #2c2c2c;
  border-color: #444;
  color: #e0e0e0;
}

:global(body.dark) .secondary-btn:hover {
  background-color: #3c3c3c;
}

/* Responsive styles */
@media (max-width: 992px) {
  .step-form {
    padding: 1.5rem;
  }
  
  h2 {
    font-size: 1.3rem;
  }

  .form-group {
    margin-bottom: 1.25rem;
  }

  .goal-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .inspiration-btn {
    width: 100%;
    justify-content: center;
  }
}

@media (max-width: 768px) {
  .step-form {
    padding: 1.25rem;
  }
  
  h2 {
    font-size: 1.2rem;
  }
  
  .step-description {
    font-size: 0.9rem;
  }
  
  .inspiration-modal-content {
    width: 95%;
    margin: 1rem;
  }

  .inspiration-actions {
    flex-direction: column;
  }

  .secondary-btn, .primary-btn {
    width: 100%;
  }
}

@media (max-width: 576px) {
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
  
  textarea {
    padding: 0.6rem;
    font-size: 0.9rem;
  }
  
  .inspiration-btn {
    padding: 0.4rem 0.75rem;
    font-size: 0.9rem;
  }
  
  .inspiration-modal-content {
    padding: 1.5rem;
  }
  
  .inspiration-prompt {
    font-size: 1rem;
    margin: 1.25rem 0;
  }
  
  .inspiration-actions {
    margin-top: 1.5rem;
  }
  
  .secondary-btn, .primary-btn {
    padding: 0.6rem 1.25rem;
    font-size: 0.9rem;
  }
}
</style> 