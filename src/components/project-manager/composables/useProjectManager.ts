import { invoke } from '@tauri-apps/api/tauri'

const exportDatabase = async () => {
  try {
    const exportPath = await invoke<string>('export_database')
    if (exportPath) {
      await invoke<void>('open_export_location')
      return true
    }
    return false
  } catch (error) {
    console.error('Error exporting database:', error)
    return false
  }
} 