<script setup>
const props = defineProps({
  field: {
    type: Object,
    required: true,
  },
  modelValue: {
    type: [String, Number, Object],
    default: "",
  },
});

const emit = defineEmits(["update:modelValue"]);

function updateValue(event) {
  const raw = event.target.value;
  const option = (props.field.options || []).find(
    (item) => String(item.value) === raw
  );
  emit("update:modelValue", option ? option.value : raw);
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
      <div class="select-wrap">
        <select class="select-input" :value="modelValue" @change="updateValue">
          <option v-for="option in field.options || []" :key="option.label" :value="option.value">
            {{ option.label }}
          </option>
        </select>
        <span class="select-caret"></span>
      </div>
    </div>
  </div>
</template>
