<script setup>
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";

const props = defineProps({
  logs: {
    type: Array,
    default: () => [],
  },
  status: {
    type: Object,
    default: () => ({ ok: true, message: "" }),
  },
});

const bodyRef = ref(null);
const panelRef = ref(null);
const isResizing = ref(false);
const currentHeight = ref(null);
let startY = 0;
let startHeight = 0;

const BASE_WINDOW_HEIGHT = 720;
const MIN_RATIO = 220 / BASE_WINDOW_HEIGHT;
let minHeight = 220;

function maxHeight() {
  return Math.max(minHeight, Math.floor(window.innerHeight * 0.7));
}

function clampHeight(value) {
  return Math.min(maxHeight(), Math.max(minHeight, value));
}

function setConsoleHeight(value) {
  const clamped = clampHeight(value);
  currentHeight.value = clamped;
  document.documentElement.style.setProperty(
    "--console-height",
    `${clamped}px`
  );
}

function onPointerMove(event) {
  if (!isResizing.value) return;
  const delta = startY - event.clientY;
  setConsoleHeight(startHeight + delta);
}

function stopResize() {
  if (!isResizing.value) return;
  isResizing.value = false;
  document.body.style.userSelect = "";
  window.removeEventListener("pointermove", onPointerMove);
  window.removeEventListener("pointerup", stopResize);
}

function startResize(event) {
  if (!panelRef.value) return;
  event.preventDefault();
  event.stopPropagation();
  isResizing.value = true;
  startY = event.clientY;
  startHeight = panelRef.value.getBoundingClientRect().height;
  document.body.style.userSelect = "none";
  window.addEventListener("pointermove", onPointerMove);
  window.addEventListener("pointerup", stopResize);
}

function handleWindowResize() {
  minHeight = Math.max(180, Math.floor(window.innerHeight * MIN_RATIO));
  if (currentHeight.value == null) return;
  setConsoleHeight(currentHeight.value);
}

onMounted(() => {
  minHeight = Math.max(180, Math.floor(window.innerHeight * MIN_RATIO));
  const initial = parseFloat(
    getComputedStyle(document.documentElement).getPropertyValue(
      "--console-height"
    )
  );

  if (!Number.isNaN(initial) && initial > 0) {
    currentHeight.value = initial;
  }

  window.addEventListener("resize", handleWindowResize);
});

onBeforeUnmount(() => {
  stopResize();
  window.removeEventListener("resize", handleWindowResize);
});

watch(
  () => props.logs.length,
  async (length) => {
    await nextTick();
    if (!bodyRef.value) return;
    if (length === 0) {
      bodyRef.value.scrollTo({ top: 0 });
      return;
    }
    bodyRef.value.scrollTo({
      top: bodyRef.value.scrollHeight,
      behavior: "smooth",
    });
  }
);
</script>

<template>
  <section ref="panelRef" class="console-panel" :class="{ resizing: isResizing }">
    <div class="console-header">
      <span class="console-title">
        <span class="icon" aria-hidden="true">
          <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
            <path d="M4 7h16M4 12h12M4 17h8" stroke="currentColor" stroke-width="2" stroke-linecap="round"
              stroke-linejoin="round" />
          </svg>
        </span>
        Центр выполнения
      </span>
      <div class="console-resize-handle" @pointerdown="startResize" title="Изменить высоту"></div>
      <span class="console-status" :class="{ error: !status.ok }">
        {{ status.message || "Ожидание" }}
      </span>
    </div>
    <div ref="bodyRef" class="console-body">
      <div v-if="logs.length" class="console-logs">
        <div v-for="(entry, index) in logs" :key="index" class="console-line" :class="entry.level"
          :style="{ '--indent': `${(entry.indent || 0) * 14}px` }">
          <span class="console-time">{{ entry.time }}</span>
          <div class="console-content">
            <div class="console-message">{{ entry.message }}</div>
            <div v-if="entry.detail" class="console-detail">{{ entry.detail }}</div>
          </div>
        </div>
      </div>
      <div v-else class="muted">Логи появятся после запуска.</div>
    </div>
  </section>
</template>
