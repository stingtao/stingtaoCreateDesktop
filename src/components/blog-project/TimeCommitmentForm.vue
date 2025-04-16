<template>
  <div class="step-form">
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
      <label for="publishing-frequency">{{ t('newProject.time.publishingFrequency') }}</label>
      <select id="publishing-frequency" v-model="projectData.publishingFrequency">
        <option value="weekly">{{ t('newProject.time.frequency.weekly') }}</option>
        <option value="biweekly">{{ t('newProject.time.frequency.biweekly') }}</option>
        <option value="monthly">{{ t('newProject.time.frequency.monthly') }}</option>
        <option value="quarterly">{{ t('newProject.time.frequency.quarterly') }}</option>
        <option value="custom">{{ t('newProject.time.frequency.custom') }}</option>
      </select>
    </div>
    
    <div class="form-group" v-if="projectData.publishingFrequency === 'custom'">
      <label for="custom-frequency">{{ t('newProject.time.customFrequency') }}</label>
      <input 
        id="custom-frequency" 
        v-model="projectData.customFrequency" 
        type="text" 
        :placeholder="t('newProject.time.customFrequencyPlaceholder')"
      />
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
    
    <div class="form-group">
      <label for="reminder-frequency">{{ t('newProject.time.reminderFrequency') }}</label>
      <select id="reminder-frequency" v-model="projectData.reminderFrequency">
        <option value="daily">{{ t('newProject.time.reminder.daily') }}</option>
        <option value="weekly">{{ t('newProject.time.reminder.weekly') }}</option>
        <option value="biweekly">{{ t('newProject.time.reminder.biweekly') }}</option>
        <option value="monthly">{{ t('newProject.time.reminder.monthly') }}</option>
        <option value="none">{{ t('newProject.time.reminder.none') }}</option>
      </select>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface ProjectData {
  timeCommitment: string
  publishingFrequency: string
  customFrequency: string
  deadline: string
  availability: string[]
  reminderFrequency: string
}

const props = defineProps<{
  projectData: ProjectData
}>()

const emit = defineEmits<{
  (e: 'update:projectData', value: ProjectData): void
}>()

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

const toggleAvailability = (day: string) => {
  const updatedData = { ...props.projectData }
  const index = updatedData.availability.indexOf(day)
  if (index === -1) {
    updatedData.availability.push(day)
  } else {
    updatedData.availability.splice(index, 1)
  }
  emit('update:projectData', updatedData)
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

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #2c3e50;
  transition: color 0.3s;
}

input, select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
  transition: all 0.3s;
  box-sizing: border-box;
}

input:focus, select:focus {
  outline: none;
  border-color: #2196F3;
  box-shadow: 0 0 0 2px rgba(33, 150, 243, 0.2);
}

.availability-options {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
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

:global(body.dark) label {
  color: #e0e0e0;
}

:global(body.dark) input, :global(body.dark) select {
  background-color: #2c2c2c;
  border-color: #444;
  color: #e0e0e0;
}

:global(body.dark) input:focus, :global(body.dark) select:focus {
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
  
  .day-option {
    width: 36px;
    height: 36px;
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
  
  input, select {
    padding: 0.6rem;
    font-size: 0.9rem;
  }
  
  .day-option {
    width: 32px;
    height: 32px;
    font-size: 0.8rem;
  }
}

@media (max-width: 360px) {
  .day-option {
    width: 28px;
    height: 28px;
    font-size: 0.7rem;
  }
}
</style> 