<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ResultCard from "./components/ResultCard.vue";
import { useStore } from "./composables/useStore";

const { saveToStore, loadFromStore } = useStore();

onMounted(async () => {
  await loadLlmConfig();
});

const keyword = ref("");
const results = ref<any[]>([]);
const showSettings = ref(false);
const isSearching = ref(false);
const searchStatus = ref("");
const useSmartFilter = ref(true);
const maxPages = ref(1);
const sortBy = ref('score'); // 'score' 或 'size'

const llmConfig = ref({
  provider: "gemini",
  api_key: "***REMOVED***",
  api_base: "https://generativelanguage.googleapis.com/v1beta",
  model: "gemini-2.5-flash",
});

watch(() => llmConfig.value.provider, (newProvider) => {
  if (newProvider === 'gemini') {
    llmConfig.value.api_base = 'https://generativelanguage.googleapis.com/v1beta';
    llmConfig.value.model = 'gemini-2.5-flash';
  } else if (newProvider === 'openai') {
    llmConfig.value.api_base = 'https://api.openai.com/v1';
    llmConfig.value.model = 'gpt-3.5-turbo';
  }
});

// 排序函数
function sortResults(resultsArray: any[]) {
  if (sortBy.value === 'score') {
    // 按分数排序（分数高的在前）
    resultsArray.sort((a: any, b: any) => {
      const scoreA = a.analysis?.purity_score || 0;
      const scoreB = b.analysis?.purity_score || 0;
      return scoreB - scoreA;
    });
  } else if (sortBy.value === 'size') {
    // 按文件大小排序（大的在前）
    resultsArray.sort((a: any, b: any) => {
      const sizeA = parseFileSize(a.file_size);
      const sizeB = parseFileSize(b.file_size);
      return sizeB - sizeA;
    });
  }
}

// 解析文件大小字符串为数字（以GB为单位）
function parseFileSize(sizeStr: string | null | undefined): number {
  if (!sizeStr) return 0;

  const match = sizeStr.match(/(\d+\.?\d*)\s*(GB|MB|TB|KB)/i);
  if (!match) return 0;

  const value = parseFloat(match[1]);
  const unit = match[2].toUpperCase();

  switch (unit) {
    case 'TB': return value * 1024;
    case 'GB': return value;
    case 'MB': return value / 1024;
    case 'KB': return value / (1024 * 1024);
    default: return value;
  }
}

// 当排序方式改变时重新排序
function onSortChange() {
  if (results.value.length > 0) {
    sortResults(results.value);
  }
}

async function search() {
  if (!keyword.value.trim()) {
    alert("Please enter a search keyword");
    return;
  }

  isSearching.value = true;
  searchStatus.value = "Searching...";
  results.value = [];

  try {
    const baseResults: any[] = await invoke("search_multi_page", {
      keyword: keyword.value,
      maxPages: maxPages.value
    });

    if (useSmartFilter.value) {
      const startTime = Date.now();
      searchStatus.value = `Analyzing ${baseResults.length} results with AI (concurrent)...`;

      try {
        // 创建所有分析任务的Promise数组，并发执行
        let completedCount = 0;
        const analysisPromises = baseResults.map(async (result, index) => {
          try {
            const analysis = await invoke('analyze_resource', {
              title: result.title,
              fileList: [], // Placeholder
              config: llmConfig.value
            });

            // 更新进度
            completedCount++;
            searchStatus.value = `Analyzing ${baseResults.length} results with AI (${completedCount}/${baseResults.length} completed)...`;

            return { result, analysis, index };
          } catch (e) {
            console.error(`Failed to analyze result: ${result.title}`, e);
            completedCount++;
            searchStatus.value = `Analyzing ${baseResults.length} results with AI (${completedCount}/${baseResults.length} completed)...`;

            return {
              result,
              analysis: { error: `Analysis Failed: ${e}` },
              index
            };
          }
        });

        // 等待所有分析任务完成
        const analysisResults = await Promise.all(analysisPromises);

        const endTime = Date.now();
        const duration = ((endTime - startTime) / 1000).toFixed(1);

        // 将分析结果映射回原始搜索结果
        const analyzedResults = analysisResults.map(({ result, analysis }) => {
          result.analysis = analysis;
          return result;
        });

        // 应用排序
        sortResults(analyzedResults);

        results.value = analyzedResults;
        searchStatus.value = `Analysis completed in ${duration}s (${baseResults.length} results processed concurrently)`;
      } catch (e) {
        console.error('Concurrent analysis failed:', e);
        // 如果并发分析失败，回退到不使用智能筛选
        alert(`Concurrent analysis failed: ${e}. Showing results without AI analysis.`);
        results.value = baseResults;
      }
    } else {
      // 对于非智能搜索，也应用排序（主要是按文件大小）
      sortResults(baseResults);
      results.value = baseResults;
    }

    searchStatus.value = `Found ${results.value.length} results`;
  } catch (error) {
    searchStatus.value = `Error: ${error}`;
    console.error("Search error:", error);
  } finally {
    isSearching.value = false;
  }
}

async function saveLlmConfig() {
  try {
    await saveToStore("llmConfig", llmConfig.value);
    alert("Settings saved!");
    showSettings.value = false;
  } catch (error) {
    console.error("Error saving settings:", error);
    alert(`Error saving settings: ${error}`);
  }
}

async function loadLlmConfig() {
  try {
    const loadedConfig = await loadFromStore("llmConfig");
    if (loadedConfig) {
      llmConfig.value = { ...llmConfig.value, ...loadedConfig };
    }
  } catch (error) {
    console.error("Could not load LLM config, using default.", error);
  }
}

async function testApiConnection() {
  try {
    const result = await invoke("test_connection", { config: llmConfig.value });
    alert(`API Test Success: ${result}`);
  } catch (error) {
    alert(`API Test Failed: ${error}`);
  }
}
</script>

<template>
  <div class="container">
    <div class="header">
      <h1>🧲 MagnetLink Optimizer Pro</h1>
      <div class="header-buttons">
        <button @click="showSettings = !showSettings" class="settings-btn">
          ⚙️ Settings
        </button>
      </div>
    </div>

    <div v-if="showSettings" class="settings">
      <h2>LLM Configuration</h2>
      <div class="settings-grid">
        <label>Provider:</label>
        <select v-model="llmConfig.provider">
          <option value="gemini">Gemini</option>
          <option value="openai">OpenAI</option>
        </select>

        <label>API Key:</label>
        <div class="input-with-button">
          <input v-model="llmConfig.api_key" type="password" placeholder="Enter API Key..." />
          <button @click="testApiConnection" class="test-api-btn-inline">Test</button>
        </div>

        <label>API Base URL:</label>
        <input v-model="llmConfig.api_base" placeholder="e.g., https://generativelanguage.googleapis.com/v1beta" />

        <label>Model:</label>
        <input v-model="llmConfig.model" placeholder="e.g., gemini-1.5-pro-latest" />
      </div>
      <div class="settings-actions">
        <button @click="saveLlmConfig" class="save-btn">Save Settings</button>
        <button @click="showSettings = false" class="cancel-btn">Cancel</button>
      </div>
    </div>

    <div class="search-section">
      <div class="search-row">
        <input
          v-model="keyword"
          placeholder="Enter search keyword..."
          @keyup.enter="search"
          :disabled="isSearching"
          class="search-input"
        />
        <button @click="search" :disabled="isSearching" class="search-btn">
          {{ isSearching ? "Searching..." : "🔍 Search" }}
        </button>
      </div>

      <div class="filter-options">
        <div class="pages-selector">
          <label for="maxPages">Search pages:</label>
          <select id="maxPages" v-model="maxPages">
            <option :value="1">1 page</option>
            <option :value="3">3 pages</option>
            <option :value="5">5 pages</option>
            <option :value="10">10 pages</option>
          </select>
        </div>

        <label class="checkbox-label">
          <input type="checkbox" v-model="useSmartFilter" />
          <span>Use AI Filter</span>
        </label>
      </div>

      <div v-if="searchStatus" class="status">
        {{ searchStatus }}
      </div>
    </div>

    <div v-if="results.length > 0" class="results-section">
      <div class="results-header">
        <h2>Search Results ({{ results.length }})</h2>
        <div class="sort-controls">
          <label for="sortBy">Sort by:</label>
          <select id="sortBy" v-model="sortBy" @change="onSortChange" class="sort-selector">
            <option value="score">Purity Score</option>
            <option value="size">File Size</option>
          </select>
        </div>
      </div>
      <div v-for="(result, index) in results" :key="index" class="result-item-wrapper">
        <ResultCard
          :title="result.title"
          :magnet-link="result.magnet_link"
          :file-size="result.file_size"
          :upload-date="result.upload_date"
          :analysis="result.analysis"
        />
        <div v-if="result.analysis && result.analysis.error" class="error-details">
          <p><strong>Error:</strong> {{ result.analysis.error }}</p>
        </div>
      </div>
    </div>

    <div v-else-if="!isSearching && keyword && searchStatus" class="no-results">
      <p>No results found for "{{ keyword }}"</p>
    </div>
  </div>
</template>

<style scoped>
.container {
  padding: 20px;
  max-width: 900px;
  margin: 0 auto;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
  padding-bottom: 15px;
  border-bottom: 2px solid #e0e0e0;
}

.header h1 {
  color: #2c3e50;
  margin: 0;
  font-size: 2.2em;
  font-weight: 600;
}

.header-buttons {
  display: flex;
  gap: 10px;
}

.settings-btn {
  border: none;
  padding: 10px 16px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.3s;
  color: white;
  background: #3498db;
}

.settings-btn:hover {
  background: #2980b9;
}

.settings {
  margin-bottom: 25px;
  padding: 20px;
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 12px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.settings h2 {
  margin-top: 0;
  color: #2c3e50;
  font-size: 1.4em;
}

.settings-grid {
  display: grid;
  grid-template-columns: 120px 1fr;
  gap: 15px;
  align-items: center;
  margin-bottom: 20px;
}

.input-with-button {
  display: flex;
  gap: 10px;
  align-items: center;
}

.input-with-button input {
  flex: 1;
  margin: 0;
}

.test-api-btn-inline {
  padding: 10px 15px;
  background: #3498db;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  white-space: nowrap;
}

.settings-grid label {
  text-align: right;
  font-weight: 500;
  color: #555;
}

.settings-grid input, .settings-grid select {
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 14px;
}

.settings-actions {
  display: flex;
  gap: 10px;
  justify-content: center;
}

.save-btn {
  background: #27ae60;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}

.cancel-btn {
  background: #95a5a6;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}

.search-section {
  margin-bottom: 30px;
}

.search-row {
  display: flex;
  gap: 10px;
  margin-bottom: 15px;
}

.search-input {
  flex: 1;
  padding: 12px 16px;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  font-size: 16px;
  transition: border-color 0.3s;
}

.search-input:focus {
  outline: none;
  border-color: #3498db;
}

.search-btn {
  background: #e74c3c;
  color: white;
  border: none;
  padding: 12px 24px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 16px;
  font-weight: 500;
  transition: background-color 0.3s;
}

.search-btn:hover:not(:disabled) {
  background: #c0392b;
}

.search-btn:disabled {
  background: #bdc3c7;
  cursor: not-allowed;
}

.filter-options {
  margin-bottom: 15px;
  display: flex;
  align-items: center;
  gap: 20px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: #555;
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  margin: 0;
}

.pages-selector {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 14px;
  color: #555;
}

.pages-selector label {
  font-weight: 500;
}

.pages-selector select {
  padding: 5px 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  background: white;
}

.status {
  padding: 10px 15px;
  background: #e8f4fd;
  border: 1px solid #bee5eb;
  border-radius: 6px;
  color: #0c5460;
  font-size: 14px;
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.results-header h2 {
  color: #2c3e50;
  margin: 0;
  font-size: 1.5em;
}

.sort-controls {
  display: flex;
  align-items: center;
  gap: 10px;
}

.sort-controls label {
  font-weight: 600;
  color: #555;
  font-size: 14px;
}

.sort-selector {
  padding: 6px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  background: white;
  font-size: 14px;
  cursor: pointer;
  transition: border-color 0.3s ease;
}

.sort-selector:hover {
  border-color: #3498db;
}

.sort-selector:focus {
  outline: none;
  border-color: #3498db;
  box-shadow: 0 0 0 2px rgba(52, 152, 219, 0.2);
}

.result-item-wrapper {
  margin-bottom: 15px;
}

.error-details {
  background-color: #fff5f5;
  border: 1px solid #fed7d7;
  border-top: none;
  padding: 10px;
  border-radius: 0 0 8px 8px;
  font-size: 14px;
  color: #e53e3e;
}

.error-details p {
  margin: 5px 0;
}

.no-results {
  text-align: center;
  padding: 40px;
  color: #7f8c8d;
  font-size: 16px;
}
</style>