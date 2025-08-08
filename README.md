[简体中文](README.zh-CN.md)
**Disclaimer:** Most of the code in this project was completed with the assistance of AI Development Tools. If you are interested in these AI tools, you are welcome to visit [https://ai.pu.edu.kg/](https://ai.pu.edu.kg/) to rate and review them.

# AI Magnet Assistant

## Introduction

**AI Magnet Assistant** is an intelligent magnet link search and optimization tool built with Rust, Tauri, and Vue.js. It leverages the power of AI to provide users with a smooth and efficient search experience by aggregating results from multiple search engines, filtering invalid links, and ranking them for quality.

## Key Features

-   **Multi-Engine Search**: Performs parallel searches across multiple magnet link providers to ensure comprehensive results.
-   **AI-Driven Optimization**: Uses an LLM to analyze, score, and rank results; cleans ads/noise from titles and normalizes them; auto-tags releases (e.g., `2160p`, `Chinese`, `Chinese Sub`, `WEB-DL`).
-   **Invalid Link Filtering**: Automatically identifies and removes dead or invalid links, saving you time and effort.
-   **Intuitive User Interface**: A clean, modern, and user-friendly interface built with Vue.js.
-   **Cross-Platform Support**: Built with Tauri, allowing the application to run on Windows, macOS, and Linux from a single codebase.
-   **Extensible Search Engine**: Includes the built-in `clmclm.com` search engine and supports adding custom sites (Note: Some sites may have anti-scraping measures, and custom sites are analyzed by AI processing the entire HTML, which can be slow).
-   **Gemini API Integration**: Currently, only Gemini API integration is supported. It is recommended to use `gemini-2.5-flash` for HTML content extraction and `gemini-2.5-flash-lite` for content analysis for optimal performance.
-   **Internationalization (i18n)**: Full English/Chinese support with runtime switching and backend persistence.
-   **Two-Phase AI Configuration**: Separate settings for HTML Extraction and Content Analysis, each with provider, API base, model, and API key; connectivity test buttons included.
-   **Batch Analysis with Fallback**: Parallel batch processing with graceful fallback to single-item analysis and real-time progress updates.
-   **Download Options**: Custom application path and auto-close page for 115 Browser workflow.

## Getting Started

### Prerequisites

-   [Node.js](https://nodejs.org/en/)
-   [Rust](https://www.rust-lang.org/tools/install)

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/Ryson-32/AI-Magnet-Assistant.git
    cd AI-Magnet-Assistant
    ```

2.  Install the dependencies:
    ```bash
    npm install
    ```

### Running in Development Mode

To run the application in development mode with hot-reloading:

```bash
npm run tauri dev
```

Alternatively, run only the frontend for UI development:

```bash
npm run vite:dev
```

### Building the Application

To build the application for your current platform:

```bash
npm run tauri build
```

## Internationalization

- Default locale: Simplified Chinese (`zh-CN`)
- Supported locales: `zh-CN`, `en`
- Switch language in Settings (or via the Debug Area if enabled). The selection is persisted on the backend and restored on startup.

## Release Notes

See `docs/RELEASES.md` for detailed changes. Current version: 1.2.0.

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.
