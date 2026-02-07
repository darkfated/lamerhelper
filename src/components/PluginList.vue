<script setup>
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

const emit = defineEmits(["select"]);

function selectPlugin(id) {
  emit("select", id);
}
</script>

<template>
  <section class="plugin-list-view">
    <header class="list-hero">
      <div class="list-hero-text">
        <div class="list-kicker">LamerHelper</div>
        <h1>Панель плагинов</h1>
        <p>Выбери действие, настрой параметры и запусти его в один клик.</p>
      </div>
      <div class="list-hero-badge">
        <span class="badge-number">{{ plugins.length }}</span>
        <span class="badge-label">плагина</span>
      </div>
    </header>

    <div class="list-grid" v-if="plugins.length">
      <button v-for="plugin in plugins" :key="plugin.id" class="list-card" @click="selectPlugin(plugin.id)">
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
      <div v-else>Плагины не найдены.</div>
    </div>
  </section>
</template>
