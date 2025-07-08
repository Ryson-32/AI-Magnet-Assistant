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
              :class="['delete-btn', { 'confirm-delete': engine.id === pendingDeleteId }]"
              :title="engine.id === pendingDeleteId ? 'Confirm deletion' : 'Delete engine'"
            >
              {{ engine.id === pendingDeleteId ? '❓' : '🗑️' }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <div class="add-engine-section">
      <div class="section-header">
        <h2>Add New Engine (Experimental, may be very slow)</h2>
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
import { ref, onMounted, inject } from 'vue';
import { invoke } from "@tauri-apps/api/core";

// 注入全局通知函数
const showNotification = inject('showNotification') as (message: string, type?: 'success' | 'error', duration?: number) => void;

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
const pendingDeleteId = ref<string | null>(null);
const deleteTimeout = ref<any>(null);

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
    showNotification(`Failed to load engines: ${error}`, 'error');
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
    showNotification(`Failed to update engine status: ${error}`, 'error');
    // Reload to restore correct state
    await loadEngines();
  }
}

async function deleteEngine(id: string) {
  clearTimeout(deleteTimeout.value);

  if (pendingDeleteId.value === id) {
    try {
      await invoke("delete_engine", { id });
      await loadEngines(); // Reload the list
      pendingDeleteId.value = null;
    } catch (error) {
      console.error("Failed to delete engine:", error);
      showNotification(`Failed to delete engine: ${error}`, 'error');
    }
  } else {
    pendingDeleteId.value = id;
    deleteTimeout.value = setTimeout(() => {
      pendingDeleteId.value = null;
    }, 3500);
  }
}

async function addEngine() {
  if (!newEngine.value.name || !newEngine.value.urlExample1 || !newEngine.value.urlExample2) {
    showNotification("Please fill in all fields", 'error');
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
    showNotification("Search engine added successfully!");
  } catch (error) {
    console.error("Failed to add engine:", error);
    showNotification(`Failed to add engine: ${error}`, 'error');
  } finally {
    isAdding.value = false;
  }
}

function generateUrlTemplate(url1: string, url2: string): string {
  console.log("🔧 Generating URL template from:", url1, "and", url2);

  try {
    const urlObj1 = new URL(url1);
    const urlObj2 = new URL(url2);

    // Start with the base URL (origin)
    let template = urlObj1.origin;

    // Process the pathname first (this is where most search engines put keyword/page info)
    const path1 = urlObj1.pathname;
    const path2 = urlObj2.pathname;

    console.log("📍 Path1:", path1);
    console.log("📍 Path2:", path2);

    // Split paths into segments for comparison
    const segments1 = path1.split('/').filter(s => s.length > 0);
    const segments2 = path2.split('/').filter(s => s.length > 0);

    console.log("📂 Segments1:", segments1);
    console.log("📂 Segments2:", segments2);

    // Build template path by comparing segments
    const templateSegments: string[] = [];
    const maxLength = Math.max(segments1.length, segments2.length);

    for (let i = 0; i < maxLength; i++) {
      const seg1 = segments1[i] || '';
      const seg2 = segments2[i] || '';

      if (seg1 === seg2) {
        // Same segment in both URLs
        templateSegments.push(seg1);
      } else {
        // Different segments - try to identify patterns
        const templateSeg = generateSegmentTemplate(seg1, seg2);
        templateSegments.push(templateSeg);
      }
    }

    template += '/' + templateSegments.join('/');

    // Now handle query parameters
    const params1 = new URLSearchParams(urlObj1.search);
    const params2 = new URLSearchParams(urlObj2.search);
    const templateParams: string[] = [];

    // Check each parameter
    for (const [key, value1] of params1) {
      const value2 = params2.get(key);

      if (value2 !== null) {
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
          // Different values - try to identify patterns
          const templateParam = generateParameterTemplate(key, value1, value2);
          templateParams.push(templateParam);
        }
      } else {
        // If the parameter doesn't exist in the second URL, just use the first one.
        templateParams.push(`${key}=${value1}`);
      }
    }

    if (templateParams.length > 0) {
      template += '?' + templateParams.join('&');
    }

    console.log("✅ Generated template:", template);
    return template;
  } catch (error) {
    console.error("❌ Failed to generate URL template:", error);
    // Enhanced fallback: handle both path and query patterns
    let fallbackTemplate = url1;

    // Replace common keyword patterns in path
    fallbackTemplate = fallbackTemplate.replace(/test/gi, '{keyword}');

    // Replace page numbers in path
    fallbackTemplate = fallbackTemplate.replace(/(\W|^)1(\W|$)/g, '$1{page}$2');

    // Replace page numbers in query parameters
    fallbackTemplate = fallbackTemplate.replace(/[&?]page=1/, '&page={page}');

    console.log("🔄 Fallback template:", fallbackTemplate);
    return fallbackTemplate;
  }
}

// Helper function to generate template for individual path segments
function generateSegmentTemplate(seg1: string, seg2: string): string {
  // This function handles segments that differ between the two URLs.
  // It's designed to find placeholders for keywords and page numbers.

  // We primarily focus on segments that are structured with hyphens,
  // as this is a common pattern for SEO-friendly URLs.
  if (seg1.includes('-') && seg2.includes('-')) {
    const parts1 = seg1.split('-');
    const parts2 = seg2.split('-');

    if (parts1.length === parts2.length) {
      const templateParts: string[] = [];

      for (let i = 0; i < parts1.length; i++) {
        const part1 = parts1[i];
        const part2 = parts2[i];

        console.log(`🔍 Comparing part ${i}: "${part1}" vs "${part2}"`);

        // Priority 1: Check for the keyword 'test'.
        // This identifies the part of the URL that holds the search term.
        if (part1.toLowerCase() === 'test' && part2.toLowerCase() === 'test') {
          console.log(`✅ Found keyword part: 'test' -> {keyword}`);
          templateParts.push('{keyword}');
          continue;
        }

        // Priority 2: Check for page numbers.
        // This looks for parts that are different and represent sequential numbers.
        if (part1 !== part2) {
          const num1Match = part1.match(/^(\d+)/);
          const num2Match = part2.match(/^(\d+)/);

          if (num1Match && num2Match) {
            const num1 = parseInt(num1Match[1], 10);
            const num2 = parseInt(num2Match[1], 10);

            // Check if they are consecutive numbers (like page 1 and 2)
            if (Math.abs(num1 - num2) === 1) {
              const restOfPart = part1.substring(num1Match[1].length);
              console.log(`✅ Found page part: ${part1} vs ${part2} -> {page}`);
              templateParts.push(`{page}${restOfPart}`);
              continue;
            }
          }
        }

        // Priority 3: If parts are identical, keep them as they are.
        if (part1 === part2) {
          console.log(`➡️ Same parts: ${part1} -> ${part1}`);
          templateParts.push(part1);
        } else {
          // Priority 4: Fallback for parts that are different but not recognized
          // as a keyword or page number. We default to the first URL's part.
          console.log(`⚠️ Different parts: ${part1} vs ${part2} -> using ${part1}`);
          templateParts.push(part1);
        }
      }

      return templateParts.join('-');
    }
  }

  // Fallback for segments that don't fit the hyphenated pattern.
  // This is a simpler check for basic page number differences.
  if (/\d+/.test(seg1) && /\d+/.test(seg2)) {
    const num1 = parseInt(seg1.match(/\d+/)?.[0] || '0');
    const num2 = parseInt(seg2.match(/\d+/)?.[0] || '0');

    if (Math.abs(num1 - num2) === 1) {
      return seg1.replace(/\d+/, '{page}');
    }
  }

  // Default: if no pattern is matched, return the segment from the first URL.
  return seg1;
}

// Helper function to generate template for query parameters
function generateParameterTemplate(key: string, value1: string, value2: string): string {
  // Check for keyword patterns
  if (value1 === 'test' || value2 === 'test') {
    return `${key}={keyword}`;
  }

  // Check for page patterns
  if (/^\d+$/.test(value1) && /^\d+$/.test(value2)) {
    const num1 = parseInt(value1);
    const num2 = parseInt(value2);

    if (Math.abs(num1 - num2) === 1) {
      return `${key}={page}`;
    }
  }

  // Default: use first value
  return `${key}=${value1}`;
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

.delete-btn.confirm-delete {
  background-color: #fbd38d; /* A yellow/orange color for confirmation */
  color: #9c4221;
}

.delete-btn.confirm-delete:hover {
  background-color: #f6ad55;
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
