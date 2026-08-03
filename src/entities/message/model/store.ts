import { defineStore } from 'pinia';
import { computed, ref } from 'vue';

import { wsClient, type WsStatus } from '@/shared/api';

import { messageService } from '../api/service';
import type { Message } from './types';

export const useMessageStore = defineStore('message', () => {
  const messages = ref<Message[]>([]);
  const status = ref<WsStatus>(wsClient.status);

  const isConnected = computed(() => status.value === 'open');
  const lastMessage = computed<Message | null>(
    () => messages.value[messages.value.length - 1] ?? null,
  );

  messageService.onMessage((message) => {
    messages.value.push(message);
  });

  wsClient.onStatusChange((next) => {
    status.value = next;
  });

  const clear = (): void => {
    messages.value = [];
  };

  return {
    messages,
    status,
    isConnected,
    lastMessage,
    clear,
  };
});
