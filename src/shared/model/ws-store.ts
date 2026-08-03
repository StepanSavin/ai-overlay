import { defineStore } from 'pinia';
import { computed, ref } from 'vue';

import { wsClient, type WsStatus } from '@/shared/api';

export const useWsStore = defineStore('ws', () => {
  const status = ref<WsStatus>(wsClient.status);

  const isConnected = computed(() => status.value === 'open');

  wsClient.onStatusChange((next) => {
    status.value = next;
  });

  const connect = (): void => wsClient.connect();

  const disconnect = (): void => wsClient.disconnect();

  const send = (text: string): boolean => {
    const trimmed = text.trim();

    return trimmed ? wsClient.send(trimmed) : false;
  };

  return {
    status,
    isConnected,
    connect,
    disconnect,
    send,
  };
});
