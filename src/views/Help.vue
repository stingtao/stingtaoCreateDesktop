<template>
  <TwoColumnLayout>
    <div class="help-content">
      <h1>{{ t('help.title') }}</h1>
      <div class="help-section">
        <h2>{{ t('help.gettingStarted') }}</h2>
        <p>{{ t('help.gettingStartedContent') }}</p>
      </div>
      <div class="help-section">
        <h2>{{ t('help.features') }}</h2>
        <ul>
          <li>{{ t('help.feature1') }}</li>
          <li>{{ t('help.feature2') }}</li>
          <li>{{ t('help.feature3') }}</li>
        </ul>
      </div>
    </div>
  </TwoColumnLayout>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import TwoColumnLayout from '../components/layouts/TwoColumnLayout.vue'

const { t } = useI18n()
</script>

<style scoped>
.help-content {
  max-width: 800px;
  margin: 0 auto;
}

h1 {
  margin-bottom: 2rem;
  font-size: 2rem;
  color: var(--color-text-primary);
  transition: color 0.3s;
}

.help-section {
  margin-bottom: 2rem;
  padding: 1.5rem;
  background-color: var(--color-bg-secondary);
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

ul {
  list-style-type: disc;
  margin-left: 1.5rem;
}

li {
  margin-bottom: 0.5rem;
  line-height: 1.6;
  color: var(--color-text-secondary);
  transition: color 0.3s;
}
</style> 