# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

AI Magnet Assistant (v1.2.0) is a cross-platform desktop application that aggregates magnet link search results from multiple engines and uses AI to clean, analyze, and rank them. Built with **Tauri** (Rust backend + Vue 3 frontend).

**Core workflow**: Multi-engine search → AI-enhanced HTML extraction (custom engines) → Batch content analysis (title cleaning, tagging, purity scoring) → User curation (favorites, downloads).

## Development Commands

### Setup
```bash
npm install              # Install frontend dependencies (also runs Rust setup)
./run/setup.sh           # Linux: one-click install all dependencies including system libs
```

### Development
```bash
npm run dev              # Start Tauri dev mode (frontend + backend with hot-reload)
npm run vite:dev         # Frontend-only development (no Tauri)
```

### Building
```bash
npm run build            # TypeScript check + Vite build (frontend only)
npx tauri build          # Production build (outputs to src-tauri/target/release/)
```

### Rust Backend
```bash
cd src-tauri
cargo fmt                # Format Rust code
cargo clippy -- -D warnings  # Lint and fix all warnings before committing
```

## Architecture

### Frontend-Backend Communication
All communication uses Tauri's `invoke` API:
```typescript
// Frontend (Vue component)
const results = await invoke('search_clmclm_first', { keyword, maxPages });
```
This calls the corresponding `#[tauri::command]` function in `src-tauri/src/main.rs`.

### Backend Modules (`src-tauri/src/`)
- **`main.rs`**: Command routing center; registers all Tauri commands and manages app state
- **`app_state.rs`**: State management and persistence to `app_data.json` (stored in OS-specific app data dir)
- **`searcher.rs`**:
  - `SearchProvider` trait defines common search engine interface
  - `ClmclmProvider`: Built-in engine using scraper
  - `GenericProvider`: Custom engines with optional LLM-based HTML extraction
  - `SearchCore`: Orchestrates concurrent multi-engine searches
- **`llm_service.rs`**:
  - `LlmClient` trait for LLM abstraction
  - `GeminiClient`: Google Gemini API implementation (currently only Gemini is supported backend)
  - Handles both HTML extraction (Stage 1) and content analysis (Stage 2)
- **`i18n.rs`**: Backend internationalization using Fluent; provides locale storage and error message translation

### Frontend Structure (`src/`)
- **`main.ts`**: App entry point
- **`App.vue`**: Root component; includes navigation and debug area (visible only on Settings page)
- **`components/`**:
  - `HomePage.vue`: Core search UI; handles progressive result loading and batch AI analysis
  - `SettingsPage.vue`: AI config, engines, download settings, language switcher
  - `FavoritesPage.vue`, `PriorityPage.vue`, `EnginesPage.vue`: Data management pages
  - `ResultCard.vue`: Individual search result display
  - `SideNavigation.vue`: Main navigation
  - `LanguageSwitcher.vue`: Language selector
- **`composables/`**:
  - `useI18n.ts`: Typed i18n wrapper; `useLocale()` for runtime language switching (persists to backend + localStorage + `<html lang>`)
  - `useConfirmDelete.ts`: Reusable delete confirmation logic
- **`i18n/`**: Vue I18n setup; preloads all locales (en, zh-CN) in composition mode

### Two-Stage AI Pipeline
1. **HTML Extraction** (custom engines only): Backend sends raw HTML to Gemini to extract structured data (title, magnet link, size, source URL)
2. **Content Analysis**: Frontend triggers parallel batch analysis to clean titles, generate tags (e.g., 4K, BluRay), and compute purity score (0-100)

### State Persistence
All data (favorites, engines, priority keywords, AI configs, locale, search settings) persists to `app_data.json` via `AppStateManager`. Access folder via Settings → Data.

### Internationalization (i18n)
- **Frontend**: Vue I18n with preloaded messages (`src/i18n/locales/{en,zh-CN}/`)
- **Backend**: Fluent framework with locale files in `src-tauri/locales/{en,zh-CN}/`
- **Synchronization**: `useLocale().setLocale()` updates frontend UI, backend state, localStorage, and `<html lang>` attribute
- **Initialization**: On app mount, tries backend-persisted locale → localStorage → browser preference

### Supported Locales
- English (`en`)
- Simplified Chinese (`zh-CN`)

Add new locales by creating directories under both `src/i18n/locales/` and `src-tauri/locales/`, then updating `SUPPORTED_LOCALES` constant.

## Important Notes

### Current Limitations
- **OpenAI not implemented**: Backend only supports Google Gemini. The OpenAI option appears in UI but is not wired up in `llm_service.rs`
- **Quick download (Windows-only)**: Custom downloader integration with 115 Browser currently works only on Windows
- **LLM dependency**: HTML extraction and analysis require Gemini API configuration

### Configuration
- Vite dev server runs on port **1424** (fixed for Tauri)
- TypeScript: Strict mode enabled with `noUnusedLocals` and `noUnusedParameters`
- Rust: `cargo clippy` must pass with `-D warnings` before commits

### Debug Area
- Toggle visibility via Settings page
- Controlled by `searchState.showDebugArea` (persisted in `SearchSettings`)
- Currently contains `LanguageSwitcher`; intended for developer-facing diagnostics

### Download Handling
Backend command `open_magnet_link` behavior (in `main.rs`):
- If custom app path is 115 Browser → generates temporary HTML to trigger offline download (can auto-close based on settings)
- Otherwise → opens magnet URI with specified app or system default

## Coding Standards

### Frontend (Vue/TypeScript)
- Follow [Vue Official Style Guide](https://vuejs.org/style-guide/)
- Component files: PascalCase (e.g., `HomePage.vue`)
- Variables/functions: camelCase
- Provide explicit TypeScript types for all variables, parameters, and return values
- Use Prettier for formatting

### Backend (Rust)
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting (`cargo fmt`)
- Use `clippy` for linting (`cargo clippy -- -D warnings`)
- Prefer `Result<T, E>` for error handling with `anyhow` crate for error propagation
- Add `#[tauri::command]` attribute to expose functions to frontend

## Key Documentation

For detailed architecture information, see:
- `docs/ARCHITECTURE.md` - Complete architectural design (bilingual)
- `docs/DEVELOPER_MANUAL.md` - Development setup and standards (bilingual)
- `docs/I18N_ARCHITECTURE.md` - Internationalization implementation details
- `README.md` - User-facing documentation and usage workflow
