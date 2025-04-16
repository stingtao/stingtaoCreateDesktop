import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { saveBlogArticle } from '../../../lib/content'

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
      
      // 調用後端 API 時使用與 Rust 函數參數相匹配的名稱 (camelCase)
      // 因為 Tauri 序列化機制期望參數名稱與函數參數完全一致
      const params = {
        prompt,
        selectedText: selectedText || '',
        blogId,
        projectData: projectData || null,
        currentTitle,
        currentContent
      };
      
      console.log(`Sending request with agent type: ${agentType}`);
      console.log(`Parameters:`, params);
      
      // 根據不同的 agentType 調用不同的後端函數
      switch (agentType) {
        case 'DraftGenerator':
          const rawResponse = await invoke<string>('generate_article_draft', params);
          let jsonStringToParse: string | null = null;
          const trimmedResponse = rawResponse.trim();
          
          try {
            // Attempt 1: Maybe it's already a JSON object string?
            const parsedDirectly = JSON.parse(trimmedResponse);
            if (typeof parsedDirectly === 'object' && parsedDirectly !== null) {
              // If direct parsing works, it means the string was like '{...}'
              jsonStringToParse = trimmedResponse;
            } else if (typeof parsedDirectly === 'string') {
              // Attempt 2: Maybe it's a string containing escaped JSON ("{...}")?
              // If parsing succeeded and it's a string, parse *that* string.
              jsonStringToParse = parsedDirectly;
               // Verify this extracted string is parseable JSON before proceeding
               try {
                  JSON.parse(jsonStringToParse);
               } catch (eInner) {
                  console.error('Inner string is invalid JSON:', jsonStringToParse, eInner);
                  jsonStringToParse = null; // Mark as failed
               }
            } else {
              // Parsed to something unexpected (number, boolean, null)
              console.warn('Parsed response is not a JSON object or a string containing JSON.');
              // Fall through to try markdown extraction
            }
          } catch (e1) {
            // Direct parsing failed or parsing the inner string failed.
             console.log('Direct JSON parse failed or result was not string/object, trying markdown.', e1)
            // Attempt 3: Check for markdown code block
            const jsonMatch = trimmedResponse.match(/```json\n([\s\S]*?)\n```/);
            if (jsonMatch && jsonMatch[1]) {
              jsonStringToParse = jsonMatch[1].trim();
               // Verify this extracted string is parseable JSON before proceeding
               try {
                  JSON.parse(jsonStringToParse);
               } catch (e2) {
                  console.error('Extracted markdown JSON is invalid:', jsonStringToParse, e2);
                  jsonStringToParse = null; // Mark as failed
               }
            }
          }


          if (jsonStringToParse === null) {
            console.error('No valid JSON found in response after multiple checks:', rawResponse);
            // Add assistant message indicating failure to parse
            conversationHistory.value.push({
                role: 'assistant',
                content: 'Sorry, the response from the AI was not in the expected format and could not be processed. Please check the format or try again. Raw response: ' + rawResponse,
                timestamp: new Date()
            });
            return; // Exit the function gracefully
    }
    
    try {
            // Clean potential invalid control characters before parsing
            // Removes characters in the range U+0000-U+001F, excluding tab, newline, carriage return
            jsonStringToParse = jsonStringToParse.replace(/[\u0000-\u0008\u000B\u000C\u000E-\u001F]/g, '');
            
            // Parse the final JSON string
            const draftResponse = JSON.parse(jsonStringToParse) as DraftGeneratorResponse;
            
            // Add null checks and default values
            const explanation = draftResponse?.explanation || 'No explanation provided.';
            const suggestions = draftResponse?.suggestions || [];
            const draftTitle = draftResponse?.draftTitle || '';
            const draftContent = draftResponse?.draftContent || '';
            
            // Format explanation and suggestions with markdown
            const formattedExplanation = `### Draft Generation Explanation\n${explanation}\n\n### Suggestions\n${suggestions.map(s => `- ${s}`).join('\n')}`;
            
            // Add formatted response to conversation history
            conversationHistory.value.push({
              role: 'assistant',
              content: formattedExplanation,
              timestamp: new Date()
            });
            
            // Show draft view - update each property individually to ensure reactivity
            draftView.value = {
              title: draftTitle,
              content: draftContent,
              isVisible: true
            };
            
            console.log('Updated draftView in sendMessage:', {
              isVisible: draftView.value.isVisible,
              title: draftView.value.title,
              content: draftView.value.content
            });
            
            // response = formattedExplanation; // Ensure 'response' is handled if needed elsewhere
          } catch (parseError) {
            console.error('Error parsing final JSON string:', parseError, 'String content:', jsonStringToParse);
             // Add assistant message indicating failure to parse
      conversationHistory.value.push({
                role: 'assistant',
                content: 'Sorry, the response from the AI was not in the expected format and could not be processed after attempting cleanup. Please check the format or try again. Content: ' + jsonStringToParse,
                timestamp: new Date()
            });
            return; // Exit the function gracefully
          }
          break;
          
        case 'Planning':
          response = await invoke<string>('plan_article_content', params);
          break;
          
        case 'Research':
          response = await invoke<string>('analyze_article_content', params);
          break;
          
        case 'Reviewer':
          response = await invoke<string>('review_article_final', params);
          break;
          
        case 'Editor':
        default:
          response = await invoke<string>('adjust_article_style', params);
          break;
      }

      // Add assistant message with animation
      const assistantMessage: Message = {
        role: 'assistant',
        content: response,
        timestamp: new Date()
      };
      conversationHistory.value.push(assistantMessage);
      await animateMessage(assistantMessage);
      
      // Scroll to bottom of conversation history
      if (conversationHistoryRef) {
        setTimeout(() => {
        conversationHistoryRef.scrollTop = conversationHistoryRef.scrollHeight
        }, 100)
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