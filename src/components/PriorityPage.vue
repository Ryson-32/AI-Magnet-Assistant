<template>
  <div class="priority-page">
    <div class="page-header">
      <h1>{{ $t('pages.priority.title') }}</h1>
      <p>{{ $t('pages.priority.subtitle') }}</p>
    </div>

    <div class="add-keyword-section">
      <div class="section-header">
        <h2>{{ $t('pages.priority.add.title') }}</h2>
      </div>
      
      <form @submit.prevent="addKeyword" class="add-keyword-form">
        <div class="input-group">
          <input 
            v-model="newKeyword" 
            type="text" 
            :placeholder="$t('pages.priority.add.placeholder')"
            class="keyword-input"
            required
          />
          <button type="submit" :disabled="isAdding" class="add-btn">
            {{ isAdding ? $t('pages.priority.add.adding') : '+ ' + $t('pages.priority.add.addButton') }}
          </button>
        </div>
      </form>
    </div>

    <div class="keywords-list">
      <div class="section-header">
        <h2>{{ $t('pages.priority.list.title') }}</h2>
        <span v-if="keywords.length > 0" class="keyword-count">{{ $t('pages.priority.list.count', { count: keywords.length }) }}</span>
      </div>
      
      <div v-if="loading" class="loading">
        {{ $t('pages.priority.list.loading') }}
      </div>
      
      <div v-else-if="keywords.length === 0" class="empty-state">
        <div class="empty-icon">📌</div>
        <h3>{{ $t('pages.priority.list.empty') }}</h3>
        <p>{{ $t('pages.priority.list.emptyMessage') }}</p>
      </div>
      
      <div v-else class="keywords-grid">
        <div v-for="keyword in keywords" :key="keyword.id" class="keyword-item">
          <div class="keyword-content">
            <span class="keyword-text">{{ keyword.keyword }}</span>
            <span class="keyword-badge">{{ $t('pages.priority.item.badge') }}</span>
          </div>
          
          <button
            @click="deleteKeyword(keyword.id)"
            :class="['delete-btn', { 'confirm-delete': keyword.id === pendingDeleteId }]"
            :title="keyword.id === pendingDeleteId ? $t('pages.priority.item.confirmDeleteTitle') : $t('pages.priority.item.deleteTitle')"
          >
            {{ keyword.id === pendingDeleteId ? '❓' : '✕' }}
          </button>
        </div>
      </div>
    </div>

    <div class="info-section">
      <div class="info-card">
        <h3>{{ $t('pages.priority.info.howItWorks.title') }}</h3>
        <ul>
          <li><strong>{{ $t('pages.priority.info.howItWorks.autoPriority').split(':')[0] }}:</strong> {{ $t('pages.priority.info.howItWorks.autoPriority').split(':')[1] }}</li>
          <li><strong>{{ $t('pages.priority.info.howItWorks.visualHighlight').split(':')[0] }}:</strong> {{ $t('pages.priority.info.howItWorks.visualHighlight').split(':')[1] }}</li>
          <li><strong>{{ $t('pages.priority.info.howItWorks.caseInsensitive').split(':')[0] }}:</strong> {{ $t('pages.priority.info.howItWorks.caseInsensitive').split(':')[1] }}</li>
          <li><strong>{{ $t('pages.priority.info.howItWorks.partialMatching').split(':')[0] }}:</strong> {{ $t('pages.priority.info.howItWorks.partialMatching').split(':')[1] }}</li>
        </ul>
      </div>
      
      <div class="tips-card">
        <h3>{{ $t('pages.priority.info.tips.title') }}</h3>
        <ul>
          <li>{{ $t('pages.priority.info.tips.specific') }}</li>
          <li>{{ $t('pages.priority.info.tips.trustedGroups') }}</li>
          <li>{{ $t('pages.priority.info.tips.qualityIndicators') }}</li>
          <li>{{ $t('pages.priority.info.tips.avoidCommon') }}</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, inject } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from '../composables/useI18n';

interface PriorityKeyword {
  id: string;
  keyword: string;
}

const keywords = ref<PriorityKeyword[]>([]);
const newKeyword = ref("");
const loading = ref(false);
const isAdding = ref(false);
const pendingDeleteId = ref<string | null>(null);
const deleteTimeout = ref<any>(null);

const { t } = useI18n();
const showNotification = inject('showNotification') as (message: string, type?: 'success' | 'error', duration?: number) => void;

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
    showNotification(t('pages.priority.messages.loadFailed', { error: String(error) }), 'error');
  } finally {
    loading.value = false;
  }
}

async function addKeyword() {
  const keyword = newKeyword.value.trim();
  if (!keyword) {
    showNotification(t('pages.priority.messages.validation.enterKeyword'), 'error');
    return;
  }

  // Check for duplicates
  if (keywords.value.some(k => k.keyword.toLowerCase() === keyword.toLowerCase())) {
    showNotification(t('pages.priority.messages.validation.keywordExists'), 'error');
    return;
  }

  isAdding.value = true;
  try {
    await invoke("add_priority_keyword", { keyword });
    newKeyword.value = "";
    await loadKeywords(); // Reload the list
  } catch (error) {
    console.error("Failed to add keyword:", error);
    showNotification(t('pages.priority.messages.addFailed', { error: String(error) }), 'error');
  } finally {
    isAdding.value = false;
  }
}

async function deleteKeyword(id: string) {
  clearTimeout(deleteTimeout.value);

  if (pendingDeleteId.value === id) {
    try {
      await invoke("delete_priority_keyword", { id });
      await loadKeywords(); // Reload the list
      pendingDeleteId.value = null;
    } catch (error) {
      console.error("Failed to delete keyword:", error);
      showNotification(t('pages.priority.messages.deleteFailed', { error: String(error) }), 'error');
    }
  } else {
    pendingDeleteId.value = id;
    deleteTimeout.value = setTimeout(() => {
      pendingDeleteId.value = null;
    }, 3500);
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

.delete-btn.confirm-delete {
  background-color: #fbd38d; /* A yellow/orange color for confirmation */
  color: #9c4221;
}

.delete-btn.confirm-delete:hover {
  background-color: #f6ad55;
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