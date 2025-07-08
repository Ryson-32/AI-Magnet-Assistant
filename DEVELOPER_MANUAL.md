# Developer Manual for AI Magnet Assistant

## 1. Project Overview

Welcome to the developer manual for AI Magnet Assistant. This document provides all the necessary information for developers to get started with building, testing, and contributing to the project.

AI Magnet Assistant is a desktop application designed to optimize and manage magnet links. It leverages AI to filter and prioritize links based on user-defined criteria. The application is built using a modern technology stack that ensures a responsive user interface and a powerful, efficient backend.

*   **Technology Stack**:
    *   **Frontend**: [Vue.js](https://vuejs.org/) - A progressive JavaScript framework for building user interfaces.
    *   **Backend**: [Rust](https://www.rust-lang.org/) - A high-performance, memory-safe systems programming language.
    *   **Framework**: [Tauri](https://tauri.app/) - A toolkit for building lightweight, secure, and cross-platform desktop applications with a web frontend.

## 2. Architecture

The project follows a standard Tauri architecture, which separates the frontend web view from the backend core logic. This separation allows for a clear distinction between UI-related code and business logic.

*   **Frontend (`src/`)**: The frontend is a standard Vue.js application. It is responsible for rendering the user interface and handling user interactions. All UI components, views, and assets are located in this directory. It communicates with the backend through a JavaScript bridge provided by Tauri.

*   **Backend (`src-tauri/`)**: The backend is a Rust application that manages the core logic. This includes tasks such as searching, filtering magnet links, interacting with the file system, and managing application state. The backend exposes functions that can be invoked from the frontend, enabling seamless communication between the two layers.

This dual-layer architecture allows developers to work on the frontend and backend independently while ensuring they integrate smoothly.

## 3. Project Structure

The project's directory structure is organized to maintain a clean and scalable codebase. Here are the key directories and files:

```
.
├── src/                      # Frontend source code (Vue.js)
│   ├── components/           # Reusable Vue components
│   ├── App.vue               # Main application component
│   └── main.ts               # Entry point for the Vue application
├── src-tauri/                # Backend source code (Rust)
│   ├── src/
│   │   ├── main.rs           # Main entry point for the Rust application
│   │   └── lib.rs            # Library crate for core logic
│   ├── tauri.conf.json       # Tauri configuration file
│   └── Cargo.toml            # Rust dependency management
├── DEVELOPER_MANUAL.md       # This developer manual
├── package.json              # Node.js dependencies and scripts
└── README.md                 # Project README file
```

*   `src/`: Contains all the frontend code. Developers familiar with Vue.js will find this structure conventional.
*   `src-tauri/`: Contains all the backend Rust code. The `main.rs` file sets up the Tauri application and runtime, while other modules contain the core business logic.
*   `tauri.conf.json`: A critical file for configuring the Tauri application, including window settings, security policies, and plugin configurations.
*   `package.json`: Defines project metadata, dependencies, and scripts for tasks like starting the development server and building the application.

## 4. Development Setup

Follow these steps to set up your local development environment.

### Prerequisites

Before you begin, ensure you have the following installed:
*   [Node.js](https://nodejs.org/) (LTS version recommended)
*   [Rust](https://www.rust-lang.org/tools/install) and Cargo (the Rust package manager)

### Steps

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/Ryson-32/AI-Magnet-Assistant.git
    cd ai-magnet-assistant
    ```

2.  **Install Dependencies**:
    Install the necessary Node.js packages for the frontend.
    ```bash
    npm install
    ```

3.  **Run the Development Server**:
    This command starts the Tauri development server, which will build both the frontend and backend, and launch the application in a development window with hot-reloading enabled.
    ```bash
    npm run tauri dev
    ```
    Any changes made to the frontend or backend code will automatically trigger a rebuild and reload the application.

## 5. Building for Production

To create a production-ready build of the application, run the following command:

```bash
npm run tauri build
```

This command will compile the frontend and backend, bundle them into a native executable for your operating system (e.g., `.exe` on Windows, `.app` on macOS), and place the output in the `src-tauri/target/release/bundle/` directory.

## 6. Contribution Guidelines

We welcome contributions from the community. To ensure a smooth and collaborative process, please adhere to the following guidelines.

### Code Style

*   **Frontend**: We follow the standard Vue.js style guide. Use a linter and formatter (like ESLint and Prettier) to maintain consistency.
*   **Backend**: Adhere to standard Rust formatting conventions, which can be automatically applied using `cargo fmt`.

### Branching Strategy

*   Create a new branch for each feature or bug fix from the `main` branch.
*   Use a descriptive branch name, such as `feature/new-search-filter` or `fix/login-bug`.

### Pull Requests (PRs)

*   Before submitting a Pull Request, ensure your code builds successfully and all tests pass.
*   Provide a clear and concise title and description for your PR, explaining the changes and the problem they solve.
*   Link to any relevant issues in your PR description.
*   Request a review from one or more of the project maintainers.

Thank you for contributing to AI Magnet Assistant!