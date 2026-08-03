import { wsClient, type Unsubscribe } from '@/shared/api';

import { createMessage } from '../lib';
import type { Message } from '../model/types';

class MessageService {
  onMessage = (listener: (message: Message) => void): Unsubscribe =>
    wsClient.onText((text) => listener(createMessage(text)));
}

export const messageService = new MessageService();
