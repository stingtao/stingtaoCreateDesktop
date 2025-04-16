import { ref } from 'vue'
import { driver, type Driver, type DriveStep } from 'driver.js'
import 'driver.js/dist/driver.css'

// Tour definitions for different views
const tours: Record<string, DriveStep[]> = {
  home: [
    {
      element: '#create-project-btn',
      popover: {
        title: 'Create New Project',
        description: 'Start by creating a new blog or book project.',
        side: 'bottom'
      }
    },
    {
      element: '#projects-list',
      popover: {
        title: 'Your Projects',
        description: 'View and manage all your writing projects here.',
        side: 'bottom'
      }
    }
  ],
  createProject: [
    {
      element: '#project-type-select',
      popover: {
        title: 'Choose Project Type',
        description: 'Select whether you want to create a blog or book project.',
        side: 'bottom'
      }
    },
    {
      element: '#project-title-input',
      popover: {
        title: 'Project Title',
        description: 'Give your project a meaningful title.',
        side: 'right'
      }
    },
    {
      element: '#project-goals',
      popover: {
        title: 'Project Goals',
        description: 'Define what you want to achieve with this project.',
        side: 'left'
      }
    }
  ],
  editor: [
    {
      element: '.editor-area',
      popover: {
        title: 'Content Editor',
        description: 'Write and edit your content here using Markdown.',
        side: 'right'
      }
    },
    {
      element: '.preview-content',
      popover: {
        title: 'Live Preview',
        description: 'See how your content will look as you write.',
        side: 'left'
      }
    },
    {
      element: '.ai-assistant',
      popover: {
        title: 'AI Assistant',
        description: 'Get AI-powered suggestions and help with your writing.',
        side: 'left'
      }
    }
  ],
  settings: [
    {
      element: '#api-key-input',
      popover: {
        title: 'API Key',
        description: 'Enter your Gemini API key to enable AI features.',
        side: 'bottom'
      }
    }
  ]
}

// Create driver instance with default options
const driverObj = driver({
  animate: true,
  showProgress: true,
  showButtons: ['next', 'previous', 'close'],
  steps: []
})

export function useTour() {
  const isFirstVisit = ref(true)
  const currentTour = ref<string | null>(null)

  // Start a specific tour
  const startTour = (tourName: keyof typeof tours) => {
    if (!tours[tourName]) return
    
    currentTour.value = tourName
    driverObj.setSteps(tours[tourName])
    driverObj.drive()
  }

  // Stop the current tour
  const stopTour = () => {
    driverObj.destroy()
    currentTour.value = null
  }

  // Check if this is user's first visit to a view
  const checkFirstVisit = (viewName: string) => {
    const visited = localStorage.getItem(`visited_${viewName}`)
    if (!visited) {
      localStorage.setItem(`visited_${viewName}`, 'true')
      return true
    }
    return false
  }

  // Start tour if it's first visit
  const startTourIfFirst = (viewName: keyof typeof tours) => {
    if (checkFirstVisit(viewName)) {
      startTour(viewName)
    }
  }

  return {
    startTour,
    stopTour,
    checkFirstVisit,
    startTourIfFirst,
    isFirstVisit,
    currentTour
  }
} 