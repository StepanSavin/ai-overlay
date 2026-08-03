<script setup lang="ts">
import { useHideOnEscape } from "@/features/overlay-visibility";
import { TOGGLE_HOTKEY_LABEL } from "@/shared/config";
import { UiKbd } from "@/shared/ui/kbd";
import { OverlayPanel } from "@/widgets/overlay-panel";

useHideOnEscape();
</script>

<template>
  <main class="overlay-page">
    <OverlayPanel>
      <h1 class="overlay-page__title">AI Overlay</h1>
      <p class="overlay-page__hint">
        <UiKbd>{{ TOGGLE_HOTKEY_LABEL }}</UiKbd>
        — показать или скрыть,
        <UiKbd>Esc</UiKbd>
        — скрыть
      </p>
    </OverlayPanel>
  </main>
</template>

<style scoped>
.overlay-page {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  padding: 32px;
}

.overlay-page__title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  letter-spacing: -0.01em;
}

.overlay-page__hint {
  margin: 0;
  color: var(--overlay-text-muted);
  font-size: 13px;
}
</style>
