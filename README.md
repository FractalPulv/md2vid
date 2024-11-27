# md2vid

**md2vid** is a journaling-inspired app designed to turn your daily notes into a visual and auditory time machine. The idea originated from my use of the Daily Notes plugin in Obsidian, where I would attach YouTube URLs and photos to my daily notes. This project aims to take that concept a step further by transforming your notes into immersive videos, complete with associated music and visuals.

If you're an avid music listener who doesn't replay the same song every day, this app can help you recapture the mindset of your past self through the combination of text, images, and music.

---

## Features

### Core Functionality

- **Daily Note Integration**: Displays all your daily notes (`*.md` files) from a specified directory.
- **YAML Property Rendering**:
    - Displays `cover_url` as a cover image (if specified).
    - Displays `Rating`, `title`, and other custom YAML metadata.
- **Obsidian Integration**: Includes a button to open selected notes directly in Obsidian via Obsidian URI.

### Video Generation

- **Sentence Segmentation**: Splits the content of each note into individual sentences.
- **Text Rendering**:
    - Each sentence is rendered individually as part of the video.
    - Markdown syntax like **bold** and _italic_ is fully supported, thanks to `.ass` file styling.
- **Image Handling**:
    - Downloads hosted images and integrates them into relevant sentences.
    - Handles local images by creating temporary copies from a specified attachments directory.
- **Audio Integration**:
    - Downloads the associated YouTube video as an audio file.
    - Combines the audio with the generated video.

### Workflow

1. Parse the content of your daily note.
2. Generate individual videos for each sentence.
3. Concatenate all sentence videos into a single cohesive video.
4. Attach the downloaded audio to the final video.

### User Feedback

- **Progress Tracking**:
    - Displays estimated time remaining during video generation.
    - Keeps users informed with detailed status updates.

---

## Why Tauri?

I chose Tauri for this project because it's a lightweight and performant alternative to Electron. Instead of relying on Chromium, Tauri uses native webviews and Rust as the backend, making it more efficient and resource-friendly. This is my second time using Rust, so I'm still learning, but it's been a rewarding experience so far!
