<template>
  <TwoColumnLayout>
    <div class="settings-container">
      <h1>Settings</h1>
      <Settings />
    </div>
  </TwoColumnLayout>
</template>

<script setup lang="ts">
import TwoColumnLayout from '../components/layouts/TwoColumnLayout.vue'
import Settings from '../components/Settings.vue'
</script>

<style scoped>
.settings-container {
  height: 100%;
  overflow: auto;
  padding: 20px;
}

h1 {
  margin-bottom: 20px;
  color: #333;
}
</style> 