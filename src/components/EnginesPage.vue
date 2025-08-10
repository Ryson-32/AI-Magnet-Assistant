<template>
  <div class="engines-page">
    <div class="page-header">
      <h1>{{ $t('pages.engines.title') }}</h1>
      <p>{{ $t('pages.engines.subtitle') }}</p>
    </div>

    <div class="engines-list">
      <div class="section-header">
        <h2>{{ $t('pages.engines.list.title') }}</h2>
      </div>
      
      <div v-if="loading" class="loading">
        {{ $t('pages.engines.list.loading') }}
      </div>
      
      <div v-else-if="engines.length === 0" class="empty-state">
        <div class="empty-icon">🔍</div>
        <h3>{{ $t('pages.engines.list.empty') }}</h3>
        <p>{{ $t('pages.engines.list.emptyMessage') }}</p>
      </div>
      
      <div v-else class="engines-grid">
        <div v-for="engine in engines" :key="engine.id" class="engine-item">
          <div class="engine-content">
            <!-- Normal View -->
            <div v-if="editingEngineId !== engine.id">
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

              <!-- URL and Actions Row -->
              <div class="engine-url-row">
                <div class="engine-url">{{ engine.url_template }}</div>
                <div v-if="engine.is_deletable" class="engine-actions-inline">
                  <button
                    @click="startEditEngine(engine)"
                    class="edit-btn"
                    :title="$t('pages.engines.engine.actions.edit')"
                  >
                    ✏️
                  </button>
                  <button
                    @click="confirmDelete(engine.id)"
                    :class="getDeleteButtonClass(engine.id, 'delete-btn')"
                    :title="getDeleteButtonTitle(engine.id, 'pages.engines.engine.actions.confirmDelete', 'pages.engines.engine.actions.delete')"
                  >
                    {{ getDeleteIcon(engine.id) }}
                  </button>
                </div>
              </div>

              <div class="engine-meta">
                <span v-if="!engine.is_deletable" class="default-badge">{{ $t('pages.engines.engine.default') }}</span>
                <span :class="['status-badge', engine.is_enabled ? 'enabled' : 'disabled']">
                  {{ engine.is_enabled ? $t('pages.engines.engine.enabled') : $t('pages.engines.engine.disabled') }}
                </span>
              </div>
            </div>

            <!-- Edit View -->
            <div v-else class="engine-edit-form">
              <div class="edit-form-group">
                <label>{{ $t('pages.engines.engine.name') }}</label>
                <input
                  v-model="editingEngine.name"
                  type="text"
                  class="edit-input"
                />
              </div>

              <!-- URL Template and Edit Actions Row -->
              <div class="edit-url-row">
                <div class="edit-form-group-inline">
                  <label>{{ $t('pages.engines.engine.url') }}</label>
                  <input
                    v-model="editingEngine.url_template"
                    type="text"
                    class="edit-input"
                  />
                </div>
                <div class="edit-actions-inline">
                  <button
                    @click="saveEditEngine()"
                    class="save-edit-btn"
                    :title="$t('pages.engines.engine.actions.save')"
                    :disabled="isSavingEdit"
                  >
                    {{ isSavingEdit ? '⏳' : '✅' }}
                  </button>
                  <button
                    @click="cancelEditEngine()"
                    class="cancel-edit-btn"
                    :title="$t('pages.engines.engine.actions.cancel')"
                  >
                    ❌
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="add-engine-section">
      <div class="section-header">
        <h2>{{ $t('pages.engines.add.title') }} <span class="experimental-note">{{ $t('pages.engines.add.experimental') }}</span></h2>
        <p>{{ $t('pages.engines.add.subtitle') }}</p>
      </div>

      <!-- Mode Selection -->
      <div class="mode-selection">
        <label class="mode-option">
          <input type="radio" v-model="addEngineMode" value="auto" />
          <span class="radio-mark"></span>
          {{ $t('pages.engines.add.mode.auto') }}
        </label>
        <label class="mode-option">
          <input type="radio" v-model="addEngineMode" value="advanced" />
          <span class="radio-mark"></span>
          {{ $t('pages.engines.add.mode.advanced') }}
        </label>
      </div>

      <form @submit.prevent="addEngine" class="add-engine-form">
        <!-- Advanced Mode -->
        <div v-if="addEngineMode === 'advanced'" class="advanced-mode">
          <p class="help-text">
            {{ $t('pages.engines.add.form.helpText.advanced') }}
          </p>

          <div class="form-group">
            <label for="engineName">{{ $t('pages.engines.add.form.engineName') }}</label>
            <input
              id="engineName"
              v-model="newEngine.name"
              type="text"
              :placeholder="$t('pages.engines.add.form.placeholders.engineName')"
              @invalid="handleInvalid"
              @input="handleInput"
              required
            />
          </div>

          <div class="form-group">
            <label for="urlTemplate">{{ $t('pages.engines.add.form.urlTemplate') }}</label>
            <input
              id="urlTemplate"
              v-model="newEngine.urlTemplate"
              type="text"
              :placeholder="urlTemplatePlaceholder"
              @invalid="handleInvalid"
              @input="handleInput"
              required
            />
          </div>

          <div class="template-examples">
            <h4>{{ $t('pages.engines.add.form.examples.title') }}</h4>
            <ul>
              <li><code>{{ example1Text }}</code></li>
              <li><code>{{ example2Text }}</code></li>
            </ul>
          </div>
        </div>

        <!-- Auto-Analysis Mode -->
        <div v-if="addEngineMode === 'auto'" class="auto-mode">
          <p class="help-text">
            {{ $t('pages.engines.add.form.helpText.auto') }}
          </p>

          <div class="form-group">
            <label for="engineNameAuto">{{ $t('pages.engines.add.form.engineName') }}</label>
            <input
              id="engineNameAuto"
              v-model="newEngine.name"
              type="text"
              :placeholder="$t('pages.engines.add.form.placeholders.engineName')"
              required
            />
          </div>

          <div class="form-group">
            <div class="label-with-help">
              <label for="urlExample1">{{ $t('pages.engines.add.form.urlExample1') }}</label>
              <small>{{ $t('pages.engines.add.form.helpText.urlExample1') }}</small>
            </div>
            <input
              id="urlExample1"
              v-model="newEngine.urlExample1"
              type="url"
              :placeholder="$t('pages.engines.add.form.placeholders.urlExample1')"
              required
            />
          </div>

          <div class="form-group">
            <div class="label-with-help">
              <label for="urlExample2">{{ $t('pages.engines.add.form.urlExample2') }}</label>
              <small>{{ $t('pages.engines.add.form.helpText.urlExample2') }}</small>
            </div>
            <input
              id="urlExample2"
              v-model="newEngine.urlExample2"
              type="url"
              :placeholder="$t('pages.engines.add.form.placeholders.urlExample2')"
              required
            />
          </div>
        </div>
        
        <div class="form-actions">
          <button type="submit" :disabled="isAdding" class="add-btn">
            {{ isAdding ? $t('pages.engines.add.adding') : $t('pages.engines.add.button') }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, inject, computed } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from '../composables/useI18n';
import { useConfirmDelete } from '../composables/useConfirmDelete';

// 注入全局通知函数
const showNotification = inject('showNotification') as (message: string, type?: 'success' | 'error', duration?: number) => void;
const { t, locale } = useI18n();

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

// 使用确认删除 composable
const { confirmDelete, getDeleteIcon, getDeleteButtonClass, getDeleteButtonTitle } = useConfirmDelete(
  async (id: string) => {
    try {
      await invoke("delete_engine", { id });
      await loadEngines(); // 重新加载列表
    } catch (error) {
      console.error("Failed to delete engine:", error);
      showNotification(t('pages.engines.messages.deleteFailed', { error: String(error) }), 'error');
    }
  }
);

// Edit engine related
const editingEngineId = ref<string | null>(null);
const editingEngine = ref<SearchEngine>({
  id: '',
  name: '',
  url_template: '',
  is_enabled: false,
  is_deletable: false
});
const isSavingEdit = ref(false);

const addEngineMode = ref('auto'); // 'advanced' or 'auto'

const newEngine = ref({
  name: '',
  urlExample1: '',
  urlExample2: '',
  urlTemplate: ''
});

// ExampleBT 示例常量，避免 i18n 将 {keyword}/{page} 当作占位符吞掉
const exampleDomain = 'examplebt.com'
const L = '\u007B'
const R = '\u007D'
const urlTemplateExample = `https://${exampleDomain}/search/${L}keyword${R}/${L}page${R}/`
const example1Text = computed(() => `${urlTemplateExample} - ${t('pages.engines.add.form.examples.suffix1')}`)
const example2Text = computed(() => `https://${exampleDomain}/search?kw=${L}keyword${R}&p=${L}page-1${R} - ${t('pages.engines.add.form.examples.suffix2')}`)
const urlTemplatePlaceholder = computed(() => {
  const prefix = (locale?.value === 'zh-CN') ? '例如：' : 'e.g., '
  return `${prefix}${urlTemplateExample}`
})

function handleInvalid(e: Event) {
  const input = e.target as HTMLInputElement
  input.setCustomValidity(t('pages.engines.add.validation.requiredField'))
}

function handleInput(e: Event) {
  const input = e.target as HTMLInputElement
  input.setCustomValidity('')
}

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
    showNotification(t('pages.engines.messages.loadFailed', { error: String(error) }), 'error');
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
    showNotification(t('pages.engines.messages.updateStatusFailed', { error: String(error) }), 'error');
    // Reload to restore correct state
    await loadEngines();
  }
}

async function addEngine() {
  // Validation based on mode
  if (addEngineMode.value === 'advanced') {
    if (!newEngine.value.name || !newEngine.value.urlTemplate) {
      showNotification(t('pages.engines.add.validation.fillAllFields'), 'error');
      return;
    }

    // Validate template format
    if (!newEngine.value.urlTemplate.includes('{keyword}')) {
      showNotification(t('pages.engines.add.validation.keywordRequired'), 'error');
      return;
    }

    if (!newEngine.value.urlTemplate.includes('{page}') && !newEngine.value.urlTemplate.includes('{page-1}')) {
      showNotification(t('pages.engines.add.validation.pageRequired'), 'error');
      return;
    }
  } else {
    if (!newEngine.value.name || !newEngine.value.urlExample1 || !newEngine.value.urlExample2) {
      showNotification(t('pages.engines.add.validation.fillAllFields'), 'error');
      return;
    }
  }

  isAdding.value = true;
  try {
    let urlTemplate: string;

    if (addEngineMode.value === 'advanced') {
      // Use the template directly
      urlTemplate = newEngine.value.urlTemplate;
    } else {
      // Generate URL template from examples
      urlTemplate = generateUrlTemplate(newEngine.value.urlExample1, newEngine.value.urlExample2);
    }

    await invoke("add_search_engine", {
      name: newEngine.value.name,
      urlTemplate
    });

    // Reset form
    newEngine.value = {
      name: '',
      urlExample1: '',
      urlExample2: '',
      urlTemplate: ''
    };

    await loadEngines(); // Reload the list
    showNotification(t('pages.engines.messages.addSuccess'));
  } catch (error) {
    console.error("Failed to add engine:", error);
    showNotification(t('pages.engines.messages.addFailed', { error: String(error) }), 'error');
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
          // This is 1-based pagination (page 1, page 2)
          templateParams.push(`${key}={page}`);
        } else if (value1 === '0' && value2 === '1') {
          // This is 0-based pagination (page 0, page 1) - need to subtract 1 from page number
          templateParams.push(`${key}={page-1}`);
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
    fallbackTemplate = fallbackTemplate.replace(/([?&])page=1/, '$1page={page}');

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

    if (num1 === 1 && num2 === 2) {
      // 1-based pagination
      return `${key}={page}`;
    } else if (num1 === 0 && num2 === 1) {
      // 0-based pagination - need to subtract 1 from page number
      return `${key}={page-1}`;
    } else if (Math.abs(num1 - num2) === 1) {
      // Other sequential patterns, assume 1-based as default
      return `${key}={page}`;
    }
  }

  // Default: use first value
  return `${key}=${value1}`;
}

// Edit engine functions
function startEditEngine(engine: SearchEngine) {
  editingEngineId.value = engine.id;
  editingEngine.value = { ...engine };
}

function cancelEditEngine() {
  editingEngineId.value = null;
  editingEngine.value = {
    id: '',
    name: '',
    url_template: '',
    is_enabled: false,
    is_deletable: false
  };
}

async function saveEditEngine() {
  if (!editingEngine.value.name || !editingEngine.value.url_template) {
    showNotification(t('pages.engines.add.validation.fillAllFields'), 'error');
    return;
  }

  // Validate template format
  if (!editingEngine.value.url_template.includes('{keyword}')) {
    showNotification(t('pages.engines.add.validation.keywordRequired'), 'error');
    return;
  }

  if (!editingEngine.value.url_template.includes('{page}') && !editingEngine.value.url_template.includes('{page-1}')) {
    showNotification(t('pages.engines.add.validation.pageRequired'), 'error');
    return;
  }

  isSavingEdit.value = true;
  try {
    await invoke("update_search_engine", {
      id: editingEngine.value.id,
      name: editingEngine.value.name,
      urlTemplate: editingEngine.value.url_template
    });

    await loadEngines(); // Reload the list
    cancelEditEngine(); // Exit edit mode
    showNotification(t('pages.engines.messages.updateSuccess'));
  } catch (error) {
    console.error("Failed to update engine:", error);
    showNotification(t('pages.engines.messages.updateFailed', { error: String(error) }), 'error');
  } finally {
    isSavingEdit.value = false;
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

/* URL Row Styles */
.engine-url-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.engine-url {
  flex: 1;
  font-size: 14px;
  color: #4a5568;
  font-family: monospace;
  background: #f7fafc;
  padding: 8px 12px;
  border-radius: 4px;
  word-break: break-all;
}

.engine-actions-inline {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
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
  gap: 12px;
}

.form-group label {
  font-weight: 600;
  color: #1a202c;
  font-size: 16px;
  margin-top: 8px;
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

/* Label with Help Text Styles */
.label-with-help {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  gap: 16px;
}

.label-with-help small {
  color: #718096;
  font-size: 12px;
  font-style: italic;
  flex-shrink: 0;
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

/* Mode Selection Styles */
.mode-selection {
  display: flex;
  gap: 20px;
  margin-bottom: 20px;
  padding: 16px;
  background: #f7fafc;
  border-radius: 8px;
  border: 1px solid #e2e8f0;
}

.mode-option {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  color: #1a202c;
  user-select: none;
}

.mode-option input[type="radio"] {
  display: none;
}

.radio-mark {
  width: 16px;
  height: 16px;
  border: 2px solid #d1d5db;
  border-radius: 50%;
  position: relative;
  transition: all 0.2s ease;
  background: white;
}

.mode-option input[type="radio"]:checked + .radio-mark {
  border-color: #3b82f6;
  background: #3b82f6;
}

.mode-option input[type="radio"]:checked + .radio-mark::after {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 6px;
  height: 6px;
  background: white;
  border-radius: 50%;
  transform: translate(-50%, -50%);
}

.mode-option:hover .radio-mark {
  border-color: #3b82f6;
}

/* Template Examples Styles */
.template-examples {
  margin-top: 16px;
  padding: 16px;
  background: #f8fafc;
  border-radius: 8px;
  border: 1px solid #e2e8f0;
}

.template-examples h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
  color: #1a202c;
}

.template-examples ul {
  margin: 0;
  padding-left: 20px;
}

.template-examples li {
  margin-bottom: 8px;
  font-size: 13px;
  color: #4a5568;
}

.template-examples code {
  background: #e2e8f0;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 12px;
  color: #2d3748;
}

.help-text code {
  background: #e2e8f0;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 12px;
  color: #2d3748;
}

/* Mode Content Styles */
.advanced-mode, .auto-mode {
  margin-top: 0px;
}

.advanced-mode .help-text, .auto-mode .help-text {
  margin-top: 4px;
  margin-bottom: 16px;
}

/* Experimental Note Styles */
.experimental-note {
  font-size: 14px;
  font-weight: 400;
  color: #f56565;
  font-style: italic;
}

/* Edit Engine Styles */
.engine-edit-form {
  display: grid;
  gap: 16px;
}

/* Edit URL Row Styles */
.edit-url-row {
  display: flex;
  align-items: flex-end;
  gap: 12px;
}

.edit-form-group-inline {
  flex: 1;
  display: grid;
  gap: 12px;
}

.edit-form-group-inline label {
  font-weight: 600;
  color: #1a202c;
  font-size: 16px;
  margin-top: 8px;
}

.edit-actions-inline {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
  margin-bottom: 2px; /* Align with input field */
}

.edit-form-group {
  display: grid;
  gap: 12px;
}

.edit-form-group label {
  font-weight: 600;
  color: #1a202c;
  font-size: 16px;
  margin-top: 8px;
}

.edit-input {
  padding: 8px 12px;
  border: 2px solid #e2e8f0;
  border-radius: 6px;
  font-size: 14px;
  transition: border-color 0.2s;
}

.edit-input:focus {
  outline: none;
  border-color: #3b82f6;
}



.edit-btn {
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 6px;
  background: #e2e8f0;
  color: #4a5568;
  cursor: pointer;
  font-size: 16px;
  transition: all 0.2s;
}

.edit-btn:hover {
  background: #cbd5e0;
}

.save-edit-btn {
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 6px;
  background: #c6f6d5;
  color: #276749;
  cursor: pointer;
  font-size: 16px;
  transition: all 0.2s;
}

.save-edit-btn:hover:not(:disabled) {
  background: #9ae6b4;
}

.save-edit-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.cancel-edit-btn {
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

.cancel-edit-btn:hover {
  background: #feb2b2;
}
</style>