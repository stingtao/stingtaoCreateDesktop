import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { saveBlogArticle } from '../../../lib/content'
import { buildAIPrompt, AgentType } from '../../../composables/useAIPromptBuilder'

interface DraftGeneratorResponse {
  explanation: string;
  draftTitle: string;
  draftContent: string;
  suggestions: string[];
}

interface Message {
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
  referencedText?: string;
  isTyping?: boolean;
}

export function useAIConversation() {
  const conversationHistory = ref<Message[]>([])
  
  const userPrompt = ref('')
  const isApiKeySet = ref(false)
  const showApiKeyWarning = ref(false)
  
  const draftView = ref<{
    title: string;
    content: string;
    isVisible: boolean;
  }>({
    title: '',
    content: '',
    isVisible: false
  })
  
  const formatTimestamp = (date: Date) => {
    return date.toLocaleTimeString()
  }
  
  const checkApiKey = async () => {
    try {
      const result = await invoke<boolean>('check_gemini_api_key')
      isApiKeySet.value = result
      showApiKeyWarning.value = !result
    } catch (error) {
      console.error('Error checking API key:', error)
      isApiKeySet.value = false
      showApiKeyWarning.value = true
    }
  }
  
  const animateMessage = async (message: Message) => {
    if (message.role === 'assistant') {
      message.isTyping = true;
      const content = message.content;
      message.content = '';
      
      // Split content into words
      const words = content.split(/(\s+)/);
      
      // Type each word with a delay
      for (const word of words) {
        message.content += word;
        // Add a small delay between words
        await new Promise(resolve => setTimeout(resolve, 50));
      }
      
      message.isTyping = false;
    }
  };
  
  const sendMessage = async (
    prompt: string,
    selectedText?: string,
    agentType: string = 'Editor',
    blogId?: number,
    conversationHistoryRef?: HTMLElement,
    projectData?: any,
    currentTitle: string = '',
    currentContent: string = ''
  ) => {
    if (!prompt.trim()) return
    
    // Add validation for blogId
    if (blogId === undefined || blogId <= 0) {
      // Save the article first to get a blog ID
      try {
        // Get project ID from projectData
        if (!projectData || !projectData.id) {
          console.error('No valid project data found');
          conversationHistory.value.push({
            role: 'assistant',
            content: 'Error: No valid project data found. Please make sure you are creating an article within a project.',
            timestamp: new Date()
          });
          return;
        }
        
        const articleToSave = {
          id: 0,
          title: currentTitle,
          content: currentContent,
          project_id: projectData.id,
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
          keywords: ''
        }
        
        const savedArticle = await saveBlogArticle(articleToSave)
        blogId = savedArticle.id
        
        // Add a message to inform the user
        conversationHistory.value.push({
          role: 'assistant',
          content: 'Article saved successfully. Proceeding with your request...',
          timestamp: new Date()
        });
      } catch (error) {
        console.error('Failed to save article:', error)
        conversationHistory.value.push({
          role: 'assistant',
          content: 'Error: Failed to save the article. Please try again.',
          timestamp: new Date()
        });
      return
      }
    }
    
    // Add user message to conversation history
    conversationHistory.value.push({
      role: 'user',
      content: prompt,
      timestamp: new Date(),
      referencedText: selectedText
    });

    try {
      let response: string = '';
      // 統一用 buildAIPrompt 組合 prompt
      const aiPrompt = buildAIPrompt(agentType as AgentType, {
        project: projectData || undefined,
        currentTitle,
        currentContent,
        selectedText,
        userPrompt: prompt
      })
      // 根據不同的 agentType 調用不同的後端函數
      switch (agentType) {
        case 'DraftGenerator':
          response = await invoke<string>('generate_article_draft', {
            prompt: aiPrompt,
            selectedText: selectedText || '',
            blogId,
            projectData: projectData || null,
            currentTitle,
            currentContent
          });
          const trimmedResponse = response.trim();
          let jsonStringToParse: string | null = null;
          if (trimmedResponse.startsWith('```json')) {
            const match = trimmedResponse.match(/```json[\r\n]+([\s\S]+?)\r?\n```/)
            if (match && match[1]) jsonStringToParse = match[1]
          } else if (trimmedResponse.startsWith('{')) {
            jsonStringToParse = trimmedResponse
          }
          if (jsonStringToParse) {
            try {
              const parsed = JSON.parse(jsonStringToParse)
              draftView.value.title = parsed.draftTitle || ''
              draftView.value.content = parsed.draftContent || ''
              draftView.value.isVisible = true
            } catch (e) {
              // fallback: 顯示原始內容
              draftView.value.title = ''
              draftView.value.content = trimmedResponse
              draftView.value.isVisible = true
            }
          } else {
            draftView.value.title = ''
            draftView.value.content = trimmedResponse
            draftView.value.isVisible = true
          }
          break;
        case 'Editor':
        case 'Reviewer':
        case 'InlineEditor':
        default:
          // 其他 agentType 統一用 aiPrompt
          response = await invoke<string>('generate_article_draft', {
            prompt: aiPrompt,
            selectedText: selectedText || '',
            blogId,
            projectData: projectData || null,
            currentTitle,
            currentContent
          });
          conversationHistory.value.push({
            role: 'assistant',
            content: response,
            timestamp: new Date()
          });
          break;
      }
    } catch (error) {
      console.error('Error generating response:', error)
      // Add error message with animation
      const errorMessage: Message = {
        role: 'assistant',
        content: 'Sorry, I encountered an error while processing your request. Please try again.',
        timestamp: new Date()
      };
      conversationHistory.value.push(errorMessage);
      await animateMessage(errorMessage);
    }
  }
  
  const onTitleUpdate = ref<((value: string) => void) | null>(null);
  const onContentUpdate = ref<((value: string) => void) | null>(null);
  
  const setUpdateCallbacks = (
    titleCallback: (value: string) => void,
    contentCallback: (value: string) => void
  ) => {
    onTitleUpdate.value = titleCallback;
    onContentUpdate.value = contentCallback;
  };
  
  // Initialize by checking API key
  checkApiKey()
  
  return {
    conversationHistory,
    userPrompt,
    isApiKeySet,
    showApiKeyWarning,
    formatTimestamp,
    checkApiKey,
    sendMessage,
    draftView,
    setUpdateCallbacks
  }
} 