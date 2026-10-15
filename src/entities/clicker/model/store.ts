import { defineStore } from 'pinia';
import { ref } from 'vue';

import { clickerService } from '../api/service';

export const useClickerStore = defineStore('clicker', () => {
  const isActive = ref(false);

  const setActive = (next: boolean): void => {
    isActive.value = next;
  };

  void clickerService.onStateChange(setActive);
  void clickerService.fetchState().then(setActive);

  return {
    isActive,
  };
});
