<template>
  <div class="favorites-page">
    <div class="page-header">
      <h1>{{ $t('pages.favorites.title') }}</h1>
      <p>{{ $t('pages.favorites.subtitle') }}</p>
    </div>

    <div class="search-section">
      <div class="search-row">
        <input
          v-model="searchQuery"
          :placeholder="$t('pages.favorites.search.placeholder')"
          @input="searchFavorites"
          class="search-input"
        />
        <button @click="loadFavorites" class="refresh-btn">
          🔄 {{ $t('pages.favorites.search.refresh') }}
        </button>
      </div>
    </div>

    <div v-if="loading" class="loading">
      {{ $t('pages.favorites.list.loading') }}
    </div>

    <div v-else-if="displayedFavorites.length === 0" class="empty-state">
      <div class="empty-icon">⭐</div>
      <h3>{{ $t('pages.favorites.list.empty') }}</h3>
      <p>{{ $t('pages.favorites.list.emptyMessage') }}</p>
    </div>

    <div v-else class="favorites-list">
      <div class="favorites-header">
        <h2>
          {{ displayedFavorites.length === 1
            ? $t('pages.favorites.list.titleOne', { count: displayedFavorites.length })
            : $t('pages.favorites.list.titleOther', { count: displayedFavorites.length })
          }}
        </h2>
      </div>
      
      <div v-for="favorite in displayedFavorites" :key="favorite.id" class="favorite-item">
        <div class="favorite-content">
          <div class="favorite-title">{{ favorite.title }}</div>
          <div class="favorite-meta">
            <span v-if="favorite.file_size" class="file-size">{{ favorite.file_size }}</span>
            <span class="created-date">{{ $t('pages.favorites.item.created') }}: {{ formatDate(favorite.created_at) }}</span>
          </div>
          <div v-if="favorite.file_list && favorite.file_list.length > 0" class="file-list">
            <details>
              <summary>{{ $t('pages.favorites.item.filesCount', { count: favorite.file_list.length }) }}</summary>
              <ul>
                <li v-for="file in favorite.file_list.slice(0, 10)" :key="file">{{ file }}</li>
                <li v-if="favorite.file_list.length > 10">{{ $t('pages.favorites.item.moreFiles', { count: favorite.file_list.length - 10 }) }}</li>
              </ul>
            </details>
          </div>
        </div>
        
        <div class="favorite-actions">
          <button @click="copyMagnetLink(favorite.magnet_link)" class="copy-btn" :title="$t('pages.favorites.item.actions.copy')">
            📋
          </button>
          <button
            @click="confirmDelete(favorite.id)"
            :class="getDeleteButtonClass(favorite.id, 'remove-btn')"
            :title="getDeleteButtonTitle(favorite.id, 'pages.favorites.item.actions.confirmRemove', 'pages.favorites.item.actions.remove')"
          >
            {{ getDeleteIcon(favorite.id) }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, inject, watch, Ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from '../composables/useI18n';
import { useConfirmDelete } from '../composables/useConfirmDelete';

interface FavoriteItem {
  id: string;
  title: string;
  magnet_link: string;
  file_size?: string;
  file_list: string[];
  created_at: string;
}

const favorites = ref<FavoriteItem[]>([]);
const displayedFavorites = ref<FavoriteItem[]>([]);
const searchQuery = ref("");
const loading = ref(false);

const { t } = useI18n();
const showNotification = inject('showNotification') as (message: string, type?: 'success' | 'error', duration?: number) => void;
const favoritesTimestamp = inject<Ref<number>>('favoritesTimestamp');

// 使用确认删除 composable
const { confirmDelete, getDeleteIcon, getDeleteButtonClass, getDeleteButtonTitle } = useConfirmDelete(
  async (id: string) => {
    try {
      await invoke("remove_from_favorites", { id });
      await loadFavorites(); // 重新加载列表
    } catch (error) {
      console.error("Failed to remove favorite:", error);
      showNotification(t('pages.favorites.messages.removeFailed', { error: String(error) }), 'error');
    }
  }
);

if (favoritesTimestamp) {
  watch(favoritesTimestamp, () => {
    loadFavorites();
  });
}

onMounted(() => {
  loadFavorites();
});

async function loadFavorites() {
  loading.value = true;
  try {
    const result = await invoke("get_all_favorites");
    favorites.value = result as FavoriteItem[];
    displayedFavorites.value = favorites.value;
  } catch (error) {
    console.error("Failed to load favorites:", error);
    showNotification(t('pages.favorites.messages.loadFailed', { error: String(error) }), 'error');
  } finally {
    loading.value = false;
  }
}

function searchFavorites() {
  if (!searchQuery.value.trim()) {
    displayedFavorites.value = favorites.value;
    return;
  }

  const query = searchQuery.value.toLowerCase();
  displayedFavorites.value = favorites.value.filter(favorite =>
    favorite.title.toLowerCase().includes(query)
  );
}

async function copyMagnetLink(magnetLink: string) {
  try {
    await navigator.clipboard.writeText(magnetLink);
    showNotification(t('pages.favorites.messages.copied'), 'success');
  } catch (error) {
    console.error("Failed to copy magnet link:", error);
    showNotification(t('pages.favorites.messages.copyFailed'), 'error');
  }
}

function formatDate(dateString: string): string {
  try {
    const date = new Date(dateString)
    const lang = (document?.documentElement?.lang || navigator.language || 'en').toString()
    const datePart = date.toLocaleDateString(lang, { year: 'numeric', month: 'short', day: '2-digit' })
    const timePart = date.toLocaleTimeString(lang, { hour: '2-digit', minute: '2-digit' })
    return `${datePart} ${timePart}`
  } catch {
    return dateString
  }
}
</script>

<style scoped>
.favorites-page {
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

.refresh-btn {
  padding: 12px 24px;
  background: #f8fafc;
  color: #64748b;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.refresh-btn:hover {
  background: #edf2f7;
  border-color: #cbd5e0;
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
  background: white;
  border-radius: 12px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
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

.favorites-list {
  background: white;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
}

.favorites-header {
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid #e2e8f0;
}

.favorites-header h2 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: #1a202c;
}

.favorite-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 16px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  margin-bottom: 12px;
  transition: all 0.2s;
}

.favorite-item:hover {
  border-color: #cbd5e0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.favorite-content {
  flex: 1;
}

.favorite-title {
  font-size: 16px;
  font-weight: 600;
  color: #1a202c;
  margin-bottom: 8px;
  line-height: 1.4;
}

.favorite-meta {
  display: flex;
  gap: 16px;
  margin-bottom: 8px;
  font-size: 14px;
  color: #718096;
}

.file-size {
  font-weight: 500;
}

.file-list {
  margin-top: 12px;
}

.file-list details {
  font-size: 14px;
}

.file-list summary {
  cursor: pointer;
  color: #667eea;
  font-weight: 500;
}

.file-list ul {
  margin: 8px 0 0 16px;
  padding: 0;
}

.file-list li {
  margin-bottom: 4px;
  color: #4a5568;
  font-size: 13px;
}

.favorite-actions {
  display: flex;
  gap: 8px;
  margin-left: 16px;
}

.copy-btn, .remove-btn {
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  transition: all 0.2s;
}

.copy-btn {
  background: #f7fafc;
  color: #4a5568;
}

.copy-btn:hover {
  background: #edf2f7;
}

.remove-btn {
  background: #fed7d7;
  color: #c53030;
}

.remove-btn:hover {
  background: #feb2b2;
}

.remove-btn.confirm-delete {
  background-color: #fbd38d; /* A yellow/orange color for confirmation */
  color: #9c4221;
}

.remove-btn.confirm-delete:hover {
  background-color: #f6ad55;
}
</style>