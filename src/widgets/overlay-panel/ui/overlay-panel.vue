<template>
  <section class="overlay-panel">
    <slot />
  </section>
</template>

<style scoped>
.overlay-panel {
  display: flex;
  flex-direction: column;
  gap: var(--overlay-gap);
  padding: 22px 26px;
  border: 1px solid var(--overlay-border);
  border-radius: var(--overlay-radius);
  background: var(--overlay-surface);
  /* Native window shadow is off (it clips on transparent windows), so draw our own. */
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.45);
  backdrop-filter: blur(24px);
}
</style>
