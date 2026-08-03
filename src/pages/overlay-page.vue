<script setup lang="ts">
import { TOGGLE_HOTKEY_LABEL } from '@/shared/constants';
import { useHideOnEscape } from '@/shared/lib';
import { UiKbd } from '@/shared/ui';

useHideOnEscape();
</script>

<template>
  <main class="overlay-page">
    <section class="overlay-panel">
      <h1 class="overlay-page__title">AI Overlay</h1>
      <p class="overlay-page__hint">
        <UiKbd>{{ TOGGLE_HOTKEY_LABEL }}</UiKbd>
        — показать или скрыть,
        <UiKbd>Esc</UiKbd>
        — скрыть
      </p>
    </section>
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

.overlay-panel {
  display: flex;
  flex-direction: column;
  gap: var(--overlay-gap);
  padding: 22px 26px;
  border: 1px solid var(--overlay-border);
  border-radius: var(--overlay-radius);
  background: var(--overlay-surface);
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.45);
  backdrop-filter: blur(24px);
}
</style>
