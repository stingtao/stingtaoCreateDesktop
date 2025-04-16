import { Settings } from './db'
import { invoke } from '@tauri-apps/api/tauri'

// Initialize the database
export const initDatabase = async (): Promise<void> => {
  try {
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('Tauri is not available, cannot initialize database')
      return
    }
    
    await invoke('init_db')
    console.log('SQLite database initialized successfully')
  } catch (error) {
    console.error('Failed to initialize SQLite database:', error)
    throw error
  }
}

// Get a setting by key
export const getSetting = async (key: string): Promise<string | null> => {
  try {
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('Tauri is not available, cannot get setting')
      return null
    }
    
    const value = await invoke<string | null>('get_setting', { key })
    return value
  } catch (error) {
    console.error('Failed to get setting:', error)
    return null
  }
}

// Set a setting
export const setSetting = async (key: string, value: string): Promise<void> => {
  try {
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('Tauri is not available, cannot set setting')
      return
    }
    
    await invoke('set_setting', { key, value })
    console.log(`Setting ${key} saved successfully`)
  } catch (error) {
    console.error('Failed to set setting:', error)
    throw error
  }
}

// Delete a setting
export const deleteSetting = async (key: string): Promise<void> => {
  try {
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('Tauri is not available, cannot delete setting')
      return
    }
    
    await invoke('delete_setting', { key })
  } catch (error) {
    console.error('Failed to delete setting:', error)
    throw error
  }
}

// Get all settings
export const getAllSettings = async (): Promise<Settings[]> => {
  try {
    if (typeof window.__TAURI__ === 'undefined') {
      console.warn('Tauri is not available, cannot get all settings')
      return []
    }
    
    const settings = await invoke<[string, string][]>('get_all_settings')
    return settings.map(([key, value], index) => ({
      id: index + 1,
      key,
      value,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString()
    }))
  } catch (error) {
    console.error('Failed to get all settings:', error)
    return []
  }
}

// Get Gemini API key
export const getGeminiApiKey = async (): Promise<string | null> => {
  return getSetting('gemini_api_key')
}

// Set Gemini API key
export const setGeminiApiKey = async (apiKey: string): Promise<void> => {
  await setSetting('gemini_api_key', apiKey)
}

// Check if Gemini API key is set
export const checkGeminiApiKey = async (): Promise<boolean> => {
  const apiKey = await getGeminiApiKey()
  return apiKey !== null && apiKey !== ''
} 