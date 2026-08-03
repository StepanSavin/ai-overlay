<script setup lang="ts">
import { useHideOnEscape } from '@/shared/lib';
import { Chat } from '@/widgets';

useHideOnEscape();
</script>

<template>
  <main class="overlay-page">
    <Chat />
  </main>
</template>

<style scoped>
.overlay-page {
  height: 100%;
  padding: 16px;
}
</style>
