import { describe, it, expect } from 'vitest'
import { buildAIPrompt } from '../composables/useAIPromptBuilder'

describe('buildAIPrompt', () => {
  it('should build correct ChapterDraft prompt', () => {
    const prompt = buildAIPrompt('ChapterDraft', {
      bookTitle: 'AI 寫作入門',
      chapterCount: 3,
      extraInfo: '希望每章有小結',
      project: { goal: '教初學者 AI 寫作', description: '從零開始的 AI 寫作書' }
    })
    expect(prompt).toContain('AI 助手')
    expect(prompt).toContain('AI 寫作入門')
    expect(prompt).toContain('章節數：3')
    expect(prompt).toContain('希望每章有小結')
    expect(prompt).toContain('教初學者 AI 寫作')
  })
}) 