<script setup>
const props = defineProps({
  field: {
    type: Object,
    required: true,
  },
  modelValue: {
    type: [String, Number],
    default: "",
  },
});

const emit = defineEmits(["update:modelValue"]);

function updateValue(event) {
  emit("update:modelValue", event.target.value);
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
      <textarea v-if="field.ui?.rows && field.ui.rows > 1" :rows="field.ui.rows"
        :placeholder="field.ui?.placeholder || ''" :value="modelValue" @input="updateValue"></textarea>
      <input v-else type="text" :placeholder="field.ui?.placeholder || ''" :value="modelValue" @input="updateValue" />
    </div>
  </div>
</template>
