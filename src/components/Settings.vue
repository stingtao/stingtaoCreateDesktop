<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ref, onMounted, watch } from 'vue'
import LanguageSelector from './LanguageSelector.vue'
import { getSetting, setSetting, initDatabase, getAllSettings, getGeminiApiKey, setGeminiApiKey } from '../lib/sqlite'
import { invoke } from '@tauri-apps/api/tauri'
import { appDataDir } from '@tauri-apps/api/path'

const { t, locale } = useI18n()

// Initialize preferences
const preferences = ref({
  language: 'en',
  autoSave: true,
  theme: 'light',
  geminiApiKey: ''
})

// Export database status
const exportStatus = ref<{ type: 'success' | 'error', message: string } | null>(null)
const isExporting = ref(false)
const exportPath = ref<string | null>(null)

// Debug function to view all settings
const viewAllSettings = async () => {
  try {
    const settings = await getAllSettings()
    console.log('All settings in SQLite:', settings)
  } catch (error) {
    console.error('Failed to get all settings:', error)
  }
}

// Load preferences from database
const loadPreferences = async () => {
  try {
    // First check if Tauri is available
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('Tauri is not available, using default preferences')
      return
    }

    // Initialize database if needed
    await initDatabase()
    
    // Get all settings
    const settings = await getAllSettings()
    
    if (settings && settings.length > 0) {
      // Convert array of settings to an object
      const settingsObj: Record<string, string> = {}
      settings.forEach(setting => {
        settingsObj[setting.key] = setting.value
      })
      
      preferences.value = {
        language: settingsObj.language || 'en',
        autoSave: settingsObj.auto_save === 'true' || true,
        theme: settingsObj.theme || 'light',
        geminiApiKey: settingsObj.gemini_api_key || ''
      }
      // Apply theme after loading preferences
      applyTheme(preferences.value.theme)
    }
  } catch (error) {
    console.error('Failed to load preferences:', error)
    // Fallback to default preferences
    preferences.value = {
      language: 'en',
      autoSave: true,
      theme: 'light',
      geminiApiKey: ''
    }
  }
}

// Save preferences to database
const savePreferences = async () => {
  try {
    // Check if Tauri is available
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('Tauri is not available, preferences will not be saved')
      return
    }

    await setSetting('language', preferences.value.language)
    await setSetting('auto_save', preferences.value.autoSave.toString())
    await setSetting('theme', preferences.value.theme)
    await setSetting('gemini_api_key', preferences.value.geminiApiKey)
  } catch (error) {
    console.error('Failed to save preferences:', error)
  }
}

// Watch for changes and save
watch(preferences, () => {
  savePreferences()
}, { deep: true })

// Initialize on mount
onMounted(async () => {
  try {
    await loadPreferences()
  } catch (error) {
    console.error('Failed to initialize preferences:', error)
  }
})

// 切換自動保存
const toggleAutoSave = async () => {
  preferences.value.autoSave = !preferences.value.autoSave
}

// 切換主題
const setTheme = async (theme: string) => {
  preferences.value.theme = theme
  applyTheme(theme)
}

// 應用主題
const applyTheme = (theme: string) => {
  // 移除所有主題類
  document.body.classList.remove('light', 'dark')
  // 添加當前主題類
  document.body.classList.add(theme)
}

// Export database function
async function exportDatabase() {
  try {
    isExporting.value = true
    exportStatus.value = { type: 'success', message: 'Exporting database...' }
    exportPath.value = null
    
    const result = await invoke<string>('export_database')
    exportPath.value = result
    exportStatus.value = { 
      type: 'success', 
      message: `Database exported successfully!` 
    }
  } catch (error) {
    console.error('Error exporting database:', error)
    exportStatus.value = { 
      type: 'error', 
      message: `Error exporting database: ${error}` 
    }
  } finally {
    isExporting.value = false
  }
}

// Export database as JSON function
async function exportDatabaseJson() {
  try {
    isExporting.value = true
    exportStatus.value = { type: 'success', message: 'Exporting database as JSON...' }
    exportPath.value = null
    
    const result = await invoke<string>('export_database_json')
    exportPath.value = result
    exportStatus.value = { 
      type: 'success', 
      message: `Database exported as JSON successfully!` 
    }
  } catch (error) {
    console.error('Error exporting database as JSON:', error)
    exportStatus.value = { 
      type: 'error', 
      message: `Error exporting database as JSON: ${error}` 
    }
  } finally {
    isExporting.value = false
  }
}

// Open export location
async function openExportLocation() {
  if (!exportPath.value) return
  
  try {
    await invoke('open_export_location')
  } catch (error) {
    console.error('Error opening export location:', error)
    exportStatus.value = { 
      type: 'error', 
      message: `Error opening export location: ${error}` 
    }
  }
}

const isSaving = ref(false)
const showApiKey = ref(false)

// 切換顯示/隱藏 API 金鑰
const toggleShowApiKey = () => {
  showApiKey.value = !showApiKey.value
}

// 保存 Gemini API 金鑰
const saveGeminiApiKey = async () => {
  if (!preferences.value.geminiApiKey) return
  
  isSaving.value = true
  try {
    await setGeminiApiKey(preferences.value.geminiApiKey)
  } catch (error) {
    console.error('Failed to save Gemini API key:', error)
  } finally {
    isSaving.value = false
  }
}
</script>

<template>
  <div class="settings">
    <h2>{{ t('settings.title') }}</h2>
    
    <div class="settings-section">
      <h3>{{ t('settings.language') }}</h3>
      <div class="language-selector-container">
        <LanguageSelector />
      </div>
    </div>
    
    <div class="settings-section">
      <h3>{{ t('settings.theme') }}</h3>
      <div class="theme-options">
        <button 
          class="theme-button light" 
          :class="{ active: preferences.theme === 'light' }"
          @click="setTheme('light')"
        >
          {{ t('settings.themeLight') }}
        </button>
        <button 
          class="theme-button dark" 
          :class="{ active: preferences.theme === 'dark' }"
          @click="setTheme('dark')"
        >
          {{ t('settings.themeDark') }}
        </button>
      </div>
    </div>
    
    <div class="settings-section">
      <h3>{{ t('settings.autoSave') }}</h3>
      <div class="toggle-container">
        <label class="toggle">
          <input 
            type="checkbox" 
            :checked="preferences.autoSave"
            @change="toggleAutoSave"
          >
          <span class="toggle-slider"></span>
        </label>
        <span>{{ t('settings.autoSaveEnabled') }}</span>
      </div>
    </div>
    
    <div class="settings-section">
      <h3>Database</h3>
      <div class="settings-actions">
        <button 
          @click="exportDatabase" 
          class="btn btn-primary"
          :disabled="isExporting"
        >
          <i :class="isExporting ? 'fas fa-spinner fa-spin' : 'fas fa-download'"></i> 
          {{ isExporting ? 'Exporting...' : 'Export Database' }}
        </button>
        <button 
          @click="exportDatabaseJson" 
          class="btn btn-secondary"
          :disabled="isExporting"
        >
          <i :class="isExporting ? 'fas fa-spinner fa-spin' : 'fas fa-file-code'"></i> 
          {{ isExporting ? 'Exporting...' : 'Export as JSON' }}
        </button>
      </div>
      
      <div v-if="exportStatus" :class="['export-status', exportStatus.type]">
        <div class="export-message">{{ exportStatus.message }}</div>
        
        <div v-if="exportPath && exportStatus.type === 'success'" class="export-path">
          <div class="path-display">
            <i class="fas fa-file-export"></i>
            <span class="path-text">{{ exportPath }}</span>
          </div>
          <button 
            @click="openExportLocation" 
            class="btn btn-secondary"
          >
            <i class="fas fa-folder-open"></i> Open Location
          </button>
        </div>
      </div>
    </div>

    <div class="settings-section">
      <h2>API Settings</h2>
      <div class="form-group">
        <label for="gemini-api-key">Gemini API Key</label>
        <div class="input-with-button">
          <input 
            id="gemini-api-key" 
            v-model="preferences.geminiApiKey" 
            :type="showApiKey ? 'text' : 'password'" 
            placeholder="Enter your Gemini API key"
            :disabled="isSaving"
          />
          <button 
            @click="toggleShowApiKey" 
            class="toggle-button"
            :title="showApiKey ? 'Hide API Key' : 'Show API Key'"
          >
            <i :class="showApiKey ? 'fas fa-eye-slash' : 'fas fa-eye'"></i>
          </button>
          <button 
            @click="saveGeminiApiKey" 
            :disabled="isSaving || !preferences.geminiApiKey"
            class="save-button"
          >
            {{ isSaving ? 'Saving...' : 'Save' }}
          </button>
        </div>
        <p class="help-text">
          Get your API key from the 
          <a href="https://ai.google.dev/" target="_blank" rel="noopener noreferrer">Google AI Studio</a>
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings {
  padding: 2rem;
  height: 100%;
  overflow: auto;
}

h2 {
  margin-bottom: 2rem;
  font-size: 1.8rem;
  border-bottom: 1px solid #eee;
  padding-bottom: 0.5rem;
}

.settings-section {
  margin-bottom: 2rem;
  padding: 1.5rem;
  border-radius: 8px;
  background-color: #f9f9f9;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: background-color 0.3s, box-shadow 0.3s;
}

h3 {
  margin-bottom: 1rem;
  font-size: 1.2rem;
  color: #2c3e50;
  transition: color 0.3s;
}

.language-selector-container {
  margin-top: 1rem;
}

.theme-options {
  display: flex;
  gap: 1rem;
}

.theme-button {
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  border: 1px solid #ddd;
  background-color: white;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 1rem;
}

.theme-button:hover {
  background-color: #f0f0f0;
  transform: translateY(-1px);
}

.theme-button.active {
  border-color: #2196F3;
  background-color: #e3f2fd;
}

.theme-button.light {
  border-color: #ddd;
}

.theme-button.dark {
  background-color: #333;
  color: white;
  border-color: #444;
}

.theme-button.dark.active {
  background-color: #1a1a1a;
  border-color: #2196F3;
}

.toggle-container {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.toggle {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: .4s;
  border-radius: 24px;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 16px;
  width: 16px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  transition: .4s;
  border-radius: 50%;
}

input:checked + .toggle-slider {
  background-color: #2196F3;
}

input:focus + .toggle-slider {
  box-shadow: 0 0 1px #2196F3;
}

input:checked + .toggle-slider:before {
  transform: translateX(26px);
}

.settings-actions {
  display: flex;
  gap: 10px;
  margin-bottom: 15px;
}

.btn {
  padding: 8px 16px;
  border-radius: 4px;
  border: none;
  cursor: pointer;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn-primary {
  background-color: #4a6cf7;
  color: white;
}

.btn-primary:hover {
  background-color: #3a5ce5;
}

.btn-secondary {
  background-color: #f0f0f0;
  color: #333;
  border: 1px solid #ddd;
}

.btn-secondary:hover {
  background-color: #e0e0e0;
}

.export-status {
  padding: 15px;
  border-radius: 8px;
  margin-top: 15px;
  transition: all 0.3s ease;
}

.export-status.success {
  background-color: #e6f7e6;
  color: #2e7d32;
  border: 1px solid #c8e6c9;
}

.export-status.error {
  background-color: #ffebee;
  color: #c62828;
  border: 1px solid #ffcdd2;
}

.export-message {
  font-weight: 500;
  margin-bottom: 10px;
}

.export-path {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid rgba(0, 0, 0, 0.1);
}

.path-display {
  display: flex;
  align-items: center;
  gap: 8px;
  background-color: rgba(0, 0, 0, 0.05);
  padding: 8px 12px;
  border-radius: 4px;
  word-break: break-all;
}

.path-text {
  font-family: monospace;
  font-size: 0.9rem;
}

:global(body.dark) .export-status.success {
  background-color: rgba(46, 125, 50, 0.2);
  border-color: rgba(46, 125, 50, 0.4);
}

:global(body.dark) .export-status.error {
  background-color: rgba(198, 40, 40, 0.2);
  border-color: rgba(198, 40, 40, 0.4);
}

:global(body.dark) .path-display {
  background-color: rgba(255, 255, 255, 0.1);
}

:global(body.dark) .btn-secondary {
  background-color: #3c3c3c;
  color: #e0e0e0;
  border-color: #555;
}

:global(body.dark) .btn-secondary:hover {
  background-color: #4a4a4a;
}

:global(body.dark) .export-path {
  border-top-color: rgba(255, 255, 255, 0.1);
}

/* Dark theme styles */
:global(body.dark) .settings-section {
  background-color: #2c2c2c;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}

:global(body.dark) h2 {
  border-bottom-color: #444;
}

:global(body.dark) h3 {
  color: #e0e0e0;
}

:global(body.dark) .theme-button {
  background-color: #333;
  border-color: #444;
  color: #e0e0e0;
}

:global(body.dark) .theme-button:hover {
  background-color: #3c3c3c;
}

:global(body.dark) .theme-button.active {
  background-color: rgba(33, 150, 243, 0.2);
}

/* Responsive styles */
@media (max-width: 768px) {
  .settings {
    padding: 1.5rem;
  }
  
  h2 {
    font-size: 1.5rem;
  }
  
  .settings-section {
    padding: 1.25rem;
  }
  
  .theme-options {
    flex-direction: column;
  }
  
  .theme-button {
    width: 100%;
  }
}

@media (max-width: 576px) {
  .settings {
    padding: 1rem;
  }
  
  h2 {
    font-size: 1.3rem;
  }
  
  .settings-section {
    padding: 1rem;
  }
  
  h3 {
    font-size: 1.1rem;
  }
}

.form-group {
  margin-bottom: 1.5rem;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #374151;
}

.input-with-button {
  display: flex;
  gap: 0.5rem;
}

input {
  flex: 1;
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 0.375rem;
  font-size: 1rem;
}

input:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.toggle-button {
  padding: 0.75rem;
  background-color: #f3f4f6;
  color: #4b5563;
  border: 1px solid #d1d5db;
  border-radius: 0.375rem;
  cursor: pointer;
  transition: background-color 0.2s;
}

.toggle-button:hover {
  background-color: #e5e7eb;
}

.save-button {
  padding: 0.75rem 1.5rem;
  background-color: #3b82f6;
  color: white;
  border: none;
  border-radius: 0.375rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.save-button:hover:not(:disabled) {
  background-color: #2563eb;
}

.save-button:disabled {
  background-color: #93c5fd;
  cursor: not-allowed;
}

.help-text {
  margin-top: 0.5rem;
  font-size: 0.875rem;
  color: #6b7280;
}

a {
  color: #3b82f6;
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}
</style> 