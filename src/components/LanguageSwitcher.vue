<template>
  <div class="language-switcher">
    <div class="language-section">
      <h3>{{ $t('pages.settings.language.title') }}</h3>
      <p>{{ $t('pages.settings.language.subtitle') }}</p>
      
      <div class="language-selector">
        <label for="languageSelect">{{ $t('pages.settings.language.current') }}</label>
        <select 
          id="languageSelect"
          :value="locale" 
          @change="switchLanguage"
          class="language-dropdown"
        >
          <option 
            v-for="lang in availableLocales" 
            :key="lang.code" 
            :value="lang.code"
          >
            {{ lang.flag }} {{ lang.nativeName }}
          </option>
        </select>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject } from 'vue';
import { useI18n } from '../composables/useI18n';
import { useLocale } from '../composables/useI18n';

// 注入全局通知函数
const showNotification = inject('showNotification') as (message: string, type?: 'success' | 'error', duration?: number) => void;

const { t } = useI18n();
const { locale, availableLocales, setLocale } = useLocale();

async function switchLanguage(event: Event) {
  const target = event.target as HTMLSelectElement;
  const newLocale = target.value as any;
  
  try {
    await setLocale(newLocale);
    showNotification(t('common.messages.operationSuccess'));
  } catch (error) {
    console.error('Failed to switch language:', error);
    showNotification(t('common.messages.operationFailed'), 'error');
  }
}
</script>

<style scoped>
.language-switcher {
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
}

.language-section h3 {
  margin: 0 0 8px 0;
  font-size: 18px;
  font-weight: 600;
  color: #2d3748;
}

.language-section p {
  margin: 0 0 16px 0;
  color: #718096;
  font-size: 14px;
  font-style: italic;
}

.language-selector {
  display: grid;
  gap: 8px;
}

.language-selector label {
  font-weight: 600;
  color: #1a202c;
  font-size: 14px;
}

.language-dropdown {
  padding: 12px 16px;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 16px;
  transition: border-color 0.2s;
  background: white;
  cursor: pointer;
}

.language-dropdown:focus {
  outline: none;
  border-color: #667eea;
}

.language-dropdown option {
  padding: 8px;
}
</style>