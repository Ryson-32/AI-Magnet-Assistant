<template>
  <div class="settings-page">
    <div class="page-header">
      <h1>Settings</h1>
      <p>Configure your application preferences</p>
    </div>

    <div class="settings-section">
      <div class="section-header">
        <h2>LLM Configuration</h2>
        <p>Configure AI services for intelligent content analysis</p>
      </div>
      
      <form @submit.prevent="saveLlmConfig" class="settings-form">
        <div class="form-group">
          <label for="provider">Provider</label>
          <select id="provider" v-model="llmConfig.provider">
            <option value="gemini">Google Gemini</option>
            <option value="openai">OpenAI</option>
          </select>
        </div>

        <div class="form-group">
          <label for="apiKey">API Key</label>
          <div class="input-with-button">
            <input 
              id="apiKey"
              v-model="llmConfig.api_key" 
              type="password" 
              placeholder="Enter your API key..."
              required
            />
            <button type="button" @click="testApiConnection" class="test-btn" :disabled="isTesting">
              {{ isTesting ? 'Testing...' : 'Test' }}
            </button>
          </div>
          <small class="help-text">
            Your API key is stored securely and only used for AI analysis
          </small>
        </div>

        <div class="form-group">
          <label for="apiBase">API Base URL</label>
          <input 
            id="apiBase"
            v-model="llmConfig.api_base" 
            type="url"
            placeholder="e.g., https://generativelanguage.googleapis.com/v1beta"
            required
          />
        </div>

        <div class="form-group">
          <label for="model">Model</label>
          <input 
            id="model"
            v-model="llmConfig.model" 
            type="text"
            placeholder="e.g., gemini-2.5-flash"
            required
          />
        </div>

        <div class="form-actions">
          <button type="submit" :disabled="isSaving" class="save-btn">
            {{ isSaving ? 'Saving...' : 'Save Settings' }}
          </button>
        </div>
      </form>
    </div>

    <div class="settings-section">
      <div class="section-header">
        <h2>Data & Configuration</h2>
        <p>Manage application data and configuration files</p>
      </div>
      <div class="data-config-grid">
        <div class="data-config-item">
          <div>
            <h4>Application Data</h4>
            <p>All settings including API keys, custom engines, priorities, and favorites are saved in <code>app_data.json</code>.</p>
          </div>
          <button @click="openConfigFolder" class="open-folder-btn">Open File Location</button>
        </div>
      </div>
    </div>

    <div class="about-section">
      <div class="section-header">
        <h2>About</h2>
      </div>
      
      <div class="about-content">
        <div class="app-info">
          <h3>🧲 MagnetLink Optimizer Pro</h3>
          <p>Version 1.0.0</p>
          <p>An intelligent magnet link search and optimization tool powered by AI.</p>
        </div>
        
        <div class="features-list">
          <h4>Features</h4>
          <ul>
            <li>Multi-engine search aggregation</li>
            <li>AI-powered content analysis and filtering</li>
            <li>Customizable search engines</li>
            <li>Priority keyword system</li>
            <li>Favorites management</li>
            <li>Smart result ranking</li>
          </ul>
        </div>
        
        <div class="tech-stack">
          <h4>Built With</h4>
          <div class="tech-badges">
            <span class="tech-badge">Tauri</span>
            <span class="tech-badge">Vue 3</span>
            <span class="tech-badge">Rust</span>
            <span class="tech-badge">TypeScript</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { appDataDir } from '@tauri-apps/api/path';
import { openPath, revealItemInDir } from '@tauri-apps/plugin-opener';

const llmConfig = ref({
  provider: "gemini",
  api_key: "",
  api_base: "https://generativelanguage.googleapis.com/v1beta",
  model: "gemini-2.5-flash",
});

const isSaving = ref(false);
const isTesting = ref(false);

onMounted(async () => {
  await loadLlmConfig();
});

watch(() => llmConfig.value.provider, (newProvider) => {
  if (newProvider === 'gemini') {
    llmConfig.value.api_base = 'https://generativelanguage.googleapis.com/v1beta';
    llmConfig.value.model = 'gemini-2.5-flash';
  } else if (newProvider === 'openai') {
    llmConfig.value.api_base = 'https://api.openai.com/v1';
    llmConfig.value.model = 'gpt-3.5-turbo';
  }
});

async function loadLlmConfig() {
  try {
    const saved = await invoke("get_llm_config");
    if (saved) {
      llmConfig.value = { ...llmConfig.value, ...saved };
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
    alert("Settings saved successfully!");
  } catch (error) {
    console.error("Failed to save LLM config:", error);
    alert(`Failed to save settings: ${error}`);
  } finally {
    isSaving.value = false;
  }
}

async function testApiConnection() {
  if (!llmConfig.value.api_key.trim()) {
    alert("Please enter an API key first");
    return;
  }

  isTesting.value = true;
  try {
    const result = await invoke("test_connection", { config: llmConfig.value });
    alert(`Connection successful: ${result}`);
  } catch (error) {
    console.error("API connection test failed:", error);
    alert(`Connection failed: ${error}`);
  } finally {
    isTesting.value = false;
  }
}

async function openConfigFolder() {
  const dir = await appDataDir();
  try {
    // 尝试使用 revealItemInDir 来显示文件夹
    await revealItemInDir(dir);
  } catch (error) {
    console.error("Failed to open config folder:", error);
    // 如果 revealItemInDir 失败，尝试使用 openPath
    try {
      await openPath(dir);
    } catch (fallbackError) {
      console.error("Fallback openPath also failed:", fallbackError);
      alert(`Could not open folder: ${fallbackError}`);
    }
  }
}
</script>

<style scoped>
.settings-page {
  padding: 24px;
  max-width: 800px;
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
  justify-content: flex-end;
  margin-top: 8px;
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
</style>
