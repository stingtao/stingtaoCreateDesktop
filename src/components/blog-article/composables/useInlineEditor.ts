import { ref, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

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
    
    // Use fixed positioning relative to the content textarea
    const contentTextarea = document.querySelector('.content-editor textarea') as HTMLElement
    if (contentTextarea) {
      const rect = contentTextarea.getBoundingClientRect()
      inlineEditorPosition.value = {
        left: `${rect.left + (rect.width / 2)}px`,
        top: `${rect.top - 10}px` // 10px above the content textarea
      }
    } else {
      // Fallback to the provided position
      inlineEditorPosition.value = position
    }
    
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
      // Use the inline editor agent
      const response = await invoke<string>('inline_edit_text', {
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
      console.error('Error calling inline_edit_text:', error)
    }
  }
  
  const positionDiffView = (diffView: HTMLElement) => {
    // Use fixed positioning relative to the content textarea
    const contentTextarea = document.querySelector('.content-editor textarea') as HTMLElement
    if (contentTextarea) {
      const rect = contentTextarea.getBoundingClientRect()
      // Position the diff view at the center of the textarea
      // We're no longer using transform: translateX(-50%) in the CSS
      // so we need to adjust the position to account for the diff view's width
      const diffViewWidth = diffView.offsetWidth || 300 // Default to 300px if width is not available
      diffViewPosition.value = {
        left: `${rect.left + (rect.width / 2) - (diffViewWidth / 2)}px`,
        top: `${rect.top - 10}px` // 10px above the content textarea
      }
    } else {
      // Fallback to window center
      const windowWidth = window.innerWidth
      const windowHeight = window.innerHeight
      const diffViewWidth = diffView.offsetWidth || 300 // Default to 300px if width is not available
      const diffViewHeight = diffView.offsetHeight || 200 // Default to 200px if height is not available
      
      diffViewPosition.value = {
        left: `${(windowWidth - diffViewWidth) / 2}px`,
        top: `${(windowHeight - diffViewHeight) / 2}px`
      }
    }
  };
  
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
    setAddToConversationHistory
  }
} 