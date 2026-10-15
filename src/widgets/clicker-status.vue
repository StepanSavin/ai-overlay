<script setup lang="ts">
import { useClickerStore } from '@/entities/clicker';
import { CLICKER_HOTKEY } from '@/shared/constants';
import { UiKbd } from '@/shared/ui';

const clickerStore = useClickerStore();
</script>

<template>
  <section class="clicker-status" :class="{ 'clicker-status--active': clickerStore.isActive }">
    <span class="clicker-status__dot" />
    <span class="clicker-status__label">
      {{ clickerStore.isActive ? 'Автоклик включён' : 'Автоклик выключен' }}
    </span>
    <span class="clicker-status__hotkey">
      <UiKbd v-for="key in CLICKER_HOTKEY" :key="key">{{ key }}</UiKbd>
    </span>
  </section>
</template>

<style scoped>
.clicker-status {
  display: flex;
  align-items: center;
  gap: 10px;
  height: 100%;
  padding: 0 14px;
  border: 1px solid var(--overlay-border);
  border-radius: 999px;
  background: var(--overlay-surface);
  backdrop-filter: blur(24px);
  font-size: 13px;
  user-select: none;
}

.clicker-status__dot {
  flex: none;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--overlay-text-muted);
}

.clicker-status--active .clicker-status__dot {
  background: var(--overlay-accent);
  box-shadow: 0 0 0 0 var(--overlay-accent);
  animation: clicker-pulse 1s ease-out infinite;
}

.clicker-status__label {
  flex: 1;
  white-space: nowrap;
}

.clicker-status__hotkey {
  display: flex;
  gap: 4px;
  color: var(--overlay-text-muted);
}

@keyframes clicker-pulse {
  to {
    box-shadow: 0 0 0 6px transparent;
  }
}
</style>
