<script setup>
import { computed } from "vue";
import SettingsPanel from "./SettingsPanel.vue";

const props = defineProps({
  plugin: {
    type: Object,
    required: true,
  },
  preview: {
    type: Object,
    default: null,
  },
  modelValue: {
    type: Object,
    default: () => ({}),
  },
  running: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(["update:modelValue", "run", "back"]);

const settingsProxy = computed({
  get: () => {
    const defaults = props.plugin?.defaults || {};
    return {
      ...defaults,
      ...props.modelValue,
    };
  },
  set: (value) => emit("update:modelValue", value),
});
</script>

<template>
  <section class="plugin-panel">
    <div class="panel-header">
      <div class="panel-title">
        <button class="btn ghost with-icon" @click="emit('back')">
          <span class="icon" aria-hidden="true">
            <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
              <path d="M15 6l-6 6 6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                stroke-linejoin="round" />
            </svg>
          </span>
          Назад
        </button>
        <div>
          <p>{{ plugin.description }}</p>
        </div>
      </div>
      <button class="btn primary with-icon" :disabled="running" @click="emit('run')">
        <span class="icon" :class="{ spin: running }" aria-hidden="true">
          <svg v-if="running" viewBox="0 0 24 24" fill="none" aria-hidden="true">
            <circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="2.4" stroke-linecap="round"
              stroke-dasharray="42 18" />
          </svg>
          <svg v-else viewBox="0 0 24 24" fill="none" aria-hidden="true">
            <path d="M5 12.5l4.2 4.2L19 7.9" stroke="currentColor" stroke-width="2.4" stroke-linecap="round"
              stroke-linejoin="round" />
          </svg>
        </span>
        {{ running ? "Выполняется..." : "Применить" }}
      </button>
    </div>

    <div class="panel-body">
      <div v-if="preview" class="panel-preview">
        <div class="preview-label">{{ preview.title }}</div>
        <div class="preview-value">{{ preview.value }}</div>
        <div v-if="preview.note" class="preview-note">{{ preview.note }}</div>
      </div>
      <SettingsPanel v-model="settingsProxy" :fields="plugin.settings" />
    </div>
  </section>
</template>
