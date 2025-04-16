import { ref } from 'vue'

export function useTextEditor() {
  const selectedText = ref('')
  const selectedTextSource = ref<'title' | 'content' | null>(null)
  const selectedTextStartLine = ref(0)
  const selectedTextEndLine = ref(0)
  const selectedTextStart = ref(0)
  const selectedTextEnd = ref(0)
  
  const showContentFloatingHint = ref(false)
  const showTitleFloatingHint = ref(false)
  const contentFloatingHintPosition = ref({
    top: '0px',
    left: '0px'
  })
  const titleFloatingHintPosition = ref({
    top: '0px',
    left: '0px'
  })

  const calculateSelectionPosition = (element: HTMLTextAreaElement | HTMLInputElement) => {
    const start = element.selectionStart ?? 0
    const end = element.selectionEnd ?? 0
    
    // Create a temporary span to measure text dimensions
    const span = document.createElement('span')
    span.style.visibility = 'hidden'
    span.style.position = 'absolute'
    span.style.whiteSpace = 'pre-wrap'
    span.style.wordWrap = 'break-word'
    span.style.width = `${element.clientWidth}px`
    span.style.font = window.getComputedStyle(element).font
    document.body.appendChild(span)
    
    // Get the text before the selection
    const textBeforeSelection = element.value.substring(0, start)
    
    // Calculate the position of the selection
    const lines = textBeforeSelection.split('\n')
    const lastLine = lines[lines.length - 1]
    
    // Set the span content to the last line of text before selection
    span.textContent = lastLine
    
    // Get the position of the last character before selection
    const lastCharWidth = span.offsetWidth / lastLine.length
    const lastCharPosition = lastCharWidth * lastLine.length
    
    // Calculate the position of the selection
    const elementRect = element.getBoundingClientRect()
    const lineHeight = parseInt(window.getComputedStyle(element).lineHeight)
    
    // Calculate the top position based on the number of lines
    const topPosition = elementRect.top + (lines.length - 1) * lineHeight
    
    // Calculate the left position based on the last character position
    const leftPosition = elementRect.left + lastCharPosition
    
    // Calculate the width and height of the selection
    const fullText = element.value
    let adjustedEnd = end
    
    // Check if the selection ends with a line break
    if (end > 0 && end <= fullText.length && fullText[end - 1] === '\n') {
      adjustedEnd = end - 1
    }
    
    const selectedText = fullText.substring(start, adjustedEnd)
    const selectedLines = selectedText.split('\n')
    
    // Calculate the width of the selection (only for the first line)
    const firstLine = selectedLines[0]
    span.textContent = firstLine
    const actualSelectionWidth = span.offsetWidth
    
    const selectionWidth = Math.min(
      element.clientWidth - lastCharPosition,
      actualSelectionWidth
    )
    
    // Calculate the height of the selection (number of lines)
    const selectionHeight = selectedLines.length * lineHeight
    
    // Clean up
    document.body.removeChild(span)
    
    return {
      top: topPosition,
      left: leftPosition,
      right: leftPosition + selectionWidth,
      bottom: topPosition + selectionHeight,
      width: selectionWidth,
      height: selectionHeight
    }
  }

  const handleTextSelection = (
    source: 'title' | 'content',
    element: HTMLInputElement | HTMLTextAreaElement,
    lineNumbersRef?: HTMLElement
  ) => {
    console.log(`Text selection detected in ${source}`)
    
    // Hide all floating hints first
    showContentFloatingHint.value = false
    showTitleFloatingHint.value = false
    
    const start = element.selectionStart ?? 0
    const end = element.selectionEnd ?? 0
    
    console.log(`Selection: ${start} to ${end}`)
    
    if (start !== end) {
      // Get the full text content
      const fullText = element.value
      
      // Check if the selection ends with a line break
      let adjustedEnd = end
      if (end > 0 && end <= fullText.length && fullText[end - 1] === '\n') {
        adjustedEnd = end - 1
        console.log(`Adjusted end from ${end} to ${adjustedEnd} to exclude line break`)
      }
      
      // Get the selected text with the adjusted end
      selectedText.value = fullText.substring(start, adjustedEnd)
      selectedTextSource.value = source
      
      // Calculate line numbers for selected text (only for content)
      if (source === 'content') {
        const textBeforeSelection = fullText.substring(0, start)
        const linesBeforeSelection = textBeforeSelection.split('\n')
        selectedTextStartLine.value = linesBeforeSelection.length
        
        const selectedTextLines = selectedText.value.split('\n')
        selectedTextEndLine.value = selectedTextStartLine.value + selectedTextLines.length - 1
        
        console.log(`Lines: ${selectedTextStartLine.value} to ${selectedTextEndLine.value}`)
      }
      
      console.log(`Selected text: "${selectedText.value.substring(0, 30)}..."`)
      
      // Get the element's position relative to the viewport
      const elementRect = element.getBoundingClientRect()
      
      // Calculate the position of the hint
      if (source === 'title') {
        // For title, position the hint below the title input
        titleFloatingHintPosition.value = {
          top: `${elementRect.bottom + 5}px`,
          left: `${elementRect.left}px`
        }
        showTitleFloatingHint.value = true
      } else {
        // For content, calculate the position of the selected text
        const selectionPosition = calculateSelectionPosition(element)
        
        // Get the line numbers element position
        const lineNumbersRect = lineNumbersRef?.getBoundingClientRect()
        
        // Implement smart positioning based on the selection position
        const hintWidth = 200 // Approximate width of the floating hint
        const hintHeight = 40 // Approximate height of the floating hint
        const windowWidth = window.innerWidth
        const windowHeight = window.innerHeight
        
        // Position the hint near the line number
        let hintLeft = 0
        let hintTop = 0
        
        if (lineNumbersRect) {
          // Position the hint to the right of the line numbers
          hintLeft = lineNumbersRect.right + 10
          
          // Get the line number element for the selected line
          const lineNumberElements = lineNumbersRef?.querySelectorAll('.line-number')
          const selectedLineIndex = selectedTextStartLine.value - 1 // 0-based index
          
          if (lineNumberElements && lineNumberElements.length > selectedLineIndex) {
            // Get the position of the selected line number
            const selectedLineNumberRect = lineNumberElements[selectedLineIndex].getBoundingClientRect()
            
            // Position the hint vertically aligned with the selected line number
            hintTop = selectedLineNumberRect.top
            
            // Adjust the hint position to be slightly above the line number
            hintTop = hintTop - hintHeight - 5
            
            // If the hint would go off the top of the window, position it below the line number
            if (hintTop < 0) {
              hintTop = selectedLineNumberRect.bottom + 5
            }
          } else {
            // Fallback if we can't find the line number element
            hintTop = selectionPosition.top - hintHeight - 5
            
            // If the hint would go off the top of the window, position it below the selection
            if (hintTop < 0) {
              hintTop = selectionPosition.bottom + 5
            }
          }
          
          // If the hint would go off the right edge of the window, position it to the left of the line numbers
          if (hintLeft + hintWidth > windowWidth) {
            hintLeft = lineNumbersRect.left - hintWidth - 10
          }
          
          // Ensure the hint stays within the vertical bounds of the window
          if (hintTop + hintHeight > windowHeight) {
            hintTop = windowHeight - hintHeight - 10
          }
          if (hintTop < 0) {
            hintTop = 10
          }
        } else {
          // Fallback to the previous positioning logic if line numbers element is not available
          hintLeft = selectionPosition.left + (selectionPosition.width / 2) - (hintWidth / 2)
          hintTop = selectionPosition.top - hintHeight - 5
          
          // If the hint would go off the top of the window, position it below the selection
          if (hintTop < 0) {
            hintTop = selectionPosition.bottom + 5
          }
          
          // Ensure the hint stays within the horizontal bounds of the window
          if (hintLeft < 0) {
            hintLeft = 10
          } else if (hintLeft + hintWidth > windowWidth) {
            hintLeft = windowWidth - hintWidth - 10
          }
          
          // Ensure the hint stays within the vertical bounds of the window
          if (hintTop + hintHeight > windowHeight) {
            hintTop = windowHeight - hintHeight - 10
          }
        }
        
        contentFloatingHintPosition.value = {
          top: `${hintTop}px`,
          left: `${hintLeft}px`
        }
        showContentFloatingHint.value = true
      }
    } else {
      console.log(`No text selected in ${source}, hiding floating hint`)
      if (source === 'title') {
        showTitleFloatingHint.value = false
      } else {
        showContentFloatingHint.value = false
      }
    }
  }

  const clearSelectedText = () => {
    selectedText.value = ''
    selectedTextSource.value = null
    showContentFloatingHint.value = false
    showTitleFloatingHint.value = false
  }

  return {
    selectedText,
    selectedTextSource,
    selectedTextStartLine,
    selectedTextEndLine,
    selectedTextStart,
    selectedTextEnd,
    showContentFloatingHint,
    showTitleFloatingHint,
    contentFloatingHintPosition,
    titleFloatingHintPosition,
    calculateSelectionPosition,
    handleTextSelection,
    clearSelectedText
  }
} 