<template>
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
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  steps: string[]
  currentStep: number
}>()
</script>

<style scoped>
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
  background-color: #e0e0e0;
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
  text-align: center;
}

.step.active .step-label {
  color: #2196F3;
  font-weight: bold;
}

/* Dark theme styles */
:global(body.dark) .step-label {
  color: #aaa;
}

:global(body.dark) .step.active .step-label {
  color: #64b5f6;
}

/* Responsive styles */
@media (max-width: 1200px) {
  .progress-indicator {
    gap: 1rem;
  }

  .step {
    min-width: 100px;
  }
}

@media (max-width: 992px) {
  .step-number {
    width: 36px;
    height: 36px;
  }
  
  .step-label {
    font-size: 0.8rem;
  }
}

@media (max-width: 768px) {
  .progress-indicator {
    margin-bottom: 2rem;
    flex-wrap: wrap;
    justify-content: center;
    gap: 1.5rem;
  }

  .progress-indicator::before {
    display: none;
  }
  
  .step-number {
    width: 32px;
    height: 32px;
    font-size: 0.9rem;
  }
  
  .step-label {
    font-size: 0.75rem;
  }
}

@media (max-width: 576px) {
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
}

@media (max-width: 360px) {
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
}
</style> 