# Developer Manual for AI-Magnet-Assistant

This document provides a detailed guide for developers working on the AI-Magnet-Assistant project. It covers the project's architecture, technical stack, and development workflow.

## 1. Project Overview

**AI-Magnet-Assistant** is a smart magnet link search and optimization tool built on the Tauri framework. It aims to solve the problem of cluttered and low-quality information in traditional magnet link searches.

### Core Features

*   **Multi-Engine Aggregated Search:** Supports simultaneous searching from multiple configurable search engines (including the built-in `clmclm.com` and user-defined engines).
*   **AI-Powered Analysis and Filtering:** Utilizes a Large Language Model (LLM), such as Google Gemini, for in-depth analysis of search results, enabling:
    *   **Title Cleaning:** Automatically removes ads and irrelevant information from titles.
    *   **Content Purity Assessment:** Evaluates the "purity" of resource content based on the file list and assigns a score.
    *   **Smart Tagging:** Automatically generates content-related tags for resources.
*   **Result Optimization and Sorting:** Allows users to sort and filter search results based on AI scores, file size, relevance, and other criteria.
*   **Customization:** Users can define custom search engines, set priority keywords to influence search result ranking, and configure their own LLM API keys.
*   **Favorites Functionality:** Users can save magnet links of interest for later viewing and management.

## 2. Technical Stack Details

The project is architected with a separation of concerns between the frontend and backend.

### 2.1. Frontend (Vue.js 3)

*   **Framework:** [Vue.js 3](https://vuejs.org/) with the Composition API for building a reactive and maintainable UI.
*   **Build Tool:** [Vite](https://vitejs.dev/) provides a fast and lean development experience with features like Hot Module Replacement (HMR).
*   **Language:** [TypeScript](https://www.typescriptlang.org/) is used for static typing, improving code quality and developer productivity.
*   **UI:** The user interface is built with Single File Components (`.vue`), encapsulating the template, script, and style for each component.
*   **Tauri API:** The frontend communicates with the Rust backend via the `@tauri-apps/api` package, which provides JavaScript bindings for invoking Rust commands.

### 2.2. Backend (Rust)

*   **Language:** [Rust](https://www.rust-lang.org/) is chosen for its performance, safety, and concurrency features, making it ideal for the backend logic.
*   **Core Libraries:**
    *   `tauri`: The core framework for building the desktop application.
    *   `tokio`: An asynchronous runtime for Rust, used for managing concurrent operations like network requests.
    *   `reqwest`: A powerful and ergonomic HTTP client for making requests to search engines.
    *   `scraper`: A library for parsing and querying HTML documents, used to extract data from web pages.
    *   `serde`: A framework for serializing and deserializing Rust data structures efficiently, primarily used for JSON communication with the frontend.
    *   `regex`: Used for pattern matching and text manipulation, serving as a fallback for title cleaning when AI services are unavailable.

### 2.3. Desktop Framework (Tauri)

*   **[Tauri](https://tauri.app/):** A framework for building lightweight, secure, and cross-platform desktop applications using web technologies. It bundles the Vue.js frontend and Rust backend into a single binary, providing a native-like experience with a smaller footprint than alternatives like Electron.

## 3. Project Structure

The project is organized into two main directories: `src/` for the frontend and `src-tauri/` for the backend.

*   **`package.json`**: Defines project metadata, frontend dependencies (e.g., `vue`, `@tauri-apps/api`), and npm scripts for development and building.
*   **`vite.config.ts`**: The configuration file for Vite, used to customize the frontend development server and build process.
*   **`src/`**: Contains all frontend source code.
    *   `main.ts`: The entry point for the Vue.js application.
    *   `App.vue`: The root component that manages the overall application layout and routing.
    *   `components/`: A directory containing all Vue components, such as `HomePage.vue`, `SettingsPage.vue`, etc. Each component corresponds to a specific feature or UI module.
*   **`src-tauri/`**: Contains the backend Rust code and Tauri configuration.
    *   `Cargo.toml`: The manifest file for the Rust project, defining backend dependencies and project metadata.
    *   `tauri.conf.json`: The core configuration file for the Tauri application, defining the application identifier, window properties, build commands, and more.
    *   `src/main.rs`: The entry point for the Rust application, where all Tauri commands exposed to the frontend are defined.
    *   `src/searcher.rs`: Encapsulates the core search logic, including web scraping and content parsing.
    *   `src/llm_service.rs`: Contains the client logic for interacting with the Large Language Model.
    *   `src/app_state.rs`: Manages and persists the application's state, such as user settings and favorites.

## 4. Build and Run Instructions

### Prerequisites

*   Install [Node.js](https://nodejs.org/) and `npm` (or `yarn`/`pnpm`).
*   Install the [Rust](https://www.rust-lang.org/tools/install) development environment.
*   Follow the Tauri official documentation to set up system-specific dependencies: [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites).

### Development Workflow

1.  **Install Frontend Dependencies:**
    ```bash
    npm install
    ```
2.  **Run in Development Mode:**
    ```bash
    npm run tauri dev
    ```
    This command will:
    *   Start the Vite development server for the frontend with HMR.
    *   Compile and run the Rust backend.
    *   Open the application in a development window.
    Frontend changes are reflected instantly, while backend changes require a restart of the command.

### Production Build

1.  **Install Dependencies (if needed):**
    ```bash
    npm install
    ```
2.  **Build the Application:**
    ```bash
    npm run tauri build
    ```
    This command orchestrates the entire build process:
    *   Vite builds and optimizes the frontend assets.
    *   The Rust compiler builds the backend in release mode.
    *   Tauri bundles everything into a native installer for your OS (e.g., `.msi` on Windows, `.dmg` on macOS), which can be found in `src-tauri/target/release/bundle/`.

---

## 5. API Recommendation

Currently, only the Gemini API is supported for integration. It is recommended to use a model that supports high concurrency (e.g., gemini-2.5-flash-lite-preview-06-17) for optimal performance.

## 6. Acknowledgment

A significant portion of the code in this project was developed with the assistance of AI Development Tools. If you are interested in these AI tools, you are welcome to visit https://ai.pu.edu.kg/ to rate and review them.