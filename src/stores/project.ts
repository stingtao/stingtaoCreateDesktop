import { defineStore } from 'pinia'

export interface ProjectState {
  currentProjectId: number | null
  currentProjectType: 'blog' | 'book' | null
  currentProjectTitle: string
  currentChapterId: number | null
  currentChapterTitle: string
  aiStatus: {
    loading: boolean
    lastAgent: string | null
    lastResult: string | null
  }
}

export const useProjectStore = defineStore('project', {
  state: (): ProjectState => ({
    currentProjectId: null,
    currentProjectType: null,
    currentProjectTitle: '',
    currentChapterId: null,
    currentChapterTitle: '',
    aiStatus: {
      loading: false,
      lastAgent: null,
      lastResult: null,
    },
  }),
  actions: {
    setProject(this: ProjectState, id: number, type: 'blog' | 'book', title: string) {
      this.currentProjectId = id
      this.currentProjectType = type
      this.currentProjectTitle = title
    },
    setChapter(this: ProjectState, id: number, title: string) {
      this.currentChapterId = id
      this.currentChapterTitle = title
    },
    setAIStatus(this: ProjectState, loading: boolean, lastAgent: string | null, lastResult: string | null) {
      this.aiStatus = { loading, lastAgent, lastResult }
    },
    reset(this: ProjectState) {
      this.currentProjectId = null
      this.currentProjectType = null
      this.currentProjectTitle = ''
      this.currentChapterId = null
      this.currentChapterTitle = ''
      this.aiStatus = { loading: false, lastAgent: null, lastResult: null }
    },
  },
}) 