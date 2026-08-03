export const isApple = /Mac/.test(navigator.userAgent);

export const TOGGLE_HOTKEY_LABEL = isApple ? '⌘ ⇧ Space' : 'Win ⇧ Space';

export const WS_URL = 'wss://ws.postman-echo.com/raw';
