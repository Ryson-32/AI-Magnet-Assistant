<script setup lang="ts">
import { ref, provide, onMounted, watch } from "vue";
import SideNavigation from "./components/SideNavigation.vue";
import HomePage from "./components/HomePage.vue";
import FavoritesPage from "./components/FavoritesPage.vue";
import EnginesPage from "./components/EnginesPage.vue";
import PriorityPage from "./components/PriorityPage.vue";
import SettingsPage from "./components/SettingsPage.vue";
import { useStore } from "./composables/useStore";

const { saveToStore, loadFromStore } = useStore();

const currentPage = ref('home');

// 全局搜索状态
const searchState = ref({
  keyword: "",
  results: [],
  searchStatus: "",
  isSearching: false,
  useSmartFilter: true,
  maxPages: 1,
  sortBy: 'score',
  titleMustContainKeyword: true,
});

// 提供搜索状态给子组件
provide('searchState', searchState);

// 在组件挂载时加载设置
onMounted(async () => {
  const savedSettings = await loadFromStore('search_settings');
  if (savedSettings) {
    searchState.value.useSmartFilter = savedSettings.useSmartFilter ?? true;
    searchState.value.maxPages = savedSettings.maxPages ?? 1;
    searchState.value.sortBy = savedSettings.sortBy ?? 'score';
    searchState.value.titleMustContainKeyword = savedSettings.titleMustContainKeyword ?? true;
  }
});

// 监听设置变化并保存
watch(
  () => ({
    useSmartFilter: searchState.value.useSmartFilter,
    maxPages: searchState.value.maxPages,
    sortBy: searchState.value.sortBy,
    titleMustContainKeyword: searchState.value.titleMustContainKeyword,
  }),
  (newSettings) => {
    saveToStore('search_settings', newSettings);
  },
  { deep: true }
);

function navigate(page: string) {
  currentPage.value = page;
}
</script>

<template>
  <div class="app">
    <SideNavigation 
      :current-page="currentPage" 
      @navigate="navigate" 
    />
    
    <main class="main-content">
      <HomePage v-show="currentPage === 'home'" />
      <FavoritesPage v-show="currentPage === 'favorites'" />
      <EnginesPage v-show="currentPage === 'engines'" />
      <PriorityPage v-show="currentPage === 'priority'" />
      <SettingsPage v-show="currentPage === 'settings'" />
    </main>
  </div>
</template>

<style>
/* 全局样式 - 防止水平滚动 */
* {
  box-sizing: border-box;
}

html, body {
  margin: 0;
  padding: 0;
  overflow-x: hidden;
  width: 100%;
}

#app {
  width: 100%;
  overflow-x: hidden;
}
</style>

<style scoped>
.app {
  display: flex;
  height: 100vh;
  background: #f8fafc;
  width: 100%;
  overflow-x: hidden;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  min-width: 0;
}
</style>
