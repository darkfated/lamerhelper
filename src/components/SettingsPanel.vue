<script setup>
import { computed } from "vue";
import SettingBoolean from "./settings/SettingBoolean.vue";
import SettingColor from "./settings/SettingColor.vue";
import SettingNumber from "./settings/SettingNumber.vue";
import SettingSelect from "./settings/SettingSelect.vue";
import SettingText from "./settings/SettingText.vue";

const props = defineProps({
  fields: {
    type: Array,
    default: () => [],
  },
  modelValue: {
    type: Object,
    default: () => ({}),
  },
});

const emit = defineEmits(["update:modelValue"]);

const componentMap = {
  boolean: SettingBoolean,
  number: SettingNumber,
  text: SettingText,
  color: SettingColor,
  select: SettingSelect,
};

function componentFor(kind) {
  return componentMap[kind] || SettingText;
}

function updateField(key, value) {
  emit("update:modelValue", {
    ...props.modelValue,
    [key]: value,
  });
}

const fieldsWithComponent = computed(() =>
  props.fields.map((field) => ({
    ...field,
    _component: componentFor(field.kind),
  }))
);
</script>

<template>
  <div class="settings-panel">
    <div v-if="fieldsWithComponent.length" class="settings-list">
      <component v-for="field in fieldsWithComponent" :key="field.key" :is="field._component" :field="field"
        :model-value="modelValue[field.key]" @update:modelValue="(value) => updateField(field.key, value)" />
    </div>

    <div v-else class="settings-empty">
      <div class="settings-empty-title">Параметры отсутствуют</div>
      <div class="settings-empty-text">У этого плагина нет настраиваемых опций.</div>
    </div>
  </div>
</template>
