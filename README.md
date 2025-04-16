# StingtaoCreateDesktop

StingtaoCreateDesktop is an AI-powered desktop application that helps content creators write better blog articles. Built with Tauri, Vue 3, and Rust, it provides an intuitive interface with AI assistance for content creation.

## Key Features

- 🤖 AI-powered writing assistance with multiple specialized agents
- 📝 Three-column layout with resizable panels:
  - Project navigation
  - Markdown editor with live preview
  - AI assistant panel
- 💾 Auto-save with smart navigation protection
- ✨ Intelligent text selection tools for targeted AI help
- 🔄 Content history with undo/redo support
- 🎯 Project-based content organization

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://www.rust-lang.org/tools/install)
- [Google Gemini API Key](https://deepmind.google/technologies/gemini/)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/StingtaoCreateDesktop.git
   cd StingtaoCreateDesktop
   ```

2. Install dependencies:
   ```bash
   npm install
   ```


4. Start the development server:
   ```bash
   npm run tauri dev
   ```

### Building for Production

```bash
npm run tauri build
```

## Using StingtaoCreateDesktop

### 1. First Launch
- Launch the application
- Go to Settings and enter your Gemini API key
- Create your first blog project

### 2. Creating a Blog Project
- Click "New Project" in Home
- Fill in project details:
  - Project name
  - Category
  - Target audience
  - Content goals
  - Keywords
  ...

### 3. Writing Articles
- Select your project in the sidebar
- Click + to create "New Article"

- Access AI features by:
  - Selecting text and using the floating toolbar
  - Using the AI Draft Generator agent to help you draft the whole article based on your project settings and current draft.

### 4. AI Assistant Features
- **Draft Generation**: Get AI-generated article drafts
- **Content Planning**: Receive structure suggestions (TODO)
- **Content Analysis**: Get feedback on writing style(TODO)
- **Style Adjustment**: Refine tone and readability(TODO)
- **Final Review**: Comprehensive content check(TODO)

### 5. Auto-save and Navigation
- Content saves automatically every 10 seconds
- Unsaved changes are protected when navigating


## Project Structure

```
StingtaoCreateDesktop/
├── src/                      # Frontend Vue code
│   ├── components/          # Vue components
│   │   ├── blog-article/   # Article editing components
│   │   └── layouts/        # Layout components
│   ├── composables/         # Vue composables
│   └── lib/                # Shared utilities
├── src-tauri/              # Rust backend code
│   └── src/
│       ├── ai_agent.rs    # AI integration
│       ├── db.rs         # SQLite database handling
│       └── main.rs       # Main Rust entry
└── package.json
```



## License

This project is licensed under the MIT License 

## Acknowledgments

- [Tauri](https://tauri.app/) - Desktop framework
- [Vue 3](https://vuejs.org/) - UI framework
- [Google Gemini](https://deepmind.google/technologies/gemini/) - AI capabilities
- [SQLite](https://www.sqlite.org/) - Local database
