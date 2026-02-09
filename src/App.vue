<script setup>
import { invoke } from '@tauri-apps/api/core'
import { openUrl } from '@tauri-apps/plugin-opener'
import { computed, onMounted, ref } from 'vue'
import ConsolePanel from './components/ConsolePanel.vue'
import PluginList from './components/PluginList.vue'
import PluginPanel from './components/PluginPanel.vue'

const plugins = ref([])
const loading = ref(false)
const viewMode = ref('list')
const selectedId = ref('')
const settings = ref({})
const logs = ref([])
const status = ref({ ok: true, message: '' })
const running = ref(false)
const error = ref('')
const logSession = ref(0)
const preview = ref(null)
const showSettings = ref(false)

const selectedPlugin = computed(
  () => plugins.value.find((plugin) => plugin.id === selectedId.value) || null,
)

function cloneValue(value) {
  return value ? JSON.parse(JSON.stringify(value)) : {}
}

function resetForPlugin(plugin) {
  logSession.value += 1
  settings.value = cloneValue(plugin.defaults || {})
  logs.value = []
  status.value = { ok: true, message: '' }
}

function showError(message) {
  error.value = message
  setTimeout(() => {
    error.value = ''
  }, 4000)
}

function openSettings() {
  showSettings.value = true
}

function closeSettings() {
  showSettings.value = false
}

function nextLogSession() {
  logSession.value += 1
  return logSession.value
}

function logDelay(count) {
  if (count > 80) return 30
  if (count > 40) return 60
  if (count > 20) return 90
  return 140
}

function stampLog(entry) {
  return {
    ...entry,
    time: new Date().toLocaleTimeString('ru-RU', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false,
    }),
  }
}

async function appendLogs(entries, sessionId, delay) {
  for (const entry of entries || []) {
    if (sessionId !== logSession.value) break
    logs.value = [...logs.value, stampLog(entry)]
    await new Promise((resolve) => setTimeout(resolve, delay))
  }
}

async function loadPlugins() {
  loading.value = true
  try {
    plugins.value = await invoke('list_plugins')

    if (viewMode.value === 'detail') {
      const exists = plugins.value.find(
        (plugin) => plugin.id === selectedId.value,
      )
      if (!exists) {
        backToList()
      }
    }
  } catch (err) {
    showError(String(err))
  } finally {
    loading.value = false
  }
}

async function loadPreview(id) {
  try {
    preview.value = await invoke('preview_plugin', { id })
  } catch (err) {
    preview.value = null
  }
}

function selectPlugin(id) {
  const plugin = plugins.value.find((item) => item.id === id)
  if (!plugin) return
  selectedId.value = id
  viewMode.value = 'detail'
  resetForPlugin(plugin)
  loadPreview(id)
}

function backToList() {
  logSession.value += 1
  viewMode.value = 'list'
  selectedId.value = ''
  logs.value = []
  status.value = { ok: true, message: '' }
  preview.value = null
}

async function runSelected() {
  if (!selectedPlugin.value) return
  running.value = true
  const sessionId = nextLogSession()
  await appendLogs(
    [{ level: 'info', message: '--- Запуск ---' }],
    sessionId,
    80,
  )
  status.value = { ok: true, message: 'Выполняется...' }

  try {
    const result = await invoke('run_plugin', {
      id: selectedPlugin.value.id,
      settings: settings.value,
    })
    const delay = logDelay((result.logs || []).length)
    await appendLogs(result.logs || [], sessionId, delay)
    status.value = { ok: result.ok, message: result.message }
  } catch (err) {
    showError(String(err))
    await appendLogs(
      [{ level: 'error', message: 'Ошибка запуска.' }],
      sessionId,
      100,
    )
    status.value = { ok: false, message: 'Ошибка запуска.' }
  } finally {
    running.value = false
  }
}

onMounted(() => {
  loadPlugins()
})
</script>

<template>
  <div class="app">
    <div class="background"></div>

    <main class="main-area" :class="{ 'no-console': viewMode === 'list' }">
      <div class="workspace" :class="{ 'detail-mode': viewMode === 'detail' }">
        <PluginList v-if="viewMode === 'list'" :plugins="plugins" :loading="loading" @select="selectPlugin"
          @settings="openSettings" />

        <PluginPanel v-else-if="selectedPlugin" v-model="settings" :plugin="selectedPlugin" :preview="preview"
          :running="running" @run="runSelected" @back="backToList" />

        <div v-else class="empty-state">
          Плагин не найден.
        </div>
      </div>
    </main>

    <div v-if="showSettings" class="settings-overlay" @click.self="closeSettings">
      <div class="settings-modal" role="dialog" aria-modal="true">
        <div class="settings-modal-header">
          <div>
            <div class="settings-modal-kicker">LamerHelper</div>
            <h3>О приложении</h3>
          </div>
          <button class="btn ghost" type="button" @click="closeSettings">Закрыть</button>
        </div>
        <div class="settings-modal-body">
          <div class="settings-item">
            <span class="settings-item-label">GitHub</span>
            <span class="settings-link" href="#" @click="openUrl('https://github.com/darkfated/lamerhelper')">
              darkfated/lamerhelper</span>
          </div>
          <div class="settings-item">
            <span class="settings-item-label">Авторское право</span>
            <span class="settings-item-value">© 2026 LamerHelper</span>
          </div>
          <div class="settings-item">
            <span class="settings-item-label">Версия программы</span>
            <span class="settings-item-value">0.2.0</span>
          </div>
        </div>
      </div>
    </div>

    <ConsolePanel v-if="viewMode === 'detail'" :logs="logs" :status="status" />

    <div v-if="error" class="toast">{{ error }}</div>
  </div>
</template>

<style>
@import url("https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=JetBrains+Mono:wght@400;600&display=swap");

:root {
  color-scheme: dark;
  font-family: "Space Grotesk", sans-serif;
  font-size: clamp(14px, 1.1vw, 16px);
  line-height: 1.4;
  --bg: #101624;
  --panel: rgba(24, 32, 46, 0.92);
  --panel-strong: rgba(30, 38, 54, 0.96);
  --panel-soft: rgba(20, 26, 38, 0.82);
  --stroke: rgba(255, 255, 255, 0.12);
  --stroke-strong: rgba(255, 255, 255, 0.2);
  --text: #f6f8fd;
  --muted: #a3afbf;
  --accent: #45f0b1;
  --accent-strong: #26d39a;
  --accent-alt: #38c6ff;
  --danger: #ff9aa2;
  --success: #86f5c8;
  --warn: #ffd188;
  --mono: "JetBrains Mono", monospace;
  --shadow: 0 18px 40px rgba(5, 8, 14, 0.45);
  --shadow-soft: 0 12px 28px rgba(5, 8, 14, 0.35);
  --console-height-base: clamp(200px, 32vh, 360px);
  --console-height: var(--console-height-base);
  --console-offset: clamp(10px, 2.4vh, 18px);
  --console-gap: clamp(10px, 2vh, 16px);
  --content-width: min(94vw, 980px);
}

* {
  box-sizing: border-box;
}

* {
  scrollbar-width: thin;
  scrollbar-color: rgba(69, 240, 177, 0.35) transparent;
}

*::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

*::-webkit-scrollbar-track {
  background: transparent;
}

*::-webkit-scrollbar-thumb {
  background: linear-gradient(180deg, rgba(69, 240, 177, 0.45), rgba(56, 198, 255, 0.35));
  border-radius: 999px;
}

body {
  margin: 0;
  background: var(--bg);
  color: var(--text);
  overflow: hidden;
}

#app {
  height: 100vh;
}

.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  padding: clamp(12px, 2vw, 20px) clamp(12px, 2.2vw, 24px) 0;
  gap: 12px;
  position: relative;
}

.background {
  position: fixed;
  inset: 0;
  background: radial-gradient(circle at 12% 18%, rgba(69, 240, 177, 0.24), transparent 48%),
    radial-gradient(circle at 86% 14%, rgba(56, 198, 255, 0.2), transparent 54%),
    radial-gradient(circle at 50% 88%, rgba(255, 255, 255, 0.1), transparent 58%),
    linear-gradient(140deg, #0f1626, #141c2e 45%, #0f1626 100%);
  z-index: -1;
}

.main-area {
  flex: 1;
  min-height: 0;
  padding-bottom: calc(var(--console-height-base) + var(--console-offset) + var(--console-gap));
}

.main-area.no-console {
  padding-bottom: clamp(12px, 2vw, 20px);
}

.settings-overlay {
  position: fixed;
  inset: 0;
  background: rgba(8, 12, 20, 0.72);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 40;
  backdrop-filter: blur(12px);
}

.settings-modal {
  width: min(92vw, 520px);
  background: linear-gradient(160deg, rgba(30, 40, 58, 0.98), rgba(16, 22, 34, 0.96));
  border: 1px solid rgba(255, 255, 255, 0.16);
  border-radius: 20px;
  box-shadow: var(--shadow);
  padding: clamp(16px, 2.4vw, 20px);
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.settings-modal-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
}

.settings-modal-header h3 {
  margin: 6px 0 4px;
  font-size: 20px;
}


.settings-modal-kicker {
  text-transform: uppercase;
  letter-spacing: 0.22em;
  font-size: 10px;
  color: rgba(69, 240, 177, 0.92);
  font-weight: 600;
}

.settings-modal-body {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.settings-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 12px;
  border-radius: 12px;
  background: rgba(14, 20, 32, 0.7);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.settings-item-label {
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.16em;
  color: rgba(214, 224, 236, 0.72);
}

.settings-item-value {
  font-size: 13px;
  font-weight: 600;
  color: var(--text);
}

.settings-link {
  border: 1px solid rgba(69, 240, 177, 0.2);
  background: rgba(69, 240, 177, 0.08);
  color: var(--accent);
  border-radius: 10px;
  padding: 6px 10px;
  font-weight: 600;
  font-size: 12px;
  cursor: default;
  text-decoration: none;
  cursor: pointer;
}

.workspace {
  height: 100%;
  min-height: 0;
  display: flex;
  justify-content: center;
}


.workspace.detail-mode {
  justify-content: center;
}

.workspace.detail-mode .plugin-panel {
  align-self: center;
}

.plugin-list-view {
  height: 100%;
  width: 100%;
  display: grid;
  grid-template-rows: auto auto 1fr;
  gap: 18px;
  background: linear-gradient(160deg, rgba(28, 36, 52, 0.96), rgba(18, 24, 36, 0.94));
  border: 1px solid var(--stroke);
  border-radius: 22px;
  padding: clamp(16px, 2vw, 22px);
  box-shadow: var(--shadow-soft);
  backdrop-filter: blur(16px);
  min-height: 0;
  max-width: var(--content-width);
  margin: 0 auto;
}

.list-hero {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 20px;
  padding: 6px 4px 12px;
}

.list-hero-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}

.list-hero-text h1 {
  margin: 6px 0 6px;
  font-size: 26px;
  letter-spacing: 0.02em;
  display: flex;
  align-items: center;
  gap: 10px;
}

.hero-icon {
  width: 22px;
  height: 22px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  background: rgba(69, 240, 177, 0.18);
  color: rgba(69, 240, 177, 0.95);
  box-shadow: inset 0 0 0 1px rgba(69, 240, 177, 0.3);
}

.hero-icon svg {
  width: 14px;
  height: 14px;
}

.list-hero-text p {
  margin: 0;
  color: var(--muted);
  font-size: 14px;
  max-width: 420px;
}

.list-kicker {
  text-transform: uppercase;
  letter-spacing: 0.24em;
  font-size: 11px;
  color: rgba(69, 240, 177, 0.95);
  font-weight: 600;
}

.list-hero-badge {
  background: rgba(16, 22, 34, 0.7);
  border: 1px solid rgba(255, 255, 255, 0.14);
  border-radius: 16px;
  padding: 0 14px;
  min-width: 110px;
  height: 40px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  text-align: center;
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.02);
}

.badge-number {
  font-size: 18px;
  font-weight: 700;
  color: var(--accent);
  line-height: 1;
}

.badge-label {
  text-transform: uppercase;
  letter-spacing: 0.18em;
  font-size: 10px;
  color: var(--muted);
  line-height: 1;
}

.list-tabs {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  padding: 0 4px;
}

.list-tab {
  border: 1px solid rgba(255, 255, 255, 0.14);
  background: rgba(14, 20, 32, 0.7);
  color: var(--muted);
  border-radius: 999px;
  padding: 6px 14px;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.14em;
  cursor: pointer;
  transition: border-color 0.2s ease, background 0.2s ease, color 0.2s ease;
}

.list-tab.active {
  background: rgba(69, 240, 177, 0.16);
  color: #e9fff6;
  border-color: rgba(69, 240, 177, 0.45);
  box-shadow: inset 0 0 0 1px rgba(69, 240, 177, 0.12);
}

.list-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(230px, 1fr));
  grid-auto-rows: minmax(170px, auto);
  gap: 14px;
  overflow: auto;
  padding-right: 6px;
  flex: 1;
  min-height: 0;
  align-content: start;
  align-items: stretch;
}


.list-card {
  text-align: left;
  background: linear-gradient(160deg, rgba(32, 40, 58, 0.96), rgba(16, 22, 34, 0.96));
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 18px;
  padding: 16px;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: 10px;
  justify-content: space-between;
  transition: transform 0.2s ease, box-shadow 0.2s ease, border-color 0.2s ease;
  align-self: start;
  height: 100%;
  min-height: 170px;
  position: relative;
  overflow: hidden;
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.03), 0 8px 18px rgba(6, 10, 18, 0.4);
}

.list-card:hover {
  border-color: rgba(69, 240, 177, 0.4);
  box-shadow: 0 12px 26px rgba(7, 12, 22, 0.5);
}

.list-card::after {
  content: "";
  position: absolute;
  inset: 0;
  background: radial-gradient(circle at top right, rgba(69, 240, 177, 0.2), transparent 55%);
  opacity: 0;
  transition: opacity 0.2s ease;
}

.list-card:hover::after {
  opacity: 1;
}

.card-head {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 12px;
  z-index: 1;
}

.card-title {
  font-weight: 600;
  font-size: 16px;
  letter-spacing: 0.01em;
  z-index: 1;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.card-body {
  z-index: 1;
  flex: 1 1 auto;
  min-height: 0;
}

.card-desc {
  margin: 0;
  font-size: 13px;
  line-height: 1.35;
  color: rgba(246, 248, 253, 0.74);
  display: -webkit-box;
  -webkit-line-clamp: 3;
  line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}


.card-footer {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: auto;
  z-index: 1;
}

.chip {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 999px;
  border: 1px solid rgba(255, 255, 255, 0.12);
  background: rgba(14, 20, 32, 0.7);
  font-size: 10px;
  color: var(--muted);
  font-family: var(--mono);
}

.list-settings-btn {
  height: 40px;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.14em;
  padding: 8px 14px;
}

.with-icon {
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.icon {
  width: 16px;
  height: 16px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: inherit;
}

.icon svg {
  width: 100%;
  height: 100%;
}

.plugin-panel {
  height: 100%;
  width: 100%;
  background: linear-gradient(180deg, rgba(28, 36, 52, 0.94), rgba(18, 24, 36, 0.92));
  border: 1px solid var(--stroke);
  border-radius: 22px;
  padding: clamp(16px, 2vw, 20px);
  display: flex;
  flex-direction: column;
  gap: 14px;
  overflow: hidden;
  box-shadow: var(--shadow-soft);
  max-width: var(--content-width);
}

.panel-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  flex-wrap: nowrap;
  gap: 16px;
  padding-bottom: 8px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.panel-header .btn.primary {
  margin-left: auto;
  flex: 0 0 auto;
  align-self: flex-start;
}

.panel-title {
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex: 1 1 auto;
  min-width: 0;
}

.panel-title .btn {
  align-self: flex-start;
}

.panel-title h2 {
  margin: 0;
  font-size: 22px;
  letter-spacing: 0.02em;
  overflow-wrap: anywhere;
}

.panel-title p {
  margin: 0;
  color: var(--muted);
  font-size: 13px;
  overflow-wrap: anywhere;
}

.panel-body {
  display: flex;
  flex-direction: column;
  gap: clamp(12px, 1.6vw, 16px);
  overflow: auto;
  padding-right: clamp(4px, 0.8vw, 8px);
  min-height: 0;
}

.panel-preview {
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: 10px 14px;
  padding: 10px 12px;
  border-radius: 12px;
  border: 1px solid rgba(69, 240, 177, 0.2);
  background: rgba(18, 26, 36, 0.65);
}

.preview-label {
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.16em;
  color: rgba(69, 240, 177, 0.9);
}

.preview-value {
  font-size: 14px;
  font-weight: 600;
  color: rgba(246, 248, 253, 0.95);
}

.preview-note {
  font-size: 12px;
  color: rgba(214, 224, 236, 0.8);
  flex-basis: 100%;
}

.settings-panel {
  display: flex;
  flex-direction: column;
  gap: clamp(10px, 1.6vw, 14px);
  background: linear-gradient(180deg, rgba(22, 30, 44, 0.94), rgba(14, 20, 32, 0.9));
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 22px;
  padding: clamp(12px, 2vw, 18px);
}

.settings-header {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.settings-title {
  font-size: 14px;
  text-transform: uppercase;
  letter-spacing: 0.18em;
  color: rgba(69, 240, 177, 0.95);
}

.settings-subtitle {
  font-size: 13px;
  color: rgba(214, 224, 236, 0.8);
}

.settings-list {
  display: flex;
  flex-direction: column;
  gap: clamp(10px, 1.4vw, 12px);
}

.setting-item {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(160px, 240px);
  gap: clamp(10px, 2vw, 16px);
  padding: clamp(12px, 2vw, 16px) clamp(12px, 2.4vw, 18px);
  align-items: center;
  border-radius: 16px;
  border: 1px solid rgba(255, 255, 255, 0.12);
  background: rgba(18, 24, 36, 0.75);
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.02);
}

.setting-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
  min-width: 0;
}

.setting-label {
  font-weight: 600;
  font-size: 15px;
  color: #f2f6ff;
}

.required {
  color: var(--danger);
  margin-left: 4px;
}

.danger-tag {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  margin-left: 8px;
  padding: 2px 8px;
  border-radius: 999px;
  border: 1px solid rgba(255, 154, 162, 0.45);
  background: rgba(255, 154, 162, 0.12);
  color: rgba(255, 185, 190, 0.95);
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
}

.setting-desc {
  font-size: 12.5px;
  color: rgba(208, 218, 232, 0.8);
  line-height: 1.45;
}

.setting-action {
  display: flex;
  align-items: center;
  gap: 12px;
  justify-self: stretch;
  justify-content: flex-end;
  min-width: 0;
}

.setting-action input[type="text"]:focus,
.setting-action input[type="number"]:focus,
.setting-action select:focus,
.setting-action textarea:focus {
  outline: none;
  border-color: rgba(255, 255, 255, 0.16);
  box-shadow: none;
}

.switch {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.field-inline {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  min-width: 0;
}

.setting-action input[type="text"],
.setting-action input[type="number"],
.setting-action select,
.setting-action textarea {
  background: rgba(20, 28, 42, 0.9);
  border: 1px solid rgba(255, 255, 255, 0.16);
  color: var(--text);
  border-radius: 14px;
  padding: 10px 12px;
  font-family: inherit;
  min-width: 0;
  width: 100%;
  font-size: 13px;
  height: 40px;
}

.select-wrap {
  position: relative;
  min-width: 0;
  width: 100%;
}

.select-input {
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;
  width: 100%;
  padding-right: 38px;
}

.select-caret {
  position: absolute;
  right: 12px;
  top: 50%;
  width: 10px;
  height: 10px;
  border-right: 2px solid rgba(246, 248, 253, 0.8);
  border-bottom: 2px solid rgba(246, 248, 253, 0.8);
  transform: translateY(-55%) rotate(45deg);
  pointer-events: none;
}

.number-wrap {
  position: relative;
  min-width: 0;
  width: 100%;
}

.number-input {
  appearance: textfield;
  -moz-appearance: textfield;
  padding-right: 38px;
  width: 100%;
}

.number-input::-webkit-outer-spin-button,
.number-input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.number-stepper {
  position: absolute;
  right: 8px;
  top: 6px;
  bottom: 6px;
  width: 18px;
  display: grid;
  grid-template-rows: 1fr 1fr;
  gap: 4px;
}

.step-btn {
  border: none;
  background: transparent;
  border-radius: 6px;
  cursor: pointer;
  padding: 0;
  position: relative;
}

.step-btn::before {
  content: "";
  position: absolute;
  inset: 0;
  margin: auto;
  width: 6px;
  height: 6px;
  border-right: 2px solid rgba(246, 248, 253, 0.8);
  border-bottom: 2px solid rgba(246, 248, 253, 0.8);
}

.step-btn.up::before {
  transform: rotate(-135deg);
}

.step-btn.down::before {
  transform: rotate(45deg);
}

.setting-action textarea {
  resize: vertical;
  min-height: 80px;
  height: auto;
}

.setting-control input[type="color"] {
  width: 40px;
  height: 28px;
  border: none;
  background: transparent;
}

.setting-unit {
  font-size: 12px;
  color: var(--muted);
  white-space: nowrap;
}

.color-picker {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  min-width: 0;
  flex-wrap: wrap;
}

.color-swatch {
  width: 46px;
  height: 36px;
  border-radius: 14px;
  border: 1px solid rgba(255, 255, 255, 0.16);
  position: relative;
  overflow: hidden;
  box-shadow: inset 0 0 0 1px rgba(0, 0, 0, 0.35);
}

.color-swatch input[type="color"] {
  position: absolute;
  inset: 0;
  opacity: 0;
  cursor: pointer;
}

.color-hex {
  background: rgba(20, 28, 42, 0.9);
  border: 1px solid rgba(255, 255, 255, 0.16);
  color: var(--text);
  border-radius: 14px;
  padding: 10px 12px;
  font-family: var(--mono);
  width: min(130px, 100%);
  flex: 1 1 120px;
  height: 40px;
  font-size: 13px;
}

.switch {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  position: relative;
  width: 46px;
  height: 26px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  inset: 0;
  background: rgba(18, 24, 36, 0.9);
  border-radius: 999px;
  border: 1px solid rgba(255, 255, 255, 0.16);
  transition: background 0.2s ease;
}

.slider::before {
  content: "";
  position: absolute;
  height: 18px;
  width: 18px;
  left: 4px;
  top: 50%;
  border-radius: 50%;
  background: #f6f8fd;
  transition: transform 0.2s ease, background 0.2s ease;
  transform: translateY(-50%);
}

.switch input:checked+.slider {
  background: rgba(69, 240, 177, 0.25);
  border-color: rgba(69, 240, 177, 0.5);
}

.switch input:checked+.slider::before {
  transform: translateX(18px) translateY(-50%);
  background: var(--accent);
}

@media (max-width: 1024px) {
  .list-grid {
    grid-template-columns: repeat(auto-fit, minmax(210px, 1fr));
  }
}

@media (max-width: 780px) {
  .setting-item {
    grid-template-columns: 1fr;
    justify-items: start;
  }

  .setting-action {
    justify-self: start;
    justify-content: flex-start;
  }

  .list-hero {
    flex-direction: column;
    align-items: stretch;
  }

  .list-hero-badge {
    align-self: flex-start;
  }

  .list-hero-actions {
    width: 100%;
    justify-content: space-between;
  }
}

@media (max-width: 560px) {
  .list-grid {
    grid-template-columns: 1fr;
  }
}

.btn {
  background: rgba(22, 30, 44, 0.9);
  color: var(--text);
  border: 1px solid rgba(255, 255, 255, 0.18);
  border-radius: 10px;
  padding: 9px 14px;
  cursor: pointer;
  font-weight: 600;
  letter-spacing: 0.01em;
  transition: border-color 0.2s ease, box-shadow 0.2s ease, background 0.2s ease;
}

.btn.primary {
  background: linear-gradient(135deg, rgba(69, 240, 177, 0.28), rgba(56, 198, 255, 0.22));
  border: 1px solid rgba(255, 255, 255, 0.22);
  color: var(--text);
  box-shadow: 0 8px 16px rgba(6, 10, 18, 0.35);
}

.btn.primary:disabled {
  background: linear-gradient(135deg, rgba(69, 240, 177, 0.18), rgba(56, 198, 255, 0.12));
  border-color: rgba(255, 255, 255, 0.12);
  color: rgba(246, 248, 253, 0.7);
  box-shadow: none;
  cursor: not-allowed;
}

.btn.ghost {
  background: rgba(18, 24, 36, 0.75);
}

.btn:hover {
  border-color: rgba(69, 240, 177, 0.45);
  box-shadow: 0 10px 18px rgba(6, 10, 16, 0.35);
}

.console-panel {
  position: fixed;
  left: var(--console-offset);
  right: var(--console-offset);
  bottom: var(--console-offset);
  height: var(--console-height);
  background: rgba(12, 16, 24, 0.96);
  border: 1px solid var(--stroke);
  border-radius: 22px;
  padding: 12px 20px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  z-index: 10;
  box-shadow: var(--shadow);
  color: #e7eef6;
  transition: height 0.2s ease, box-shadow 0.2s ease;
}

.console-panel.resizing {
  transition: none;
}

.console-resize-handle {
  position: absolute;
  left: 50%;
  top: 50%;
  width: 56px;
  height: 6px;
  transform: translate(-50%, -50%);
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.08);
  border: 1px solid rgba(255, 255, 255, 0.12);
  cursor: ns-resize;
  z-index: 1;
}

.console-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.14em;
  color: #97a6b4;
  position: relative;
}

.console-title {
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.console-status {
  text-transform: none;
  letter-spacing: normal;
  color: var(--success);
  font-size: 12px;
}

.console-status.error {
  color: var(--danger);
}

.console-body {
  flex: 1;
  background: rgba(8, 10, 16, 0.7);
  border-radius: 12px;
  border: 1px solid var(--stroke);
  padding: 10px 14px;
  overflow: auto;
  font-family: var(--mono);
  font-size: 12px;
  scroll-behavior: smooth;
}

.console-body {
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.console-body::-webkit-scrollbar {
  width: 0;
  height: 0;
}

.console-logs {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.console-line {
  display: grid;
  grid-template-columns: 70px 1fr;
  gap: 8px;
  animation: logIn 0.25s ease;
}

.console-line.info {
  color: #cfe6ff;
}

.console-line.warn {
  color: var(--warn);
}

.console-line.error {
  color: var(--danger);
}

.console-line.success {
  color: var(--success);
}

.console-time {
  color: #97a6b4;
}

.console-content {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.console-detail {
  color: #97a6b4;
}

.console-panel .muted {
  color: #97a6b4;
}

@keyframes logIn {
  from {
    opacity: 0;
    transform: translateY(4px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.toast {
  position: fixed;
  right: 24px;
  top: 24px;
  background: rgba(43, 18, 20, 0.96);
  border: 1px solid rgba(255, 128, 128, 0.4);
  color: #ffb3b3;
  padding: 10px 16px;
  border-radius: 12px;
  z-index: 20;
}

.empty-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
}

.muted {
  color: var(--muted);
}
</style>
