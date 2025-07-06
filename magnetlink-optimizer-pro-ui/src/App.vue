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

// 收藏夹刷新时间戳
const favoritesTimestamp = ref(Date.now());
provide('favoritesTimestamp', favoritesTimestamp);

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

// Toast 通知状态
const notification = ref({
  show: false,
  message: '',
  type: 'success' as 'success' | 'error',
});

// 显示通知的函数
function showNotification(message: string, type: 'success' | 'error' = 'success', duration: number = 3000) {
  notification.value = {
    show: true,
    message,
    type,
  };
  setTimeout(() => {
    notification.value.show = false;
  }, duration);
}

// 全局提供 showNotification 函数
provide('showNotification', showNotification);
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

    <!-- 全局 Toast 通知 -->
    <Transition name="fade">
      <div 
        v-if="notification.show" 
        class="toast-notification"
        :class="`toast-${notification.type}`"
      >
        {{ notification.message }}
      </div>
    </Transition>
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

.toast-notification {
  position: fixed;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  padding: 12px 20px;
  border-radius: 8px;
  color: white;
  font-size: 14px;
  z-index: 1000;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.toast-success {
  background-color: #4CAF50; /* Green */
}

.toast-error {
  background-color: #F44336; /* Red */
}

/* 过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
