<template>
  <div class="side-navigation">
    <div class="nav-header">
      <h2 class="app-title">
        <div class="app-icon">🧲</div>
        <div class="app-text-container">
          <div class="app-title-line1">AI Magnet</div>
          <div class="app-title-line2">Assistant</div>
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
    <div class="footer-links">
      <a href="https://github.com/Ryson-32/AI-Magnet-Assistant" target="_blank" rel="noopener noreferrer">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="white" class="github-icon"><path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path></svg>
      </a>
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
  gap: 8px;
  padding-left: 12px;
}

.app-icon {
  font-size: 28px;
  line-height: 1;
  display: flex;
  align-items: center;
}

.app-text-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.app-title-line1,
.app-title-line2 {
  font-weight: 700;
  color: #1e293b;
  line-height: 1.3;
}

.app-title-line1 {
  font-size: 1.4rem;
}

.app-title-line2 {
  font-size: 1.4rem;
}

.nav-menu {
  flex-grow: 1;
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
  display: flex;
  align-items: center;
  justify-content: center;
}

.nav-label {
  font-size: 14px;
  font-weight: 500;
}

.nav-footer {
  padding: 16px 0;
  border-top: 1px solid #e2e8f0;
}

.footer-links {
  display: flex;
  justify-content: flex-start;
  padding: 16px;
  gap: 16px;
  border-top: 1px solid #e2e8f0;
}

.footer-links a {
  display: flex;
  align-items: center;
  justify-content: center;
}

.github-icon {
  background: #1a1a1a;
  border-radius: 50%;
  padding: 4px;
  transition: all 0.2s ease;
}

.footer-links a:hover .github-icon {
  background: #333;
  transform: scale(1.1);
}
</style>
