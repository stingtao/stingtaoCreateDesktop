import type { Project } from '../lib/project'

export type AgentType = 'DraftGenerator' | 'ChapterDraft' | 'Editor' | 'Reviewer' | 'InlineEditor'

interface PromptMeta {
  project?: Partial<Project>
  bookTitle?: string
  chapterCount?: number
  extraInfo?: string
  currentTitle?: string
  currentContent?: string
  selectedText?: string
  userPrompt?: string
}

/**
 * 統一組合 AI prompt，根據 agentType 與 meta 組合格式
 */
export function buildAIPrompt(agentType: AgentType, meta: PromptMeta): string {
  switch (agentType) {
    case 'DraftGenerator':
      // 用於 blog/article 草稿
      return `You are an AI writing assistant for a content creator.\nI need your help drafting an article with the following details:\n\nProject Goal: ${meta.project?.goal || ''}\nProject Description: ${meta.project?.description || ''}\nTarget Audience: ${meta.project?.target_audience || ''}\nKeywords: ${meta.project?.keywords || ''}\n\nUser's Request:\n${meta.userPrompt || ''}\n\nSelected Text Reference (if any):\n${meta.selectedText || ''}\n\nI've already started working on a draft:\nDraft Article Title: ${meta.currentTitle || ''}\nDraft Article Content: ${meta.currentContent || ''}\n\nPlease help me create a comprehensive, well-structured article based on these details. The article should:\n1. Be tailored to the target audience\n2. Focus on achieving the project goal\n3. Incorporate the provided keywords naturally\n4. Maintain a coherent structure with clear sections\n5. Expand on my existing draft while preserving its key ideas\n\nInclude an engaging introduction, well-developed main sections with appropriate subheadings, and a conclusion that reinforces the key points.\n\nIMPORTANT: Your response MUST be formatted as a JSON object wrapped in a markdown code block with the language specified as 'json'. The JSON should have the following structure:\n\u0060\u0060\u0060json\n{\n  "explanation": "Your explanation of how you made this draft",\n  "draftTitle": "The title of the draft",\n  "draftContent": "The content of the draft",\n  "suggestions": [\n    "Suggestion 1",\n    "Suggestion 2",\n    "etc."\n  ]\n}\n\u0060\u0060\u0060\nDo not include any text outside of this JSON code block. Keep your response concise and focused on the task.`
    case 'ChapterDraft':
      // 書籍章節草稿
      return `你是一位專業的書籍寫作 AI 助手。請根據下列資訊，幫我規劃本書章節，並為每章產生一份 300 字以上的章節草稿：\n\n書名：${meta.bookTitle || ''}\n章節數：${meta.chapterCount || ''}\n${meta.extraInfo ? `補充說明：${meta.extraInfo}\n` : ''}${meta.project ? '\n【專案資訊】\n' + Object.entries(meta.project).map(([k, v]) => v ? `${k}: ${v}` : '').filter(Boolean).join('\n') : ''}\n\n請回傳格式如下（JSON array，markdown code block）：\n\u0060\u0060\u0060json\n[{\n  "chapterTitle": "章節標題1",\n  "chapterContent": "章節內容1"\n}, ...]\n\u0060\u0060\u0060\n只需回傳 JSON code block，不要有多餘說明。`
    case 'Editor':
      // 文章/章節內容優化
      return `Review the following content and suggest improvements to style, tone, and readability based on the user's request:\n\nTitle: ${meta.currentTitle || ''}\n\nUser's Request:\n${meta.userPrompt || ''}\n\nSelected Text to Focus on (if any):\n${meta.selectedText || ''}\n\nContent:\n${meta.currentContent || ''}\n\nProvide specific suggestions for enhancing the writing style while preserving the original meaning and intent. Focus on making the content more engaging, clear, and effective for the target audience.`
    case 'Reviewer':
      // 文章/章節最終審查
      return `Perform a comprehensive final review of the following content, addressing the user's specific request:\n\nTitle: ${meta.currentTitle || ''}\n\nUser's Request:\n${meta.userPrompt || ''}\n\nSelected Text to Focus on (if any):\n${meta.selectedText || ''}\n\nContent:\n${meta.currentContent || ''}\n\nCheck for grammar, spelling, style consistency, and overall quality. Provide a thorough evaluation and specific recommendations based on the user's request. Include both strengths and areas for improvement in your assessment.`
    case 'InlineEditor':
      // 內聯 AI 編輯
      return `You are an AI writing assistant. Please help me improve the following selected text based on my instruction.\n\nInstruction: ${meta.userPrompt || ''}\n\nSelected Text:\n${meta.selectedText || ''}\n\nContext Title: ${meta.currentTitle || ''}\nContext Content: ${meta.currentContent || ''}\n\nReturn only a JSON code block with improved_text, explanation, and suggestions.`
    default:
      return meta.userPrompt || ''
  }
} 