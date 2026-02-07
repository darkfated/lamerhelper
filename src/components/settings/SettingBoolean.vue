<script setup>
const props = defineProps({
  field: {
    type: Object,
    required: true,
  },
  modelValue: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(["update:modelValue"]);

function updateValue(event) {
  emit("update:modelValue", event.target.checked);
}
</script>

<template>
  <div class="setting-item">
    <div class="setting-info">
      <div class="setting-label">
        {{ field.label }}
        <span v-if="field.required" class="required">*</span>
        <span v-if="field.ui?.danger" class="danger-tag">Осторожно</span>
      </div>
      <div v-if="field.description" class="setting-desc">{{ field.description }}</div>
    </div>
    <div class="setting-action">
      <label class="switch">
        <input type="checkbox" :checked="modelValue" @change="updateValue" />
        <span class="slider"></span>
      </label>
    </div>
  </div>
</template>
