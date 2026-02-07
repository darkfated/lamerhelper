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
  get: () => props.modelValue,
  set: (value) => emit("update:modelValue", value),
});
</script>

<template>
  <section class="plugin-panel">
    <div class="panel-header">
      <div class="panel-title">
        <button class="btn ghost" @click="emit('back')">Назад</button>
        <div>
          <p>{{ plugin.description }}</p>
        </div>
      </div>
      <button class="btn primary" :disabled="running" @click="emit('run')">
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
