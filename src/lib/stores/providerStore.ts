import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Provider } from '../types/provider';

export interface CreateProviderRequest {
  name: string;
  base_url: string;
  api_key: string;
  remark?: string;
}

export interface UpdateProviderRequest {
  id: string;
  name: string;
  base_url: string;
  api_key?: string;
  remark?: string;
}

export const providers = writable<Provider[]>([]);
export const selectedId = writable<string | null>(null);
export const loading = writable<boolean>(false);
export const editing = writable<boolean>(false);
export const creating = writable<boolean>(false);
export const searchQuery = writable<string>('');

export const selectedProvider = derived(
  [providers, selectedId],
  ([$providers, $selectedId]) => $providers.find(p => p.id === $selectedId) || null
);

export const filteredProviders = derived(
  [providers, searchQuery],
  ([$providers, $searchQuery]) => 
    $searchQuery
      ? $providers.filter(p => 
          p.name.toLowerCase().includes($searchQuery.toLowerCase()) ||
          p.base_url.toLowerCase().includes($searchQuery.toLowerCase()) ||
          (p.remark?.toLowerCase().includes($searchQuery.toLowerCase()) ?? false)
        )
      : $providers
);

export const providerStore = {
  async load() {
    loading.set(true);
    try {
      const data = await invoke<Provider[]>('list_providers');
      providers.set(data);
    } catch (e) {
      console.error('加载失败:', e);
    } finally {
      loading.set(false);
    }
  },

  select(id: string | null) {
    selectedId.set(id);
    editing.set(false);
  },

  startEdit() { editing.set(true); },
  cancelEdit() { editing.set(false); },
  startCreate() { creating.set(true); selectedId.set(null); editing.set(false); },
  cancelCreate() { creating.set(false); },

  setSearchQuery(q: string) { searchQuery.set(q); },

  async create(req: CreateProviderRequest) {
    const newProvider = await invoke<Provider>('create_provider', {
      request: {
        name: req.name,
        base_url: req.base_url,
        api_key: req.api_key,
        remark: req.remark || null
      }
    });
    providers.update(p => [...p, newProvider]);
    selectedId.set(newProvider.id);
    creating.set(false);
  },

  async update(req: UpdateProviderRequest) {
    const updated = await invoke<Provider>('update_provider', {
      request: {
        id: req.id,
        name: req.name,
        base_url: req.base_url,
        api_key: req.api_key || null,
        remark: req.remark || null
      }
    });
    providers.update(list => list.map(p => p.id === updated.id ? updated : p));
    editing.set(false);
  },

  async delete(id: string) {
    await invoke('delete_provider', { id });
    providers.update(list => list.filter(p => p.id !== id));
    selectedId.update(current => current === id ? null : current);
  },

  async getApiKey(id: string): Promise<string> {
    return invoke<string>('get_api_key', { id });
  },

  async copyToClipboard(text: string): Promise<void> {
    return invoke('copy_to_clipboard', { text });
  },
};
