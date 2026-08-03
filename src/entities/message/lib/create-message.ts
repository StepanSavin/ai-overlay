import type { Message } from '../model/types';

let lastLocalId = 0;

export const createMessage = (text: string): Message => {
  lastLocalId += 1;

  return {
    id: `m${lastLocalId}`,
    text,
    createdAt: Date.now(),
  };
};
