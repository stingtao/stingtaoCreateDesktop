<template>
  <div class="inspiration-quote" @click="rotateQuote">
    <div class="quote-content">
      <p>"{{ currentQuote.quote }}"</p>
      <p class="quote-author">- {{ currentQuote.author }}</p>
    </div>
    <div class="quote-hint">Click to see another inspiring quote</div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Quote {
  quote: string
  author: string
}

const quotes: Quote[] = [
  {
    quote: "A blog is only as valuable as the value it provides to its readers.",
    author: "Content Marketing Institute"
  },
  {
    quote: "The best time to start was yesterday. The next best time is now.",
    author: "Chinese Proverb"
  },
  {
    quote: "Your blog is your unedited version of yourself.",
    author: "Seth Godin"
  },
  {
    quote: "Don't focus on having a great blog. Focus on producing a blog that's great for your readers.",
    author: "Brian Clark"
  }
]

const currentQuoteIndex = ref(0)
const currentQuote = ref(quotes[0])

const rotateQuote = () => {
  currentQuoteIndex.value = (currentQuoteIndex.value + 1) % quotes.length
  currentQuote.value = quotes[currentQuoteIndex.value]
}
</script>

<style scoped>
.inspiration-quote {
  background-color: #f8f9fa;
  border-radius: 8px;
  padding: 1.5rem;
  margin-bottom: 2rem;
  cursor: pointer;
  transition: all 0.3s;
}

.inspiration-quote:hover {
  background-color: #e9ecef;
  transform: translateY(-2px);
}

.quote-content {
  text-align: center;
  font-style: italic;
}

.quote-content p {
  margin: 0;
  line-height: 1.6;
}

.quote-author {
  margin-top: 1rem;
  color: #666;
  font-size: 0.9rem;
}

.quote-hint {
  text-align: center;
  margin-top: 1rem;
  font-size: 0.8rem;
  color: #666;
}

/* Dark theme styles */
:global(body.dark) .inspiration-quote {
  background-color: #2c2c2c;
}

:global(body.dark) .inspiration-quote:hover {
  background-color: #3c3c3c;
}

:global(body.dark) .quote-author {
  color: #aaa;
}

:global(body.dark) .quote-hint {
  color: #aaa;
}

/* Responsive styles */
@media (max-width: 576px) {
  .inspiration-quote {
    padding: 1.25rem;
    margin-bottom: 1.5rem;
  }
  
  .quote-content p {
    font-size: 0.9rem;
  }
  
  .quote-author {
    font-size: 0.8rem;
  }
  
  .quote-hint {
    font-size: 0.75rem;
  }
}
</style> 