<template>
  <div class="side-navigation">
    <div class="nav-header">
      <h2 class="app-title">
        <div class="app-icon">🧲</div>
        <div class="app-text">
          <div class="app-title-line1">MagnetLink</div>
          <div class="app-title-line2">Optimizer Pro</div>
        </div>
      </h2>
    </div>
    
    <nav class="nav-menu">
      <div 
        v-for="item in menuItems" 
        :key="item.id"
        :class="['nav-item', { active: currentPage === item.id }]"
        @click="$emit('navigate', item.id)"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span class="nav-label">{{ item.label }}</span>
      </div>
    </nav>
    
    <div class="nav-footer">
      <div class="nav-item" @click="$emit('navigate', 'settings')">
        <span class="nav-icon">⚙️</span>
        <span class="nav-label">Settings</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface MenuItem {
  id: string;
  label: string;
  icon: string;
}

interface Props {
  currentPage: string;
}

defineProps<Props>();
defineEmits<{
  navigate: [page: string];
}>();

const menuItems: MenuItem[] = [
  { id: 'home', label: 'Home', icon: '🏠' },
  { id: 'favorites', label: 'Favorites', icon: '⭐' },
  { id: 'engines', label: 'Search Engines', icon: '🔍' },
  { id: 'priority', label: 'Priority Keywords', icon: '📌' },
];
</script>

<style scoped>
.side-navigation {
  width: 220px;
  height: 100vh;
  background: linear-gradient(180deg, #f8fafc 0%, #f1f5f9 100%);
  color: #334155;
  display: flex;
  flex-direction: column;
  box-shadow: 1px 0 3px rgba(0, 0, 0, 0.05);
  border-right: 1px solid #e2e8f0;
}

.nav-header {
  padding: 24px 16px;
  border-bottom: 1px solid #e2e8f0;
}

.app-title {
  margin: 0;
  display: flex;
  align-items: center;
  justify-content: flex-start;
  gap: 12px;
  padding-left: 8px;
}

.app-icon {
  font-size: 24px;
  line-height: 1;
  display: flex;
  align-items: center;
}

.app-text {
  display: flex;
  flex-direction: column;
  line-height: 1.3;
  text-align: center;
}

.app-title-line1 {
  font-size: 16px;
  font-weight: 700;
  color: #1e293b;
  margin-bottom: 4px;
}

.app-title-line2 {
  font-size: 16px;
  font-weight: 700;
  color: #1e293b;
}

.nav-menu {
  flex: 1;
  padding: 20px 0;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  cursor: pointer;
  transition: all 0.2s ease;
  border-radius: 8px;
  margin: 2px 8px;
}

.nav-item:hover {
  background: #e2e8f0;
  color: #1e293b;
}

.nav-item.active {
  background: #3b82f6;
  color: white;
}

.nav-icon {
  font-size: 18px;
  margin-right: 12px;
  width: 24px;
  text-align: center;
}

.nav-label {
  font-size: 14px;
  font-weight: 500;
}

.nav-footer {
  padding: 16px 0;
  border-top: 1px solid #e2e8f0;
}
</style>
