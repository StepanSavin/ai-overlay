<template>
  <kbd class="ui-kbd"><slot /></kbd>
</template>

<style scoped>
.ui-kbd {
  display: inline-block;
  padding: 2px 7px;
  border: 1px solid var(--overlay-border);
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.06);
  color: var(--overlay-text);
  font: inherit;
  font-size: 0.85em;
  white-space: nowrap;
}
</style>
