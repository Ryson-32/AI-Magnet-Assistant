[简体中文](README.zh-CN.md)
**Disclaimer:** Most of the code in this project was completed with the assistance of AI Development Tools. If you are interested in these AI tools, you are welcome to visit [https://ai.pu.edu.kg/](https://ai.pu.edu.kg/) to rate and review them.

# AI Magnet Assistant

## Introduction

**AI Magnet Assistant** is an intelligent magnet link search and optimization tool built with Rust, Tauri, and Vue.js. It leverages the power of AI to provide users with a smooth and efficient search experience by aggregating results from multiple search engines, filtering invalid links, and ranking them for quality.

## Key Features

-   **Multi-Engine Search**: Performs parallel searches across multiple magnet link providers to ensure comprehensive results.
-   **AI-Driven Optimization**: Utilizes AI algorithms to analyze, score, and rank search results, prioritizing the most relevant and reliable links.
-   **Invalid Link Filtering**: Automatically identifies and removes dead or invalid links, saving you time and effort.
-   **Intuitive User Interface**: A clean, modern, and user-friendly interface built with Vue.js.
-   **Cross-Platform Support**: Built with Tauri, allowing the application to run on Windows, macOS, and Linux from a single codebase.
-   **Extensible Search Engine**: Includes the built-in `clmclm.com` search engine and supports adding custom sites (Note: Some sites may have anti-scraping measures, and custom sites are analyzed by AI processing the entire HTML, which can be slow).
-   **Gemini API Integration**: Currently, only Gemini API integration is supported. It is recommended to use models that support high concurrency (e.g., `gemini-2.5-flash`) for optimal performance.

## Getting Started

### Prerequisites

-   [Node.js](https://nodejs.org/en/)
-   [Rust](https://www.rust-lang.org/tools/install)

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/Ryson-32/AI-Magnet-Assistant.git
    cd ai-magnet-assistant
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

### Building the Application

To build the application for your current platform:

```bash
npm run tauri build
```

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.
