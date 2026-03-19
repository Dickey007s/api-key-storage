import { writable } from 'svelte/store';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

interface Toast {
  id: string;
  type: ToastType;
  message: string;
}

export const toasts = writable<Toast[]>([]);

export const toastStore = {
  show(message: string, type: ToastType = 'success') {
    const id = crypto.randomUUID();
    toasts.update(list => [...list, { id, type, message }]);
    setTimeout(() => {
      toasts.update(list => list.filter(t => t.id !== id));
    }, 2000);
  },

  success(message: string) { this.show(message, 'success'); },
  error(message: string) { this.show(message, 'error'); },
};