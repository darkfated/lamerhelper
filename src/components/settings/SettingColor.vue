<script setup>
import { computed } from "vue";

const props = defineProps({
  field: {
    type: Object,
    required: true,
  },
  modelValue: {
    type: String,
    default: "#000000",
  },
});

const emit = defineEmits(["update:modelValue"]);

const safeValue = computed(() => {
  const value = props.modelValue || "";
  return /^#([0-9a-fA-F]{6})$/.test(value) ? value : "#000000";
});

const textValue = computed(() => props.modelValue || safeValue.value);

function updateColor(event) {
  emit("update:modelValue", event.target.value);
}

function updateText(event) {
  let value = event.target.value.trim();
  if (!value) {
    emit("update:modelValue", "");
    return;
  }
  if (!value.startsWith("#")) {
    value = `#${value}`;
  }
  emit("update:modelValue", value);
}
</script>

<template>
  <div class="setting-item">
    <div class="setting-info">
      <div class="setting-label">
        {{ field.label }}
        <span v-if="field.required" class="required">*</span>
      </div>
      <div v-if="field.description" class="setting-desc">{{ field.description }}</div>
    </div>
    <div class="setting-action">
      <div class="color-picker">
        <label class="color-swatch" :style="{ background: safeValue }">
          <input type="color" :value="safeValue" @input="updateColor" />
        </label>
        <input class="color-hex" type="text" :placeholder="field.ui?.placeholder || '#RRGGBB'" :value="textValue"
          @input="updateText" />
      </div>
    </div>
  </div>
</template>
