# StingtaoCreateDesktop

StingtaoCreateDesktop is your personal writing companion that helps streamline your content creation process. Easily plan your writing projects by defining goals, managing time commitments, and identifying target audiences. With powerful content creation tools, you'll write better quality content in less time.

I developed this tool initially to enhance my own English blog writing experience, and I'm actively expanding its capabilities. As a fellow content creator, I'd love to hear your feedback and suggestions to make StingtaoCreateDesktop even better for our writing community.

Feel free to open an issue or reach out with your ideas - your input will help shape the future of this tool!


## Key Features

- 🤖 Streamline Your Writing Process - Transform your ideas into well-structured content with our comprehensive project planning tools. Whether you're writing a blog post, book, speech, novel, or screenplay, our platform helps you organize your thoughts and create professional-quality content.

- 📝 Instant AI Writing Enhancement - Get real-time AI assistance as you write. Simply select any text and let AI refine your writing, improving clarity, tone, and impact while maintaining your unique voice. No more switching between multiple apps or waiting for feedback.

- 💾 Turn Ideas into Polished Drafts - Don't let writer's block hold you back.  Draft Generator turns your rough ideas and bullet points into complete first drafts, giving you a solid foundation to build upon. Save hours of time and overcome the blank page syndrome.

- 🎯 Achieve Professional Results - Whether you're a blogger, author, or content creator, our tools help you produce higher quality content more efficiently. Focus on what matters most - sharing your message with the world.

## Screenshots

### Application Interface
![Project Interface](screenshots/Screenshot%202025-04-17%20at%202.27.55%E2%80%AFAM.png)

### Project Management
![Project Management](screenshots/Screenshot%202025-04-17%20at%202.28.25%E2%80%AFAM.png)

### Article Editor
![Article Editor](screenshots/Screenshot%202025-04-17%20at%202.29.42%E2%80%AFAM.png)

### AI Assistant Panel
![AI Assistant](screenshots/Screenshot%202025-04-17%20at%202.30.14%E2%80%AFAM.png)

### Easy management
![Content Creation](screenshots/Screenshot%202025-04-17%20at%202.31.12%E2%80%AFAM.png)

### Select sentences for AI to assist you
![inline support](screenshots/Screenshot%202025-04-17%20at%202.33.39%E2%80%AFAM.png)

### Write your prompt while you are thinking with your work
![Seamless integration](screenshots/Screenshot%202025-04-17%20at%202.33.52%E2%80%AFAM.png)

## Download and Installation

### Pre-built Applications

#### Windows
Download the installer package:
- Latest release (v0.1.0): [StingtaoCreateDesktop-0.1.0-x64.msi](https://github.com/stingtao/stingtaoCreateDesktop/blob/main/src-tauri/target/release/bundle/msi/stingtaocreatedesktop_0.1.0_x64_en-US.msi)
- Run the installer and follow the prompts to complete installation

#### macOS
Download the application bundle:
- Latest release: Coming soon
- Drag the .app file to your Applications folder

### Building from Source

If you prefer to build from source code, ensure you have the following prerequisites installed:

#### System Requirements
- [Node.js](https://nodejs.org/) v16.0.0 or higher
- [Rust](https://www.rust-lang.org/tools/install) latest stable version
- [Google Gemini API Key](https://deepmind.google/technologies/gemini/) for AI features

#### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/stingtao/StingtaoCreateDesktop.git
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
