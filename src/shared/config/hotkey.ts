const isApple = /Mac/.test(navigator.userAgent);

export const TOGGLE_HOTKEY_LABEL = isApple ? "⌘ ⇧ Space" : "Win ⇧ Space";
