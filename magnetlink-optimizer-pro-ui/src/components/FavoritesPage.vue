<template>
  <div class="favorites-page">
    <div class="page-header">
      <h1>Favorites</h1>
      <p>Your saved magnet links</p>
    </div>

    <div class="search-section">
      <div class="search-row">
        <input
          v-model="searchQuery"
          placeholder="Search in favorites..."
          @input="searchFavorites"
          class="search-input"
        />
        <button @click="loadFavorites" class="refresh-btn">
          🔄 Refresh
        </button>
      </div>
    </div>

    <div v-if="loading" class="loading">
      Loading favorites...
    </div>

    <div v-else-if="displayedFavorites.length === 0" class="empty-state">
      <div class="empty-icon">⭐</div>
      <h3>No favorites yet</h3>
      <p>Start adding your favorite magnet links from search results!</p>
    </div>

    <div v-else class="favorites-list">
      <div class="favorites-header">
        <h2>{{ displayedFavorites.length }} Favorite{{ displayedFavorites.length !== 1 ? 's' : '' }}</h2>
      </div>
      
      <div v-for="favorite in displayedFavorites" :key="favorite.id" class="favorite-item">
        <div class="favorite-content">
          <div class="favorite-title">{{ favorite.title }}</div>
          <div class="favorite-meta">
            <span v-if="favorite.file_size" class="file-size">{{ favorite.file_size }}</span>
            <span class="created-date">Added: {{ formatDate(favorite.created_at) }}</span>
          </div>
          <div v-if="favorite.file_list && favorite.file_list.length > 0" class="file-list">
            <details>
              <summary>Files ({{ favorite.file_list.length }})</summary>
              <ul>
                <li v-for="file in favorite.file_list.slice(0, 10)" :key="file">{{ file }}</li>
                <li v-if="favorite.file_list.length > 10">... and {{ favorite.file_list.length - 10 }} more</li>
              </ul>
            </details>
          </div>
        </div>
        
        <div class="favorite-actions">
          <button @click="copyMagnetLink(favorite.magnet_link)" class="copy-btn" title="Copy magnet link">
            📋
          </button>
          <button
            @click="removeFavorite(favorite.id)"
            :class="['remove-btn', { 'confirm-delete': favorite.id === pendingDeleteId }]"
            :title="favorite.id === pendingDeleteId ? 'Confirm deletion' : 'Remove from favorites'"
          >
            {{ favorite.id === pendingDeleteId ? '❓' : '🗑️' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, inject, watch, Ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";

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
const pendingDeleteId = ref<string | null>(null);
const deleteTimeout = ref<any>(null);

const favoritesTimestamp = inject<Ref<number>>('favoritesTimestamp');

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
    alert(`Failed to load favorites: ${error}`);
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

async function removeFavorite(id: string) {
  clearTimeout(deleteTimeout.value);

  if (pendingDeleteId.value === id) {
    try {
      await invoke("remove_from_favorites", { id });
      await loadFavorites(); // Reload the list
      pendingDeleteId.value = null;
    } catch (error) {
      console.error("Failed to remove favorite:", error);
      alert(`Failed to remove favorite: ${error}`);
    }
  } else {
    pendingDeleteId.value = id;
    deleteTimeout.value = setTimeout(() => {
      pendingDeleteId.value = null;
    }, 3500);
  }
}

async function copyMagnetLink(magnetLink: string) {
  try {
    await navigator.clipboard.writeText(magnetLink);
    alert("Magnet link copied to clipboard!");
  } catch (error) {
    console.error("Failed to copy magnet link:", error);
    alert("Failed to copy magnet link");
  }
}

function formatDate(dateString: string): string {
  try {
    const date = new Date(dateString);
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  } catch {
    return dateString;
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
