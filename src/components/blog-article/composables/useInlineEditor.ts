import { ref, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { buildAIPrompt } from '../../../composables/useAIPromptBuilder'

export function useInlineEditor() {
  const showInlineEditor = ref(false)
  const inlineEditorPosition = ref({
    top: '0px',
    left: '0px'
  })
  const inlineEditPrompt = ref('')
  
  const inlineEditResponse = ref<{
    explanation: string
    improved_text: string
    suggestions: string[]
  } | null>(null)
  
  const showDiffView = ref(false)
  const diffViewPosition = ref({
    top: '0px',
    left: '0px'
  })
  
  // 添加選中文本的位置和長度追蹤
  const selectedTextInfo = ref<{
    start: number
    end: number
    text: string
    source: 'title' | 'content'
  } | null>(null)
  
  // 設置選中文本信息
  const setSelectedTextInfo = (info: {
    start: number
    end: number
    text: string
    source: 'title' | 'content'
  }) => {
    selectedTextInfo.value = info
    console.log('Selected text info updated:', info)
  }
  
  // 清除選中文本信息
  const clearSelectedTextInfo = () => {
    selectedTextInfo.value = null
  }
  
  // 更新內容的函數
  const updateContent = (
    start: number,
    end: number,
    newText: string,
    source: 'title' | 'content'
  ) => {
    if (!selectedTextInfo.value) return
    
    console.log('Updating content:', {
      start,
      end,
      newText,
      source,
      currentInfo: selectedTextInfo.value
    })
    
    // 返回更新後的內容和新的選中範圍
    return {
      start,
      end: start + newText.length,
      text: newText
    }
  }
  
  // 添加一個回調函數，用於將 AI 響應添加到對話歷史
  const addToConversationHistory = ref<((message: { role: string, content: string, referencedText?: string }) => void) | null>(null)
  
  // 設置回調函數
  const setAddToConversationHistory = (callback: (message: { role: string, content: string, referencedText?: string }) => void) => {
    addToConversationHistory.value = callback
  }
  
  const showInlineEdit = (source: 'title' | 'content', position: { top: string, left: string }) => {
    showInlineEditor.value = true
    
    // Use the provided position directly
    inlineEditorPosition.value = position
    
    // Set a default prompt based on the source
    if (source === 'title') {
      inlineEditPrompt.value = `Help me improve this title`
    } else {
      inlineEditPrompt.value = `Help me improve this sentence`
    }
  }
  
  const hideInlineEdit = () => {
    showInlineEditor.value = false
    inlineEditPrompt.value = ''
  }
  
  // 提取 JSON 內容的輔助函數
  const extractJsonFromMarkdown = (text: string): string => {
    // 檢查是否包含 Markdown 代碼塊
    const jsonBlockRegex = /```(?:json)?\s*(\{[\s\S]*?\})\s*```/
    const match = text.match(jsonBlockRegex)
    
    if (match && match[1]) {
      // 如果找到代碼塊，返回代碼塊內的 JSON 內容
      return match[1]
    }
    
    // 如果沒有找到代碼塊，嘗試直接解析整個文本
    return text
  }
  
  const submitInlineEdit = async (
    selectedText: string,
    articleTitle: string,
    articleContent: string,
    prompt: string,
    blogId: number,
    conversationHistoryRef?: HTMLElement
  ) => {
    console.log('submitInlineEdit started')
    if (!prompt.trim() || !selectedText) {
      console.log('Submit conditions not met:', { 
        prompt, 
        selectedText 
      })
      return
    }
    console.log('Preparing to submit inline edit request:', {
      prompt,
      selectedText,
      blogId
    })
    // Hide the inline editor
    hideInlineEdit()
    // Scroll to bottom of conversation
    if (conversationHistoryRef) {
      conversationHistoryRef.scrollTop = conversationHistoryRef.scrollHeight
    }
    try {
      console.log('Calling inline_edit_text function')
      // 用 buildAIPrompt 統一組合 prompt
      const aiPrompt = buildAIPrompt('InlineEditor', {
        currentTitle: articleTitle,
        currentContent: articleContent,
        selectedText,
        userPrompt: prompt
      })
      // Use the inline editor agent
      const response = await invoke<string>('inline_edit_text', {
        prompt: aiPrompt,
        selectedText,
        articleTitle,
        articleContent,
        userPrompt: prompt,
        blogId: Number(blogId)
      })
      
      console.log('Received inline_edit_text response:', response)
      
      // Parse the JSON response
      try {
        console.log('Attempting to parse JSON response')
        
        // 提取 JSON 內容
        const jsonContent = extractJsonFromMarkdown(response)
        console.log('Extracted JSON content:', jsonContent)
        
        // 解析 JSON
        const parsedResponse = JSON.parse(jsonContent)
        
        // Validate the response format
        if (!parsedResponse.improved_text) {
          console.error('Invalid response format: missing improved_text field')
          return
        }
        
        // Ensure all required fields are present
        const validatedResponse = {
          explanation: parsedResponse.explanation || 'No explanation provided.',
          improved_text: parsedResponse.improved_text,
          suggestions: Array.isArray(parsedResponse.suggestions) ? parsedResponse.suggestions : []
        }
        
        inlineEditResponse.value = validatedResponse
        console.log('JSON parsing successful:', inlineEditResponse.value)
        
        // 將 AI 響應添加到對話歷史
        if (addToConversationHistory.value) {
          // 構建完整的響應內容
          let responseContent = `I've improved the text as requested.\n\n`;
          responseContent += `**Original Text:**\n${selectedText}\n\n`;
          responseContent += `**Improved Text:**\n${validatedResponse.improved_text}\n\n`;
          responseContent += `**Explanation:**\n${validatedResponse.explanation}\n\n`;
          
          if (validatedResponse.suggestions.length > 0) {
            responseContent += `**Additional Suggestions:**\n`;
            validatedResponse.suggestions.forEach((suggestion: string, index: number) => {
              responseContent += `${index + 1}. ${suggestion}\n`;
            });
          }
          
          // 添加到對話歷史
          addToConversationHistory.value({
            role: 'assistant',
            content: responseContent,
            referencedText: selectedText
          });
        }
        
        // 顯示 diff 視圖
        console.log('Showing diff view')
        showDiffView.value = true
        
        // 定位 diff 視圖 - 使用 setTimeout 確保 DOM 已更新
        setTimeout(() => {
          const diffViewElement = document.querySelector('.diff-view') as HTMLElement
          if (diffViewElement) {
            console.log('Diff view element found, positioning it')
            positionDiffView(diffViewElement)
          } else {
            console.error('Diff view element not found in DOM')
          }
        }, 100)
      } catch (error) {
        console.error('JSON parsing error:', error)
      }
    } catch (error) {
      console.error('Error in submitInlineEdit:', error)
    }
  }
  
  const positionDiffView = (diffView: HTMLElement) => {
    // Get viewport dimensions
    const viewportWidth = window.innerWidth
    const viewportHeight = window.innerHeight
    
    // Get diff view dimensions
    const diffViewWidth = diffView.offsetWidth || 400 // Default to 400px if width is not available
    const diffViewHeight = diffView.offsetHeight || 300 // Default to 300px if height is not available
    
    // Get current mouse position from the last known position
    const mouseX = mousePosition.value?.x || viewportWidth / 2
    const mouseY = mousePosition.value?.y || viewportHeight / 2
    
    const OFFSET_X = 10 // Offset from mouse pointer
    const OFFSET_Y = 10 // Offset from mouse pointer
    
    // Calculate initial position
    let left = mouseX
    let top = mouseY + OFFSET_Y
    
    // Adjust horizontal position if it would go off screen
    if (left + diffViewWidth + OFFSET_X > viewportWidth) {
      // If it would go off the right edge, position it to the left of the mouse
      left = left - diffViewWidth - OFFSET_X
    } else {
      // Otherwise, position it to the right of the mouse
      left = left + OFFSET_X
    }
    
    // Adjust vertical position if it would go off screen
    if (top + diffViewHeight > viewportHeight) {
      // If it would go off the bottom, position it above the mouse
      top = mouseY - diffViewHeight - OFFSET_Y
    }
    
    // Ensure minimum distances from viewport edges
    left = Math.max(OFFSET_X, left)
    top = Math.max(OFFSET_Y, top)
    
    // Update position
    diffViewPosition.value = {
      left: `${left}px`,
      top: `${top}px`
    }
  }
  
  const acceptImprovedText = (
    selectedTextStart: number,
    selectedTextEnd: number,
    improvedText: string,
    updateContent: (start: number, end: number, text: string) => void
  ) => {
    if (!improvedText) return;
    
    // 更新內容為改進後的文本
    updateContent(selectedTextStart, selectedTextEnd, improvedText);
    
    // 隱藏 diff 視圖
    showDiffView.value = false;
    inlineEditResponse.value = null;
  };
  
  const rejectImprovedText = () => {
    // 隱藏 diff 視圖
    showDiffView.value = false;
    inlineEditResponse.value = null;
  };
  
  // Add mousePosition ref
  const mousePosition = ref<{ x: number; y: number }>({ x: 0, y: 0 })
  
  // Add function to update mouse position
  const updateMousePosition = (x: number, y: number) => {
    mousePosition.value = { x, y }
  }
  
  return {
    showInlineEditor,
    inlineEditorPosition,
    inlineEditPrompt,
    inlineEditResponse,
    showDiffView,
    diffViewPosition,
    selectedTextInfo,
    showInlineEdit,
    hideInlineEdit,
    submitInlineEdit,
    acceptImprovedText,
    rejectImprovedText,
    setSelectedTextInfo,
    clearSelectedTextInfo,
    updateContent,
    setAddToConversationHistory,
    updateMousePosition
  }
} 