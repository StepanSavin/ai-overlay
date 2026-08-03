import { onBeforeUnmount, onMounted } from "vue";

import { hideOverlayWindow } from "@/shared/lib/tauri";

export function useHideOnEscape(): void {
  function handleKeydown(event: KeyboardEvent): void {
    if (event.key !== "Escape" || event.defaultPrevented) return;

    event.preventDefault();
    void hideOverlayWindow();
  }

  onMounted(() => window.addEventListener("keydown", handleKeydown));
  onBeforeUnmount(() => window.removeEventListener("keydown", handleKeydown));
}
