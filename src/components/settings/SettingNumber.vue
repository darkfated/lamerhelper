<script setup>
const props = defineProps({
  field: {
    type: Object,
    required: true,
  },
  modelValue: {
    type: [Number, String],
    default: 0,
  },
});

const emit = defineEmits(["update:modelValue"]);

function updateValue(event) {
  const value = event.target.value;
  emit("update:modelValue", value === "" ? "" : Number(value));
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
      <div class="field-inline">
        <div class="number-wrap">
          <input class="number-input" type="number" :value="modelValue" :min="field.ui?.min ?? null"
            :max="field.ui?.max ?? null" :step="field.ui?.step ?? 1" @input="updateValue" />
          <div class="number-stepper">
            <button type="button" class="step-btn up" aria-label="Increase"
              @click="updateValue({ target: { value: Number(modelValue || 0) + (field.ui?.step ?? 1) } })"></button>
            <button type="button" class="step-btn down" aria-label="Decrease"
              @click="updateValue({ target: { value: Number(modelValue || 0) - (field.ui?.step ?? 1) } })"></button>
          </div>
        </div>
        <span v-if="field.ui?.unit" class="setting-unit">{{ field.ui.unit }}</span>
      </div>
    </div>
  </div>
</template>
