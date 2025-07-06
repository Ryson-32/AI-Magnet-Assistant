<template>
  <div class="priority-page">
    <div class="page-header">
      <h1>Priority Keywords</h1>
      <p>Set keywords that will make search results appear at the top and be highlighted</p>
    </div>

    <div class="add-keyword-section">
      <div class="section-header">
        <h2>Add Priority Keyword</h2>
      </div>
      
      <form @submit.prevent="addKeyword" class="add-keyword-form">
        <div class="input-group">
          <input 
            v-model="newKeyword" 
            type="text" 
            placeholder="Enter a keyword..."
            class="keyword-input"
            required
          />
          <button type="submit" :disabled="isAdding" class="add-btn">
            {{ isAdding ? 'Adding...' : '+ Add' }}
          </button>
        </div>
      </form>
    </div>

    <div class="keywords-list">
      <div class="section-header">
        <h2>Current Priority Keywords</h2>
        <span v-if="keywords.length > 0" class="keyword-count">{{ keywords.length }} keyword{{ keywords.length !== 1 ? 's' : '' }}</span>
      </div>
      
      <div v-if="loading" class="loading">
        Loading keywords...
      </div>
      
      <div v-else-if="keywords.length === 0" class="empty-state">
        <div class="empty-icon">📌</div>
        <h3>No priority keywords set</h3>
        <p>Add keywords above to prioritize search results containing them!</p>
      </div>
      
      <div v-else class="keywords-grid">
        <div v-for="keyword in keywords" :key="keyword.id" class="keyword-item">
          <div class="keyword-content">
            <span class="keyword-text">{{ keyword.keyword }}</span>
            <span class="keyword-badge">Priority</span>
          </div>
          
          <button @click="deleteKeyword(keyword.id)" class="delete-btn" title="Remove keyword">
            ✕
          </button>
        </div>
      </div>
    </div>

    <div class="info-section">
      <div class="info-card">
        <h3>How Priority Keywords Work</h3>
        <ul>
          <li><strong>Automatic Prioritization:</strong> Search results containing any of these keywords will automatically appear at the top of the results list</li>
          <li><strong>Visual Highlighting:</strong> Prioritized results will have a special visual indicator to distinguish them</li>
          <li><strong>Case Insensitive:</strong> Keywords match regardless of capitalization</li>
          <li><strong>Partial Matching:</strong> Keywords match if they appear anywhere in the result title</li>
        </ul>
      </div>
      
      <div class="tips-card">
        <h3>Tips for Effective Keywords</h3>
        <ul>
          <li>Use specific terms that identify high-quality content (e.g., "BluRay", "1080p", "REMUX")</li>
          <li>Add trusted release group names or encoders</li>
          <li>Include quality indicators like "DTS", "Atmos", "HDR"</li>
          <li>Avoid overly common words that might match too many results</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";

interface PriorityKeyword {
  id: string;
  keyword: string;
}

const keywords = ref<PriorityKeyword[]>([]);
const newKeyword = ref("");
const loading = ref(false);
const isAdding = ref(false);

onMounted(() => {
  loadKeywords();
});

async function loadKeywords() {
  loading.value = true;
  try {
    const result = await invoke("get_all_priority_keywords");
    keywords.value = result as PriorityKeyword[];
  } catch (error) {
    console.error("Failed to load keywords:", error);
    alert(`Failed to load keywords: ${error}`);
  } finally {
    loading.value = false;
  }
}

async function addKeyword() {
  const keyword = newKeyword.value.trim();
  if (!keyword) {
    alert("Please enter a keyword");
    return;
  }

  // Check for duplicates
  if (keywords.value.some(k => k.keyword.toLowerCase() === keyword.toLowerCase())) {
    alert("This keyword already exists");
    return;
  }

  isAdding.value = true;
  try {
    await invoke("add_priority_keyword", { keyword });
    newKeyword.value = "";
    await loadKeywords(); // Reload the list
  } catch (error) {
    console.error("Failed to add keyword:", error);
    alert(`Failed to add keyword: ${error}`);
  } finally {
    isAdding.value = false;
  }
}

async function deleteKeyword(id: string) {
  if (!confirm("Are you sure you want to remove this priority keyword?")) {
    return;
  }

  try {
    await invoke("delete_priority_keyword", { id });
    await loadKeywords(); // Reload the list
  } catch (error) {
    console.error("Failed to delete keyword:", error);
    alert(`Failed to delete keyword: ${error}`);
  }
}
</script>

<style scoped>
.priority-page {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
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

.add-keyword-section, .keywords-list, .info-section {
  background: white;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid #e2e8f0;
}

.section-header h2 {
  margin: 0 0 4px 0;
  font-size: 24px;
  font-weight: 600;
  color: #1a202c;
}

.section-header p {
  margin: 0;
  color: #718096;
  font-size: 14px;
}

.keyword-count {
  background: #f7fafc;
  color: #4a5568;
  padding: 4px 12px;
  border-radius: 16px;
  font-size: 14px;
  font-weight: 500;
}

.add-keyword-form {
  max-width: 500px;
}

.input-group {
  display: flex;
  gap: 12px;
}

.keyword-input {
  flex: 1;
  padding: 12px 16px;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 16px;
  transition: border-color 0.2s;
}

.keyword-input:focus {
  outline: none;
  border-color: #667eea;
}

.add-btn {
  padding: 12px 24px;
  background: #3b82f6;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}

.add-btn:hover:not(:disabled) {
  background: #2563eb;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.add-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.loading {
  text-align: center;
  padding: 48px;
  color: #718096;
  font-size: 16px;
}

.empty-state {
  text-align: center;
  padding: 48px;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.empty-state h3 {
  margin: 0 0 8px 0;
  font-size: 24px;
  color: #1a202c;
}

.empty-state p {
  margin: 0;
  color: #718096;
  font-size: 16px;
}

.keywords-grid {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
}

.keyword-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  transition: all 0.2s;
}

.keyword-item:hover {
  border-color: #cbd5e0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.keyword-content {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}

.keyword-text {
  font-size: 16px;
  font-weight: 500;
  color: #1a202c;
}

.keyword-badge {
  background: #3b82f6;
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.delete-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 6px;
  background: #fed7d7;
  color: #c53030;
  cursor: pointer;
  font-size: 14px;
  font-weight: bold;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.delete-btn:hover {
  background: #feb2b2;
}

.info-section {
  display: grid;
  gap: 24px;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
}

.info-card, .tips-card {
  padding: 20px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  background: #f9fafb;
}

.info-card h3, .tips-card h3 {
  margin: 0 0 16px 0;
  font-size: 18px;
  font-weight: 600;
  color: #1a202c;
}

.info-card ul, .tips-card ul {
  margin: 0;
  padding-left: 20px;
}

.info-card li, .tips-card li {
  margin-bottom: 8px;
  color: #4a5568;
  line-height: 1.5;
}

.info-card strong, .tips-card strong {
  color: #1a202c;
}
</style>
