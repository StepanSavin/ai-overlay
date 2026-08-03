import { WS_URL } from '@/shared/constants';

export type WsStatus = 'idle' | 'connecting' | 'open' | 'closed' | 'error';

export type Unsubscribe = () => void;

type TextListener = (text: string) => void;
type StatusListener = (status: WsStatus) => void;

export class WsClient {
  private readonly url: string;
  private readonly textListeners = new Set<TextListener>();
  private readonly statusListeners = new Set<StatusListener>();
  private socket: WebSocket | null = null;
  private currentStatus: WsStatus = 'idle';

  constructor(url: string) {
    this.url = url;
  }

  get status(): WsStatus {
    return this.currentStatus;
  }

  connect = (): void => {
    const readyState = this.socket?.readyState;
    if (readyState === WebSocket.CONNECTING || readyState === WebSocket.OPEN) return;

    this.setStatus('connecting');

    const socket = new WebSocket(this.url);
    this.socket = socket;

    socket.onopen = () => this.setStatus('open');

    socket.onmessage = (event: MessageEvent<unknown>) => {
      const { data } = event;
      if (typeof data !== 'string') return;

      this.textListeners.forEach((listener) => listener(data));
    };

    socket.onerror = () => this.setStatus('error');

    socket.onclose = () => {
      if (this.socket === socket) this.socket = null;
      this.setStatus('closed');
    };
  };

  disconnect = (): void => {
    const socket = this.socket;
    this.socket = null;

    if (socket) {
      socket.close();
      this.setStatus('closed');
    }
  };

  send = (text: string): boolean => {
    if (this.socket?.readyState !== WebSocket.OPEN) return false;

    this.socket.send(text);
    return true;
  };

  onText = (listener: TextListener): Unsubscribe => {
    this.textListeners.add(listener);

    return () => {
      this.textListeners.delete(listener);
    };
  };

  onStatusChange = (listener: StatusListener): Unsubscribe => {
    this.statusListeners.add(listener);

    return () => {
      this.statusListeners.delete(listener);
    };
  };

  private setStatus = (next: WsStatus): void => {
    if (this.currentStatus === next) return;

    this.currentStatus = next;
    this.statusListeners.forEach((listener) => listener(next));
  };
}

export const wsClient = new WsClient(WS_URL);
