import { wsClient, type Unsubscribe } from '@/shared/api';

class MessageService {
  onMessage = (listener: (chunk: string) => void): Unsubscribe => wsClient.onText(listener);
}

export const messageService = new MessageService();
