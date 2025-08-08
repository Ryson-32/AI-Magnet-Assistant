<template>
  <div class="settings-page">
    <div class="page-header">
      <h1>{{ $t('pages.settings.title') }}</h1>
      <p>{{ $t('pages.settings.subtitle') }}</p>
    </div>

    <!-- 语言设置区域 -->
    <div class="settings-section">
      <LanguageSwitcher />
    </div>

    <div class="settings-section">
      <div class="section-header">
        <h2>{{ $t('pages.settings.ai.title') }}</h2>
        <p>{{ $t('pages.settings.ai.subtitle') }}</p>
      </div>

      <!-- 第一次API调用配置：HTML提取 -->
      <div class="ai-config-section">
        <h3>{{ $t('pages.settings.ai.extraction.title') }}</h3>
        <p class="config-description">{{ $t('pages.settings.ai.extraction.description') }}</p>

        <div class="settings-form">
          <div class="form-group">
            <label for="extractionProvider">{{ $t('pages.settings.ai.extraction.provider') }}</label>
            <select id="extractionProvider" v-model="llmConfig.extraction_config.provider">
              <option value="gemini">Google Gemini</option>
              <option value="openai">OpenAI</option>
            </select>
          </div>

          <div class="form-group">
            <label for="extractionApiKey">{{ $t('pages.settings.ai.extraction.apiKey') }}</label>
            <div class="input-with-button">
              <input
                id="extractionApiKey"
                v-model="llmConfig.extraction_config.api_key"
                type="password"
                :placeholder="$t('pages.settings.ai.extraction.placeholders.apiKey')"
                required />
              <button type="button" @click="testExtractionConnection" class="test-btn" :disabled="isTestingExtraction">
                {{ $t('pages.settings.ai.extraction.testing') }}
              </button>
            </div>
            <small class="help-text">{{ $t('pages.settings.ai.extraction.helpText') }}</small>
          </div>

          <div class="form-group">
            <label for="extractionApiBase">{{ $t('pages.settings.ai.extraction.apiBase') }}</label>
            <input
              id="extractionApiBase"
              v-model="llmConfig.extraction_config.api_base"
              type="url"
              :placeholder="$t('pages.settings.ai.extraction.placeholders.apiBase')"
              required
            />
          </div>

          <div class="form-group">
            <label for="extractionModel">{{ $t('pages.settings.ai.extraction.model') }}</label>
            <input
              id="extractionModel"
              v-model="llmConfig.extraction_config.model"
              type="text"
              :placeholder="$t('pages.settings.ai.extraction.placeholders.model')"
              required
            />
          </div>
        </div>
      </div>

      <!-- 第二次API调用配置：内容分析 -->
      <div class="ai-config-section">
        <h3>{{ $t('pages.settings.ai.analysis.title') }}</h3>
        <p class="config-description">{{ $t('pages.settings.ai.analysis.description') }}</p>

        <div class="settings-form">
          <div class="form-group">
            <label for="analysisProvider">{{ $t('pages.settings.ai.analysis.provider') }}</label>
            <select id="analysisProvider" v-model="llmConfig.analysis_config.provider">
              <option value="gemini">Google Gemini</option>
              <option value="openai">OpenAI</option>
            </select>
          </div>

          <div class="form-group">
            <label for="analysisApiKey">{{ $t('pages.settings.ai.analysis.apiKey') }}</label>
            <div class="input-with-button">
              <input
                id="analysisApiKey"
                v-model="llmConfig.analysis_config.api_key"
                type="password"
                :placeholder="$t('pages.settings.ai.analysis.placeholders.apiKey')"
                required />
              <button type="button" @click="testAnalysisConnection" class="test-btn" :disabled="isTestingAnalysis">
                {{ $t('pages.settings.ai.analysis.testing') }}
              </button>
            </div>
            <small class="help-text">{{ $t('pages.settings.ai.analysis.helpText') }}</small>
          </div>

          <div class="form-group">
            <label for="analysisApiBase">{{ $t('pages.settings.ai.analysis.apiBase') }}</label>
            <input
                id="analysisApiBase"
                v-model="llmConfig.analysis_config.api_base"
                type="url"
                :placeholder="$t('pages.settings.ai.analysis.placeholders.apiBase')"
                required
            />
          </div>

          <div class="form-group">
            <label for="analysisModel">{{ $t('pages.settings.ai.analysis.model') }}</label>
            <input  
              id="analysisModel"
              v-model="llmConfig.analysis_config.model"
              type="text"
              :placeholder="$t('pages.settings.ai.analysis.placeholders.model')"
              required
            />
          </div>

          <div class="form-group">
            <label for="analysisBatchSize">{{ $t('pages.settings.ai.analysis.batchSize') }}</label>
            <input
              id="analysisBatchSize"
              v-model.number="llmConfig.analysis_config.batch_size"
              type="number"
              min="1"
              max="20"
              :placeholder="$t('pages.settings.ai.analysis.placeholders.batchSize')"
              required
            />
            <small class="help-text">{{ $t('pages.settings.ai.analysis.batchSizeHelp') }}</small>
          </div>
        </div>
      </div>

      <div class="form-actions">
        <div class="info-section">
          <div class="rate-limit-info" @mouseenter="showRateLimit = true" @mouseleave="hideRateLimit">
            <svg
              class="table-icon"
              viewBox="0 0 24 24"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path d="M3 6h18v2H3V6zm0 5h18v2H3v-2zm0 5h18v2H3v-2z" fill="currentColor"/>
            </svg>
            <span class="rate-limit-text">{{ $t('pages.settings.ai.rateLimit.title') }}</span>

            <!-- 悬停显示的速率限制表格 -->
            <div v-if="showRateLimit" class="rate-limit-tooltip" @mouseenter="clearHideTimeout" @mouseleave="hideRateLimit">
              <h4>{{ $t('pages.settings.ai.rateLimit.tableTitle') }}</h4>
              <table class="rate-limit-table">
                <thead>
                  <tr>
                    <th>{{ $t('pages.settings.ai.rateLimit.headers.model') }}</th>
                    <th>{{ $t('pages.settings.ai.rateLimit.headers.requestsPerMin') }}</th>
                    <th>{{ $t('pages.settings.ai.rateLimit.headers.tokensPerMin') }}</th>
                    <th>{{ $t('pages.settings.ai.rateLimit.headers.requestsPerDay') }}</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>gemini-2.5-pro</td>
                    <td>5</td>
                    <td>250,000</td>
                    <td>100</td>
                  </tr>
                  <tr>
                    <td>gemini-2.5-flash</td>
                    <td>10</td>
                    <td>250,000</td>
                    <td>250</td>
                  </tr>
                  <tr class="highlight">
                    <td>gemini-2.5-flash-lite-preview-06-17</td>
                    <td>15</td>
                    <td>250,000</td>
                    <td>1,000</td>
                  </tr>
                  <tr>
                    <td>gemini-2.0-flash</td>
                    <td>15</td>
                    <td>1,000,000</td>
                    <td>200</td>
                  </tr>
                  <tr>
                    <td>gemini-2.0-flash-lite</td>
                    <td>30</td>
                    <td>1,000,000</td>
                    <td>200</td>
                  </tr>
                </tbody>
              </table>
              <div class="rate-limit-footer">
                <a href="https://ai.google.dev/gemini/api/docs/rate-limits" target="_blank" rel="noopener noreferrer" class="rate-limit-link">
                  {{ $t('pages.settings.ai.rateLimit.documentation') }}
                </a>
              </div>
            </div>
          </div>

          <div class="gemini-balance-info">
            <span class="balance-text">{{ $t('pages.settings.ai.rateLimit.rateLimitTooLow') }}</span>
            <a href="https://github.com/snailyp/gemini-balance" target="_blank" rel="noopener noreferrer" class="balance-link">
              <span>{{ $t('pages.settings.ai.rateLimit.tryGeminiBalance') }}</span>
              <svg class="github-icon" viewBox="0 0 24 24" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222 v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
            </a>
          </div>
        </div>

        <button type="button" @click="saveLlmConfig" :disabled="isSaving" class="save-btn">{{ isSaving ? $t('pages.settings.ai.saving') : $t('pages.settings.ai.save') }}</button>
      </div>
    </div>

    <div class="settings-section">
      <div class="section-header">
        <h2>{{ $t('pages.settings.download.title') }}</h2>
        <p>{{ $t('pages.settings.download.subtitle') }}</p>
      </div>

      <div class="settings-form">
        <div class="form-group">
          <label for="customAppPath">{{ $t('pages.settings.download.applicationPath') }}</label>
          <div class="input-with-button">
            <input
              id="customAppPath"
              v-model="downloadConfig.custom_app_path"
              type="text"
              :placeholder="$t('pages.settings.download.applicationPathPlaceholder')"
              required />
            <button type="button" @click="browseForApplication" class="browse-btn">
              {{ $t('pages.settings.download.browse') }}
            </button>
          </div>
          <small class="help-text">{{ $t('pages.settings.download.applicationPathHelp') }}</small>
        </div>

        <div class="form-group">
          <div class="checkbox-container">
            <label class="checkbox-only">
              <input 
                type="checkbox" 
                v-model="downloadConfig.auto_close_page" 
              />
              <span class="checkmark"></span>
            </label>
            <span class="checkbox-text">{{ $t('pages.settings.download.autoClosePage') }}</span>
          </div>
          <small class="help-text">{{ $t('pages.settings.download.autoClosePageHelp') }}</small>
        </div>

        <div class="form-group checkbox-with-button">
          <div class="checkbox-section">
            <div class="checkbox-container">
              <label class="checkbox-only">
                <input
                  type="checkbox" 
                  v-model="downloadConfig.enable_quick_download" 
                />
                <span class="checkmark"></span>
              </label>
              <span class="checkbox-text">{{ $t('pages.settings.download.enableQuickDownload') }}</span>
            </div>
            <small class="help-text">{{ $t('pages.settings.download.enableQuickDownloadHelp') }}</small>
          </div>
          <button type="button" @click="saveDownloadConfig" :disabled="isSavingDownload" class="save-btn-inline">
            {{ isSavingDownload ? $t('pages.settings.download.saving') : $t('pages.settings.download.save') }}
          </button>
        </div>
      </div>
    </div>

    <div class="settings-section">
      <div class="section-header">
        <h2>{{ $t('pages.settings.data.title') }}</h2>
        <p>{{ $t('pages.settings.data.subtitle') }}</p>
      </div>
      <div class="data-config-grid">
        <div class="data-config-item">
          <div>
            <h4>{{ $t('pages.settings.data.applicationData.title') }}</h4>
            <p>{{ $t('pages.settings.data.applicationData.description') }}</p>
          </div>
          <button @click="openConfigFolder" class="open-folder-btn">
            {{ $t('pages.settings.data.applicationData.openLocation') }}
          </button>
        </div>
      </div>
    </div>

    <div class="about-section">
      <div class="section-header">
        <h2>{{ $t('pages.settings.about.title') }}</h2>
      </div>
      
      <div class="about-content">
        <div class="app-info">
          <h3>{{ $t('pages.settings.about.appInfo.name') }}</h3>
          <p>{{ $t('pages.settings.about.appInfo.version') }}</p>
          <p>{{ $t('pages.settings.about.appInfo.description') }}</p>
        </div>
        
        <div class="features-list">
          <h4>{{ $t('pages.settings.about.features.title') }}</h4>
          <ul>
            <li v-for="(feature, index) in getFeaturesList()" :key="`feature-${index}`">{{ feature }}</li>
          </ul>
        </div>
        
        <div class="tech-stack">
          <h4>{{ $t('pages.settings.about.techStack.title') }}</h4>
          <div class="tech-badges">
            <span v-for="(tech, index) in getTechStackList()" :key="`tech-${index}`" class="tech-badge">{{ tech }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, inject } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { appDataDir } from '@tauri-apps/api/path';
import { openPath } from '@tauri-apps/plugin-opener';
import { useI18n } from '../composables/useI18n';
import LanguageSwitcher from './LanguageSwitcher.vue';

// 注入全局通知函数
const showNotification = inject('showNotification') as (message: string, type?: 'success' | 'error', duration?: number) => void;
const { t } = useI18n();

const llmConfig = ref({
  extraction_config: {
    provider: "gemini",
    api_key: "",
    api_base: "https://generativelanguage.googleapis.com",
    model: "gemini-2.5-flash",
  },
  analysis_config: {
    provider: "gemini",
    api_key: "",
    api_base: "https://generativelanguage.googleapis.com",
    model: "gemini-2.5-flash-lite",
    batch_size: 5,
  },
});

const isSaving = ref(false);
const isTestingExtraction = ref(false);
const isTestingAnalysis = ref(false);
const showRateLimit = ref(false);
let hideTimeout: number | null = null;

// 下载配置相关
const downloadConfig = ref({
  custom_app_path: null as string | null,
  enable_quick_download: true,
  auto_close_page: true,
});

const isSavingDownload = ref(false);

onMounted(async () => {
  await loadLlmConfig();
  await loadDownloadConfig();
});

watch(() => llmConfig.value.extraction_config.provider, (newProvider) => {
  if (newProvider === 'gemini') {
    llmConfig.value.extraction_config.api_base = 'https://generativelanguage.googleapis.com';
    llmConfig.value.extraction_config.model = 'gemini-2.5-flash';
  } else if (newProvider === 'openai') {
    llmConfig.value.extraction_config.api_base = 'https://api.openai.com/v1';
    llmConfig.value.extraction_config.model = 'gpt-3.5-turbo'; 
  }
});

watch(() => llmConfig.value.analysis_config.provider, (newProvider) => {
  if (newProvider === 'gemini') {
    llmConfig.value.analysis_config.api_base = 'https://generativelanguage.googleapis.com';
    llmConfig.value.analysis_config.model = 'gemini-2.5-flash-lite';
    // Keep existing batch_size or set default if not set
    if (!llmConfig.value.analysis_config.batch_size) {
      llmConfig.value.analysis_config.batch_size = 5;
    }
  } else if (newProvider === 'openai') {
    llmConfig.value.analysis_config.api_base = 'https://api.openai.com/v1';
    llmConfig.value.analysis_config.model = 'gpt-3.5-turbo';
    // Keep existing batch_size or set default if not set
    if (!llmConfig.value.analysis_config.batch_size) {
      llmConfig.value.analysis_config.batch_size = 5;
    }
  }
}); 

async function loadLlmConfig() {
  try {
    const saved = await invoke("get_llm_config");
    if (saved) {
      llmConfig.value = { ...llmConfig.value, ...saved };  
      // Ensure batch_size has a default value for backward compatibility
      if (!llmConfig.value.analysis_config.batch_size) {
        llmConfig.value.analysis_config.batch_size = 5;
      }
    }
  } catch (error) {
    console.error("Failed to load LLM config:", error);
  }
}

async function saveLlmConfig() {
  isSaving.value = true;
  try {
    console.log("Saving LLM config:", llmConfig.value);
    await invoke("update_llm_config", { config: llmConfig.value });
    console.log("LLM config saved successfully to app_data.json");
    showNotification(t('pages.settings.messages.settingsSaved'));
  } catch (error) {
    console.error("Failed to save LLM config:", error); 
    showNotification(t('pages.settings.messages.settingsSaveFailed', { error: String(error) }), 'error');
  } finally {
    isSaving.value = false;
  }
}

async function testExtractionConnection() {
  if (!llmConfig.value.extraction_config.api_key.trim()) 
  {
    showNotification(t('pages.settings.messages.pleaseEnterApiKey', { type: t('pages.settings.ai.extraction.title') }), 'error');
    return;
  }

  isTestingExtraction.value = true;
  try {
    const result = await invoke("test_extraction_connection", { config: llmConfig.value.extraction_config });
    showNotification(t('pages.settings.messages.testConnectionSuccess', { type: t('pages.settings.ai.extraction.title'), result: String(result) }));
  } catch (error) {
    console.error("Extraction API connection test failed:", error);
    showNotification(t('pages.settings.messages.testConnectionFailed', { type: t('pages.settings.ai.extraction.title'), error: String(error) }), 'error');
  } finally {
    isTestingExtraction.value = false; 
  }
}

async function testAnalysisConnection() {
  if (!llmConfig.value.analysis_config.api_key.trim()) 
  {
    showNotification(t('pages.settings.messages.pleaseEnterApiKey', { type: t('pages.settings.ai.analysis.title') }), 'error');
    return;
  }

  isTestingAnalysis.value = true;
  try {
    const result = await invoke("test_analysis_connection", { config: llmConfig.value.analysis_config });
    showNotification(t('pages.settings.messages.testConnectionSuccess', { type: t('pages.settings.ai.analysis.title'), result: String(result) })); 
  } catch (error) {
    console.error("Analysis API connection test failed:", error);
    showNotification(t('pages.settings.messages.testConnectionFailed', { type: t('pages.settings.ai.analysis.title'), error: String(error) }), 'error');
  } finally {
    isTestingAnalysis.value = false;
  }
}

function hideRateLimit() {
  if (hideTimeout) {
    clearTimeout(hideTimeout);
  } 
  hideTimeout = setTimeout(() => {
    showRateLimit.value = false;
  }, 100); // 100ms延迟，给用户时间移动鼠标到浮窗
}

function clearHideTimeout() {
  if (hideTimeout) {
    clearTimeout(hideTimeout);
    hideTimeout = null;
  }
  showRateLimit.value = true;
}

// 下载配置相关函数
async function loadDownloadConfig() {
  try {
    const config = await invoke("get_download_config");
    downloadConfig.value = config as any;
    console.log("Download config loaded:", config);
  } catch (error) {
    console.error("Failed to load download config:", error);
    showNotification(t('pages.settings.messages.loadDownloadConfigFailed', { error: String(error) }), 'error');
  }
}

async function saveDownloadConfig() {
  isSavingDownload.value = true;
  try {
    console.log("Saving download config:", downloadConfig.value);
    await invoke("update_download_config", { config: downloadConfig.value });
    console.log("Download config saved successfully");
    showNotification(t('pages.settings.messages.downloadSettingsSaved')); 
  } catch (error) {
    console.error("Failed to save download config:", error);
    showNotification(t('pages.settings.messages.downloadSettingsSaveFailed', { error: String(error) }), 'error');    
  } finally {
    isSavingDownload.value = false;
  }
}

async function browseForApplication() {
  try {
    const result = await invoke("browse_for_file");
    if (result) {
      downloadConfig.value.custom_app_path = result as string;
      console.log("Selected application path:", result);
    }
  } catch (error) {
    console.error("Failed to browse for application:", error);
    showNotification(t('pages.settings.messages.browseFileFailed', { error: String(error) }), 'error');
  }
}

async function openConfigFolder() {
  const dir = await appDataDir();
  try {
    // 直接打开应用数据目录（com.ai-magnet-assistant.app 文件夹内部）
    await openPath(dir);
  } catch (error) {
    console.error("Failed to open config folder:", error);
    showNotification(t('pages.settings.messages.openFolderFailed', { error: String(error) }), 'error');
  }
}

// 获取功能列表
function getFeaturesList() {
  const { t, locale } = useI18n()
  
  try {
    // 尝试直接访问翻译数据
    const features = t('pages.settings.about.features.items');
    if (Array.isArray(features)) {
      return features;
    }
  } catch (error) {
    console.warn('Error accessing features list:', error);
  }
  
  // 备选方案：硬编码的功能列表（临时解决方案）
  return locale.value === 'zh-CN' ? [
    '多引擎搜索聚合',
    'AI驱动的内容分析和过滤',
    '可自定义搜索引擎',
    '优先级关键词系统',
    '收藏夹管理',
    '智能结果排名'
  ] : [
    'Multi-engine search aggregation',
    'AI-powered content analysis and filtering',
    'Customizable search engines',
    'Priority keyword system',
    'Favorites management',
    'Smart result ranking'
  ];
}

// 获取技术栈列表
function getTechStackList() {
  const { t } = useI18n()
  
  try {
    const techStack = t('pages.settings.about.techStack.badges');
    if (Array.isArray(techStack)) {
      return techStack;
    }
  } catch (error) {
    console.warn('Error accessing tech stack list:', error);
  }
  
  // 备选方案：硬编码的技术栈（临时解决方案）
  return ['Tauri', 'Vue 3', 'Rust', 'TypeScript'];
}
</script>

<style scoped>
.settings-page {
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

.settings-section, .about-section {
  background: white;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  margin-bottom: 24px;
}

.section-header {
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid #e2e8f0;
}

.section-header h2 {
  margin: 0 0 8px 0;
  font-size: 24px;
  font-weight: 600;
  color: #1a202c;
}

.section-header p {
  margin: 0;
  color: #718096;
  font-size: 14px;
}

.ai-config-section {
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
}

.ai-config-section h3 {
  margin: 0 0 8px 0;
  font-size: 18px;
  font-weight: 600;
  color: #2d3748;
}

.config-description {
  margin: 0 0 16px 0;
  color: #718096;
  font-size: 14px;
  font-style: italic;
}

.settings-form {
  display: grid;
  gap: 20px;
}

.form-group {
  display: grid;
  gap: 8px;
}

.form-group label {
  font-weight: 600;
  color: #1a202c;
  font-size: 14px;
}

.form-group input, .form-group select {
  padding: 12px 16px;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 16px;
  transition: border-color 0.2s;
}

.form-group input:focus, .form-group select:focus {
  outline: none;
  border-color: #667eea;
}

.input-with-button {
  display: flex;
  gap: 8px;
}

.input-with-button input {
  flex: 1;
}

.test-btn {
  padding: 12px 20px;
  background: #f7fafc;
  color: #4a5568;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}

.test-btn:hover:not(:disabled) {
  background: #edf2f7;
  border-color: #cbd5e0;
}

.test-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.help-text {
  color: #718096;
  font-size: 12px;
  margin-top: 4px;
}

.form-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 24px;
  padding-top: 16px;
  border-top: 1px solid #e2e8f0;
}

.info-section {
  display: flex;
  align-items: center;
  gap: 24px;
}

.rate-limit-info {
  position: relative;
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
  color: #718096;
  font-size: 12px;
  transition: color 0.2s ease;
}

.rate-limit-info:hover {
  color: #4a5568;
}

.table-icon {
  width: 14px;
  height: 14px;
  color: #718096;
}

.rate-limit-text {
  user-select: none;
}

.rate-limit-tooltip {
  position: absolute;
  bottom: 100%;
  left: 0;
  margin-bottom: 8px;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
  padding: 16px;
  z-index: 1000;
  min-width: 500px;
  animation: fadeIn 0.2s ease;
}

.rate-limit-tooltip h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
  color: #1a202c;
}

.rate-limit-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 12px;
}

.rate-limit-table th,.rate-limit-table td {
  padding: 8px 12px;
  text-align: left;
  border-bottom: 1px solid #e2e8f0;
}

.rate-limit-table th {
  background: #f8fafc;
  font-weight: 600;
  color: #4a5568;
}

.rate-limit-table tr.highlight {
  background: #f0fff4;
}

.rate-limit-table tr.highlight td {
  color: #22543d;
  font-weight: 500;
}

.rate-limit-footer {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid #e2e8f0;
}

.rate-limit-link {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  color: #3182ce;
  text-decoration: none;
  font-size: 12px;
  font-weight: 500;
  transition: color 0.2s ease;
}

.rate-limit-link:hover {
  color: #2c5aa0;
  text-decoration: underline;
}

.gemini-balance-info {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: #718096;
}

.balance-text {
  user-select: none;
}

.balance-link {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  color: #1a202c;
  text-decoration: none;
  font-weight: 500;
  transition: color 0.2s ease;
  padding: 4px 8px;
  border-radius: 4px;
  background: #f8fafc;
  border: 1px solid #e2e8f0;
}

.balance-link:hover {
  color: #2d3748;
  background: #f1f5f9;
  border-color: #cbd5e0;
}

.github-icon {
  width: 14px;
  height: 14px;
  color: #1a202c;
  flex-shrink: 0;
  transition: color 0.2s ease;
}

.balance-link:hover .github-icon {
  color: #2d3748;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(4px);
  }  
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.save-btn {
  padding: 12px 24px;
  background: #3b82f6;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.save-btn:hover:not(:disabled) {
  background: #2563eb;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.save-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.about-content {
  display: grid;
  gap: 24px;
}

.app-info h3 {
  margin: 0 0 8px 0;
  font-size: 20px;
  font-weight: 600;
  color: #1a202c;
}

.app-info p {
  margin: 0 0 8px 0;
  color: #4a5568;
  line-height: 1.5;
}

.features-list h4, .tech-stack h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: 600;
  color: #1a202c;
}

.features-list ul {
  margin: 0;
  padding-left: 20px;
}

.features-list li {
  margin-bottom: 6px;
  color: #4a5568;
  line-height: 1.4;
}

.tech-badges {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.tech-badge {
  background: #3b82f6;
  color: white;
  padding: 6px 12px;
  border-radius: 16px;
  font-size: 12px;
  font-weight: 500;
}

.data-config-grid {
  display: grid;
  gap: 16px;
}

.data-config-item {
  background-color: #f9fafb;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  padding: 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
}

.data-config-item h4 {
  margin: 0 0 4px 0;
  font-size: 16px;
  font-weight: 600;
  color: #1a202c;
}

.data-config-item p {
  margin: 0;
  color: #4a5568;
  font-size: 14px;
}

.data-config-item code {
  background-color: #e2e8f0;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Courier New', Courier, monospace;
  font-size: 13px;
}

.open-folder-btn {
  padding: 10px 16px;
  background: white;
  color: #3b82f6;
  border: 1px solid #3b82f6;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}

.open-folder-btn:hover {
  background: #eff6ff;
  border-color: #2563eb;
}

/* Checkbox styles */
.checkbox-container {
  display: flex;
  align-items: center;
  gap: 12px;
}

.checkbox-only {
  cursor: pointer;
  user-select: none;
}

.checkbox-only input[type="checkbox"] {
  display: none;
}

.checkbox-text {
  font-size: 14px;
  font-weight: 500;
  color: #1a202c;
  user-select: none;
}

.checkmark {
  width: 20px;
  height: 20px;
  border: 2px solid #d1d5db;
  border-radius: 4px;
  position: relative;
  transition: all 0.2s ease;
  background: white;
  display: flex;
  align-items: center;
  justify-content: center;
}

.checkbox-only input[type="checkbox"]:checked + .checkmark {
  background: #3b82f6;
  border-color: #3b82f6;
}

.checkbox-only input[type="checkbox"]:checked + .checkmark::after {
  content: '✓';
  color: white;
  font-size: 14px;
  font-weight: bold;
  line-height: 1;
}

.checkbox-only:hover .checkmark {
  border-color: #3b82f6;
}

/* Checkbox with button layout */
.checkbox-with-button {
  display: flex;
  align-items: flex-start;
  gap: 20px;
} 

.checkbox-section {
  flex: 1;
}

.save-btn-inline {
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
  align-self: flex-start;
  margin-top: 4px;
}

.save-btn-inline:hover:not(:disabled) {
  background: #2563eb;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.save-btn-inline:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* Browse button style */
.browse-btn {
  padding: 12px 20px;
  background: #f7fafc;
  color: #4a5568;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}

.browse-btn:hover {
  background: #edf2f7;
  border-color: #cbd5e0;
}
</style>