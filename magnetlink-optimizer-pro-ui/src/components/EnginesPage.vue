<template>
  <div class="engines-page">
    <div class="page-header">
      <h1>Search Engines</h1>
      <p>Manage and customize your search engines</p>
    </div>

    <div class="engines-list">
      <div class="section-header">
        <h2>Configured Engines</h2>
      </div>
      
      <div v-if="loading" class="loading">
        Loading engines...
      </div>
      
      <div v-else-if="engines.length === 0" class="empty-state">
        <div class="empty-icon">🔍</div>
        <h3>No search engines configured</h3>
        <p>Add your first search engine below!</p>
      </div>
      
      <div v-else class="engines-grid">
        <div v-for="engine in engines" :key="engine.id" class="engine-item">
          <div class="engine-content">
            <div class="engine-header">
              <h3>{{ engine.name }}</h3>
              <div class="engine-status">
                <label class="switch">
                  <input
                    type="checkbox"
                    :checked="engine.is_enabled"
                    @change="toggleEngine(engine.id, ($event.target as HTMLInputElement).checked)"
                  />
                  <span class="slider"></span>
                </label>
              </div>
            </div>
            <div class="engine-url">{{ engine.url_template }}</div>
            <div class="engine-meta">
              <span v-if="!engine.is_deletable" class="default-badge">Default</span>
              <span :class="['status-badge', engine.is_enabled ? 'enabled' : 'disabled']">
                {{ engine.is_enabled ? 'Enabled' : 'Disabled' }}
              </span>
            </div>
          </div>
          
          <div class="engine-actions">
            <button 
              v-if="engine.is_deletable"
              @click="deleteEngine(engine.id)" 
              class="delete-btn"
              title="Delete engine"
            >
              🗑️
            </button>
          </div>
        </div>
      </div>
    </div>

    <div class="add-engine-section">
      <div class="section-header">
        <h2>Add New Engine</h2>
        <p>Add a custom search engine by providing example URLs</p>
      </div>
      
      <form @submit.prevent="addEngine" class="add-engine-form">
        <div class="form-group">
          <label for="engineName">Engine Name</label>
          <input 
            id="engineName"
            v-model="newEngine.name" 
            type="text" 
            placeholder="e.g., My Custom Engine"
            required
          />
        </div>
        
        <div class="form-group">
          <label for="urlExample1">URL Example 1 (search for 'test', page 1)</label>
          <input 
            id="urlExample1"
            v-model="newEngine.urlExample1" 
            type="url" 
            placeholder="e.g., https://example.com/search?q=test&page=1"
            required
          />
          <small>Paste the complete URL when searching for "test" on the first page</small>
        </div>
        
        <div class="form-group">
          <label for="urlExample2">URL Example 2 (search for 'test', page 2)</label>
          <input 
            id="urlExample2"
            v-model="newEngine.urlExample2" 
            type="url" 
            placeholder="e.g., https://example.com/search?q=test&page=2"
            required
          />
          <small>Paste the complete URL when searching for "test" on the second page</small>
        </div>
        
        <div class="form-actions">
          <button type="submit" :disabled="isAdding" class="add-btn">
            {{ isAdding ? 'Adding...' : 'Add Engine' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";

interface SearchEngine {
  id: string;
  name: string;
  url_template: string;
  is_enabled: boolean;
  is_deletable: boolean;
}

const engines = ref<SearchEngine[]>([]);
const loading = ref(false);
const isAdding = ref(false);

const newEngine = ref({
  name: '',
  urlExample1: '',
  urlExample2: ''
});

onMounted(() => {
  loadEngines();
});

async function loadEngines() {
  loading.value = true;
  try {
    const result = await invoke("get_all_engines");
    engines.value = result as SearchEngine[];
  } catch (error) {
    console.error("Failed to load engines:", error);
    alert(`Failed to load engines: ${error}`);
  } finally {
    loading.value = false;
  }
}

async function toggleEngine(id: string, isEnabled: boolean) {
  try {
    await invoke("update_engine_status", { id, isEnabled });
    // Update local state
    const engine = engines.value.find(e => e.id === id);
    if (engine) {
      engine.is_enabled = isEnabled;
    }
  } catch (error) {
    console.error("Failed to update engine status:", error);
    alert(`Failed to update engine status: ${error}`);
    // Reload to restore correct state
    await loadEngines();
  }
}

async function deleteEngine(id: string) {
  if (!confirm("Are you sure you want to delete this search engine?")) {
    return;
  }

  try {
    await invoke("delete_engine", { id });
    await loadEngines(); // Reload the list
  } catch (error) {
    console.error("Failed to delete engine:", error);
    alert(`Failed to delete engine: ${error}`);
  }
}

async function addEngine() {
  if (!newEngine.value.name || !newEngine.value.urlExample1 || !newEngine.value.urlExample2) {
    alert("Please fill in all fields");
    return;
  }

  isAdding.value = true;
  try {
    // Generate URL template from examples
    const urlTemplate = generateUrlTemplate(newEngine.value.urlExample1, newEngine.value.urlExample2);
    
    await invoke("add_search_engine", {
      name: newEngine.value.name,
      urlTemplate
    });
    
    // Reset form
    newEngine.value = {
      name: '',
      urlExample1: '',
      urlExample2: ''
    };
    
    await loadEngines(); // Reload the list
    alert("Search engine added successfully!");
  } catch (error) {
    console.error("Failed to add engine:", error);
    alert(`Failed to add engine: ${error}`);
  } finally {
    isAdding.value = false;
  }
}

function generateUrlTemplate(url1: string, url2: string): string {
  // Simple URL template generation
  // This is a basic implementation - in a real app, you'd want more sophisticated logic
  try {
    const urlObj1 = new URL(url1);
    const urlObj2 = new URL(url2);
    
    // Find differences in query parameters
    const params1 = new URLSearchParams(urlObj1.search);
    const params2 = new URLSearchParams(urlObj2.search);
    
    let template = urlObj1.origin + urlObj1.pathname;
    const templateParams: string[] = [];
    
    // Check each parameter
    for (const [key, value1] of params1) {
      const value2 = params2.get(key);
      
      if (value1 === 'test' && value2 === 'test') {
        // This is likely the keyword parameter
        templateParams.push(`${key}={keyword}`);
      } else if (value1 === '1' && value2 === '2') {
        // This is likely the page parameter
        templateParams.push(`${key}={page}`);
      } else if (value1 === value2) {
        // Same value in both URLs
        templateParams.push(`${key}=${value1}`);
      } else {
        // Different values - use the first one as default
        templateParams.push(`${key}=${value1}`);
      }
    }
    
    if (templateParams.length > 0) {
      template += '?' + templateParams.join('&');
    }
    
    return template;
  } catch (error) {
    console.error("Failed to generate URL template:", error);
    // Fallback: just replace 'test' with {keyword} and page numbers with {page}
    return url1.replace(/test/gi, '{keyword}').replace(/[&?]page=1/, '&page={page}');
  }
}
</script>

<style scoped>
.engines-page {
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

.engines-list, .add-engine-section {
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

.loading {
  text-align: center;
  padding: 48px;
  color: #718096;
  font-size: 16px;
}

.empty-state {
  text-align: center;
  padding: 48px;
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

.engines-grid {
  display: grid;
  gap: 16px;
}

.engine-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 20px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  transition: all 0.2s;
}

.engine-item:hover {
  border-color: #cbd5e0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.engine-content {
  flex: 1;
}

.engine-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.engine-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #1a202c;
}

.engine-url {
  font-size: 14px;
  color: #4a5568;
  font-family: monospace;
  background: #f7fafc;
  padding: 8px 12px;
  border-radius: 4px;
  margin-bottom: 12px;
  word-break: break-all;
}

.engine-meta {
  display: flex;
  gap: 8px;
}

.default-badge, .status-badge {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.default-badge {
  background: #bee3f8;
  color: #2b6cb0;
}

.status-badge.enabled {
  background: #c6f6d5;
  color: #276749;
}

.status-badge.disabled {
  background: #fed7d7;
  color: #c53030;
}

.switch {
  position: relative;
  display: inline-block;
  width: 48px;
  height: 24px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #cbd5e0;
  transition: 0.3s;
  border-radius: 24px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
}

input:checked + .slider {
  background-color: #667eea;
}

input:checked + .slider:before {
  transform: translateX(24px);
}

.engine-actions {
  margin-left: 16px;
}

.delete-btn {
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 6px;
  background: #fed7d7;
  color: #c53030;
  cursor: pointer;
  font-size: 16px;
  transition: all 0.2s;
}

.delete-btn:hover {
  background: #feb2b2;
}

.add-engine-form {
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

.form-group input {
  padding: 12px 16px;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 16px;
  transition: border-color 0.2s;
}

.form-group input:focus {
  outline: none;
  border-color: #667eea;
}

.form-group small {
  color: #718096;
  font-size: 12px;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
}

.add-btn {
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

.add-btn:hover:not(:disabled) {
  background: #2563eb;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.add-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
