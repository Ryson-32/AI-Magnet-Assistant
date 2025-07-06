<script setup lang="ts">
import { ref, computed } from 'vue';

interface Props {
  title?: string;
  magnetLink?: string;
  fileSize?: string;
  uploadDate?: string;
  analysis?: any;
  isPriority?: boolean;
  fileList?: string[];
  sourceUrl?: string;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  addToFavorites: [result: any];
}>();

const showFullLink = ref(false);
const copied = ref(false);

// 计算剩余文件的tooltip内容
const remainingFilesTooltip = computed(() => {
  if (!props.fileList || props.fileList.length <= 7) return '';
  const remainingFiles = props.fileList.slice(7);
  return remainingFiles.join('\n');
});

function copyToClipboard(text: string | undefined) {
  if (!text) return;
  navigator.clipboard.writeText(text).then(() => {
    copied.value = true;
    setTimeout(() => {
      copied.value = false;
    }, 2000);
  });
}

function toggleLinkDisplay() {
  showFullLink.value = !showFullLink.value;
}

function getDisplayLink(link: string | undefined) {
  if (!link) return '';
  if (showFullLink.value) return link;
  return link.length > 60 ? link.substring(0, 60) + '...' : link;
}

function addToFavorites() {
  const result = {
    title: props.title,
    magnet_link: props.magnetLink,
    file_size: props.fileSize,
    upload_date: props.uploadDate,
    file_list: props.fileList || [],
  };
  emit('addToFavorites', result);
}

function openSourceUrl() {
  if (props.sourceUrl) {
    window.open(props.sourceUrl, '_blank');
  }
}

</script>

<template>
  <div class="card">
    <div class="card-header">
      <div class="title-section">
        <div class="title-row">
          <div class="title-wrapper">
            <h3 class="title" :title="title">{{ title }}</h3>
            <span v-if="isPriority" class="priority-badge">📌 Priority</span>
          </div>
          <div class="actions">
            <button v-if="sourceUrl" @click="openSourceUrl" class="action-btn" title="Go to source URL">
              🔗
            </button>
            <button @click="addToFavorites" class="favorite-btn" title="Add to favorites">
              ⭐
            </button>
          </div>
        </div>
        <div class="metadata-row" v-if="fileSize || uploadDate || analysis">
          <div class="metadata-left">
            <span v-if="fileSize" class="file-size">📁 {{ fileSize }}</span>
            <span v-if="uploadDate" class="upload-date">📅 {{ uploadDate }}</span>
            <span v-if="analysis && analysis.purity_score" class="purity-score">
              🎯 Score: {{ analysis.purity_score }}
            </span>
            <span v-if="analysis && analysis.tags && analysis.tags.length > 0" class="tags-item">
              🏷️ {{ analysis.tags.join(', ') }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- 文件列表预览区域 -->
    <div v-if="fileList && fileList.length > 0" class="file-list-section">
      <div class="file-grid">
        <div
          v-for="(file, index) in fileList.slice(0, 7)"
          :key="index"
          class="file-item"
          :title="file"
        >
          <div class="file-icon">📄</div>
          <div class="file-name">{{ file }}</div>
        </div>
        <div v-if="fileList.length > 7" class="more-files" :title="remainingFilesTooltip">
          +{{ fileList.length - 7 }} more files
        </div>
      </div>
    </div>

    <div class="magnet-section">
      <div class="magnet-link-container">
        <div class="magnet-link" @click="toggleLinkDisplay">
          <code>{{ getDisplayLink(magnetLink) }}</code>
          <span v-if="magnetLink && magnetLink.length > 60" class="toggle-hint">
            {{ showFullLink ? 'Click to collapse' : 'Click to expand' }}
          </span>
        </div>
        <button
          @click="copyToClipboard(magnetLink)"
          class="copy-btn-icon"
          :class="{ 'copied': copied }"
          :title="copied ? 'Copied!' : 'Copy magnet link'"
        >
          {{ copied ? '✓' : '📋' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.card {
  border: 1px solid #e0e0e0;
  border-radius: 12px;
  padding: 16px;
  background: white;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  transition: box-shadow 0.3s ease;
  height: 100%;
  display: flex;
  flex-direction: column;
  width: 100%;
  box-sizing: border-box;
  min-width: 0;
  overflow: hidden;
}

.card:hover {
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
}

.card-header {
  margin-bottom: 10px;
}

.title-section {
  display: flex;
  flex-direction: column;
  width: 100%;
}

.title-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  width: 100%;
}

.title-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0; /* Allow shrinking */
  flex: 1; /* Allow growing */
  overflow: hidden; /* Prevent content from overflowing */
}

.title {
  margin: 0;
  color: #2c3e50;
  font-size: 1.1em;
  font-weight: 600;
  line-height: 1.4;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.priority-badge {
  background: #3b82f6;
  color: white;
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 500;
  white-space: nowrap;
  flex-shrink: 0;
}

.metadata-row {
  margin-top: 8px;
  width: 100%;
}

.metadata-left {
  display: flex;
  flex-wrap: wrap;
  gap: 15px;
  font-size: 12px;
  color: #7f8c8d;
  align-items: center;
}

.file-size, .upload-date, .purity-score {
  display: flex;
  align-items: center;
  gap: 4px;
  white-space: nowrap;
  flex-shrink: 0;
}

.purity-score {
  color: #27ae60;
  font-weight: 600;
}

.tags-item {
  display: flex;
  align-items: center;
  gap: 4px;
  color: #3498db;
  font-style: italic;
  flex: 1;
  min-width: 0;
  line-height: 1.4;
}

.actions {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-shrink: 0;
}

.action-btn, .favorite-btn, .copy-btn-icon {
  border: none;
  cursor: pointer;
  transition: all 0.3s ease;
  white-space: nowrap;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  padding: 4px;
  border-radius: 50%;
}

.action-btn:hover, .favorite-btn:hover, .copy-btn-icon:hover {
  background: #f0f0f0;
}

.action-btn {
  font-size: 18px;
  color: #3498db;
}

.copy-btn-icon {
  font-size: 18px;
  color: #3498db;
  flex-shrink: 0;
}

.copy-btn-icon.copied {
  color: #27ae60;
}

.favorite-btn {
  background: transparent;
  color: #ffc107;
  font-size: 20px;
  padding: 4px;
  border-radius: 50%;
}

.favorite-btn:hover {
  background: #f0f0f0;
}

.file-list-section {
  margin-top: 5px;
  padding-top: 5px;
  width: 100%;
  box-sizing: border-box;
}

.file-grid {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
  gap: 6px;
  width: 100%;
  min-width: 0;
  overflow-x: hidden;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 4px;
  transition: all 0.2s;
  cursor: pointer;
  width: 100%;
  box-sizing: border-box;
  min-width: 0;
  overflow: hidden;
  word-break: break-all;
}

.file-item:hover {
  background: #f1f5f9;
  border-color: #cbd5e0;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.file-icon {
  font-size: 14px;
  flex-shrink: 0;
}

.file-name {
  font-size: 12px;
  color: #4a5568;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  line-height: 1.3;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
  min-width: 0;
}

.more-files {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  background: #f1f5f9;
  border: 1px solid #cbd5e0;
  border-radius: 4px;
  transition: all 0.2s;
  cursor: pointer;
  width: 100%;
  box-sizing: border-box;
  min-width: 0;
  overflow: hidden;
  word-break: break-all;
  font-size: 12px;
  color: #64748b;
  font-weight: 500;
  position: relative;
}

.more-files:hover {
  background: #e2e8f0;
  border-color: #a0aec0;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.more-files::before {
  content: "📁";
  font-size: 14px;
  flex-shrink: 0;
}

.magnet-section {
  margin-top: 15px;
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
}

.magnet-link-container {
  display: flex;
  align-items: center;
  gap: 8px;
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 6px;
  padding: 10px;
}

.magnet-link {
  flex-grow: 1;
  cursor: pointer;
  min-width: 0;
}

.magnet-link:hover {
  background: #e9ecef;
}

.magnet-link code {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 11px;
  color: #2c3e50;
  word-break: break-all;
  display: block;
  overflow-wrap: break-word;
  min-width: 0;
}

.toggle-hint {
  display: block;
  font-size: 11px;
  color: #95a5a6;
  margin-top: 5px;
  font-style: italic;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .card { padding: 12px; }
  .file-grid { gap: 4px; }
  .file-item { padding: 4px 6px; gap: 4px; }
  .file-list-section { margin-top: 5px; padding-top: 5px; }
}

@media (max-width: 900px) {
  .card { padding: 10px; }
  .file-item { padding: 3px 5px; gap: 3px; }
  .file-name { font-size: 11px; }
  .file-icon { font-size: 10px; }
}

@media (max-width: 600px) {
  .card { padding: 8px; }
  .title { font-size: 1em; }
  .file-grid { gap: 3px; max-height: 100px; }
  .file-item { padding: 2px 4px; gap: 2px; }
  .file-name { font-size: 10px; }
  .file-icon { font-size: 9px; }
  .file-list-section { margin-top: 5px; padding-top: 5px; }
  .magnet-link-container { padding: 6px; }
  .magnet-link code { font-size: 10px; }
  .metadata { font-size: 11px; }
}

@media (max-width: 400px) {
  .card { padding: 6px; }
  .title { font-size: 0.9em; }
  .file-grid { gap: 2px; max-height: 80px; }
  .file-item { padding: 1px 3px; gap: 1px; }
  .file-name { font-size: 9px; }
  .file-icon { font-size: 8px; }
  .magnet-link-container { padding: 4px; }
  .magnet-link code { font-size: 9px; }
}
</style>