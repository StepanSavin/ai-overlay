import { defineStore } from 'pinia';
import { computed, ref } from 'vue';

import { messageService } from '../api/service';
import type { Message } from './types';

export const useMessageStore = defineStore('message', () => {
  const messages = ref<Message[]>([]);

  const lastMessage = computed<Message | null>(
    () => messages.value[messages.value.length - 1] ?? null,
  );

  messageService.onMessage((message) => {
    messages.value.push(message);
  });

  const clear = (): void => {
    messages.value = [];
  };

  return {
    messages,
    lastMessage,
    clear,
  };
});
