<script setup lang="ts">
import { onMounted, ref } from 'vue';

import { splitIntoWords, useMessageStore } from '@/entities/message';
import { WORD_SEND_INTERVAL_MS } from '@/shared/constants';
import { delay } from '@/shared/lib';
import { useWsStore } from '@/shared/model';
import { UiKbd } from '@/shared/ui';

const wsStore = useWsStore();
const messageStore = useMessageStore();

const draft = ref('');
const isSending = ref(false);

onMounted(() => {
  wsStore.connect();
});

const submit = async (): Promise<void> => {
  const words = splitIntoWords(draft.value);
  if (!words.length || !wsStore.isConnected || isSending.value) return;

  draft.value = '';
  isSending.value = true;
  messageStore.startMessage();

  for (const word of words) {
    wsStore.send(word);
    await delay(WORD_SEND_INTERVAL_MS);
  }

  isSending.value = false;
};
</script>

<template>
  <section class="chat">
    <header class="chat__header">
      <h1 class="chat__title">AI Overlay</h1>
      <span class="chat__status">{{ wsStore.status }}</span>
    </header>

    <ul v-if="messageStore.messages.length" class="chat__list">
      <li v-for="message in messageStore.messages" :key="message.id" class="chat__message">
        {{ message.text || '…' }}
      </li>
    </ul>
    <p v-else class="chat__empty">Отправь сообщение, оно вернётся эхом, слово за словом</p>

    <form class="chat__form" @submit.prevent="submit">
      <input
        v-model="draft"
        class="chat__input"
        type="text"
        placeholder="Сообщение"
        :disabled="!wsStore.isConnected"
      />
      <button
        class="chat__submit"
        type="submit"
        :disabled="!wsStore.isConnected || isSending"
      >
        Отправить
      </button>
    </form>

    <footer class="chat__footer"><UiKbd>Esc</UiKbd> — скрыть</footer>
  </section>
</template>

<style scoped>
.chat {
  display: flex;
  flex-direction: column;
  gap: var(--overlay-gap);
  height: 100%;
  padding: 18px 22px;
  border: 1px solid var(--overlay-border);
  border-radius: var(--overlay-radius);
  background: var(--overlay-surface);
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.45);
  backdrop-filter: blur(24px);
}

.chat__header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 12px;
}

.chat__title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  letter-spacing: -0.01em;
}

.chat__status {
  color: var(--overlay-text-muted);
  font-size: 12px;
}

.chat__list {
  display: flex;
  flex: 1;
  flex-direction: column;
  gap: 6px;
  margin: 0;
  padding: 0;
  overflow-y: auto;
  list-style: none;
}

.chat__message {
  align-self: flex-start;
  max-width: 100%;
  padding: 7px 11px;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.07);
  overflow-wrap: anywhere;
}

.chat__empty {
  flex: 1;
  margin: 0;
  color: var(--overlay-text-muted);
  font-size: 13px;
}

.chat__form {
  display: flex;
  gap: 8px;
}

.chat__input {
  flex: 1;
  min-width: 0;
  padding: 9px 12px;
  border: 1px solid var(--overlay-border);
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.06);
  color: inherit;
  font: inherit;
  outline: none;
}

.chat__input:focus-visible {
  border-color: rgba(255, 255, 255, 0.3);
}

.chat__submit {
  padding: 9px 14px;
  border: 1px solid var(--overlay-border);
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.12);
  color: inherit;
  font: inherit;
  cursor: pointer;
}

.chat__submit:disabled {
  opacity: 0.45;
  cursor: default;
}

.chat__footer {
  color: var(--overlay-text-muted);
  font-size: 12px;
}
</style>
