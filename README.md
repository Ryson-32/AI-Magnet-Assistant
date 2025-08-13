<!-- markdownlint-disable MD033 MD041 -->
<div align="center">
  <h1>AI Magnet Assistant</h1>
  <p>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
    <img src="https://img.shields.io/badge/platform-Cross%20Platform-lightgrey.svg" alt="Platform: Windows/macOS/Linux">
    <img src="https://img.shields.io/badge/version-1.2.0-green.svg" alt="Version 1.2.0">
  </p>
  <p><strong><a href="README.md">English</a> | <a href="README.zh-CN.md">中文</a></strong></p>
</div>
<!-- markdownlint-enable MD033 MD041 -->

<em>If this project helps you, please give it a ⭐!</em>

<sub>Disclaimer: Most of the code in this project was completed with the assistance of AI Development Tools. If you are interested in these AI tools, you are welcome to visit <a href="https://ai.pu.edu.kg/">https://ai.pu.edu.kg/</a> to rate and review them.</sub>

###

**AI Magnet Assistant** is an intelligent magnet link search and optimization tool built with Rust, Tauri, and Vue 3. It aggregates results from multiple engines, enhances non-structured pages with AI, and helps you rank and curate clean results.

## Highlights & Use Cases ⭐

- **Pain points solved**: Unstructured HTML and noisy titles; mixed-quality results; manual curation effort.
- **Tech stack**: Tauri + Rust (backend/system integration), Vue 3 + TypeScript (frontend & i18n), Gemini (LLM provider).
- **Core features**:
  - Multi-engine orchestration: built-in engine first, others concurrently; live status shows the model used.
  - Two-phase AI: Stage 1 HTML extraction for custom engines → Stage 2 content analysis (clean titles, tags, purity score 0–100) with parallel batching and fallback.
  - Favorites & centralized download; priority keywords; sort by score/size; quick copy/open source link.
  - Internationalization; configurable provider/API base/model per stage with per-field Test buttons.
  - Download integrations for browsers with offline download capability; custom application path and optional auto-close page.
- **Use cases**: Aggregated magnet search, noise reduction and tagging, curation with favorites and centralized downloads, extending engines for new sites.

## Screenshots 🖼️

<!-- Replace the following placeholders with your actual screenshots -->
<!-- <img width="1280" alt="screenshot-1" src="..." /> -->
<!-- <img width="1280" alt="screenshot-2" src="..." /> -->
<!-- <img width="1280" alt="screenshot-3" src="..." /> -->

## How It Works ⚙️

### Search Orchestration

- **Built-in engine first**: Fetches initial results quickly from the built-in engine (if enabled), then merges other engines' results in parallel.
- **Real-time status**: The UI streams status (search, analysis progress, and the model in use) during the process.

### Two-Phase AI Pipeline (for custom engines and analysis)

1. **HTML Content Extraction**: The backend calls Gemini to extract `{ title, magnet_link, file_size, source_url }` from raw HTML. This is used for custom engines. API base and model are configurable.
2. **Content Analysis**: The frontend triggers parallel batches to clean titles, compute a purity score, and generate tags (e.g., 4K, Chinese, Chinese Sub, BluRay). It falls back to individual analysis on failure and updates status live.

### Persistence

- All configuration and data (engines, favorites, priority keywords, AI configs, locale, etc.) are stored in `app_data.json`. Open its folder via Settings → Data.

## Notes 📝

Note: Current backend implementation supports Google Gemini. The OpenAI option is visible in the UI but not wired up in the backend yet.

- **gemini-2.5-flash**: Recommended for HTML extraction (Stage 1).
- **gemini-2.5-flash-lite**: Recommended for content analysis (Stage 2), faster and cost-effective.

Actual speed depends on network and page complexity; batch analysis is parallel with automatic fallbacks.

## Usage Workflow 🧭

1. **Initial Setup**
   - Go to Settings → AI Configuration; fill API base, model, and API key for both Extraction and Analysis; use the Test buttons.
   - Optionally add custom engines (Engines page) using template or auto-from-examples.
   - Configure Download settings (application path, quick download, auto-close page) and Language.

2. **Search**
   - Enter a keyword on Home; choose pages and toggles (AI filter, title must contain keyword).
   - Results appear as soon as the built-in engine returns; additional engines merge in; analysis can refine titles/tags/score.

3. **Curate**
   - Sort by score or size; add Favorites; manage Priority keywords to boost matches; open source pages; copy/open magnet links quickly.

## Deployment Instructions 🛠️

#### Prerequisites

- Node.js 18+
- Rust (latest stable)
- Tauri CLI

#### Clone

```bash
git clone https://github.com/Ryson-32/AI-Magnet-Assistant.git
cd AI-Magnet-Assistant
```

#### Install

```bash
npm install
npm install -g @tauri-apps/cli
```

#### Run (development)

```bash
npm run tauri dev
```

Frontend only:

```bash
npm run vite:dev
```

#### Build

```bash
npm run tauri build
```

## Known Issues 🐞

- OpenAI provider is not supported by the backend yet; Gemini is required.
- Some sites use heavy JS or anti-bot; HTML may be JS or garbled. The app logs a preview and may fall back or return fewer results.
- Hitting rate limits can cause analysis failures; see Settings for rate limits and tips (e.g., gemini-balance).
- The custom downloader quick-download feature is currently available on Windows only.

## License 📄

MIT License. See the [LICENSE](LICENSE).

