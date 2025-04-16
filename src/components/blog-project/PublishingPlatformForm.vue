<template>
  <div class="step-form">
    <h2>{{ t('newProject.platform.title') }}</h2>
    <p class="step-description">{{ t('newProject.platform.description') }}</p>
    
    <div class="platform-options">
      <div 
        v-for="platform in platforms" 
        :key="platform.id"
        class="platform-card"
        :class="{ selected: selectedPlatforms.includes(platform.id) }"
        @click="togglePlatform(platform.id)"
      >
        <div class="platform-icon">
          <img :src="platform.icon" :alt="platform.name" />
        </div>
        <div class="platform-info">
          <h3>{{ platform.name }}</h3>
          <p>{{ platform.description }}</p>
        </div>
        <div class="platform-check">
          <div class="check-icon" v-if="selectedPlatforms.includes(platform.id)">✓</div>
        </div>
      </div>
    </div>
    
    <div class="form-group" v-if="selectedPlatforms.length > 0">
      <label for="platform-notes">{{ t('newProject.platform.notes') }}</label>
      <textarea 
        id="platform-notes" 
        v-model="projectData.platformNotes" 
        :placeholder="t('newProject.platform.notesPlaceholder')"
        rows="4"
      ></textarea>
    </div>
    
    <div class="form-group" v-if="selectedPlatforms.includes('custom')">
      <label for="custom-platform">{{ t('newProject.platform.customPlatform') }}</label>
      <input 
        id="custom-platform" 
        v-model="projectData.customPlatform" 
        type="text" 
        :placeholder="t('newProject.platform.customPlatformPlaceholder')"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface ProjectData {
  selectedPlatforms: string[]
  platformNotes: string
  customPlatform: string
}

const props = defineProps<{
  projectData: ProjectData
}>()

const emit = defineEmits<{
  (e: 'update:projectData', value: ProjectData): void
}>()

const platforms = [
  {
    id: 'wordpress',
    name: 'WordPress',
    description: t('newProject.platform.options.wordpress'),
    icon: '/icons/wordpress.svg'
  },
  {
    id: 'medium',
    name: 'Medium',
    description: t('newProject.platform.options.medium'),
    icon: '/icons/medium.svg'
  },
  {
    id: 'substack',
    name: 'Substack',
    description: t('newProject.platform.options.substack'),
    icon: '/icons/substack.svg'
  },
  {
    id: 'ghost',
    name: 'Ghost',
    description: t('newProject.platform.options.ghost'),
    icon: '/icons/ghost.svg'
  },
  {
    id: 'custom',
    name: t('newProject.platform.options.custom'),
    description: t('newProject.platform.options.customDesc'),
    icon: '/icons/custom.svg'
  }
]

const selectedPlatforms = computed(() => props.projectData.selectedPlatforms)

const togglePlatform = (platformId: string) => {
  const updatedData = { ...props.projectData }
  const index = updatedData.selectedPlatforms.indexOf(platformId)
  if (index === -1) {
    updatedData.selectedPlatforms.push(platformId)
  } else {
    updatedData.selectedPlatforms.splice(index, 1)
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

.platform-options {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.platform-card {
  display: flex;
  align-items: center;
  padding: 1rem;
  border: 1px solid #ddd;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
  position: relative;
}

.platform-card:hover {
  border-color: #2196F3;
  box-shadow: 0 2px 8px rgba(33, 150, 243, 0.1);
}

.platform-card.selected {
  border-color: #2196F3;
  background-color: rgba(33, 150, 243, 0.05);
}

.platform-icon {
  width: 48px;
  height: 48px;
  margin-right: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.platform-icon img {
  width: 32px;
  height: 32px;
  object-fit: contain;
}

.platform-info {
  flex: 1;
}

.platform-info h3 {
  margin: 0 0 0.5rem 0;
  font-size: 1.1rem;
  color: #2c3e50;
  transition: color 0.3s;
}

.platform-info p {
  margin: 0;
  font-size: 0.9rem;
  color: #666;
  transition: color 0.3s;
}

.platform-check {
  position: absolute;
  top: 1rem;
  right: 1rem;
}

.check-icon {
  width: 24px;
  height: 24px;
  background-color: #2196F3;
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.8rem;
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

textarea, input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
  transition: all 0.3s;
  box-sizing: border-box;
  resize: vertical;
}

textarea:focus, input:focus {
  outline: none;
  border-color: #2196F3;
  box-shadow: 0 0 0 2px rgba(33, 150, 243, 0.2);
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

:global(body.dark) .platform-card {
  border-color: #444;
  background-color: #2c2c2c;
}

:global(body.dark) .platform-card:hover {
  border-color: #64b5f6;
  box-shadow: 0 2px 8px rgba(100, 181, 246, 0.1);
}

:global(body.dark) .platform-card.selected {
  border-color: #64b5f6;
  background-color: rgba(100, 181, 246, 0.1);
}

:global(body.dark) .platform-info h3 {
  color: #e0e0e0;
}

:global(body.dark) .platform-info p {
  color: #aaa;
}

:global(body.dark) .check-icon {
  background-color: #64b5f6;
}

:global(body.dark) label {
  color: #e0e0e0;
}

:global(body.dark) textarea, :global(body.dark) input {
  background-color: #2c2c2c;
  border-color: #444;
  color: #e0e0e0;
}

:global(body.dark) textarea:focus, :global(body.dark) input:focus {
  border-color: #64b5f6;
  box-shadow: 0 0 0 2px rgba(100, 181, 246, 0.2);
}

/* Responsive styles */
@media (max-width: 992px) {
  .step-form {
    padding: 1.5rem;
  }
  
  h2 {
    font-size: 1.3rem;
  }
  
  .platform-options {
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
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
  
  .platform-card {
    padding: 0.75rem;
  }
  
  .platform-icon {
    width: 40px;
    height: 40px;
  }
  
  .platform-icon img {
    width: 28px;
    height: 28px;
  }
  
  .platform-info h3 {
    font-size: 1rem;
  }
  
  .platform-info p {
    font-size: 0.85rem;
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
  
  .platform-options {
    grid-template-columns: 1fr;
  }
  
  .platform-card {
    padding: 0.75rem;
  }
  
  .platform-icon {
    width: 36px;
    height: 36px;
  }
  
  .platform-icon img {
    width: 24px;
    height: 24px;
  }
  
  .platform-info h3 {
    font-size: 0.95rem;
  }
  
  .platform-info p {
    font-size: 0.8rem;
  }
  
  textarea, input {
    padding: 0.6rem;
    font-size: 0.9rem;
  }
}

@media (max-width: 360px) {
  .platform-icon {
    width: 32px;
    height: 32px;
  }
  
  .platform-icon img {
    width: 20px;
    height: 20px;
  }
  
  .platform-info h3 {
    font-size: 0.9rem;
  }
  
  .platform-info p {
    font-size: 0.75rem;
  }
}
</style> 