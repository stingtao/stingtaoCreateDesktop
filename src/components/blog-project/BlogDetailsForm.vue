<template>
  <div class="step-form">
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
      <label for="blog-category">{{ t('newProject.blog.category') }}</label>
      <select id="blog-category" v-model="projectData.category">
        <option value="technology">{{ t('newProject.blog.categories.technology') }}</option>
        <option value="lifestyle">{{ t('newProject.blog.categories.lifestyle') }}</option>
        <option value="travel">{{ t('newProject.blog.categories.travel') }}</option>
        <option value="food">{{ t('newProject.blog.categories.food') }}</option>
        <option value="personal">{{ t('newProject.blog.categories.personal') }}</option>
        <option value="business">{{ t('newProject.blog.categories.business') }}</option>
        <option value="education">{{ t('newProject.blog.categories.education') }}</option>
        <option value="health">{{ t('newProject.blog.categories.health') }}</option>
      </select>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface ProjectData {
  title: string
  description: string
  category: string
}

defineProps<{
  projectData: ProjectData
}>()
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

input, textarea, select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
  transition: all 0.3s;
  box-sizing: border-box;
}

input:focus, textarea:focus, select:focus {
  outline: none;
  border-color: #2196F3;
  box-shadow: 0 0 0 2px rgba(33, 150, 243, 0.2);
}

textarea {
  resize: vertical;
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

:global(body.dark) input, :global(body.dark) textarea, :global(body.dark) select {
  background-color: #2c2c2c;
  border-color: #444;
  color: #e0e0e0;
}

:global(body.dark) input:focus, :global(body.dark) textarea:focus, :global(body.dark) select:focus {
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
  
  input, textarea, select {
    padding: 0.6rem;
    font-size: 0.9rem;
  }
}
</style> 