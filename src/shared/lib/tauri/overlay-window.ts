export const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

export async function hideOverlayWindow(): Promise<void> {
  if (!isTauri) return;

  const { getCurrentWindow } = await import('@tauri-apps/api/window');
  await getCurrentWindow().hide();
}
