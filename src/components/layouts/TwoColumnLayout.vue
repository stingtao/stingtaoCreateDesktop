<template>
  <BaseLayout>
    <!-- 主要內容區域 -->
    <div class="main-content">
      <slot></slot>
    </div>
  </BaseLayout>
</template>

<script setup lang="ts">
import BaseLayout from './BaseLayout.vue'
import '../../styles/TwoColumnLayout.css'
</script>

<style scoped>
/* All styles moved to src/styles/TwoColumnLayout.css */
</style> 