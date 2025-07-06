<script setup lang="ts">
import { ref } from 'vue';

defineProps({
  title: String,
  magnetLink: String,
  fileSize: String,
  uploadDate: String,
  analysis: Object, // 添加 analysis 属性
});

const showFullLink = ref(false);
const copied = ref(false);

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
</script>

<template>
  <div class="card">
    <div class="card-header">
      <div class="title-section">
        <h3 class="title">{{ title }}</h3>
        <div class="metadata" v-if="fileSize || uploadDate || analysis">
          <span v-if="fileSize" class="file-size">📁 {{ fileSize }}</span>
          <span v-if="uploadDate" class="upload-date">📅 {{ uploadDate }}</span>
          <span v-if="analysis && analysis.purity_score" class="purity-score">
            🎯 Score: {{ analysis.purity_score }}
          </span>
        </div>
        <div v-if="analysis && analysis.tags && analysis.tags.length > 0" class="tags-section">
          <span class="tags-label">🏷️ Tags:</span>
          <span class="tags">{{ analysis.tags.join(', ') }}</span>
        </div>
      </div>
      <div class="actions">
        <button
          @click="copyToClipboard(magnetLink)"
          class="copy-btn"
          :class="{ 'copied': copied }"
        >
          {{ copied ? '✓ Copied!' : '📋 Copy' }}
        </button>
        <button class="favorite-btn" title="Add to favorites">
          ⭐
        </button>
      </div>
    </div>

    <div class="magnet-section">
      <div class="magnet-label">Magnet Link:</div>
      <div class="magnet-link" @click="toggleLinkDisplay">
        <code>{{ getDisplayLink(magnetLink) }}</code>
        <span v-if="magnetLink && magnetLink.length > 60" class="toggle-hint">
          {{ showFullLink ? 'Click to collapse' : 'Click to expand' }}
        </span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.card {
  border: 1px solid #e0e0e0;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 16px;
  background: white;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  transition: box-shadow 0.3s ease;
}

.card:hover {
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 15px;
}

.title-section {
  flex: 1;
  margin-right: 15px;
}

.title {
  margin: 0 0 8px 0;
  color: #2c3e50;
  font-size: 1.1em;
  font-weight: 600;
  line-height: 1.4;
}

.metadata {
  display: flex;
  gap: 15px;
  font-size: 12px;
  color: #7f8c8d;
}

.file-size, .upload-date, .purity-score {
  display: flex;
  align-items: center;
  gap: 4px;
}

.purity-score {
  color: #27ae60;
  font-weight: 600;
}

.tags-section {
  margin-top: 8px;
  font-size: 12px;
  color: #555;
}

.tags-label {
  font-weight: 600;
  margin-right: 6px;
}

.tags {
  color: #3498db;
  font-style: italic;
}

.actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

.copy-btn, .favorite-btn {
  padding: 6px 12px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
  transition: all 0.3s ease;
}

.copy-btn {
  background: #3498db;
  color: white;
}

.copy-btn:hover {
  background: #2980b9;
}

.copy-btn.copied {
  background: #27ae60;
}

.favorite-btn {
  background: #f8f9fa;
  color: #ffc107;
  border: 1px solid #e9ecef;
}

.favorite-btn:hover {
  background: #ffc107;
  color: white;
}

.magnet-section {
  margin-top: 15px;
}

.magnet-label {
  font-size: 12px;
  color: #7f8c8d;
  margin-bottom: 5px;
  font-weight: 500;
}

.magnet-link {
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 6px;
  padding: 10px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.magnet-link:hover {
  background: #e9ecef;
}

.magnet-link code {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 12px;
  color: #2c3e50;
  word-break: break-all;
  display: block;
}

.toggle-hint {
  display: block;
  font-size: 11px;
  color: #95a5a6;
  margin-top: 5px;
  font-style: italic;
}
</style>