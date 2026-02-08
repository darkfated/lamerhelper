<script setup>
import {computed, ref, watch} from "vue";

const props = defineProps({
  plugins: {
    type: Array,
    default: () => [],
  },
  loading: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(["select", "settings"]);

const tabs = computed(() => {
  const seen = new Set();
  const ordered = [];

  for (const plugin of props.plugins) {
    const category = plugin.category || "Другое";
    if (!seen.has(category)) {
      seen.add(category);
      ordered.push(category);
    }
  }

  return ["Все", ...ordered];
});
const activeTab = ref("Все");

watch(
    tabs,
    (nextTabs) => {
      if (!nextTabs.includes(activeTab.value)) {
        activeTab.value = "Все";
      }
    },
    {immediate: true},
);

const filteredPlugins = computed(() => {
  const current = activeTab.value;
  if (current === "Все") {
    return props.plugins;
  }
  return props.plugins.filter((plugin) => {
    const category = plugin.category || "Другое";
    return category === current;
  });
});

function selectPlugin(id) {
  emit("select", id);
}

function openSettings() {
  emit("settings");
}
</script>

<template>
  <section class="plugin-list-view">
    <header class="list-hero">
      <div class="list-hero-text">
        <div class="list-kicker">LamerHelper</div>
        <h1>
          <span class="hero-icon" aria-hidden="true">
            <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
              <path d="M4 6h16M4 12h16M4 18h10" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                    stroke-linejoin="round"/>
            </svg>
          </span>
          Панель плагинов
        </h1>
        <p>Выбери действие, настрой параметры и запусти его в один клик.</p>
      </div>
      <div class="list-hero-actions">
        <div class="list-hero-badge">
          <span class="badge-number">{{ filteredPlugins.length }}</span>
          <span class="badge-label">плагина</span>
        </div>
        <button class="btn ghost list-settings-btn with-icon" type="button" @click="openSettings">
          <span class="icon" aria-hidden="true">
            <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
              <line x1="4" y1="6" x2="14" y2="6" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              <line x1="18" y1="6" x2="20" y2="6" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              <circle cx="16" cy="6" r="2" stroke="currentColor" stroke-width="2"/>
              <line x1="4" y1="12" x2="10" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              <line x1="14" y1="12" x2="20" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              <circle cx="12" cy="12" r="2" stroke="currentColor" stroke-width="2"/>
              <line x1="4" y1="18" x2="6" y2="18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              <line x1="10" y1="18" x2="20" y2="18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              <circle cx="8" cy="18" r="2" stroke="currentColor" stroke-width="2"/>
            </svg>
          </span>
          Настройки
        </button>
      </div>
    </header>

    <div class="list-tabs">
      <button v-for="tab in tabs" :key="tab" class="list-tab" :class="{ active: activeTab === tab }" type="button"
              @click="activeTab = tab">
        {{ tab }}
      </button>
    </div>

    <div class="list-grid" v-if="filteredPlugins.length">
      <button v-for="plugin in filteredPlugins" :key="plugin.id" class="list-card" @click="selectPlugin(plugin.id)">
        <div class="card-head">
          <div class="card-title">{{ plugin.name }}</div>
        </div>
        <div class="card-body">
          <p class="card-desc">{{ plugin.description || "Описание не задано." }}</p>
        </div>
        <div class="card-footer">
          <span class="chip">Настроек: {{ plugin.settings.length }}</span>
          <span class="chip">Готов к запуску</span>
        </div>
      </button>
    </div>

    <div v-else class="empty-state">
      <div v-if="loading">Загрузка плагинов...</div>
      <div v-else>В этой категории нет плагинов.</div>
    </div>
  </section>
</template>
