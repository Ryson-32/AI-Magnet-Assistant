[简体中文](README.zh-CN.md)
**Disclaimer:** A significant portion of this project was developed with the assistance of AI Development Tools. If you are interested in these tools, we welcome you to visit [https://ai.pu.edu.kg/](https://ai.pu.edu.kg/) to rate and review them.

# AI Magnet Assistant

## Introduction

**AI Magnet Assistant** is an intelligent magnet link search and optimization tool built with Rust, Tauri, and Vue.js. It leverages the power of AI to provide users with a streamlined and efficient search experience by aggregating results from multiple search engines, filtering out invalid links, and ranking them for quality.

## Key Features

-   **Multi-Engine Search**: Conducts parallel searches across various magnet link providers to ensure comprehensive results.
-   **AI-Powered Optimization**: Utilizes AI algorithms to analyze, score, and rank search results, prioritizing the most relevant and reliable links.
-   **Invalid Link Filtering**: Automatically identifies and removes dead or invalid links, saving you time and effort.
-   **Intuitive User Interface**: A clean, modern, and user-friendly interface built with Vue.js for a seamless user experience.
-   **Cross-Platform**: Built with Tauri, allowing the application to run on Windows, macOS, and Linux from a single codebase.

## Getting Started

### Prerequisites

-   [Node.js](https://nodejs.org/en/)
-   [Rust](https://www.rust-lang.org/tools/install)

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/your-username/ai-magnet-assistant.git
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
