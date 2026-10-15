import { isTauri } from '@/shared/lib';

const CLICKER_EVENT = 'clicker-state';

type Unsubscribe = () => void;

class ClickerService {
  fetchState = async (): Promise<boolean> => {
    if (!isTauri) return false;

    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<boolean>('clicker_state');
  };

  onStateChange = async (listener: (isActive: boolean) => void): Promise<Unsubscribe> => {
    if (!isTauri) return () => {};

    const { listen } = await import('@tauri-apps/api/event');
    return listen<boolean>(CLICKER_EVENT, (event) => listener(event.payload));
  };
}

export const clickerService = new ClickerService();
