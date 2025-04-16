const exportDatabase = async () => {
  try {
    const success = await projectManager.exportDatabase()
    if (success) {
      ElMessage.success('Database exported successfully')
    } else {
      ElMessage.error('Failed to export database')
    }
  } catch (error) {
    console.error('Error exporting database:', error)
    ElMessage.error('Failed to export database')
  }
} 