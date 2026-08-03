import { defineStore } from 'pinia';
import { ref } from 'vue';

import { messageService } from '../api/service';
import { createMessage } from '../lib';
import type { Message } from './types';

export const useMessageStore = defineStore('message', () => {
  const messages = ref<Message[]>([]);

  messageService.onMessage((chunk) => {
    const current = messages.value[messages.value.length - 1];
    if (!current) return;

    current.text = current.text ? `${current.text} ${chunk}` : chunk;
  });

  const startMessage = (): void => {
    messages.value.push(createMessage(''));
  };

  const clear = (): void => {
    messages.value = [];
  };

  return {
    messages,
    startMessage,
    clear,
  };
});
