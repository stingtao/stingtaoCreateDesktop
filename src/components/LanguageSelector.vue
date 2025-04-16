<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ref, onMounted } from 'vue'

const { locale, availableLocales } = useI18n()

const currentLocale = ref(locale.value)

// 切換語言
const changeLocale = (newLocale: string) => {
  currentLocale.value = newLocale
  locale.value = newLocale
  localStorage.setItem('locale', newLocale)
}

// 初始化時從 localStorage 加載語言設置
onMounted(() => {
  const savedLocale = localStorage.getItem('locale')
  if (savedLocale) {
    currentLocale.value = savedLocale
    locale.value = savedLocale
  }
})
</script>

<template>
  <div class="language-selector">
    <select 
      v-model="currentLocale" 
      @change="changeLocale(currentLocale)"
      class="language-select"
    >
      <option 
        v-for="locale in availableLocales" 
        :key="locale" 
        :value="locale"
      >
        {{ locale === 'en' ? 'English' : locale === 'zh_TW' ? '繁體中文' : locale }}
      </option>
    </select>
  </div>
</template>

<style scoped>
.language-selector {
  margin-bottom: 1rem;
}

.language-select {
  padding: 0.5rem;
  border-radius: 4px;
  border: 1px solid #ddd;
  background-color: white;
  font-size: 1rem;
  min-width: 150px;
  cursor: pointer;
}

.language-select:focus {
  outline: none;
  border-color: #2196F3;
  box-shadow: 0 0 0 2px rgba(33, 150, 243, 0.2);
}
</style> 