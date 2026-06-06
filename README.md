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

###

**AI Magnet Assistant** is an intelligent magnet link search and optimization tool. It aggregates results from multiple engines, enhances non-structured pages with AI, and helps you rank and curate clean results.

## Highlights & Use Cases ⭐

- **Pain points**: Messy page structure and noisy titles; uneven result quality; heavy manual filtering.
- **Tech stack**: Tauri + Rust (backend/system), Vue 3 + TypeScript (frontend).
- **Core features**:
  - Multi-engine aggregation: recommended Prowlarr API integration plus legacy HTML engines.
  - Two-stage AI: Stage 1 HTML extraction (for custom engines) → Stage 2 content analysis (title cleaning, tags, purity 0–100).
  - Efficiency & management: Favorites and centralized download; priority keywords; sort by score/size; quick copy/open source link.
  - Download integration: quick download via custom application path.
- **Use cases**: Aggregated search; noise reduction and tagging; magnet favorites and centralized downloads.

## Screenshots 🖼️

<img width="1282" height="992" alt="image" src="https://github.com/user-attachments/assets/c8be8473-801e-4fac-b4d0-4ace9b0b9972" />
<img width="1282" height="992" alt="image" src="https://github.com/user-attachments/assets/d2ff6cc9-93d8-4090-886b-a50901025c80" />
<img width="1282" height="992" alt="image" src="https://github.com/user-attachments/assets/9cb651cc-915c-4670-ae76-9f274d4dd19c" />

## How It Works ⚙️

### Search Orchestration

- **Built-in engine first**: Fetches initial results quickly from the built-in engine (if enabled), then merges other engines' results in parallel.
- **Real-time status**: The UI streams status (search, analysis progress, and the model in use) during the process.

### Two-Phase AI Pipeline (for custom engines and analysis)

1. **HTML Content Extraction**: The backend calls an OpenAI-compatible provider to extract `{ title, magnet_link, file_size, source_url }` from raw HTML.
2. **Content Analysis**: The frontend triggers parallel batches to clean titles, compute a purity score, and generate tags (e.g., 4K, Chinese, Chinese Sub, BluRay). It falls back to individual analysis on failure and updates status live.

### Persistence

- All configuration and data (engines, favorites, priority keywords, AI configs, locale, etc.) are stored in `app_data.json`. Open its folder via Settings → Data.

## Notes 📝

The backend supports Agnes, DeepSeek, and custom OpenAI-compatible endpoints.

- **agnes-2.0-flash**: Default for HTML extraction.
- **deepseek-chat**: Default for content analysis.

Actual speed depends on network and page complexity; batch analysis is parallel with automatic fallbacks.

## Usage Workflow 🧭

1. **Initial Setup**
   - Go to Settings → AI Configuration; fill API base, model, and API key for both Extraction and Analysis; use the Test buttons.
   - Add a Prowlarr engine with its URL and API key, or optionally add a legacy HTML engine.
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

#### Clone

```bash
git clone https://github.com/Ryson-32/AI-Magnet-Assistant.git
cd AI-Magnet-Assistant
```

#### Install

```bash
npm install
```

#### Run (development)

```bash
npm run dev
```

Frontend only:

```bash
npm run vite:dev
```

#### Build

```bash
npx tauri build
```

## Known Issues 🐞

- Some sites use heavy JS or anti-bot challenges; legacy HTML search may fail or return fewer results.
- Prowlarr is the recommended search source. Adult categories are filtered by default.
- Hitting provider rate limits can cause analysis failures.
- The custom downloader quick-download feature is currently available on Windows only.

### Prowlarr in Published Builds

- Prowlarr is not bundled with the application. Users install a local instance or connect to an accessible remote instance.
- Chinese setup notes with a working local Prowlarr, proxy, and FlareSolverr example are available in [docs/PROWLARR_LOCAL_SETUP.zh-CN.md](docs/PROWLARR_LOCAL_SETUP.zh-CN.md).
- Add at least one indexer in Prowlarr, then copy its API key from Settings → General → Security.
- Configure and test the connection from Search Engines → Prowlarr in this application.
- Engine settings and API keys stay in the current user's app-data directory and are not included in Git or release packages.
- Adult categories are filtered by default and can be enabled per Prowlarr engine.
- This app only displays indexers already configured in Prowlarr and lets users enable or disable them. Add, test, and troubleshoot indexers in Prowlarr itself.
- Prowlarr proxy download URLs are converted to standard `magnet:?xt=urn:btih:...` links when an info hash is available.
- Chinese-language coverage depends on configured indexers. Public options include ACG.RIP, Bangumi Moe, and dmhy; broader Chinese media indexers may require accounts.
- Private-site credentials remain in Prowlarr. Depending on the site, Prowlarr may request an API key, cookie, or passkey; this application does not read or store those credentials.

Bundling both Prowlarr and Jackett is not recommended: they overlap heavily and would add substantial package size, background-process management, and GPL redistribution maintenance. Jackett/Torznab is better treated as an optional future compatibility interface.

## License 📄

MIT License. See the [LICENSE](LICENSE).
