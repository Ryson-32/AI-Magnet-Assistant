<template>
  <div class="home-page">
    <div class="page-header">
      <h1>Search</h1>
      <p>Find and optimize magnet links with AI-powered filtering</p>
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
          <span>AI Filter</span>
        </label>

        <label class="checkbox-label">
          <input type="checkbox" v-model="titleMustContainKeyword" />
          <span>Title must contain keyword</span>
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
      <div class="results-grid">
        <div v-for="(result, index) in results" :key="index" class="result-item-wrapper">
          <ResultCard
            :title="result.title"
            :magnet-link="result.magnet_link"
            :file-size="result.file_size"
            :upload-date="result.upload_date"
            :analysis="result.analysis"
            :is-priority="result.isPriority"
            :file-list="result.file_list"
            :source-url="result.source_url"
            @add-to-favorites="addToFavorites"
          />
          <div v-if="result.analysis && result.analysis.error" class="error-details">
            <p><strong>Error:</strong> {{ result.analysis.error }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, inject, computed } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import ResultCard from './ResultCard.vue';
import { useStore } from '../composables/useStore';

const { loadFromStore } = useStore();

// 注入全局搜索状态
const searchState = inject('searchState') as any;

// 使用全局状态，如果不存在则创建本地状态
const keyword = searchState ? computed({
  get: () => searchState.value.keyword,
  set: (val) => searchState.value.keyword = val
}) : ref("");

const results = searchState ? computed({
  get: () => searchState.value.results,
  set: (val) => searchState.value.results = val
}) : ref([]);

const isSearching = searchState ? computed({
  get: () => searchState.value.isSearching,
  set: (val) => searchState.value.isSearching = val
}) : ref(false);

const searchStatus = searchState ? computed({
  get: () => searchState.value.searchStatus,
  set: (val) => searchState.value.searchStatus = val
}) : ref("");

const useSmartFilter = searchState ? computed({
  get: () => searchState.value.useSmartFilter,
  set: (val) => searchState.value.useSmartFilter = val
}) : ref(true);

const maxPages = searchState ? computed({
  get: () => searchState.value.maxPages,
  set: (val) => searchState.value.maxPages = val
}) : ref(1);

const sortBy = searchState ? computed({
  get: () => searchState.value.sortBy,
  set: (val) => searchState.value.sortBy = val
}) : ref('score');

const titleMustContainKeyword = searchState ? computed({
  get: () => searchState.value.titleMustContainKeyword,
  set: (val) => searchState.value.titleMustContainKeyword = val
}) : ref(true);

// 排序函数
async function sortResults(resultsArray: any[]) {
  // 首先获取优先关键词
  let priorityKeywords: any[] = [];
  try {
    priorityKeywords = await invoke("get_all_priority_keywords");
  } catch (error) {
    console.error("Failed to load priority keywords:", error);
  }

  // 为每个结果添加优先级标记
  resultsArray.forEach((result: any) => {
    result.isPriority = priorityKeywords.some((pk: any) => {
      const keyword = pk.keyword.toLowerCase();
      // 检查标题
      if (result.title.toLowerCase().includes(keyword)) {
        return true;
      }
      // 检查文件列表
      if (result.file_list && Array.isArray(result.file_list)) {
        return result.file_list.some((fileName: string) => fileName.toLowerCase().includes(keyword));
      }
      return false;
    });
  });

  // 排序：优先关键词结果在前，然后按选择的排序方式排序
  resultsArray.sort((a: any, b: any) => {
    // 首先按优先级排序
    if (a.isPriority && !b.isPriority) return -1;
    if (!a.isPriority && b.isPriority) return 1;

    // 如果优先级相同，按选择的排序方式排序
    if (sortBy.value === 'score') {
      const scoreA = a.analysis?.purity_score || 0;
      const scoreB = b.analysis?.purity_score || 0;
      return scoreB - scoreA;
    } else if (sortBy.value === 'size') {
      const sizeA = parseSizeToBytes(a.file_size || '0');
      const sizeB = parseSizeToBytes(b.file_size || '0');
      return sizeB - sizeA;
    }

    return 0;
  });
}

function parseSizeToBytes(sizeStr: string): number {
  if (!sizeStr) return 0;
  const match = sizeStr.match(/^([\d.]+)\s*([KMGT]?B)$/i);
  if (!match) return 0;
  
  const value = parseFloat(match[1]);
  const unit = match[2].toUpperCase();
  
  const multipliers: { [key: string]: number } = {
    'B': 1,
    'KB': 1024,
    'MB': 1024 * 1024,
    'GB': 1024 * 1024 * 1024,
    'TB': 1024 * 1024 * 1024 * 1024
  };
  
  return value * (multipliers[unit] || 1);
}

async function onSortChange() {
  await sortResults(results.value);
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
    const searchResults = await invoke("search_multi_page", {
      keyword: keyword.value,
      maxPages: maxPages.value,
    });

    results.value = searchResults as any[];
    searchStatus.value = `Found ${results.value.length} results`;

    // 1. 首先进行排序以识别优先项
    await sortResults(results.value);

    // 2. 如果启用了智能筛选，则进行分析
    if (useSmartFilter.value && results.value.length > 0) {
      await analyzeResults();
    }

    // 3. 再次排序以应用AI分数，同时保持优先级
    await sortResults(results.value);
  } catch (error) {
    console.error("Search failed:", error);
    searchStatus.value = `Search failed: ${error}`;
  } finally {
    isSearching.value = false;
  }
}

async function analyzeResults() {
  try {
    // 加载LLM配置
    const llmConfig = await loadFromStore("llm_config");
    if (!llmConfig || !llmConfig.api_key) {
      searchStatus.value = "AI analysis requires API key configuration. Please check Settings.";
      return;
    }

    const startTime = Date.now();
    searchStatus.value = `Analyzing ${results.value.length} results with AI...`;

    // 并发分析所有结果
    let completedCount = 0;
    const analysisPromises = results.value.map(async (result: any, index: number) => {
      try {
        const analysis = await invoke('analyze_resource', {
          title: result.title,
          fileList: result.file_list || [],
          config: llmConfig
        });

        // 更新进度
        completedCount++;
        searchStatus.value = `Analyzing ${results.value.length} results with AI (${completedCount}/${results.value.length} completed)...`;

        return { result, analysis, index };
      } catch (e) {
        console.error(`Failed to analyze result: ${result.title}`, e);
        completedCount++;
        searchStatus.value = `Analyzing ${results.value.length} results with AI (${completedCount}/${results.value.length} completed)...`;

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

    results.value = analyzedResults;
    searchStatus.value = `AI analysis completed in ${duration}s (${results.value.length} results processed)`;
  } catch (error) {
    console.error('AI analysis failed:', error);
    searchStatus.value = `AI analysis failed: ${error}`;
  }
}

async function addToFavorites(result: any) {
  try {
    await invoke("add_to_favorites", {
      title: result.title,
      magnetLink: result.magnet_link,
      fileSize: result.file_size,
      fileList: result.file_list || [],
    });
    alert("Added to favorites!");
  } catch (error) {
    console.error("Failed to add to favorites:", error);
    alert(`Failed to add to favorites: ${error}`);
  }
}
</script>

<style scoped>
.home-page {
  padding: 24px;
  width: 100%;
  box-sizing: border-box;
  overflow-x: hidden;
  min-width: 0;
}

/* 响应式padding调整 */
@media (max-width: 1200px) {
  .home-page {
    padding: 16px;
  }
}

@media (max-width: 768px) {
  .home-page {
    padding: 12px;
  }
}

.page-header {
  margin-bottom: 32px;
}

.page-header h1 {
  margin: 0 0 8px 0;
  font-size: 32px;
  font-weight: 700;
  color: #1a202c;
}

.page-header p {
  margin: 0;
  color: #718096;
  font-size: 16px;
}

.search-section {
  background: white;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  margin-bottom: 24px;
}

.search-row {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.search-input {
  flex: 1;
  padding: 12px 16px;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 16px;
  transition: border-color 0.2s;
}

.search-input:focus {
  outline: none;
  border-color: #667eea;
}

.search-btn {
  padding: 12px 24px;
  background: #3b82f6;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.search-btn:hover:not(:disabled) {
  background: #2563eb;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.search-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.filter-options {
  display: flex;
  gap: 24px;
  align-items: center;
  flex-wrap: wrap;
}

.pages-selector {
  display: flex;
  align-items: center;
  gap: 8px;
}

.pages-selector select {
  padding: 8px 12px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 14px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
}

.status {
  margin-top: 16px;
  padding: 12px;
  background: #f7fafc;
  border-radius: 6px;
  color: #4a5568;
  font-size: 14px;
}

.results-section {
  background: white;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  width: 100%;
  overflow-x: hidden;
  min-width: 0;
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid #e2e8f0;
}

.results-header h2 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: #1a202c;
}

.sort-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.sort-selector {
  padding: 8px 12px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 14px;
}

.results-grid {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
  gap: 20px;
  align-items: start;
  width: 100%;
  min-width: 0;
  overflow: hidden;
}

/* 响应式设计：在极小宽度时切换到单列 */
@media (max-width: 1200px) {
  .results-grid {
    gap: 16px;
  }
}

@media (max-width: 900px) {
  .results-grid {
    gap: 12px;
  }
}

@media (max-width: 700px) {
  .results-grid {
    grid-template-columns: 1fr;
    gap: 15px;
  }
}

@media (max-width: 600px) {
  .results-grid {
    gap: 12px;
  }
}

.result-item-wrapper {
  margin-bottom: 0;
}

.error-details {
  margin-top: 8px;
  padding: 12px;
  background: #fed7d7;
  border-radius: 6px;
  color: #c53030;
  font-size: 14px;
}
</style>
